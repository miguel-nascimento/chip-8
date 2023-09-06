# Chip-8

A simple Chip-8 emulator written in Rust and compiled to WebAssembly. 

It passes all the test from [Timendus Chip-8 test suite](https://github.com/Timendus/chip8-test-suite) and it works with PONG too.


> Lack of sound effects.

Chip-8 doesnt have a good sound system, so I decided to not implement it.

## Usage 

```bash
cd wasm/web
python3 -m http.server
```

Then open your browser at `http://localhost:8000/` and you should see the emulator running.

## Wasm build

In case you want to build the wasm module again, you can do it with the following commands:

```bash
cd wasm
wasm-pack build --target web
mv ./pkg/wasm_bg.wasm ./web
mv ./pkg/wasm.js ./web
```

### More info
Good reading:

- [Wikipedia Page](https://en.wikipedia.org/wiki/CHIP-8)
- [Write a Chip-8 Emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Matthew Mikolay Chip 8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference)
- [awesome-chip-8](https://github.com/tobiasvl/awesome-chip-8)