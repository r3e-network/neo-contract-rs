#!/bin/bash

echo "🔍 Analyzing compilation status of all 27 examples..."
echo ""

successful=0
failed=0

echo "✅ Successfully Compiled Examples:"
find /home/neo/git/neo-contract-rs/examples -name "Cargo.toml" | while read toml; do
    dir=$(dirname "$toml")
    name=$(basename "$dir")
    wasm_name="${name//-/_}"
    
    if [ -f "/home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/${wasm_name}.wasm" ]; then
        echo "  ✅ $name"
        successful=$((successful + 1))
    fi
done

echo ""
echo "❌ Failed Examples:"
find /home/neo/git/neo-contract-rs/examples -name "Cargo.toml" | while read toml; do
    dir=$(dirname "$toml")
    name=$(basename "$dir")
    wasm_name="${name//-/_}"
    
    if [ ! -f "/home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/${wasm_name}.wasm" ]; then
        echo "  ❌ $name"
        failed=$((failed + 1))
    fi
done

echo ""
echo "📊 Summary:"
echo "  Total examples: 27"
echo "  Working WASM files: $(ls /home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/*.wasm | wc -l)"
echo "  Working NEF files: $(find build -name "*.nef" | wc -l)"
echo "  Success rate: $(echo "scale=1; $(ls /home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/*.wasm | wc -l) * 100 / 27" | bc)%"