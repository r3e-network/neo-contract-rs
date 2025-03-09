# Makefile for the compilation example

# Include the template makefile
include ../../templates/Makefile.template

# Additional targets can be added below
.PHONY: clean-all

# Build with the neo-compiler
build-with-compiler: build-wasm
	@echo "Building with neo-compiler..."
	@mkdir -p dist
	cargo run --package neo-compiler -- compile \
		--wasm-file target/wasm32-unknown-unknown/release/compilation_example.wasm \
		--output-dir dist \
		--name compilation_example \
		--verbose

# Clean all build artifacts
clean-all: clean
	@echo "Cleaning compiler build artifacts..."
	@rm -rf dist