# Kotor2 Autosplitter

An auto splitter for Star Wars: Knights of the Old Republic II: The Sith Lords.
- Pauses the timer when in a loading or saving screen, unless an AMG is activated or the window is clicked out of.
- Includes settings for splitting on the first time entering each module, as well as unlimited splits for every module in the game.
- Auto starts on loading the New Game into the prologue and ends time when killing Traya in the final module.

## Release

Latest version will always be found at https://github.com/RedMage08/kotor2-autosplitter-wasm/releases/latest/download/k2.wasm

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b --release
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/k2.wasm
```

Make sure to look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.
