#!/bin/bash
# Build the neo-wasm compiler

set -e  # Exit on error

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to build neo-wasm
build_neo_wasm() {
    echo -e "${GREEN}Building neo-wasm compiler...${NC}"
    
    # Navigate to the neo-wasm directory
    cd "$(dirname "$0")/../../tools/neo-wasm"
    
    # Build the neo-wasm compiler
    go build -o ../../bin/neo-wasm
    
    echo -e "${GREEN}Successfully built neo-wasm compiler${NC}"
    echo ""
}

# Make sure the bin directory exists
mkdir -p "$(dirname "$0")/../../bin"

# Build neo-wasm
build_neo_wasm
