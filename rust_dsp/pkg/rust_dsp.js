/* @ts-self-types="./rust_dsp.d.ts" */
import * as wasm from "./rust_dsp_bg.wasm";
import { __wbg_set_wasm } from "./rust_dsp_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    CompressorInstance, CompressorInstanceParams, CompressorParams, start
} from "./rust_dsp_bg.js";
