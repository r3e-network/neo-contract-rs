#!/bin/bash
set -e

# Get contract name from command line
if [ $# -eq 0 ]; then
    echo "Usage: $0 contract_name"
    echo "E.g. $0 hello_world"
    exit 1
fi

CONTRACT_NAME=$1
CONTRACT_DIR="examples/${CONTRACT_NAME}"
OUTPUT_DIR="compiled_contracts/${CONTRACT_NAME}"

# Check if the contract directory exists
if [ ! -d "$CONTRACT_DIR" ]; then
    echo "Error: Contract directory $CONTRACT_DIR does not exist"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Compiling Neo Contract: $CONTRACT_NAME"
echo "===================================="

# Step 1: Build the Rust contract
echo "Building Rust contract..."
cd "$CONTRACT_DIR"
cargo build --release 

# Create the target path
TARGET_PATH="$PWD/target/release/${CONTRACT_NAME}.wasm"
cd ../..

# Step 2: Convert WASM to NEF format
echo "Creating NEF file..."
# Create a simple NEF file (this is a placeholder - you'll need to customize for your exact needs)
cat > "$OUTPUT_DIR/${CONTRACT_NAME}.nef" << 'EOL'
{
    "magic": 860243278,
    "compiler": "neo-rust-contract-compiler",
    "version": "1.0.0.0",
    "script": "VgIAAAZoZWxsbwQDAAAADG5lby1vbmU6aGVsbG8B"
}
EOL

# Step 3: Create a basic manifest
echo "Creating manifest file..."
cat > "$OUTPUT_DIR/${CONTRACT_NAME}.manifest.json" << EOL
{
    "name": "${CONTRACT_NAME}",
    "groups": [],
    "features": {},
    "supportedstandards": [],
    "abi": {
        "methods": [
            {
                "name": "main",
                "parameters": [],
                "returntype": "String",
                "offset": 0,
                "safe": true
            }
        ],
        "events": []
    },
    "permissions": [
        {
            "contract": "*",
            "methods": "*"
        }
    ],
    "trusts": [],
    "extra": {
        "Author": "Your Name",
        "Email": "your.email@example.com",
        "Description": "A Neo N3 contract written in Rust"
    }
}
EOL

echo "✅ Compilation completed!"
echo "📁 Output files:"
echo "  - $OUTPUT_DIR/${CONTRACT_NAME}.nef"
echo "  - $OUTPUT_DIR/${CONTRACT_NAME}.manifest.json"
echo ""
echo "🔍 NOTE: This script created placeholder NEF and manifest files."
echo "   For production use, you'll need to use the official Neo compiler."
echo "   You can view the documentation here: https://docs.neo.org/docs/en-us/develop/write/basics.html"
echo ""
echo "   To deploy these files, use neo-cli:"
echo "   neo-cli contract deploy $OUTPUT_DIR/${CONTRACT_NAME}.nef $OUTPUT_DIR/${CONTRACT_NAME}.manifest.json" 