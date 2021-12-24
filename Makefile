prepare:
	rustup target add wasm32-unknown-unknown

build-contracts:
	cd contracts/counter-define && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contracts/counter-define/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true
	cd contracts/counter-call && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contracts/counter-call/target/wasm32-unknown-unknown/release/contract.wasm 2>/dev/null | true

test-only:
	cd tests && cargo test

test: build-contracts
	mkdir -p tests/wasm
	cp contracts/counter-define/target/wasm32-unknown-unknown/release/counter-define.wasm tests/wasm
	cp contracts/counter-call/target/wasm32-unknown-unknown/release/counter-call.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contracts/counter-define && cargo clippy --all-targets -- -D warnings
	cd contracts/counter-call && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts/counter-define && cargo fmt -- --check
	cd contracts/counter-call && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contracts/counter-define && cargo fmt
	cd contracts/counter-call && cargo fmt
	cd tests && cargo fmt

clean:
	cd contracts/counter-define && cargo clean
	cd contracts/counter-call && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
