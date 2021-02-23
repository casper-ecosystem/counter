prepare:
	rustup target add wasm32-unknown-unknown

clean:
	cargo clean

lint:
	cargo fmt
	cargo clippy --all-targets --all -- -D warnings -A renamed_and_removed_lints

build-contracts:
	rm -f target/wasm32-unknown-unknown/release/*wasm
	cargo build --release --target wasm32-unknown-unknown

deploy: build-contracts
	cd scripts && ./deploy-smart-contract.sh

increment: build-contracts
	cd scripts && ./increment-counter.sh

check-counter:
	cd scripts && ./check-counter.sh
