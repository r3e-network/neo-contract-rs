#!/bin/bash

echo "❌ Remaining Failed Examples:"

all_examples=(
    "01-hello-world"
    "01-hello-world-solana-style" 
    "01-hello-world-solana-style-simple"
    "02-simple-storage"
    "02-simple-token"
    "03-counter"
    "04-nep17-token"
    "04-nep17-token-solana-style"
    "05-nep11-nft"
    "06-nep24-royalty-nft"
    "07-crowdfunding"
    "08-staking"
    "09-simple-dex"
    "10-multisig-wallet"
    "11-governance"
    "12-oracle-price-feed"
    "13-nft-marketplace"
    "14-neo-features-showcase"
    "15-neo-complete-features"
    "defi/aave-flashloan"
    "defi/compound-lending"
    "defi/real-aave-flash"
    "defi/real-compound-lending"
    "defi/real-nep17-token"
    "defi/real-uniswap-amm"
    "defi/test-tokens"
    "defi/uniswap-v2-amm"
)

working_wasm=($(find /home/neo/git/neo-contract-rs/target/wasm32-unknown-unknown/release/deps -name "*.wasm" -exec basename {} .wasm \;))

failed_count=0
for example in "${all_examples[@]}"; do
    wasm_name="${example//-/_}"
    wasm_name="${wasm_name//\//_}"
    wasm_name="${wasm_name//defi_/}"
    
    found=false
    for wasm in "${working_wasm[@]}"; do
        if [[ "$wasm" == *"$wasm_name"* ]] || [[ "$wasm_name" == *"$wasm"* ]]; then
            found=true
            break
        fi
    done
    
    if [ "$found" = false ]; then
        echo "  ❌ $example"
        failed_count=$((failed_count + 1))
    fi
done

echo ""
echo "📊 Final Status:"
echo "  Working: ${#working_wasm[@]}/27"
echo "  Failed: $failed_count/27"
echo "  Success Rate: $(echo "scale=1; ${#working_wasm[@]} * 100 / 27" | bc)%"