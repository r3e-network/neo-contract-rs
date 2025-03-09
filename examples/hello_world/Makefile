NAME=$(shell basename $(CURDIR))
OUTPUT_DIR=./build
WASM_TARGET=wasm32-unknown-unknown
BUILD_MODE=debug
NEO_WASM_PATH=../../bin/neo-wasm

.PHONY: all build clean

all: build

build:
	@mkdir -p $(OUTPUT_DIR)
	@echo "Building $(NAME) in $(BUILD_MODE) mode..."
	@cargo build --target $(WASM_TARGET) $(if $(filter release,$(BUILD_MODE)),--release,)
	@cp ./target/$(WASM_TARGET)/$(BUILD_MODE)/$(NAME).wasm $(OUTPUT_DIR)/
	@$(NEO_WASM_PATH) -input $(OUTPUT_DIR)/$(NAME).wasm -output $(OUTPUT_DIR) -name $(NAME) $(if $(filter release,$(BUILD_MODE)),-release,)
	@echo "Build completed: $(OUTPUT_DIR)/$(NAME).nef and $(OUTPUT_DIR)/$(NAME).manifest.json"

clean:
	@rm -rf $(OUTPUT_DIR)
	@cargo clean

# Deploy to Neo Express (requires Neo Express to be installed)
deploy: build
	@echo "Deploying to Neo Express..."
	@neoxp contract deploy $(OUTPUT_DIR)/$(NAME).nef owner

# Run contract methods
run:
	@echo "Running contract..."
	@neoxp contract invoke $(NAME) get_message [] --account owner

set-message:
	@echo "Setting message..."
	@neoxp contract invoke $(NAME) set_message ["\"$(MESSAGE)\""] --account owner

say-hello:
	@echo "Saying hello..."
	@neoxp contract invoke $(NAME) say_hello ["\"$(NAME)\""] --account owner

contract-info:
	@echo "Getting contract info..."
	@neoxp contract invoke $(NAME) contract_info [] --account owner

help:
	@echo "Makefile for $(NAME)"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  build          Build the contract (default)"
	@echo "  clean          Clean build artifacts"
	@echo "  deploy         Deploy contract to Neo Express"
	@echo "  run            Run get_message method"
	@echo "  set-message    Set a new message (use MESSAGE=your_message)"
	@echo "  say-hello      Say hello to someone (use NAME=name)"
	@echo "  contract-info  Get contract info"
	@echo "  help           Show this help message"
	@echo ""
	@echo "Options:"
	@echo "  BUILD_MODE=release    Build in release mode"