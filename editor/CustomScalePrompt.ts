// Copyright (c) 2026 froupbox developers and contributing authors, distributed under the GNU AGPL v3, see the accompanying LICENSE file.

import { HTML } from "imperative-html/dist/esm/elements-strict";
import { SongDocument } from "./SongDocument";
import { Prompt } from "./Prompt";
import { ChangeCustomScale } from "./changes";


//namespace beepbox {
const { button, div, h2, input, p } = HTML;

export class CustomScalePrompt implements Prompt {
    private readonly _scaleInput: HTMLInputElement = input({ type: "text", value: this._doc.song.scaleCustom });
    private readonly _cancelButton: HTMLButtonElement = button({ class: "cancelButton" });
    private readonly _okayButton: HTMLButtonElement = button({ class: "okayButton", style: "width:45%;" }, "Okay");

    public readonly container: HTMLDivElement;

    constructor(private _doc: SongDocument) {

        this._okayButton.addEventListener("click", this._saveChanges);
        this._cancelButton.addEventListener("click", this._close);

        this.container = div({ class: "prompt noSelection", style: "width: 250px;" },
            h2("Custom Scale"),
            p("Here, you can make your own scale to use in your song. Type in the intervals of the scale. You can use \"/\" for harmonic intervals and \"\\\" for edostep intervals. For this to work, you'll need to have the \"Custom\" scale selected."),
            this._scaleInput,
            div({ style: "display: flex; flex-direction: row-reverse; justify-content: space-between;" },
                this._okayButton,
            ),
            this._cancelButton,
        )
        this.container.addEventListener("keydown", this.whenKeyPressed);
    }

    private _close = (): void => {
        this._doc.undo();
    }

    public cleanUp = (): void => {
        this._okayButton.removeEventListener("click", this._saveChanges);
        this._cancelButton.removeEventListener("click", this._close);
        this.container.removeEventListener("keydown", this.whenKeyPressed);
    }

    public whenKeyPressed = (event: KeyboardEvent): void => {
        if ((<Element>event.target).tagName != "BUTTON" && event.keyCode == 13) { // Enter key
            this._saveChanges();
        }
    }


    private _saveChanges = (): void => {
        this._doc.prompt = null;
        this._doc.record(new ChangeCustomScale(this._doc, this._scaleInput.value));
    }
}
//}