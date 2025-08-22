#!/bin/bash

echo "📊 Neo N3 Contract Syntax Pattern Analysis"
echo "=========================================="
echo ""

solana_count=$(find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "#\[contract_impl\]" {} \; | wc -l)
extern_count=$(find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "extern.*fn" {} \; | wc -l)  
program_count=$(find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "#\[program\]" {} \; | wc -l)

echo "✅ Solana-style #[contract_impl] examples: $solana_count"
echo "✅ Neo N3 extern C examples: $extern_count"
echo "❌ Traditional #[program] examples: $program_count"
echo ""

total_modern=$((solana_count + extern_count))
echo "📈 Modern pattern coverage: $total_modern/27 examples ($(echo "scale=1; $total_modern * 100 / 27" | bc)%)"
echo ""

if [ $program_count -gt 0 ]; then
    echo "📋 Examples needing conversion:"
    find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "#\[program\]" {} \; | while read file; do
        example=$(echo "$file" | sed 's|.*/examples/\([^/]*\)/.*|\1|')
        echo "  ❌ $example"
    done
    echo ""
fi

echo "✅ Examples with proper Neo N3 + Solana patterns:"
find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "#\[contract_impl\]" {} \; | while read file; do
    example=$(echo "$file" | sed 's|.*/examples/\([^/]*\)/.*|\1|')
    echo "  ✅ $example (Solana-style)"
done

echo ""
echo "✅ Examples with proper Neo N3 native patterns:" 
find /home/neo/git/neo-contract-rs/examples -name "lib.rs" -exec grep -l "extern.*fn" {} \; | while read file; do
    example=$(echo "$file" | sed 's|.*/examples/\([^/]*\)/.*|\1|')
    echo "  ✅ $example (Neo N3 native)"
done