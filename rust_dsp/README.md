# Rust DSP code

Hi! You're looking at the DSP code. Normally, the precompiled WebAssembly module is checked into the repository, so you don't actually need to build this to compile froupbox.

Okay, that's it! You don't need to touch this! Goodbye!

---

Fine, you want to _change_ the DSP code - to add a new effect, for instance. In that case:

# Setup

This requires your terminal to be in this folder i.e. the `rust_dsp` folder. Verify you're here:

```
$ pwd
<something here>/froupbox/rust_dsp
```

Install Rust using [rustup](https://rustup.rs/). Follow the instructions; once you're done, execute `rustup` in a terminal (you should be using the same terminal you use to compile froupbox) to check that it's installed:

```
$ rustup
rustup 1.29.0 (2026-03-23)

The Rust toolchain installer
Usage: rustup[EXE] [OPTIONS] [+toolchain] [COMMAND]

...and so on...
```

Now, install the Rust compiler for WebAssembly (WASM):

```
rustup target add wasm32-unknown-unknown
```

If that ran successfully, you should be all set!

# Building

In this directory, run:

```
npx wasm-pack build
```

And you're done! The newly-built module should be automatically picked up when you build froupbox.

# That's cool and all, but I still don't know how to code in Rust

I can't help you with this, but [the official Rust website can](https://rust-lang.org/learn/). Good luck!

# License

The `rust_dsp` part of froupbox is licensed under the [GNU Affero General Public License Version 3](./LICENSE). That's because this code currently only consists of the compressor from [cy!box](https://gitlab.com/cyphers-stuff/cybox), which is AGPLv3. (You should definitely check out cy!box by the way, hint hint wink wink.)
