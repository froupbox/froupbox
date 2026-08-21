#![feature(portable_simd)]

mod buffer;
mod colourizer;
mod compressor;
mod delay_line;
mod filters;
mod flanger;
mod phaser;
mod sample;
mod util;

pub(crate) use sample::{Sample, SamplePair, lerp};

mod debug {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn start() {
        console_error_panic_hook::set_once();
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: String);
    }
}
macro_rules! log {
    ($($arg:tt)+) => {
        $crate::debug::log(format!($($arg)+));
    };
}
use log;
