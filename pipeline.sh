#!/bin/sh

cd $(dirname $0)

$(cd warmup && cargo run --release)

$(cd gen-native && cargo build --release)

$(cd gen-wasm && RUSTFLAGS= RUSTC_WRAPPER= wasm-pack build --release --target web)

$(cd demo-wasm && bun i && bun run build && bun run serve)
