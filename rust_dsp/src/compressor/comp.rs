//! https://gitlab.com/cyphers-stuff/cybox/-/blob/31c2eda59748f321a09141b41552d8e65a755dfe/dsp/beepbox/src/comp.rs
//! Port/tweak of [CALF Studio Gear's `gain_reduction_audio_module`](https://github.com/calf-studio-gear/calf/blob/master/src/modules_comp.cpp#L100) by Thor Harald Johanssen and contributors
//!
//! I don't exactly know how it works but...

use wasm_bindgen::prelude::*;

use crate::{SamplePair, lerp, sample::Sample};

#[derive(Clone)]
#[wasm_bindgen]
pub struct CompressorParams {
    pub attack: f32,
    pub decay: f32,
    pub threshold: f32,

    pub ratio_up: f32,
    pub ratio_down: f32,

    pub knee: f32,

    time_scale: f32,
}
impl CompressorParams {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            time_scale: sample_rate / 4000.0,

            attack: 1.0,
            decay: 1.0,
            threshold: 1.0,
            ratio_up: 1.0,
            ratio_down: 3.0,

            knee: 2.0,
        }
    }

    pub fn zip(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self {
        debug_assert_eq!(self.time_scale, other.time_scale);
        Self {
            time_scale: self.time_scale,

            attack: f(self.attack, other.attack),
            decay: f(self.decay, other.decay),
            threshold: f(self.threshold, other.threshold),
            ratio_up: f(self.ratio_up, other.ratio_up),
            ratio_down: f(self.ratio_down, other.ratio_down),
            knee: f(self.knee, other.knee),
        }
    }

    fn output_gain(&self, lin_slope: f32) -> f32 {
        let slope = f32::ln(lin_slope) * 0.5 - self.threshold;
        let gain = if slope <= -self.knee {
            slope / self.ratio_up
        } else if slope >= self.knee {
            slope / self.ratio_down
        } else {
            normalized_hermite(
                (slope + self.knee) / (self.knee * 2.0),
                -self.knee / self.ratio_up,
                self.knee / self.ratio_down,
                self.knee * 2.0 / self.ratio_up,
                self.knee * 2.0 / self.ratio_down,
            )
        };
        f32::exp(gain - slope)
    }
}

pub struct Compressor {
    lin_slope: f32,
}
impl Default for Compressor {
    fn default() -> Self {
        Self {
            lin_slope: f32::NAN,
        }
    }
}
impl Compressor {
    pub fn process(&mut self, params: &CompressorParams, sample: SamplePair) -> SamplePair {
        let attack_coeff = 1.0 / (params.attack * params.time_scale);
        let release_coeff = 1.0 / (params.decay * params.time_scale);

        let absample = sample.abs().to_mono();
        let absample = (absample * absample).sanitize_finite();

        let mut lin_slope = self.lin_slope;
        if !lin_slope.is_finite() {
            lin_slope = absample;
        }

        let diff = absample / lin_slope;
        lin_slope = lerp(
            lin_slope,
            absample,
            f32::min(
                if absample > lin_slope {
                    attack_coeff
                } else {
                    release_coeff
                } * f32::clamp(diff.abs() * 10.0, 1.0, 4e3),
                1.0,
            ),
        );

        let gain = if lin_slope > 0.0 {
            params.output_gain(lin_slope)
        } else {
            1.0
        };

        self.lin_slope = lin_slope;

        sample * gain
    }
}

fn normalized_hermite(t: f32, p0: f32, p1: f32, m0: f32, m1: f32) -> f32 {
    let t2 = t * t;
    let t3 = t2 * t;
    (2.0 * t3 - 3.0 * t2 + 1.0) * p0 + (t3 - 2.0 * t2 + t) * m0 - (2.0 * t3 - 3.0 * t2) * p1
        + (t3 - t2) * m1
}
