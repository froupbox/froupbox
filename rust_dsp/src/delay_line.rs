// https://gitlab.com/cyphers-stuff/cybox/-/blob/31c2eda59748f321a09141b41552d8e65a755dfe/dsp/beepbox/src/delay_line.rs
use crate::{Sample, SamplePair};

#[derive(Default)]
pub struct DelayLine<T: Sample = SamplePair> {
    index: usize,
    pub buf: Box<[T]>,
}

impl<T: Sample> DelayLine<T> {
    pub fn new(length: usize) -> Self {
        Self {
            index: 0,
            buf: unsafe { Box::new_zeroed_slice(length).assume_init() },
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn push(&mut self, pair: T) {
        if self.index == 0 {
            self.index = self.len();
        }
        self.index -= 1;
        let sanitized = pair.sanitize_finite();
        self.buf[self.index] = sanitized;
    }
    pub fn compute(&mut self, samples: f32) -> T {
        let trunced = samples as usize;
        if samples.is_sign_negative() || trunced >= self.len() {
            // out of bounds
            return T::ZERO;
        }
        fn wrap_once(val: usize, n: usize) -> usize {
            val.min(val.wrapping_sub(n))
        }

        let index = wrap_once(self.index + trunced, self.len());
        let next_index = wrap_once(self.index + trunced + 1, self.len());

        self.buf[index].lerp(self.buf[next_index], samples - trunced as f32)
    }

    pub fn reserve_at_least(&mut self, n: usize) {
        if self.len() >= n {
            return;
        }
        let cur_len = self.len().max(1);

        let new_len = n.div_ceil(cur_len).next_power_of_two() * cur_len;
        let mut new_delay_line = Self::new(new_len);
        if self.len() > 0 {
            new_delay_line.buf[0..cur_len - self.index].copy_from_slice(&self.buf[self.index..]);
            new_delay_line.buf[cur_len - self.index..cur_len]
                .copy_from_slice(&self.buf[0..self.index]);
        }

        *self = new_delay_line;
    }
}

impl<T: Sample> std::fmt::Debug for DelayLine<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DelayLine {{ <{}> }}", self.len())
    }
}
