# Master Build Configuration for MOM-System
# Target: WebAssembly (Wasm32)

all: proto build-go build-c build-zig build-as build-rust

# 1. Protobuf Generation (The Data Schema)
proto:
	protoc --rust_out=src/ --go_out=src/ --cpp_out=src/ proto/meeting.proto

# 2. Go (TinyGo) - Header Validation & Analytics
build-go:
	tinygo build -o build/validator.wasm -target wasm ./src/validator.go
	tinygo build -o build/analytics.wasm -target wasm ./src/handlers/analytics.go

# 3. C (Clang) - Security & Obfuscation
build-c:
	clang --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -o build/crypto.wasm src/crypto.c

# 4. Zig - Student ID & Voter Pattern Matching
build-zig:
	zig build-lib src/main.zig -target wasm32-freestanding -dynamic --release-small
	mv main.wasm build/verifier.wasm

# 5. AssemblyScript - Fast Checksums
build-as:
	npx asc assembly/index.ts -b build/integrity.wasm --optimize

# 6. Rust (Cargo) - Core Orchestrator
build-rust:
	cargo build --target wasm32-unknown-unknown --release

# 7. Deployment to Cloudflare Edge
deploy:
	wrangler deploy

# 8. Local Cleanup
clean:
	rm -rf build/*.wasm
	cargo clean
