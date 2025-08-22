#!/bin/bash

# Test compilation of all Neo N3 smart contract examples
# This script tests each example and reports compilation status

echo "Testing compilation of all Neo N3 smart contract examples..."
echo "============================================================="

# Track results
passing=0
failing=0
failed_examples=()

# Find all Cargo.toml files (excluding target directories)
examples=$(find . -name "Cargo.toml" -not -path "./target/*" | sort)

echo "Found $(echo "$examples" | wc -l) examples to test"
echo ""

for toml in $examples; do
    dir=$(dirname "$toml")
    example_name=$(basename "$dir")
    
    echo "Testing: $example_name ($dir)"
    echo "----------------------------------------"
    
    cd "$dir"
    
    # Try to build for WASM target
    if cargo build --target wasm32-unknown-unknown --release --quiet 2>/dev/null; then
        echo "✅ PASS: $example_name"
        ((passing++))
    else
        echo "❌ FAIL: $example_name"
        echo "   Error details:"
        cargo build --target wasm32-unknown-unknown --release 2>&1 | head -10 | sed 's/^/   /'
        ((failing++))
        failed_examples+=("$example_name")
    fi
    
    echo ""
    cd - > /dev/null
done

echo "============================================================="
echo "SUMMARY:"
echo "✅ Passing: $passing examples"
echo "❌ Failing: $failing examples"
echo "Total: $((passing + failing)) examples"
echo ""

if [ ${#failed_examples[@]} -gt 0 ]; then
    echo "Failed examples:"
    for failed in "${failed_examples[@]}"; do
        echo "  - $failed"
    done
fi