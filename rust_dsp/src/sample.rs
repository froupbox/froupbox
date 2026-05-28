//! https://gitlab.com/cyphers-stuff/cybox/-/blob/31c2eda59748f321a09141b41552d8e65a755dfe/dsp/cybox/src/sample.rs

use core::ops;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SamplePair {
    pub l: f32,
    pub r: f32,
}
impl SamplePair {
    pub const ZERO: Self = Self { l: 0.0, r: 0.0 };
    #[inline]
    pub fn to_mono(self) -> f32 {
        (self.l + self.r) * 0.5
    }
    #[inline]
    pub fn lerp(self, other: Self, alpha: f32) -> Self {
        self + (other - self) * alpha
    }
    #[inline]
    pub fn splat(v: f32) -> Self {
        Self { l: v, r: v }
    }
    #[inline]
    pub fn swap(self) -> Self {
        let Self { l, r } = self;
        Self { l: r, r: l }
    }

    #[inline]
    pub fn abs(self) -> Self {
        let Self { l, r } = self;
        Self {
            l: l.abs(),
            r: r.abs(),
        }
    }

    #[inline]
    pub fn map(self, f: impl Fn(f32) -> f32) -> Self {
        Self {
            l: f(self.l),
            r: f(self.r),
        }
    }
}
impl Default for SamplePair {
    fn default() -> Self {
        Self::ZERO
    }
}
impl ops::Add for SamplePair {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            l: self.l + rhs.l,
            r: self.r + rhs.r,
        }
    }
}
impl ops::Sub for SamplePair {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            l: self.l - rhs.l,
            r: self.r - rhs.r,
        }
    }
}
impl ops::Mul<f32> for SamplePair {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            l: self.l * rhs,
            r: self.r * rhs,
        }
    }
}
impl ops::Mul<SamplePair> for f32 {
    type Output = SamplePair;
    fn mul(self, rhs: SamplePair) -> Self::Output {
        rhs * self
    }
}
impl ops::Div<f32> for SamplePair {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            l: self.l / rhs,
            r: self.r / rhs,
        }
    }
}
impl ops::AddAssign for SamplePair {
    fn add_assign(&mut self, rhs: Self) {
        self.l += rhs.l;
        self.r += rhs.r;
    }
}
impl ops::SubAssign for SamplePair {
    fn sub_assign(&mut self, rhs: Self) {
        self.l -= rhs.l;
        self.r -= rhs.r;
    }
}
impl ops::MulAssign<f32> for SamplePair {
    fn mul_assign(&mut self, rhs: f32) {
        self.l *= rhs;
        self.r *= rhs;
    }
}
impl ops::DivAssign<f32> for SamplePair {
    fn div_assign(&mut self, rhs: f32) {
        self.l /= rhs;
        self.r /= rhs;
    }
}

pub trait Sample:
    Sized
    + Clone
    + Copy
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::Mul<f32, Output = Self>
    + ops::Div<f32, Output = Self>
{
    #[allow(unused)]
    const ZERO: Self;
    fn sanitize_finite(self) -> Self;
    #[allow(unused)]
    fn is_silent_below(self, threshold: f32) -> bool;
    fn lerp(self, other: Self, alpha: f32) -> Self {
        self + (other - self) * alpha
    }
}
impl Sample for SamplePair {
    const ZERO: Self = SamplePair::ZERO;
    fn sanitize_finite(self) -> Self {
        Self {
            l: self.l.sanitize_finite(),
            r: self.r.sanitize_finite(),
        }
    }
    fn is_silent_below(self, threshold: f32) -> bool {
        f32::max(self.l.abs(), self.r.abs()) < threshold
    }
}
impl Sample for f32 {
    const ZERO: Self = 0.0;
    fn sanitize_finite(self) -> Self {
        if !self.is_finite() { 0.0 } else { self }
    }
    fn is_silent_below(self, threshold: f32) -> bool {
        self.abs() < threshold
    }
}

pub fn lerp<T: Sample>(a: T, b: T, alpha: f32) -> T {
    a.lerp(b, alpha)
}
