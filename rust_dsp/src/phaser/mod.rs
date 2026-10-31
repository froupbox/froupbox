// use core::simd::{cmp::SimdPartialOrd as _, f32x4, num::SimdFloat as _};
use core::simd::f32x4;
use std::{fmt::Debug, iter::zip};

use wasm_bindgen::prelude::*;

use crate::{
    filters::AngularFrequency,
    util::{self, Interpolator, Zippable},
};

mod bipole;
mod legacy;
mod unipole;

#[wasm_bindgen]
#[derive(Default, Clone, Copy)]
struct PhaserInstanceParams {
    pub freq: f32,
    pub q: f32,
    pub mix: f32,
    pub feedback: f32,
}
#[wasm_bindgen]
impl PhaserInstanceParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }
}

trait PhaserAlgorithm {
    fn begin(
        &mut self,
        start: PhaserInstanceParams,
        end: PhaserInstanceParams,
        sample_rate: f32,
        run_length: f32,
    );
    fn resize(&mut self, num_stages: usize);
    // returns (dry, wet)
    fn compute(&mut self, sample: f32) -> (f32, f32);
}

trait SimdPhaserStage: Clone + Copy + Default {
    type Coefficients: Zippable + Debug;
    type SimdCoefficients;

    fn calculate_coefficients(w0: AngularFrequency, q: f32) -> Self::Coefficients;
    fn simd_coefficients_from_array(arr: &[Self::Coefficients; 4]) -> Self::SimdCoefficients;
    fn compute(&mut self, idx: usize, coef: &Self::Coefficients, input: &mut f32);
    fn compute_simd(&mut self, coef: &Self::SimdCoefficients, input: &mut f32x4);

    fn concat_rotate(&mut self, other: &mut Self);
}

/// SIMD-accelerated one-channel phaser.
#[derive(Default)]
struct SimdPhaserWrapper<S: SimdPhaserStage> {
    dry_simd: f32x4,
    wet_simd: f32x4,

    simd_index: u8,

    i_w0: Interpolator<AngularFrequency>,
    i_q: Interpolator<f32>,

    i_feedback_mult: Interpolator<f32>,

    // invariant: `self.stages.len() * 4 >= self.num_stages`
    num_stages: usize,
    // are_all_stages_zero: bool,
    stages: Vec<S>,
}
impl<S: SimdPhaserStage> SimdPhaserWrapper<S> {
    fn compute_one(&mut self, i: usize, coef: &S::Coefficients, val: &mut f32) {
        unsafe { self.stages.get_unchecked_mut(i / 4) }.compute(i % 4, coef, val);
    }

    fn next_coef(&mut self) -> S::Coefficients {
        S::calculate_coefficients(self.i_w0.next(), self.i_q.next())
    }

    // fn compute_simd(&mut self) {
    //     fn is_silent(simd: f32x4) -> bool {
    //         const SILENCE_THRESHOLD: f32 = 1e-5;
    //         simd.abs().simd_le(f32x4::splat(SILENCE_THRESHOLD)).all()
    //     }

    //     let prev_sample = self.wet_simd;
    //     let mut sample = self.dry_simd + prev_sample * f32x4::splat(self.i_feedback_mult.next());
    //     if is_silent(sample) {
    //         sample = f32x4::splat(0.0);

    //         if is_silent(prev_sample) {
    //             self.wet_simd = f32x4::splat(0.0);

    //             if !self.are_all_stages_zero {
    //                 self.are_all_stages_zero = true;
    //                 // reset all stages to prevent denormals
    //                 for stage in &mut self.stages {
    //                     *stage = Default::default();
    //                 }
    //             }
    //             return;
    //         }
    //     } else {
    //         self.are_all_stages_zero = false;
    //     }
    //     self.compute_simd_inner(&mut sample);
    //     self.wet_simd = sample;
    // }

    // fn compute_simd_inner(&mut self, val_simd: &mut f32x4) {
    fn compute_simd(&mut self, val_simd: &mut f32x4) {
        let simd_len = self.num_stages / 4;
        assert!(self.stages.len() >= simd_len);

        let coef_array =
            std::array::from_fn(|_| S::calculate_coefficients(self.i_w0.next(), self.i_q.next()));
        let coef_simd = S::simd_coefficients_from_array(&coef_array);

        // arbitrary threshold below which simd isn't worth it
        if simd_len < 4 {
            for (val, coef) in zip(val_simd.as_mut_array(), &coef_array) {
                for i in 0..self.num_stages {
                    self.compute_one(i, coef, val);
                }
            }
            return;
        }

        // SOME people *cough cough* use hundreds of phaser stages on their instruments. i respect that but at that point might as well do computations in parallel to save cpu time. right?
        // anyways, this algorithm does that for a set of 4 mono samples. the gist is:
        // consider a sample being processed through some prefix of the phaser stages. in order for it to exist:
        // - the previous sample needs to be processed by the current phaser
        // - the previous phaser needs to have processed the current sample
        // so no simple "do it in parallel" approach here. instead, the solution is to stagger each batch of 4 samples:
        //
        //    s1 s2 s3 s4
        // p1 .. .. .. XX
        // p2 .. .. XX |
        // p3 .. XX |  V
        // p4 XX |  V
        // p5 |  V
        // p6 V
        // ...
        //
        // where `XX` represents the values stored by the f32x4 in `val_simd` and `..` represents values that were previously processed.

        // stagger the samples to prepare for the simd hot loop
        {
            for (i, (val, break_coef)) in zip(val_simd.as_mut_array(), &coef_array).enumerate() {
                for stage in 0..3 - i {
                    self.compute_one(stage, break_coef, val);
                }
            }
        }

        // reverse the value; this aligns the simd values with the stage values
        // because e.g. the first value needs to reach the last stage before any of the other values reach it
        *val_simd = val_simd.reverse();

        let mut cur_stage = self.stages[0];
        for i in 1..simd_len {
            let mut next_stage = self.stages[i];

            for _iter in 0..4 {
                cur_stage.compute_simd(&coef_simd, val_simd);
                cur_stage.concat_rotate(&mut next_stage);
            }

            // by rotating 4 times, next_stage is now the new value of cur_stage. set it!
            self.stages[i - 1] = next_stage;
            // this also means that cur_stage is now the new value of next_stage; continue to the next loop iteration
        }
        self.stages[simd_len - 1] = cur_stage;

        // put the values back where they started
        *val_simd = val_simd.reverse();

        // destagger the samples to clean up
        {
            let non_simd_start = (simd_len - 1) * f32x4::LEN;
            for (i, (val, break_coef)) in zip(val_simd.as_mut_array(), &coef_array).enumerate() {
                for stage in non_simd_start + 3 - i..self.num_stages {
                    self.compute_one(stage, break_coef, val);
                }
            }
        }
    }
}

impl<S: SimdPhaserStage> PhaserAlgorithm for SimdPhaserWrapper<S> {
    fn begin(
        &mut self,
        start: PhaserInstanceParams,
        end: PhaserInstanceParams,
        sample_rate: f32,
        run_length: f32,
    ) {
        self.i_w0 = util::interpolate(
            run_length,
            AngularFrequency::new(start.freq, sample_rate),
            AngularFrequency::new(end.freq, sample_rate),
        );
        self.i_q = util::interpolate(run_length, start.q, end.q);

        self.i_feedback_mult = util::interpolate(run_length, start.feedback, end.feedback);
    }
    fn resize(&mut self, num_stages: usize) {
        if self.num_stages == num_stages {
            return;
        }
        let vecsize = num_stages.div_ceil(f32x4::LEN);
        self.stages.resize(vecsize, Default::default());
        self.num_stages = num_stages;
    }

    fn compute(&mut self, sample: f32) -> (f32, f32) {
        let Self {
            wet_simd,
            dry_simd,

            simd_index,
            ..
        } = self;

        let mut simd_index = *simd_index as usize;
        assert!(simd_index < 4);
        let dry = std::mem::replace(&mut dry_simd.as_mut_array()[simd_index], sample);
        let wet = wet_simd.as_mut_array()[simd_index];

        simd_index += 1;
        if simd_index >= 4 {
            simd_index = 0;

            let mut sample = *dry_simd + self.wet_simd * f32x4::splat(self.i_feedback_mult.next());
            self.compute_simd(&mut sample);
            self.wet_simd = sample;
        }

        self.simd_index = simd_index as u8;

        // let simd_index_usize = *simd_index as usize;
        // assert!(simd_index_usize < 4);
        // let dry = std::mem::replace(&mut dry_simd.as_mut_array()[simd_index_usize], sample);
        // let wet = wet_simd.as_mut_array()[simd_index_usize];

        // *simd_index += 1;
        // if *simd_index >= 4 {
        //     *simd_index = 0;

        //     self.compute_simd();
        // }

        (dry, wet)
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PhaserAlgorithmMode {
    Unipole,
    Bipole,
    Legacy,
}

#[wasm_bindgen]
#[derive(Default)]
struct PhaserInstance {
    pub disperse: bool,

    i_mix: Interpolator<f32>,

    mode: Option<PhaserAlgorithmMode>,
    imp: Option<Box<dyn PhaserAlgorithm>>,
}

#[wasm_bindgen]
impl PhaserInstance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(setter)]
    pub fn set_num_stages(&mut self, num_stages: i32) {
        self.imp
            .as_mut()
            .unwrap()
            .resize(num_stages.try_into().unwrap_or(0));
    }
    #[wasm_bindgen(setter)]
    pub fn set_type(&mut self, mode: PhaserAlgorithmMode) {
        if self.mode == Some(mode) {
            return;
        }
        self.mode = Some(mode);
        self.imp = Some(match mode {
            PhaserAlgorithmMode::Legacy => Box::new(legacy::LegacyPhaser::default()),
            PhaserAlgorithmMode::Unipole => {
                Box::new(SimdPhaserWrapper::<unipole::PhaserStage>::default())
            }
            PhaserAlgorithmMode::Bipole => {
                Box::new(SimdPhaserWrapper::<bipole::PhaserStage>::default())
            }
        });
    }

    #[wasm_bindgen]
    pub fn begin(
        &mut self,
        start: PhaserInstanceParams,
        end: PhaserInstanceParams,
        sample_rate: f32,
        run_length: f32,
    ) {
        self.i_mix = util::interpolate(run_length, start.mix, end.mix);
        self.imp
            .as_mut()
            .unwrap()
            .begin(start, end, sample_rate, run_length);
    }

    #[wasm_bindgen]
    pub fn process(&mut self, sample: f32) -> f32 {
        let (dry, wet) = self.imp.as_mut().unwrap().compute(sample);

        if self.disperse {
            dry + (wet - dry) * self.i_mix.next()
        } else {
            dry + wet * self.i_mix.next()
        }
    }
}
