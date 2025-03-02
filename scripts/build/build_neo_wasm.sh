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
    
    # Check if we're on the jimmy branch
    if [ "$(git rev-parse --abbrev-ref HEAD)" != "jimmy" ]; then
        echo -e "${RED}Warning: Not on jimmy branch. Switching to jimmy branch...${NC}"
        git checkout jimmy
    fi
    
    # Build the neo-wasm compiler
    go build -o ../../bin/neo-wasm
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Successfully built neo-wasm compiler${NC}"
        echo -e "Binary location: $(dirname "$0")/../../bin/neo-wasm"
        echo ""
        return 0
    else
        echo -e "${RED}Failed to build neo-wasm compiler${NC}"
        return 1
    fi
}

# Make sure the bin directory exists
mkdir -p "$(dirname "$0")/../../bin"

# Build neo-wasm
build_neo_wasm
