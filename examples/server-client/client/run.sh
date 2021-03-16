#!/bin/bash

# cargo build --target wasm32-unknown-unknown
cargo watch -x 'build --target wasm32-unknown-unknown &&
wasm-bindgen ../target/wasm32-unknown-unknown/debug/client.wasm \
  --out-dir pkg \
  --out-name index \
  --target web &&
http'
# wasm-bindgen ../target/wasm32-unknown-unknown/debug/client.wasm \
#   --out-dir pkg \
#   --out-name index \
#   --target web

# http
