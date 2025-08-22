#!/bin/bash

# Fix examples compilation issues

echo "🔧 Fixing Neo N3 Contract Examples..."

# Fix serialization import issues
echo "📦 Adding NeoSerializable imports..."
find examples -name "*.rs" -exec grep -l "to_bytes\|from_bytes\|deserialize" {} \; | while read file; do
    if ! grep -q "use neo_contract::serialize::NeoSerializable" "$file"; then
        echo "  Adding NeoSerializable import to $file"
        sed -i '/use neo_contract::prelude::\*/a use neo_contract::serialize::NeoSerializable;' "$file"
    fi
done

# Fix missing wee_alloc dependency in Cargo.toml files
echo "📦 Adding wee_alloc dependencies..."
find examples -name "Cargo.toml" | while read file; do
    if ! grep -q "wee_alloc" "$file"; then
        echo "  Adding wee_alloc to $file"
        echo 'wee_alloc = "0.4"' >> "$file"
    fi
done

# Fix panic handler and allocator for simple examples
echo "🛠️ Adding panic handlers and allocators..."
simple_examples=(
    "examples/02-simple-storage/src/lib.rs"
    "examples/02-simple-token/src/lib.rs"
    "examples/03-counter/src/lib.rs"
    "examples/01-hello-world-solana-style-simple/src/lib.rs"
)

for file in "${simple_examples[@]}"; do
    if [ -f "$file" ]; then
        if ! grep -q "panic_handler" "$file"; then
            echo "  Adding panic handler to $file"
            # Add allocator and panic handler after imports
            sed -i '/use neo_contract::prelude::\*/a \\n\/\/ WASM global allocator\nextern crate wee_alloc;\n#[global_allocator]\nstatic ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;\n\n\/\/ Panic handler for WASM no_std builds\n#[cfg(target_arch = "wasm32")]\n#[panic_handler]\nfn panic(_: \&core::panic::PanicInfo) -> ! {\n    core::arch::wasm32::unreachable()\n}' "$file"
        fi
        
        if ! grep -q "extern crate alloc" "$file"; then
            echo "  Adding alloc crate to $file"
            sed -i '/^#!\[no_main\]/a \\nextern crate alloc;' "$file"
        fi
    fi
done

echo "✅ Fixed common compilation issues!"
echo "🔄 Next: Run 'cargo build --target wasm32-unknown-unknown --release --workspace' to test fixes"