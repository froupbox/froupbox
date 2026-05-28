// Copyright (C) 2020 John Nesky, distributed under the MIT license.
// A few wrapper classes that add functionality onto existing HTML elements, namely binding some events to an implementation-specified change class

import { clamp } from "../synth/synth";
import { Change } from "./Change";
import { SongDocument } from "./SongDocument";
import { HTML } from "imperative-html/dist/esm/elements-strict";

const { span, div } = HTML;

export class InputBox {
    private _change: Change | null = null;
    private _value: string = "";
    private _oldValue: string = "";

    constructor(public readonly input: HTMLInputElement, private readonly _doc: SongDocument, private readonly _getChange: (oldValue: string, newValue: string) => Change) {
        input.addEventListener("input", this._whenInput);
        input.addEventListener("change", this._whenChange);
    }

    public updateValue(value: string): void {
        this._value = value;
        this.input.value = String(value);
    }

    private _whenInput = (): void => {
        const continuingProspectiveChange: boolean = this._doc.lastChangeWas(this._change);
        if (!continuingProspectiveChange) this._oldValue = this._value;
        this._change = this._getChange(this._oldValue, this.input.value);
        this._doc.setProspectiveChange(this._change);
    };

    private _whenChange = (): void => {
        this._doc.record(this._change!);
        this._change = null;
    };
}

export class Slider {
    private _change: Change | null = null;
    private _value: number = 0;
    private _oldValue: number = 0;
    public container: HTMLSpanElement;

    constructor(public readonly input: HTMLInputElement, private readonly _doc: SongDocument, private readonly _getChange: ((oldValue: number, newValue: number) => Change) | null, midTick: boolean) {
        // A container is created around the input to allow for spec-compliant pseudo css classes (e.g ::before and ::after, which must be added to containers, not the input itself)
        this.container = (midTick) ? span({ class: "midTick", style: "position: sticky; width: 61.5%;" }, input) : span({ style: "position: sticky;" }, input);
        input.addEventListener("input", this._whenInput);
        input.addEventListener("change", this._whenChange);
    }

    public updateValue(value: number): void {
        this._value = value;
        this.input.value = String(value);
    }

    private _whenInput = (): void => {
        const continuingProspectiveChange: boolean = this._doc.lastChangeWas(this._change);
        if (!continuingProspectiveChange) this._oldValue = this._value;
        if (this._getChange != null) {
            this._change = this._getChange(this._oldValue, parseFloat(this.input.value));
            this._doc.setProspectiveChange(this._change);
        }
    };

    public getValueBeforeProspectiveChange(): number {
        return this._oldValue;
    }

    private _whenChange = (): void => {
        if (this._getChange != null) {
            this._doc.record(this._change!);
            this._change = null;
        }
    };
}

export interface KnobOptions {
  label: string,
  min: number,
  max: number,
  precision?: number,
  mapping?: "linear" | "logarithmic",
  unit?: ":1" | "Hz",
}

export class Knob {
  private _change: Change | null = null;
  private _oldValue: number = 0;
  public container: HTMLElement;
  private bodyEl: HTMLElement;
  private valueEl: HTMLElement;
  private value: number;
  private isDragging: boolean = false;
  
  min: number;
  max: number;
  precision: number;
  mapping: "linear" | "logarithmic";

  constructor(
    private readonly _doc: SongDocument,
    private readonly _getChange: ((oldValue: number, newValue: number) => Change) | null,
    { label, min, max, precision = 2, mapping = "linear", unit }: KnobOptions,
  ) {
    this.min = min;
    this.max = max;
    this.precision = precision;
    this.mapping = mapping;
    
    this.value = min;
    document.body.addEventListener("mousemove", this._whenMove);
    document.body.addEventListener("mouseup", this._whenChange);
    
    this.bodyEl = div({ class: "sbk-body" }, div({ class: "sbk-nub" }));
    this.bodyEl.addEventListener("mousedown", this._whenDown)
    this.valueEl = span({ class: "sb-value" });
    
    const valueComponents: any[] = [this.valueEl];
    if (unit) valueComponents.push(span({ class: "sb-unit" }, unit));
    
    this.container = div(
      { class: "sbk" },
      span({ class: "sbk-label" }, label),
      this.bodyEl,
      span({ class: "sbk-label" }, ...valueComponents),
    );
  }

  public updateValue(value: number): void {
    let { min, max, mapping, precision } = this;
    // from 0 to 1
    let mapped;
    if (mapping === "linear")
      mapped = (value - min) / (max - min)
    else
      mapped = Math.log(value / min) / Math.log(max / min);
  
    this.value = value;
    this.bodyEl.style.rotate = (mapped - 0.5) * 0.9 + "turn";
    this.valueEl.textContent = value.toFixed(precision);
  }

  private _whenDown = (e: MouseEvent): void => {
    this.isDragging = true;
  };
  
  private _whenMove = (e: MouseEvent): void => {
    if (!this.isDragging) return;
    if (e.buttons === 0) {
      this._whenChange();
      return;
    }
    let { min, max, mapping, value } = this;
    const delta = -e.movementY / 100;
    if (mapping === "linear")
      value += (max - min) * delta;
    else
      value *= (max / min) ** delta;
    value = clamp(min, max, value);
    
    this.updateValue(value);
  
    const continuingProspectiveChange: boolean = this._doc.lastChangeWas(this._change);
    if (!continuingProspectiveChange) this._oldValue = this.value;
  
    if (this._getChange != null) {
      this._change = this._getChange(this._oldValue, this.value);
      this._doc.setProspectiveChange(this._change);
    }
  };

  public getValueBeforeProspectiveChange(): number {
    return this._oldValue;
  }

  private _whenChange = (): void => {
    if (!this.isDragging) return;
    this.isDragging = false;
    if (this._getChange != null) {
        this._doc.record(this._change!);
        this._change = null;
    }
  };
}

export interface LabeledeSliderOptions {
  label: string,
  vertical?: boolean,
  midTick?: boolean,
  unit: "dB", // TODO: different units
}

export class LabeledSlider extends Slider {
  private _valueEl: HTMLElement;
  
  constructor(
    input: HTMLInputElement,
    doc: SongDocument,
    getChange: (oldValue: number, newValue: number) => Change,
    { label, vertical, unit, midTick }: LabeledeSliderOptions,
  ) {
    super(input, doc, getChange, midTick ?? false);
    this.container.classList.add("sbls");
    if (vertical) {
      input.classList.add("vertical");
    }

    this._valueEl = span({ class: "sb-value" });
    const valueContainer = div(
      { class: "sbls-label" },
      this._valueEl,
      span({ class: "sb-unit" }, unit),
    );
    const labelEl = div({ class: "sbls-label" }, label);

    this.container.append(labelEl, valueContainer);
  }
  
  public updateValue(value: number): void {
    super.updateValue(value);
    this._valueEl.textContent = (value > 0 ? "+" : "") + value.toFixed(1);
  }
}
