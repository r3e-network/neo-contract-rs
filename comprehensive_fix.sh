#!/bin/bash

# Comprehensive fix for all remaining compilation issues
PROJECT_ROOT="/home/neo/git/neo-contract-rs"
cd "$PROJECT_ROOT"

echo "=== Comprehensive Neo N3 Compilation Fixes ==="

# Examples with duplicate error code issues (need conversion from Solana to Neo style)
solana_style_examples=(
    "06-nep24-royalty-nft"
    "07-crowdfunding"
    "08-staking"
    "14-neo-features-showcase"
    "15-neo-complete-features"
)

# Examples needing WASM boilerplate (manually add since script had issues)
wasm_examples=(
    "10-multisig-wallet"
    "11-governance"
    "12-oracle-price-feed"
    "13-nft-marketplace"
)

echo "Phase 1: Adding WASM boilerplate to examples..."
for example in "${wasm_examples[@]}"; do
    lib_file="examples/${example}/src/lib.rs"
    if [ -f "$lib_file" ]; then
        echo "Checking $example..."
        if ! grep -q "#\[global_allocator\]" "$lib_file"; then
            echo "  ⚠️  Missing WASM boilerplate - needs manual fix"
        else
            echo "  ✅ Has WASM boilerplate"
        fi
    fi
done

echo
echo "Phase 2: Checking Solana-style examples for duplicate errors..."
for example in "${solana_style_examples[@]}"; do
    lib_file="examples/${example}/src/lib.rs"
    if [ -f "$lib_file" ]; then
        echo "Checking $example..."
        cd "examples/${example}"
        error_output=$(cargo build --target wasm32-unknown-unknown --release 2>&1 | grep -E "(error|Error)" | head -2)
        if echo "$error_output" | grep -q "defined multiple times"; then
            echo "  ⚠️  Has duplicate error definitions - needs Solana->Neo conversion"
        elif echo "$error_output" | grep -q "error"; then
            echo "  ⚠️  Has other compilation errors"
            echo "    $error_output"
        else
            echo "  ✅ Compiles successfully"
        fi
        cd "$PROJECT_ROOT"
    fi
done

echo
echo "Phase 3: Testing current status..."
success_count=0
total_examples=27

examples=(
    "01-hello-world" "01-hello-world-solana-style" "01-hello-world-solana-style-simple"
    "02-simple-storage" "02-simple-token" "03-counter"
    "04-nep17-token" "04-nep17-token-solana-style" "05-nep11-nft" "06-nep24-royalty-nft"
    "07-crowdfunding" "08-staking" "09-simple-dex" "10-multisig-wallet" "11-governance"
    "12-oracle-price-feed" "13-nft-marketplace" "14-neo-features-showcase" "15-neo-complete-features"
    "defi/aave-flashloan" "defi/compound-lending" "defi/test-tokens" "defi/uniswap-v2-amm"
    "defi/real-aave-flash" "defi/real-compound-lending" "defi/real-nep17-token" "defi/real-uniswap-amm"
)

for example in "${examples[@]}"; do
    if [ -d "examples/${example}" ]; then
        cd "examples/${example}"
        package_name=$(grep "^name" Cargo.toml | sed 's/name = "\(.*\)"/\1/' | tr '-' '_')
        if cargo build --target wasm32-unknown-unknown --release --quiet 2>/dev/null; then
            wasm_file="$PROJECT_ROOT/target/wasm32-unknown-unknown/release/${package_name}.wasm"
            if [ -f "$wasm_file" ]; then
                echo "✅ $example"
                ((success_count++))
            else
                echo "⚠️  $example (compiles but no WASM output)"
            fi
        else
            echo "❌ $example (compilation error)"
        fi
        cd "$PROJECT_ROOT"
    else
        echo "❓ $example (directory not found)"
    fi
done

echo
echo "=== SUMMARY ==="
echo "Successfully compiling: $success_count/$total_examples"
echo "Success rate: $(( success_count * 100 / total_examples ))%"

if [ $success_count -lt $total_examples ]; then
    echo
    echo "Remaining issues need manual fixes:"
    echo "  - Solana-style examples need complete conversion to Neo N3 syntax"
    echo "  - Missing WASM boilerplate needs manual addition"
    echo "  - Some examples may have framework compatibility issues"
fi