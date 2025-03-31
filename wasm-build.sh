set -e


#rustup target add wasm32-unknown-unknown
#cargo install -f wasm-bindgen-cli
#cargo install simple-http-server

cargo build --target wasm32-unknown-unknown --example request

wasm-bindgen target/wasm32-unknown-unknown/debug/examples/request.wasm --target web --no-typescript --out-dir target/generated --out-name request --debug --keep-debug

simple-http-server . -c wasm,html,js -i --coep --coop --ip 127.0.0.1
