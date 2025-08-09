#!/bin/bash

set -e

echo "========================================="
echo "   NEO CONTRACT RS - FULL BUILD SYSTEM   "
echo "========================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Change to project root
cd "$(dirname "$0")/.."

# Function to print colored output
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
    fi
}

# Function to compile a contract
compile_contract() {
    local package=$1
    local name=$2
    
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Compiling $name...${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    if cargo build --target wasm32-unknown-unknown --release -p $package 2>&1 | tee /tmp/${package}_build.log | tail -5; then
        print_status 0 "$name compiled to WASM"
        
        # Check if WASM file was generated
        wasm_file="target/wasm32-unknown-unknown/release/${package//-/_}.wasm"
        if [ -f "$wasm_file" ]; then
            print_status 0 "WASM file generated: $(basename $wasm_file)"
            ls -lh "$wasm_file" | awk '{print "   Size: " $5}'
            
            # Generate NEF and manifest
            echo -e "${YELLOW}Generating NEF and manifest...${NC}"
            if python3 scripts/compile_to_nef.py "$wasm_file" 2>&1; then
                print_status 0 "NEF and manifest generated"
            else
                print_status 1 "Failed to generate NEF"
            fi
        else
            print_status 1 "WASM file not found"
        fi
    else
        print_status 1 "$name compilation failed"
        echo "Check /tmp/${package}_build.log for details"
        return 1
    fi
    
    echo ""
}

# Step 1: Build neo-contract core
echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}Step 1: Building Core Library${NC}"
echo -e "${GREEN}=========================================${NC}"
echo ""

compile_contract "neo-contract" "Neo Contract Core"

# Step 2: Build DeFi contracts
echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}Step 2: Building DeFi Contracts${NC}"
echo -e "${GREEN}=========================================${NC}"
echo ""

# List of DeFi contracts to build
declare -a contracts=(
    "uniswap-v2-amm:Uniswap V2 AMM"
    "compound-lending:Compound Lending Protocol"
    "aave-flashloan:Aave Flash Loan"
    "test-tokens:Test Tokens (NEP-17)"
)

# Track success
total_contracts=0
successful_contracts=0

for contract_info in "${contracts[@]}"; do
    IFS=':' read -r package name <<< "$contract_info"
    total_contracts=$((total_contracts + 1))
    
    if compile_contract "$package" "$name"; then
        successful_contracts=$((successful_contracts + 1))
    fi
done

# Step 3: Summary
echo -e "${GREEN}=========================================${NC}"
echo -e "${GREEN}Step 3: Build Summary${NC}"
echo -e "${GREEN}=========================================${NC}"
echo ""

echo "Build Results:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total contracts: $total_contracts"
echo "Successful: $successful_contracts"
echo "Failed: $((total_contracts - successful_contracts))"
echo ""

# List generated files
echo "Generated Files:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo -e "${YELLOW}WASM Files:${NC}"
ls -la target/wasm32-unknown-unknown/release/*.wasm 2>/dev/null | grep -E "(uniswap|compound|aave|test)" | awk '{print "  • " $9 " (" $5 ")"}' || echo "  None found"

echo ""
echo -e "${YELLOW}NEF Files:${NC}"
ls -la target/wasm32-unknown-unknown/release/*.nef 2>/dev/null | awk '{print "  • " $9 " (" $5 ")"}' || echo "  None found"

echo ""
echo -e "${YELLOW}Manifest Files:${NC}"
ls -la target/wasm32-unknown-unknown/release/*.manifest.json 2>/dev/null | awk '{print "  • " $9 " (" $5 ")"}' || echo "  None found"

echo ""
echo -e "${GREEN}=========================================${NC}"

if [ $successful_contracts -eq $total_contracts ]; then
    echo -e "${GREEN}🎉 Build completed successfully!${NC}"
    echo -e "${GREEN}All contracts are ready for deployment.${NC}"
else
    echo -e "${YELLOW}⚠️  Build completed with errors.${NC}"
    echo -e "${YELLOW}Check build logs in /tmp/ for details.${NC}"
fi

echo -e "${GREEN}=========================================${NC}"
echo ""

# Step 4: Deployment instructions
echo "Next Steps:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1. Start Neo Express:"
echo "   neoxp create -f"
echo "   neoxp run --seconds-per-block 1"
echo ""
echo "2. Deploy a contract:"
echo "   neoxp contract deploy target/wasm32-unknown-unknown/release/<contract>.nef alice"
echo ""
echo "3. Invoke contract methods:"
echo "   neoxp contract invoke <contract-hash> <method> [args] alice"
echo ""
echo "4. Check contract storage:"
echo "   neoxp contract storage <contract-hash>"
echo ""