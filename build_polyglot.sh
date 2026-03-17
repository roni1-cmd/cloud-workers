#!/bin/bash

# 1. Compile Zig to Wasm
echo "Compiling Zig Validation Module..."
zig build-lib src/main.zig -target wasm32-freestanding -dynamic --release-small

# 2. Compile Rust Backend
echo "Compiling Rust Core..."
cargo build --target wasm32-unknown-unknown --release

# 3. Deploy Python Worker (if separate)
echo "Deploying Python Analysis Worker..."
npx wrangler deploy src/analysis.py --name ctumc-analysis

echo "Polyglot System Ready for Deployment."
