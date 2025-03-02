# Copyright @ 2023 - 2024, R3E Network
# All Rights Reserved

# Makefile for ink_style_complete example

PHONY += compile

RUST_FLAGS = "-Ctarget-feature=+multivalue \
    -Cllvm-args=--combiner-store-merging=false \
    -Clink-arg=--initial-memory=262144 \
    -Clink-arg=-zstack-size=131072"

# compile with optimization Oz and strip, abort on panic
compile:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/ink_style_complete.wasm .

# If wasm-opt is installed, use it to optimize the WASM file
optimize:
	@if command -v wasm-opt > /dev/null; then \
		wasm-opt -Oz -o ink_style_complete.optimized.wasm ink_style_complete.wasm; \
		mv ink_style_complete.optimized.wasm ink_style_complete.wasm; \
		echo "WASM file optimized"; \
	else \
		echo "wasm-opt not found, skipping optimization"; \
	fi

# All-in-one target to build and prepare for deployment
all: compile optimize

.PHONY: $(PHONY) all compile optimize
