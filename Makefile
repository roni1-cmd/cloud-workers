deploy:
	wrangler deploy

dev:
	wrangler dev

build:
	cargo build --target wasm32-unknown-unknown --release
