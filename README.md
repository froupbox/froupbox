# froupbox

froupbox is an online tool for sketching and sharing instrumental music.
You can find it [here](https://github.com/froupbox/froupbox/).
It is a modification of [Slarmoo's Box] (https://slarmoo.github.io/slarmoosbox/website/), which is a modification of [Ultrabox](https://ultraabox.github.io), which is a modification of [JummBox](https://github.com/jummbus/jummbox), which in turn is a modification of the [original BeepBox](https://beepbox.co).

froupbox is a mod of various BeepBox mods that aims to advance their capabilities. Feel free to contribute!

All song data is packaged into the URL at the top of your browser. When you make
changes to the song, the URL is updated to reflect your changes. When you are
satisfied with your song, just copy and paste the URL to save and share your
song!

froupbox, as well as the beepmods which it's based on, are free projects. If you ever feel so inclined, please support the original creator of BeepBox, [John Nesky](http://www.johnnesky.com/), via
[PayPal](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=QZJTX9GRYEV9N&currency_code=USD)!

## Building

The compilation procedure is **different** from the one BeepBox uses, but the dependencies are the same:

The code is written in [TypeScript](https://www.typescriptlang.org/), which requires [node & npm](https://www.npmjs.com/get-npm), so install those first. Then to build this project, open a terminal and run:

```sh
git clone https://github.com/froupbox/froupbox.git
cd froupbox
npm install

# you can now run
npm run build
# and upload the contents of the dist/ folder to a static hosting website, or run
npm run dev
# to host a local live-reloading web server.

# additionally, to enable the "this is the testing version" alerts, run
DO_ALERTS=1 npm run build
```

## License

froupbox is licensed under the [AGPL v3](./LICENSE).

### I want to use froupbox code in other projects! How does this matter?

_This is not legal advice. Consult a lawyer for details._

Summarized, if you fork froupbox (to make a new mod) and credit it while having the source code openly accessible (via GitHub or similar), you're completely fine as long as you don't change froupbox's original licenses.

This also applies if you **use** froupbox's source code in other projects. "Use" means create a [derivative work](https://en.wikipedia.org/wiki/Derivative_work) that uses froupbox's code. This includes copy-paste, translations, and anything that incorporates the source code of froupbox in any form. That is, if you include froupbox source code _at all_ in your own project, the entire project must be licensed under the AGPL.

For BeepBox forks: you can relicense MIT code (like BeepBox) under the AGPL, but not the other way around; i.e. if your project is AGPL, you can take code from MIT licensed projects, but if your project is MIT, you can't take code from AGPL projects without changing the license.
