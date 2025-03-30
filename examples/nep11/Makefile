# Copyright @ 2023 - 2024, R3E Network
# All Rights Reserved


PHONY += compile

RUST_FLAGS = "-Ctarget-feature=+multivalue \
    -Cllvm-args=--combiner-store-merging=false \
    -Clink-arg=--initial-memory=262144 \
    -Clink-arg=-zstack-size=131072"

# compile with optimzation Oz and strip, abort on panic
compile:
	RUSTFLAGS=$(RUST_FLAGS) cargo +nightly build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/*.wasm .
