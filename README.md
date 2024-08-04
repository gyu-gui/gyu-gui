## Run Example:

```shell
cargo run --example counter
```

## Run Tests:

```shell
cargo test --package oku_core
```

cargo build --example test_binary --target wasm32-unknown-unknown --release
wasm-bindgen --out-name test_binary --out-dir examples/wasm/target --target web
target/wasm32-unknown-unknown/release/examples/test_binary.wasm
basic-http-server examples/wasm