// Copyright (c) 2026 froupbox developers and contributing authors, distributed under the GNU AGPL v3, see the accompanying LICENSE file.

export class ChangeNotifier {
    private _watchers: (() => void)[] = [];
    private _dirty: boolean = false;

    public watch(watcher: () => void): void {
        if (this._watchers.indexOf(watcher) == -1) {
            this._watchers.push(watcher);
        }
    }

    public unwatch(watcher: () => void): void {
        const index: number = this._watchers.indexOf(watcher);
        if (index != -1) {
            this._watchers.splice(index, 1);
        }
    }

    public changed(): void {
        this._dirty = true;
    }

    public notifyWatchers(): void {
        if (!this._dirty) return;
        this._dirty = false;
        for (const watcher of this._watchers.concat()) {
            watcher();
        }
    }
}