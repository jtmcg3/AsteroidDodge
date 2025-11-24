#!/bin/bash

# Build the WASM binary
cargo build --release --target wasm32-unknown-unknown

# Generate JS bindings
wasm-bindgen --out-dir ./out \
  --target web \
  --no-typescript \
  target/wasm32-unknown-unknown/release/asteroid_dodge.wasm

# Optimize the WASM file (optional but recommended)
wasm-opt -Oz -o ./out/asteroid_dodge_bg_optimized.wasm ./out/asteroid_dodge_bg.wasm
mv ./out/asteroid_dodge_bg_optimized.wasm ./out/asteroid_dodge_bg.wasm

# Copy assets if you have any
cp -r assets ./out/