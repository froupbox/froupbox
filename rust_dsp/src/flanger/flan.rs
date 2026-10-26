//! the compressor is called comp.rs so i copied it
//!
//! this is the flanger

use crate::{
    Sample,
    delay_line::DelayLine,
    lerp,
    sample::Multisample,
    util::{Interpolator, Zippable},
};

#[derive(Default, Debug)]
pub struct FlangerParams {
    pub delay: f32,
    pub mix: f32,
    pub feedmix: f32,
    pub voices: f32,
}
impl Zippable for FlangerParams {
    fn zip(&self, other: &Self, f: impl Fn(f32, f32) -> f32) -> Self {
        Self {
            delay: f(self.delay, other.delay),
            mix: f(self.mix, other.mix),
            feedmix: f(self.feedmix, other.feedmix),
            voices: f(self.voices, other.voices),
        }
    }
}
impl FlangerParams {
    pub fn total_delay(&self) -> usize {
        (self.delay * self.voices) as usize + 328
    }
}

/// flanger using linear interpolation
#[derive(Default, Debug)]
pub struct Flanger<T: Sample> {
    // 0 is delayed input, 1 is delayed output
    pub delay_line: DelayLine<Multisample<T, 2>>,

    pub interpolator: Interpolator<FlangerParams>,
}
impl<T: Sample> Flanger<T> {
    pub fn process(&mut self, x: T, use_larger_delay_line: bool) -> T {
        let params = self.interpolator.next();

        let (dx, dy) = if (params.voices - 1.0).abs() < 1e-5 {
            let Multisample([l, r]) = self.delay_line.compute(params.delay);
            (l, r)
        } else {
            let mut result = T::ZERO;
            let num_voices_int = params.voices.ceil() as usize;
            let delay_scale = if use_larger_delay_line {
                1.0
            } else {
                params.voices.recip().min(1.0)
            };
            for i in 1..=num_voices_int {
                let Multisample([mut val, ..]) = self
                    .delay_line
                    .compute(params.delay * delay_scale * i as f32);
                if i as f32 - params.voices > 0.0 {
                    val *= 1.0 - (i as f32 - params.voices);
                }
                result += val;
            }
            let Multisample([.., r]) = self.delay_line.compute(params.delay);

            (result, r)
        };

        let y = lerp(x, lerp(dx, dy, params.feedmix), params.mix);
        self.delay_line.push(Multisample([x, y]));
        y
    }
}
