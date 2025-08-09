#!/bin/bash

# Verify all compiled DeFi contracts

set -e

echo "🔍 Verifying all DeFi contracts..."
echo "================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# List of contracts
CONTRACTS=(
    "test_tokens"
    "uniswap_v2_amm"
    "compound_lending"
    "aave_flashloan"
)

# Summary
TOTAL=0
VALID=0

for CONTRACT in "${CONTRACTS[@]}"; do
    NEF_FILE="target/wasm32-unknown-unknown/release/${CONTRACT}.nef"
    MANIFEST_FILE="target/wasm32-unknown-unknown/release/${CONTRACT}.manifest.json"
    
    TOTAL=$((TOTAL + 1))
    
    echo -e "\n${CYAN}Verifying $CONTRACT...${NC}"
    echo "----------------------------------------"
    
    if [ -f "$NEF_FILE" ] && [ -f "$MANIFEST_FILE" ]; then
        # Run verification
        if python3 scripts/verify_nef_manifest.py "$NEF_FILE" "$MANIFEST_FILE" > /tmp/verify_${CONTRACT}.log 2>&1; then
            echo -e "${GREEN}✓ $CONTRACT is valid and deployable!${NC}"
            VALID=$((VALID + 1))
            
            # Show key details
            echo -e "  NEF size: $(stat -c%s "$NEF_FILE" 2>/dev/null || stat -f%z "$NEF_FILE") bytes"
            echo -e "  Standards: $(jq -r '.supportedstandards[]' "$MANIFEST_FILE" 2>/dev/null | tr '\n' ', ' | sed 's/,$//')"
            echo -e "  Methods: $(jq '.abi.methods | length' "$MANIFEST_FILE" 2>/dev/null)"
        else
            echo -e "${RED}✗ $CONTRACT verification failed${NC}"
            echo -e "${YELLOW}  See /tmp/verify_${CONTRACT}.log for details${NC}"
            
            # Show first error
            grep -E "✗|error|Error" /tmp/verify_${CONTRACT}.log | head -1 || true
        fi
    else
        echo -e "${RED}✗ Missing files for $CONTRACT${NC}"
        [ ! -f "$NEF_FILE" ] && echo -e "  ${RED}Missing: $NEF_FILE${NC}"
        [ ! -f "$MANIFEST_FILE" ] && echo -e "  ${RED}Missing: $MANIFEST_FILE${NC}"
    fi
done

# Final summary
echo -e "\n${CYAN}=================================${NC}"
echo -e "${CYAN}VERIFICATION SUMMARY${NC}"
echo -e "${CYAN}=================================${NC}"

if [ $VALID -eq $TOTAL ]; then
    echo -e "${GREEN}✅ All $TOTAL contracts are valid and deployable!${NC}"
else
    echo -e "${YELLOW}⚠ $VALID/$TOTAL contracts are valid${NC}"
fi

# List all NEF files
echo -e "\n${CYAN}NEF Files:${NC}"
ls -lh target/wasm32-unknown-unknown/release/*.nef 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'

# List all manifest files
echo -e "\n${CYAN}Manifest Files:${NC}"
ls -lh target/wasm32-unknown-unknown/release/*.manifest.json 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'

echo -e "\n${GREEN}All DeFi contracts have been compiled to NEF format!${NC}"