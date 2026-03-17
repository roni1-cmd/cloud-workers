#!/bin/bash
set -e

echo "Step 1: Compiling Go Validation Logic..."
tinygo build -o build/validator.wasm -target wasm ./src/validator.go

echo "Step 2: Compiling C Crypto Logic..."
clang --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -o build/crypto.wasm src/crypto.c

echo "Step 3: Compiling Rust Core Engine..."
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/comelec_backend.wasm build/main.wasm

echo "Step 4: Deploying to COMELEC-CTUMC Production Environment..."
wrangler deploy --compatibility-date=2026-03-17
