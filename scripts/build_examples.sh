#!/bin/bash
# Build all Neo N3 smart contract examples

set -e  # Exit on error

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to build a specific example
build_example() {
    local example=$1
    echo -e "${GREEN}Building example: $example${NC}"
    
    cd "$(dirname "$0")/../examples/$example"
    cargo build --target wasm32-unknown-unknown --release
    
    # Create output directory if it doesn't exist
    mkdir -p "../../dist/$example"
    
    # Copy the WASM file
    cp "../../target/wasm32-unknown-unknown/release/$example.wasm" "../../dist/$example/"
    
    echo -e "${GREEN}Successfully built $example${NC}"
    echo ""
}

# Make sure the dist directory exists
mkdir -p "$(dirname "$0")/../dist"

# Build each example
echo -e "${GREEN}Starting to build all examples...${NC}"
echo ""

# Get all examples
cd "$(dirname "$0")/../examples"
for example in */; do
    example=${example%/}
    build_example "$example"
done

echo -e "${GREEN}All examples built successfully!${NC}"
echo -e "Output files can be found in the dist/ directory."
