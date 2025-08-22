#!/bin/bash

# Comprehensive compilation testing script for all Neo N3 examples
set -e

PROJECT_ROOT="/home/neo/git/neo-contract-rs"
cd "$PROJECT_ROOT"

echo "=== Neo N3 Smart Contract Compilation Test ==="
echo "Testing all examples for WASM compilation..."
echo

# Define all examples in priority order
examples=(
    # Simple Examples (01, 02, 03 series)
    "01-hello-world"
    "01-hello-world-solana-style" 
    "01-hello-world-solana-style-simple"
    "02-simple-storage"
    "02-simple-token"
    "03-counter"
    
    # NEP Standards (04, 05, 06 series)
    "04-nep17-token"
    "04-nep17-token-solana-style"
    "05-nep11-nft"
    "06-nep24-royalty-nft"
    
    # Complex Examples (07-15 series)
    "07-crowdfunding"
    "08-staking"
    "09-simple-dex"
    "10-multisig-wallet"
    "11-governance"
    "12-oracle-price-feed"
    "13-nft-marketplace"
    "14-neo-features-showcase"
    "15-neo-complete-features"
    
    # DeFi Examples
    "defi/aave-flashloan"
    "defi/compound-lending"
    "defi/test-tokens"
    "defi/uniswap-v2-amm"
    "defi/real-aave-flash"
    "defi/real-compound-lending"
    "defi/real-nep17-token"
    "defi/real-uniswap-amm"
)

success_count=0
total_count=${#examples[@]}
failed_examples=()

echo "Found ${total_count} examples to test"
echo

# Test each example
for example in "${examples[@]}"; do
    echo -n "Testing ${example}... "
    
    if [ ! -d "examples/${example}" ]; then
        echo "❌ MISSING - Directory not found"
        failed_examples+=("${example}")
        continue
    fi
    
    cd "examples/${example}"
    
    # Test WASM compilation
    if cargo build --target wasm32-unknown-unknown --release --quiet 2>/dev/null; then
        # Check if WASM file was created in project-wide target dir
        package_name=$(grep "^name" Cargo.toml | sed 's/name = "\(.*\)"/\1/' | tr '-' '_')
        wasm_file="$PROJECT_ROOT/target/wasm32-unknown-unknown/release/${package_name}.wasm"
        if [ -f "$wasm_file" ]; then
            echo "✅ SUCCESS ($(basename "$wasm_file"))"
            ((success_count++))
        else
            echo "❌ FAILED - No WASM output ($package_name.wasm not found)"
            failed_examples+=("${example}")
        fi
    else
        echo "❌ FAILED - Compilation error"
        failed_examples+=("${example}")
    fi
    
    cd "$PROJECT_ROOT"
done

echo
echo "=== COMPILATION SUMMARY ==="
echo "Success: ${success_count}/${total_count} ($(( success_count * 100 / total_count ))%)"

if [ ${#failed_examples[@]} -gt 0 ]; then
    echo
    echo "Failed examples:"
    for failed in "${failed_examples[@]}"; do
        echo "  - ${failed}"
    done
    
    echo
    echo "=== DETAILED ERROR ANALYSIS ==="
    for failed in "${failed_examples[@]}"; do
        if [ -d "examples/${failed}" ]; then
            echo
            echo "--- ${failed} ---"
            cd "examples/${failed}"
            cargo build --target wasm32-unknown-unknown --release 2>&1 | grep -E "(error|Error)" | head -5
            cd "$PROJECT_ROOT"
        fi
    done
fi

echo
if [ $success_count -eq $total_count ]; then
    echo "🎉 ALL EXAMPLES COMPILED SUCCESSFULLY!"
    exit 0
else
    echo "⚠️  ${#failed_examples[@]} examples need fixes"
    exit 1
fi