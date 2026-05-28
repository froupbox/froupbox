//! https://gitlab.com/cyphers-stuff/cybox/-/blob/31c2eda59748f321a09141b41552d8e65a755dfe/dsp/beepbox/src/filters.rs

use core::f32;

use crate::SamplePair;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FilterDirection {
    Low,
    High,
}

pub fn to_w0(freq: f32, sample_rate: f32) -> f32 {
    freq / sample_rate * f32::consts::TAU
}
pub fn sincos_w0(w0: f32) -> (f32, f32) {
    let cos = f32::cos(w0);
    // assuming the frequency is below the nyquist frequency, 0 < w0 < PI and thus sin(w0) > 0
    (f32::sqrt(1.0 - cos * cos), cos)
}

#[derive(Debug, Clone)]
pub struct BiquadFilterCoefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}
impl BiquadFilterCoefficients {
    pub fn new(b0: f32, b1: f32, b2: f32, a0: f32, a1: f32, a2: f32) -> Self {
        let inv_a0 = a0.recip();
        Self {
            b0: b0 * inv_a0,
            b1: b1 * inv_a0,
            b2: b2 * inv_a0,
            a1: a1 * inv_a0,
            a2: a2 * inv_a0,
        }
    }

    pub fn pass(dir: FilterDirection, w0: f32, mult: f32) -> Self {
        let (sw0, cw0) = sincos_w0(w0);

        let alpha = sw0 / mult;

        let bmult = match dir {
            FilterDirection::Low => 1.0,
            FilterDirection::High => -1.0,
        };
        let b1 = bmult - cw0;

        Self::new(
            bmult * b1,
            2.0 * b1,
            bmult * b1,
            2.0 + alpha,
            -4.0 * cw0,
            2.0 - alpha,
        )
    }
}

#[derive(Default)]
pub struct BiquadFilterValues {
    v1: SamplePair,
    v2: SamplePair,
}
impl BiquadFilterValues {
    pub fn run(&mut self, c: &BiquadFilterCoefficients, x0: SamplePair) -> SamplePair {
        let v0 = x0 - c.a1 * self.v1 - c.a2 * self.v2;
        let output = v0 * c.b0 + self.v1 * c.b1 + self.v2 * c.b2;
        self.v2 = self.v1;
        self.v1 = v0;
        output
    }
}

#[derive(Clone)]
pub struct CrossoverCoefficients {
    pub lo: BiquadFilterCoefficients,
    pub hi: BiquadFilterCoefficients,
}
impl CrossoverCoefficients {
    pub fn new(w0: f32) -> Self {
        Self {
            lo: BiquadFilterCoefficients::pass(
                FilterDirection::Low,
                w0,
                f32::consts::FRAC_1_SQRT_2,
            ),
            hi: BiquadFilterCoefficients::pass(
                FilterDirection::High,
                w0,
                f32::consts::FRAC_1_SQRT_2,
            ),
        }
    }
}

/// A [Linkwitz-Riley](https://en.wikipedia.org/wiki/Linkwitz%E2%80%93Riley_filter) crossover to split an audio signal at a given frequency.
#[derive(Default)]
pub struct Crossover {
    pub l0: BiquadFilterValues,
    pub l1: BiquadFilterValues,
    pub h0: BiquadFilterValues,
    pub h1: BiquadFilterValues,
}
impl Crossover {
    pub fn run(&mut self, coef: &CrossoverCoefficients, lo: &mut SamplePair, hi: &mut SamplePair) {
        *lo = self.l0.run(&coef.lo, *lo);
        *lo = self.l1.run(&coef.lo, *lo);
        *hi = self.h0.run(&coef.hi, *hi);
        *hi = self.h1.run(&coef.hi, *hi);
    }
}
