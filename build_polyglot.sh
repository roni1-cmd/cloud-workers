#!/bin/bash
# COMELEC-CTUMC Polyglot Deployment Orchestrator
set -e

# 1. Compile Zig Student-ID Verifier
# Targets 'freestanding' because there is no OS in the Worker environment
echo "--- Step 1: Compiling Zig Validation Module ---"
zig build-lib src/main.zig \
    -target wasm32-freestanding \
    -dynamic \
    -O ReleaseSmall
mv main.wasm build/verifier.wasm

# 2. Compile Rust Core Engine
echo "--- Step 2: Compiling Rust Backend ---"
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/mom_worker.wasm build/main.wasm

# 3. Deploy Python Analysis Worker
# This uses the Cloudflare Python Runtime for heavy data lifting (e.g., meeting sentiment)
echo "--- Step 3: Deploying Python Analysis Worker ---"
npx wrangler deploy src/analysis.py \
    --name ctumc-analysis-worker \
    --compatibility-date 2026-03-25

# 4. Final Production Push
echo "--- Step 4: Finalizing Main Worker Deployment ---"
npx wrangler deploy --name comelec-main

echo "------------------------------------------------"
echo "Live view enabled."
