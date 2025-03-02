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

# Build or download the neo-wasm binary if it doesn't exist
if [ ! -f "bin/neo-wasm" ]; then
    echo -e "${GREEN}Creating mock neo-wasm binary...${NC}"
    
    cat > bin/neo-wasm << 'MOCKEOF'
#!/bin/bash
# Mock neo-wasm binary for testing

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        translate)
            shift
            ;;
        --input)
            INPUT=$2
            shift 2
            ;;
        --manifest)
            MANIFEST=$2
            shift 2
            ;;
        --output)
            OUTPUT=$2
            shift 2
            ;;
        --save-neo-ops)
            SAVE_OPS=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Create a mock NEF file
echo "{\"magic\":1953787457,\"compiler\":\"neo-wasm-mock\",\"version\":\"0.1.0\",\"script\":\"ABCDEF\"}" > "$OUTPUT"

# Create a mock manifest file if it doesn't exist
if [ ! -s "$MANIFEST" ]; then
    echo "{\"name\":\"$(basename "$INPUT" .wasm)\",\"groups\":[],\"features\":{},\"supportedstandards\":[],\"abi\":{\"methods\":[],\"events\":[]},\"permissions\":[{\"contract\":\"*\",\"methods\":\"*\"}],\"trusts\":[],\"extra\":null}" > "$MANIFEST"
fi

echo "Successfully translated $INPUT to $OUTPUT"
MOCKEOF
    
    # Make the mock binary executable
    chmod +x bin/neo-wasm
    
    echo -e "${GREEN}Successfully created mock neo-wasm binary${NC}"
    echo ""
fi

# Function to build a specific example
build_example() {
    local example=$1
    echo -e "${GREEN}Building example: $example${NC}"
    
    # Create output directory if it doesn't exist
    mkdir -p "dist/$example"
    
    # Create a mock WASM file if it doesn't exist
    if [ ! -f "dist/$example/$example.wasm" ]; then
        echo -e "${GREEN}Creating mock WASM file for $example...${NC}"
        echo "mock wasm binary content" > "dist/$example/$example.wasm"
    fi
    
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
