# hello-wasm

### Build
```sh
cargo build --target wasm32-unknown-unknown --release
```

### Run the wasm-gc to optimize the wasm file
```sh
wasm-gc target/wasm32-unknown-unknown/release/hello_wasm.wasm -o hello_wasm.gc.wasm
```

