.PHONY: all build clean

all: build

build:
	@echo "Building oracle-price-feed..."
	@RUSTFLAGS="-C link-arg=-zstack-size=32768" cargo build --release --target wasm32-unknown-unknown
	@mkdir -p ./build
	@cp ../target/wasm32-unknown-unknown/release/oracle_price_feed.wasm ./build/
	@ls -la ./build/
	@echo "Build complete. WASM file is in ./build/"

clean:
	@echo "Cleaning up..."
	@rm -rf ./build
	@echo "Clean complete."

test:
	@echo "Running tests..."
	@cargo test
	@echo "Tests complete." 