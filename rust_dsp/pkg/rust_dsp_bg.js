export class CompressorInstance {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CompressorInstanceFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_compressorinstance_free(ptr, 0);
    }
    /**
     * @returns {Float32Array}
     */
    get_buffer() {
        const ret = wasm.compressorinstance_get_buffer(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} frame_size
     */
    constructor(frame_size) {
        const ret = wasm.compressorinstance_new(frame_size);
        this.__wbg_ptr = ret;
        CompressorInstanceFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} sample_rate
     * @param {number} run_length
     */
    process(sample_rate, run_length) {
        wasm.compressorinstance_process(this.__wbg_ptr, sample_rate, run_length);
    }
    /**
     * @returns {CompressorInstanceParams}
     */
    get end() {
        const ret = wasm.__wbg_get_compressorinstance_end(this.__wbg_ptr);
        return CompressorInstanceParams.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    get frame_size() {
        const ret = wasm.__wbg_get_compressorinstance_frame_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {CompressorInstanceParams}
     */
    get start() {
        const ret = wasm.__wbg_get_compressorinstance_start(this.__wbg_ptr);
        return CompressorInstanceParams.__wrap(ret);
    }
    /**
     * @param {CompressorInstanceParams} arg0
     */
    set end(arg0) {
        _assertClass(arg0, CompressorInstanceParams);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_compressorinstance_end(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} arg0
     */
    set frame_size(arg0) {
        wasm.__wbg_set_compressorinstance_frame_size(this.__wbg_ptr, arg0);
    }
    /**
     * @param {CompressorInstanceParams} arg0
     */
    set start(arg0) {
        _assertClass(arg0, CompressorInstanceParams);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_compressorinstance_start(this.__wbg_ptr, ptr0);
    }
}
if (Symbol.dispose) CompressorInstance.prototype[Symbol.dispose] = CompressorInstance.prototype.free;

export class CompressorInstanceParams {
    static __wrap(ptr) {
        const obj = Object.create(CompressorInstanceParams.prototype);
        obj.__wbg_ptr = ptr;
        CompressorInstanceParamsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CompressorInstanceParamsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_compressorinstanceparams_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get attack() {
        const ret = wasm.__wbg_get_compressorinstanceparams_attack(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get decay() {
        const ret = wasm.__wbg_get_compressorinstanceparams_decay(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get freq_lo_mid() {
        const ret = wasm.__wbg_get_compressorinstanceparams_freq_lo_mid(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get freq_mid_hi() {
        const ret = wasm.__wbg_get_compressorinstanceparams_freq_mid_hi(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get hi_gain() {
        const ret = wasm.__wbg_get_compressorinstanceparams_hi_gain(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get lo_gain() {
        const ret = wasm.__wbg_get_compressorinstanceparams_lo_gain(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mid_gain() {
        const ret = wasm.__wbg_get_compressorinstanceparams_mid_gain(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get ratio_down() {
        const ret = wasm.__wbg_get_compressorinstanceparams_ratio_down(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get ratio_up() {
        const ret = wasm.__wbg_get_compressorinstanceparams_ratio_up(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get threshold() {
        const ret = wasm.__wbg_get_compressorinstanceparams_threshold(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set attack(arg0) {
        wasm.__wbg_set_compressorinstanceparams_attack(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set decay(arg0) {
        wasm.__wbg_set_compressorinstanceparams_decay(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set freq_lo_mid(arg0) {
        wasm.__wbg_set_compressorinstanceparams_freq_lo_mid(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set freq_mid_hi(arg0) {
        wasm.__wbg_set_compressorinstanceparams_freq_mid_hi(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set hi_gain(arg0) {
        wasm.__wbg_set_compressorinstanceparams_hi_gain(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set lo_gain(arg0) {
        wasm.__wbg_set_compressorinstanceparams_lo_gain(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mid_gain(arg0) {
        wasm.__wbg_set_compressorinstanceparams_mid_gain(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set ratio_down(arg0) {
        wasm.__wbg_set_compressorinstanceparams_ratio_down(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set ratio_up(arg0) {
        wasm.__wbg_set_compressorinstanceparams_ratio_up(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set threshold(arg0) {
        wasm.__wbg_set_compressorinstanceparams_threshold(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) CompressorInstanceParams.prototype[Symbol.dispose] = CompressorInstanceParams.prototype.free;

export class CompressorParams {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CompressorParamsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_compressorparams_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get attack() {
        const ret = wasm.__wbg_get_compressorparams_attack(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get decay() {
        const ret = wasm.__wbg_get_compressorparams_decay(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get knee() {
        const ret = wasm.__wbg_get_compressorparams_knee(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get ratio_down() {
        const ret = wasm.__wbg_get_compressorparams_ratio_down(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get ratio_up() {
        const ret = wasm.__wbg_get_compressorparams_ratio_up(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get threshold() {
        const ret = wasm.__wbg_get_compressorparams_threshold(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set attack(arg0) {
        wasm.__wbg_set_compressorparams_attack(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set decay(arg0) {
        wasm.__wbg_set_compressorparams_decay(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set knee(arg0) {
        wasm.__wbg_set_compressorparams_knee(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set ratio_down(arg0) {
        wasm.__wbg_set_compressorparams_ratio_down(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set ratio_up(arg0) {
        wasm.__wbg_set_compressorparams_ratio_up(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set threshold(arg0) {
        wasm.__wbg_set_compressorparams_threshold(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) CompressorParams.prototype[Symbol.dispose] = CompressorParams.prototype.free;

export function start() {
    wasm.start();
}
export function __wbg___wbindgen_throw_9c75d47bf9e7731e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_error_a6fa202b58aa1cd3(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}
export function __wbg_new_227d7c05414eb861() {
    const ret = new Error();
    return ret;
}
export function __wbg_stack_3b0d974bbf31e44f(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
    const ret = getArrayF32FromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
const CompressorInstanceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compressorinstance_free(ptr, 1));
const CompressorInstanceParamsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compressorinstanceparams_free(ptr, 1));
const CompressorParamsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_compressorparams_free(ptr, 1));

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
