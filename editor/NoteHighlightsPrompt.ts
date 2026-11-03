// Copyright (c) 2026 froupbox developers and contributing authors, distributed under the GNU AGPL v3, see the accompanying LICENSE file.

import { HTML } from "imperative-html/dist/esm/elements-strict";
import { SongDocument } from "./SongDocument";
import { Prompt } from "./Prompt";


//namespace beepbox {
const { button, div, h2, input, p } = HTML;

export class NoteHighlightsPrompt implements Prompt {
    private readonly _highlightsInput: HTMLInputElement = input({ type: "text", value: localStorage.getItem("noteHighlights") || `1/1, 3/2` });
    private readonly _cancelButton: HTMLButtonElement = button({ class: "cancelButton" });
    private readonly _okayButton: HTMLButtonElement = button({ class: "okayButton", style: "width:45%;" }, "Okay");

    public readonly container: HTMLDivElement;

    constructor(private _doc: SongDocument) {

        this._okayButton.addEventListener("click", this._saveChanges);
        this._cancelButton.addEventListener("click", this._close);

        this.container = div({ class: "prompt noSelection", style: "width: 250px;" },
            h2("Note Highlights"),
            p("Here, you can set which intervals to highlight. Type in the intervals relative to the tonic. Intervals are equave reduced. You can use \"/\" for harmonic intervals and \"\\\" for edostep intervals."),
            this._highlightsInput,
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
        window.localStorage.setItem("noteHighlights", this._highlightsInput.value);
        this._close();
    }
}
//}