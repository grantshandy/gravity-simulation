all:
	rustup target add wasm32-unknown-unknown
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/gravity-simulation.wasm ./web/