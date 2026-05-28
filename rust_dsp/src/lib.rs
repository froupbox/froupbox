mod compressor;
mod sample;

pub(crate) use sample::{SamplePair, lerp};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
