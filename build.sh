#!/bin/bash

echo "Compiling AssemblyScript Integrity Module..."
asc assembly/index.ts -b build/integrity.wasm --optimize

echo "Compiling C++ Export Module..."
emcc src/handlers/export.cpp -s WASM=1 -s SIDE_MODULE=1 -o build/export.wasm

echo "Compiling Main Rust Worker..."
cargo build --target wasm32-unknown-unknown --release

echo "COMELEC-CTUMC Polyglot Build Complete."
