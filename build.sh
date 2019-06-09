rustc --target wasm32-unknown-unknown -O -C debuginfo=0 --crate-type=cdylib demo/demo.rs -o dist/demo.wasm
wasm-gc dist/demo.wasm dist/demo.strip.wasm