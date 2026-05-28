//! Compressor algorithm taken from cy!box, which is based on CALF's compressor.
//! https://gitlab.com/cyphers-stuff/cybox/-/blob/31c2eda59748f321a09141b41552d8e65a755dfe/dsp/beepbox/src/effect/compressor.rs

use std::{f32, iter::zip};

use wasm_bindgen::prelude::*;

use crate::{
    SamplePair,
    compressor::{
        comp::{Compressor, CompressorParams},
        filters::{Crossover, CrossoverCoefficients, to_w0},
    },
};
mod comp;
mod filters;

#[wasm_bindgen]
#[derive(Default, Clone, Copy, Debug)]
struct CompressorInstanceParams {
    pub attack: f32,
    pub decay: f32,
    pub threshold: f32,

    pub ratio_up: f32,
    pub ratio_down: f32,

    pub freq_lo_mid: f32,
    pub freq_mid_hi: f32,

    pub lo_gain: f32,
    pub mid_gain: f32,
    pub hi_gain: f32,
}
#[wasm_bindgen]
impl CompressorInstanceParams {
    fn comp_params(&self, sample_rate: f32) -> CompressorParams {
        let mut params = CompressorParams::new(sample_rate);
        params.attack = self.attack;
        params.decay = self.decay;
        params.threshold = self.threshold;
        params.ratio_up = self.ratio_up;
        params.ratio_down = self.ratio_down;
        params
    }
}

#[wasm_bindgen]
struct CompressorInstance {
    pub frame_size: usize,
    pub start: CompressorInstanceParams,
    pub end: CompressorInstanceParams,

    split_lo_mid: Crossover,
    split_mid_hi: Crossover,

    lo: Compressor,
    mid: Compressor,
    hi: Compressor,

    buffer: Box<[f32]>,
}

macro_rules! interpolate {
    ($run_length:expr, $start:expr, $end:expr) => {{
        let (mut start, end) = ($start, $end);
        let diff = end.zip(&start, |x, y| (x - y) / $run_length as f32);
        std::iter::repeat_with(move || {
            let new = start.zip(&diff, |x, y| x + y);
            std::mem::replace(&mut start, new)
        })
    }};
}
fn interpolate_gain(run_length: usize, mut start: f32, end: f32) -> impl Iterator<Item = f32> {
    let diff = (end - start) / run_length as f32;
    std::iter::repeat_with(move || {
        let new = start + diff;
        std::mem::replace(&mut start, new)
    })
}

#[wasm_bindgen]
impl CompressorInstance {
    #[wasm_bindgen(constructor)]
    pub fn new(frame_size: usize) -> Self {
        Self {
            frame_size,
            split_lo_mid: Default::default(),
            split_mid_hi: Default::default(),

            lo: Default::default(),
            mid: Default::default(),
            hi: Default::default(),

            end: Default::default(),
            start: Default::default(),

            buffer: vec![0.0; frame_size * 2].into_boxed_slice(),
        }
    }

    #[wasm_bindgen]
    pub fn get_buffer(&mut self) -> js_sys::Float32Array {
        unsafe { js_sys::Float32Array::view(&self.buffer) }
    }

    #[wasm_bindgen]
    pub fn process(&mut self, sample_rate: f32, run_length: usize) {
        let Self {
            frame_size,
            ref start,
            ref end,
            ..
        } = *self;

        if start.freq_lo_mid < 10.0 {
            // compressor hasn't been initialized yet; ignore
            return;
        }

        let (left, right) = self.buffer.split_at_mut(frame_size);

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(msg: String);
        }

        let coef_lo_mid = CrossoverCoefficients::new(to_w0(start.freq_lo_mid, sample_rate));
        let coef_mid_hi = CrossoverCoefficients::new(to_w0(start.freq_mid_hi, sample_rate));
        let mut comp_params = interpolate!(
            run_length,
            start.comp_params(sample_rate),
            end.comp_params(sample_rate)
        );

        let mut lo_mult = interpolate_gain(run_length, start.lo_gain, end.lo_gain);
        let mut mid_mult = interpolate_gain(run_length, start.mid_gain, end.mid_gain);
        let mut hi_mult = interpolate_gain(run_length, start.hi_gain, end.hi_gain);

        for (l, r) in zip(&mut left[..run_length], &mut right[..run_length]) {
            let [mut lo, mut mid, mut hi] = [SamplePair { l: *l, r: *r }; 3];

            self.split_mid_hi.run(&coef_mid_hi, &mut mid, &mut hi);
            self.split_lo_mid.run(&coef_lo_mid, &mut lo, &mut mid);

            let cur_comp_params = comp_params.next().unwrap();

            let sample = self.lo.process(&cur_comp_params, lo) * lo_mult.next().unwrap()
                + self.mid.process(&cur_comp_params, mid) * mid_mult.next().unwrap()
                + self.hi.process(&cur_comp_params, hi) * hi_mult.next().unwrap();

            *l = sample.l.clamp(-1.0, 1.0);
            *r = sample.r.clamp(-1.0, 1.0);
        }
    }
}
