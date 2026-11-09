use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct DspBuffer {
    buffer: Box<[f32]>,
    run_length: usize,
    pub sample_rate: f32,
}
#[wasm_bindgen]
impl DspBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(frame_size: usize) -> Self {
        Self {
            buffer: vec![0.0; frame_size * 2].into_boxed_slice(),
            ..Default::default()
        }
    }
    #[wasm_bindgen(getter)]
    pub fn buffer(&mut self) -> js_sys::Float32Array {
        unsafe { js_sys::Float32Array::view(&self.buffer) }
    }
    #[wasm_bindgen(setter)]
    pub fn set_run_length(&mut self, run_length: usize) {
        self.run_length = run_length;
        if self.buffer.len() / 2 < run_length {
            // when bpm changes fast, run length can occasionally grow above the buffer size during export.
            // i don't know why this only breaks during export and not playing in the editor
            self.buffer = vec![0.0; run_length.next_power_of_two() * 2].into_boxed_slice()
        }
    }
}
impl DspBuffer {
    pub fn run_length(&self) -> usize {
        self.run_length
    }

    pub fn frame_size(&self) -> usize {
        self.buffer.len() / 2
    }
    pub fn as_channels(&mut self) -> (&mut [f32], &mut [f32]) {
        let (left, right) = self.buffer.split_at_mut(self.frame_size());
        (&mut left[..self.run_length], &mut right[..self.run_length])
    }
    pub fn as_zipped(&mut self) -> impl Iterator<Item = (&mut f32, &mut f32)> {
        let (left, right) = self.as_channels();
        std::iter::zip(left, right)
    }

    pub fn clear(&mut self) {
        let (l, r) = self.as_channels();
        l.fill(0.0);
        r.fill(0.0);
    }
    pub fn set(&mut self, other: &mut Self) {
        let (l1, r1) = self.as_channels();
        let (l2, r2) = other.as_channels();
        l1.copy_from_slice(l2);
        r1.copy_from_slice(r2);
    }
}
