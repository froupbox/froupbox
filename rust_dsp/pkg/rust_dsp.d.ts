/* tslint:disable */
/* eslint-disable */

export class ColourizerInstance {
    free(): void;
    [Symbol.dispose](): void;
    begin(start: ColourizerInstanceParams, end: ColourizerInstanceParams, sample_rate: number, run_length: number): void;
    constructor();
    process(buffer: DspBuffer): void;
}

export class ColourizerInstanceParams {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    set freqs(value: Float32Array);
    mix: number;
    voices: number;
}

export class CompressorInstance {
    free(): void;
    [Symbol.dispose](): void;
    begin(start: CompressorInstanceParams, end: CompressorInstanceParams, sample_rate: number, run_length: number): void;
    constructor();
    process(buffer: DspBuffer): void;
}

export class CompressorInstanceParams {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    attack: number;
    decay: number;
    freq_lo_mid: number;
    freq_mid_hi: number;
    hi_gain: number;
    lo_gain: number;
    mid_gain: number;
    ratio_down: number;
    ratio_up: number;
    threshold: number;
}

export class CompressorParams {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    attack: number;
    decay: number;
    knee: number;
    ratio_down: number;
    ratio_up: number;
    threshold: number;
}

export class DspBuffer {
    free(): void;
    [Symbol.dispose](): void;
    constructor(frame_size: number);
    readonly buffer: Float32Array;
    run_length: number;
    sample_rate: number;
}

export class FlangerInstance {
    free(): void;
    [Symbol.dispose](): void;
    begin(start: FlangerInstanceParams, end: FlangerInstanceParams, sample_rate: number, run_length: number): void;
    constructor();
    process(buffer: DspBuffer): void;
    use_larger_delay_line: boolean;
}

export class FlangerInstanceParams {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    delay: number;
    feedmix: number;
    mix: number;
    panning: number;
    voices: number;
}

export enum PhaserAlgorithmMode {
    Unipole = 0,
    Bipole = 1,
    Legacy = 2,
}

export class PhaserInstance {
    free(): void;
    [Symbol.dispose](): void;
    begin(start: PhaserInstanceParams, end: PhaserInstanceParams, sample_rate: number, run_length: number): void;
    constructor();
    process(sample: number): number;
    disperse: boolean;
    set num_stages(value: number);
    set type(value: PhaserAlgorithmMode);
}

export class PhaserInstanceParams {
    free(): void;
    [Symbol.dispose](): void;
    constructor();
    feedback: number;
    freq: number;
    mix: number;
    q: number;
}

export function start(): void;
