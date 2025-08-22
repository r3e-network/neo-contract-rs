#!/bin/bash

# Fix missing WASM boilerplate in examples 09-13
examples_to_fix=(
    "10-multisig-wallet"
    "11-governance"
    "12-oracle-price-feed"
    "13-nft-marketplace"
)

boilerplate='
extern crate alloc;
use neo_contract::prelude::*;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}'

for example in "${examples_to_fix[@]}"; do
    lib_file="examples/${example}/src/lib.rs"
    if [ -f "$lib_file" ]; then
        echo "Fixing $example..."
        
        # Check if it already has WASM boilerplate
        if ! grep -q "#\[global_allocator\]" "$lib_file"; then
            # Add boilerplate after the first use statement
            sed -i "/^use neo_contract::prelude::\*;/a\\$boilerplate" "$lib_file"
            echo "  ✅ Added WASM boilerplate"
        else
            echo "  ⏭️  Already has boilerplate"
        fi
    else
        echo "  ❌ File not found: $lib_file"
    fi
done

echo "✅ WASM boilerplate fixes complete"