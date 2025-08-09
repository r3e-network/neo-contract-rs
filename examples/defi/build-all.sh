#!/bin/bash

echo "Building all DeFi contracts..."

# Set build flags for WASM
export RUSTFLAGS="-C target-feature=+multivalue -C link-arg=--export-all"

# Function to build a contract
build_contract() {
    local name=$1
    local dir=$2
    
    echo "Building $name..."
    cd "$dir" || exit 1
    
    # Build for wasm32 target
    cargo build --target wasm32-unknown-unknown --release
    
    if [ $? -eq 0 ]; then
        echo "✅ $name built successfully"
        
        # Copy WASM file to output directory
        mkdir -p ../build
        cp target/wasm32-unknown-unknown/release/*.wasm ../build/${name}.wasm 2>/dev/null || true
    else
        echo "❌ Failed to build $name"
    fi
    
    cd - > /dev/null
}

# Build each contract
build_contract "uniswap-v2-amm" "uniswap-v2-amm"
build_contract "compound-lending" "compound-lending"
build_contract "aave-flashloan" "aave-flashloan"
build_contract "test-token-usdt" "test-tokens"
build_contract "test-token-wbtc" "test-tokens"
build_contract "test-token-weth" "test-tokens"

echo ""
echo "Build complete! WASM files are in the build/ directory"
ls -la build/*.wasm 2>/dev/null || echo "No WASM files found"