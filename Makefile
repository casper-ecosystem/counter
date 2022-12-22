prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract-v1 && cargo build --release --target wasm32-unknown-unknown
	cd contract-v2 && cargo build --release --target wasm32-unknown-unknown
	cd counter-call && cargo build --release --target wasm32-unknown-unknown

	wasm-strip contract-v1/target/wasm32-unknown-unknown/release/contract-v1.wasm 2>/dev/null | true
	wasm-strip contract-v2/target/wasm32-unknown-unknown/release/contract-v2.wasm 2>/dev/null | true
	wasm-strip counter-call/target/wasm32-unknown-unknown/release/counter-call.wasm 2>/dev/null | true

test-only:
	cd tests && cargo test

test: build-contract
	mkdir -p tests/wasm
	cp contract-v1/target/wasm32-unknown-unknown/release/counter-v1.wasm tests/wasm
	cp contract-v2/target/wasm32-unknown-unknown/release/counter-v2.wasm tests/wasm
	cp counter-call/target/wasm32-unknown-unknown/release/counter-call.wasm tests/wasm
	cd tests && cargo test
	
clippy:
	cd contract-v1 && cargo clippy --all-targets -- -D warnings
	cd contract-v2 && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract-v1 && cargo fmt -- --check
	cd contract-v2 && cargo fmt -- --check
	cd counter-call && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract-v1 && cargo fmt
	cd contract-v2 && cargo fmt
	cd counter-call && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract-v1 && cargo clean
	cd contract-v2 && cargo clean
	cd counter-call && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
