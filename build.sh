#!/bin/bash
set -e # Exit immediately if a command fails

echo "--- 1. Generating Data Schemas (Protobuf) ---"
protoc --rust_out=src/ --go_out=src/ proto/meeting.proto

echo "--- 2. Compiling AssemblyScript Integrity Module ---"
# Using the optimized flag for high-speed djb2 checksums
npx asc assembly/index.ts -b build/integrity.wasm --optimize

echo "--- 3. Compiling C++ Export & Archive Module ---"
# Using Emscripten to create a side-module for PDF/CSV generation
emcc src/handlers/export.cpp \
     -O3 \
     -s WASM=1 \
     -s SIDE_MODULE=1 \
     -s EXPORTED_FUNCTIONS='["_generate_report"]' \
     -o build/export.wasm

echo "--- 4. Compiling Go (TinyGo) Validation Module ---"
tinygo build -o build/validator.wasm -target wasm ./src/validator.go

echo "--- 5. Compiling Zig Student-ID Verifier ---"
zig build-lib src/main.zig -target wasm32-freestanding -dynamic --release-small
mv main.wasm build/verifier.wasm

echo "--- 6. Compiling Main Rust Worker Orchestrator ---"
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/mom_worker.wasm build/main.wasm

echo "----------------------------------------------"
echo "✅ COMELEC-CTUMC Polyglot Build Complete."
echo "Ready for deployment: 'wrangler deploy'"
