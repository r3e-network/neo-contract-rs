#!/bin/bash

echo "🔄 Compiling all WASM files to NEF format..."

for wasm in /home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/*.wasm; do
    if [ -f "$wasm" ]; then
        name=$(basename "$wasm" .wasm)
        echo "✓ Compiling $name to NEF..."
        cargo run -p neo-compiler -- compile "$wasm" -o "build/$name" > /dev/null 2>&1
        if [ -f "build/$name/$name.nef" ]; then
            echo "  ✅ Success: build/$name/"
        else
            echo "  ❌ Failed: $name"
        fi
    fi
done

echo ""
echo "📊 Summary:"
nef_count=$(find build -name "*.nef" -type f | wc -l)
echo "  Total NEF files: $nef_count"
echo "  Total WASM files: $(ls /home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps/*.wasm | wc -l)"