PROJECT := documented-token
REPO_ROOT := ../..
NEO_WASM := $(REPO_ROOT)/target/release/neo-wasm

.PHONY: all build compile clean

all: build compile

build:
	RUSTFLAGS="-Ctarget-feature=+multivalue -Cllvm-args=--combiner-store-merging=false -Clink-arg=--initial-memory=262144 -Clink-arg=-zstack-size=131072" cargo +nightly build --release --target wasm32-unknown-unknown

compile: build
	cp $(REPO_ROOT)/target/wasm32-unknown-unknown/release/$(PROJECT).wasm ./$(PROJECT).wasm
	$(NEO_WASM) ./$(PROJECT).wasm

clean:
	rm -f ./$(PROJECT).wasm ./$(PROJECT).nef ./$(PROJECT).manifest.json ./$(PROJECT).neo.asm
	cargo clean