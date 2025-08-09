#!/bin/bash

# Deploy all DeFi contracts to Neo Express

set -e

echo "🚀 Neo N3 DeFi Contracts Deployment Script"
echo "=========================================="

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

# Check Neo Express
if ! command -v neoxp &> /dev/null; then
    echo -e "${RED}❌ Neo Express not installed${NC}"
    echo -e "${YELLOW}Install with: dotnet tool install Neo.Express -g${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Neo Express found${NC}"

# List of contracts to deploy
CONTRACTS=(
    "test_tokens:NEP-17 Test Token"
    "uniswap_v2_amm:Uniswap V2 AMM Pool"
    "compound_lending:Compound Lending Market"
    "aave_flashloan:Aave Flash Loan Pool"
)

# Contract directory
CONTRACT_DIR="target/wasm32-unknown-unknown/release"

echo -e "\n${CYAN}Contracts to deploy:${NC}"
for CONTRACT_INFO in "${CONTRACTS[@]}"; do
    CONTRACT=$(echo $CONTRACT_INFO | cut -d: -f1)
    DESCRIPTION=$(echo $CONTRACT_INFO | cut -d: -f2)
    echo -e "  • ${CONTRACT} - ${DESCRIPTION}"
done

# Check if Neo Express is running
echo -e "\n${CYAN}Checking Neo Express status...${NC}"
if ! neoxp show state > /dev/null 2>&1; then
    echo -e "${YELLOW}Neo Express not running. Starting...${NC}"
    echo -e "${BLUE}Run: neoxp run --seconds-per-block 1${NC}"
    echo -e "${YELLOW}Please start Neo Express and run this script again${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Neo Express is running${NC}"

# Deploy contracts
echo -e "\n${CYAN}Deploying contracts...${NC}"
echo "----------------------------------------"

DEPLOYED=()
FAILED=()

for CONTRACT_INFO in "${CONTRACTS[@]}"; do
    CONTRACT=$(echo $CONTRACT_INFO | cut -d: -f1)
    DESCRIPTION=$(echo $CONTRACT_INFO | cut -d: -f2)
    
    NEF_FILE="${CONTRACT_DIR}/${CONTRACT}.nef"
    MANIFEST_FILE="${CONTRACT_DIR}/${CONTRACT}.manifest.json"
    
    echo -e "\n${BLUE}Deploying ${CONTRACT}...${NC}"
    
    if [ ! -f "$NEF_FILE" ] || [ ! -f "$MANIFEST_FILE" ]; then
        echo -e "${RED}✗ Missing files for ${CONTRACT}${NC}"
        FAILED+=("$CONTRACT")
        continue
    fi
    
    # Deploy contract
    if neoxp contract deploy "$NEF_FILE" alice 2>/dev/null; then
        echo -e "${GREEN}✓ ${CONTRACT} deployed successfully${NC}"
        DEPLOYED+=("$CONTRACT")
        
        # Get contract hash
        CONTRACT_HASH=$(neoxp contract list 2>/dev/null | grep -i "$CONTRACT" | awk '{print $1}' | head -1)
        if [ ! -z "$CONTRACT_HASH" ]; then
            echo -e "  Contract hash: ${CYAN}${CONTRACT_HASH}${NC}"
        fi
    else
        echo -e "${RED}✗ Failed to deploy ${CONTRACT}${NC}"
        FAILED+=("$CONTRACT")
    fi
done

# Summary
echo -e "\n${CYAN}========================================${NC}"
echo -e "${CYAN}DEPLOYMENT SUMMARY${NC}"
echo -e "${CYAN}========================================${NC}"

if [ ${#DEPLOYED[@]} -gt 0 ]; then
    echo -e "${GREEN}✅ Successfully deployed:${NC}"
    for CONTRACT in "${DEPLOYED[@]}"; do
        echo -e "  • $CONTRACT"
    done
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    echo -e "${RED}❌ Failed to deploy:${NC}"
    for CONTRACT in "${FAILED[@]}"; do
        echo -e "  • $CONTRACT"
    done
fi

# Show deployed contracts
echo -e "\n${CYAN}Deployed contracts:${NC}"
neoxp contract list 2>/dev/null || echo "Unable to list contracts"

# Invocation examples
echo -e "\n${CYAN}========================================${NC}"
echo -e "${CYAN}INVOCATION EXAMPLES${NC}"
echo -e "${CYAN}========================================${NC}"

echo -e "\n${BLUE}NEP-17 Token (test_tokens):${NC}"
echo "  Get symbol:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> symbol [] alice${NC}"
echo "  Get balance:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> balanceOf [\"NXjtqYERuvSWGawjVux8UerNejvwdYg7eE\"] alice${NC}"
echo "  Transfer:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> transfer [\"<from>\", \"<to>\", 1000000, null] alice${NC}"

echo -e "\n${BLUE}Uniswap V2 AMM (uniswap_v2_amm):${NC}"
echo "  Initialize pool:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> initialize_pool [\"<token_a>\", \"<token_b>\", 30] alice${NC}"
echo "  Add liquidity:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> add_liquidity [1000000, 1000000] alice${NC}"
echo "  Swap tokens:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> swap [100000, \"<token_in>\"] alice${NC}"

echo -e "\n${BLUE}Compound Lending (compound_lending):${NC}"
echo "  Initialize market:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> initialize_market [\"<asset>\", \"<model>\"] alice${NC}"
echo "  Supply assets:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> supply [\"<asset>\", 1000000] alice${NC}"
echo "  Borrow assets:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> borrow [\"<asset>\", 500000] alice${NC}"

echo -e "\n${BLUE}Aave Flash Loan (aave_flashloan):${NC}"
echo "  Initialize pool:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> initialize_pool [9] alice${NC}"
echo "  Execute flash loan:"
echo -e "    ${YELLOW}neoxp contract invoke <hash> flash_loan [\"<receiver>\", \"<asset>\", 1000000, null] alice${NC}"

echo -e "\n${GREEN}✅ Deployment complete!${NC}"
echo -e "${CYAN}Replace <hash> with actual contract hashes from 'neoxp contract list'${NC}"