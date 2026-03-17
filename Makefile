all: build-go build-c build-rust

build-go:
	tinygo build -o build/validator.wasm -target wasm ./src/validator.go

build-c:
	clang --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -o build/crypto.wasm src/crypto.c

build-rust:
	cargo build --target wasm32-unknown-unknown --release

deploy:
	wrangler deploy
