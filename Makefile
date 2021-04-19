prepare:
	rustup target add wasm32-unknown-unknown

clean:
	cargo clean

lint:
	cargo fmt
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

build-contracts:
	cargo build --release --target wasm32-unknown-unknown