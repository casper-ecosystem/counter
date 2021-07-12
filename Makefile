prepare:
	rustup target add wasm32-unknown-unknown

build-AS-contract:
	npm run --prefix counter-as/counter-call asbuild
	npm run --prefix counter-as/counter-define asbuild

build-RS-contract:
	cargo build --manifest-path counter-rs/Cargo.toml --release -p counter-define --target wasm32-unknown-unknown
	cargo build --manifest-path counter-rs/Cargo.toml --release -p counter-call --target wasm32-unknown-unknown

test-only:
	cargo test --manifest-path counter-rs/Cargo.toml -p tests

copy-RS-wasm-file-to-test:
	cp counter-rs/target/wasm32-unknown-unknown/release/counter*.wasm counter-rs/tests/wasm

copy-AS-wasm-file-to-test:
	cp counter-as/counter-call/dist/counter-call.wasm counter-rs/tests/wasm
	cp counter-as/counter-define/dist/counter-define.wasm counter-rs/tests/wasm

test-RS: build-RS-contract copy-RS-wasm-file-to-test test-only
test-AS: build-AS-contract copy-AS-wasm-file-to-test test-only

clippy:
	cargo clippy --manifest-path counter-rs/Cargo.toml --all-targets --all -- -D warnings -A renamed_and_removed_lints

check-lint: clippy
	cargo fmt --manifest-path counter-rs/Cargo.toml --all -- --check

lint: clippy
	cargo fmt --manifest-path counter-rs/Cargo.toml --all
	
clean:
	cargo clean --manifest-path counter-rs/Cargo.toml
	rm -rf counter-rs/tests/wasm/*.wasm
