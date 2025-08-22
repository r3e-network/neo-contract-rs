#!/bin/bash

# Fix Cargo.toml syntax for wee_alloc dependency

echo "🔧 Fixing Cargo.toml dependency syntax..."

find examples -name "Cargo.toml" -exec grep -l 'wee_alloc = "0.4"' {} \; | while read file; do
    echo "  Fixing $file"
    # Remove the incorrectly added line
    sed -i '/^wee_alloc = "0.4"$/d' "$file"
    
    # Add it properly in dependencies section
    if ! grep -q "\[dependencies\]" "$file"; then
        echo "" >> "$file"
        echo "[dependencies]" >> "$file"
    fi
    
    # Add wee_alloc dependency if not already there
    if ! grep -A 10 "\[dependencies\]" "$file" | grep -q "wee_alloc"; then
        sed -i '/\[dependencies\]/a wee_alloc = "0.4"' "$file"
    fi
done

echo "✅ Fixed Cargo.toml syntax!"