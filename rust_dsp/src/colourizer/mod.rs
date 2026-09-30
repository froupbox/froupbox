//! https://en.wiktionary.org/wiki/colorize#English
//! > "colourize (Canada, Oxford British English)"

use std::iter::zip;

use crate::{
    SamplePair,
    buffer::DspBuffer,
    flanger::{Flanger, FlangerParams},
    util::{self, Interpolator},
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ColourizerInstanceParams {
    pub mix: f32,
    pub voices: f32,
    freqs: Vec<f32>,
}
#[wasm_bindgen]
impl ColourizerInstanceParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }
    #[wasm_bindgen(setter)]
    pub fn set_freqs(&mut self, freqs: Vec<f32>) {
        self.freqs = freqs;
    }
}
impl ColourizerInstanceParams {
    fn mix(&self) -> f32 {
        self.mix * (1.0 / 63.0)
    }
    fn total_scale(&self, num_flangers: usize) -> f32 {
        // this is the default calculation that i implemented
        1.0 / (num_flangers as f32)
        // you can do inverse square root
        // 1.0 / (num_flangers as f32).sqrt()
        // or the cube root
        // 1.0 / (num_flangers as f32).cbrt()
        // or anything, even taking into account self.voices!
        // 1.0 / (num_flangers as f32 * self.voices).powf(0.8)
    }
    fn for_freq(&self, sample_rate: f32, freq: f32) -> FlangerParams {
        FlangerParams {
            delay: sample_rate / freq,
            mix: self.mix(),
            feedmix: 0.0,
            voices: self.voices,
        }
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct ColourizerInstance {
    flangers: Vec<ColourizerFlanger>,
    cur_num_active_flangers: usize,

    pub use_legacy_mix: bool,

    output_buf: DspBuffer,
    mix_interp: Interpolator<f32>,
    total_scale_interp: Interpolator<f32>,
}

#[derive(Default)]
struct ColourizerFlanger {
    enabled: bool,
    i: Flanger<SamplePair>,
}

const FREQ_MIN: f32 = 20.0;

#[wasm_bindgen]
impl ColourizerInstance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen]
    pub fn begin(
        &mut self,
        start: ColourizerInstanceParams,
        end: ColourizerInstanceParams,
        sample_rate: f32,
        run_length: f32,
    ) {
        // don't shrink self.flangers; those are expensive to create and thus keep them in the pool
        if start.freqs.len() > self.flangers.len() {
            self.flangers
                .resize_with(start.freqs.len(), Default::default);
        }

        for flanger in &mut self.flangers {
            flanger.enabled = false;
        }
        for ((&freq_start, &freq_end), flanger) in
            zip(zip(&start.freqs, &end.freqs), &mut self.flangers)
        {
            if freq_start.min(freq_end) < FREQ_MIN {
                // freq either -1 (nonexistent) or too low
                continue;
            }
            flanger.enabled = true;
            let params_start = start.for_freq(sample_rate, freq_start);
            let params_end = end.for_freq(sample_rate, freq_end);

            let max_delay_samples = params_start.total_delay().max(params_end.total_delay());
            flanger.i.delay_line.reserve_at_least(max_delay_samples);

            flanger.i.interpolator = util::interpolate(run_length, params_start, params_end);
        }

        let num_flangers = self
            .flangers
            .iter_mut()
            .partition_in_place(|flanger| flanger.enabled);
        self.cur_num_active_flangers = num_flangers;

        self.mix_interp = util::interpolate(run_length, start.mix(), end.mix());
        self.total_scale_interp = util::interpolate(
            run_length,
            start.total_scale(num_flangers),
            end.total_scale(num_flangers),
        )
    }

    #[wasm_bindgen]
    pub fn process(&mut self, buffer: &mut DspBuffer) {
        if self.cur_num_active_flangers == 0 {
            buffer.clear();
            return;
        }

        if self.output_buf.frame_size() != buffer.frame_size() {
            self.output_buf = DspBuffer::new(buffer.frame_size());
        }
        self.output_buf.run_length = buffer.run_length;

        let flangers = &mut self.flangers[..self.cur_num_active_flangers];
        if cfg!(debug_assertions) {
            for flanger in &mut *flangers {
                debug_assert!(flanger.enabled);
            }
        }

        let total_scale = 1.0 / self.cur_num_active_flangers as f32;

        for ((input_l, input_r), (output_l, output_r)) in
            zip(buffer.as_zipped(), self.output_buf.as_zipped())
        {
            let wet = self.mix_interp.next();

            let input = SamplePair {
                l: *input_l,
                r: *input_r,
            };
            let mut output = SamplePair::ZERO;

            for flanger in &mut *flangers {
                output += flanger.i.process(input, true);
            }

            if self.use_legacy_mix {
                output += input * (1.0 - wet);
            } else {
                output *= total_scale;
            }

            *output_l = output.l;
            *output_r = output.r;
        }

        buffer.set(&mut self.output_buf);
    }
}
