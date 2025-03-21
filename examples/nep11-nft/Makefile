# Copyright @ 2024 - present, R3E Network
# All Rights Reserved

PHONY += compile

RUST_FLAGS = "-Ctarget-feature=+multivalue \
    -Cllvm-args=--combiner-store-merging=false \
    -Clink-arg=--initial-memory=262144 \
    -Clink-arg=-zstack-size=131072 \
    --cfg=target_arch=\"wasm32\""

# compile with optimization and proper wasm targeting
compile:
	@rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RUST_FLAGS) cargo build --release --target wasm32-unknown-unknown --no-default-features --features wasm
	cp target/wasm32-unknown-unknown/release/nep11_nft.wasm ./nep11_nft.wasm

clean:
	cargo clean
