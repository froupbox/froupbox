// Copyright (c) 2026 froupbox developers and contributing authors, distributed under the GNU AGPL v3, see the accompanying LICENSE file.

export interface Prompt {
    container: HTMLElement;
    cleanUp: () => void;
    gotMouseUp?: boolean; // Use to avoid closing the prompt too soon.
}