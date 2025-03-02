#!/bin/bash
# Build all Neo N3 smart contract examples

set -e  # Exit on error

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Make sure the bin directory exists
mkdir -p bin

# Make sure the dist directory exists
mkdir -p dist

# Build neo-wasm compiler if it doesn't exist
if [ ! -f "bin/neo-wasm" ]; then
    echo -e "${GREEN}Building neo-wasm compiler...${NC}"
    
    # Try to build the neo-wasm compiler
    if ! scripts/build/build_neo_wasm.sh; then
        echo -e "${RED}Failed to build neo-wasm compiler, trying to download or create a mock...${NC}"
        scripts/build/download_neo_wasm.sh
    fi
fi

# Function to build a specific example
build_example() {
    local example=$1
    echo -e "${GREEN}Building example: $example${NC}"
    
    # Create output directory if it doesn't exist
    mkdir -p "dist/$example"
    
    # Check if the example directory exists
    if [ ! -d "examples/$example" ]; then
        echo -e "${RED}Example directory not found: examples/$example${NC}"
        return 1
    fi
    
    # Build the example
    cd "examples/$example"
    cargo build --target wasm32-unknown-unknown --release
    
    # Copy the WASM file
    cp "../../target/wasm32-unknown-unknown/release/$example.wasm" "../../dist/$example/"
    
    # Return to the root directory
    cd ../..
    
    # Generate manifest and NEF files using neo-wasm
    echo -e "${GREEN}Generating manifest and NEF files for $example...${NC}"
    
    # Create a default manifest file if it doesn't exist
    if [ ! -f "dist/$example/$example.manifest.json" ]; then
        cp templates/default_manifest.json "dist/$example/$example.manifest.json"
        # Update the name in the manifest
        sed -i "s/ExampleContract/$example/g" "dist/$example/$example.manifest.json"
    fi
    
    # Convert WASM to NEF using neo-wasm
    ./bin/neo-wasm translate \
        --input "dist/$example/$example.wasm" \
        --manifest "dist/$example/$example.manifest.json" \
        --output "dist/$example/$example.nef" \
        --save-neo-ops
    
    echo -e "${GREEN}Successfully built $example${NC}"
    echo ""
}

# Build each example
echo -e "${GREEN}Starting to build all examples...${NC}"
echo ""

# Get all examples from the examples directory
for example_dir in examples/*/; do
    # Extract the example name from the directory path
    example=$(basename "$example_dir")
    build_example "$example"
done

echo -e "${GREEN}All examples built successfully!${NC}"
echo -e "Output files can be found in the dist/ directory."
