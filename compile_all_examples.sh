#!/bin/bash

# Neo N3 Smart Contract Examples Compilation Script
# Systematically builds all 27 examples to WASM and NEF format

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_EXAMPLES=0
WASM_SUCCESS=0
WASM_FAILED=0
NEF_SUCCESS=0
NEF_FAILED=0

# Arrays to track results
WASM_SUCCESSES=()
WASM_FAILURES=()
NEF_SUCCESSES=()
NEF_FAILURES=()
BUILD_ERRORS=()

# Create build output directory
BUILD_DIR="/home/neo/git/neo-contract-rs/build/examples"
mkdir -p "$BUILD_DIR"

# Results file
RESULTS_FILE="$BUILD_DIR/compilation_results.txt"
echo "Neo N3 Smart Contract Compilation Results" > "$RESULTS_FILE"
echo "=========================================" >> "$RESULTS_FILE"
echo "Timestamp: $(date)" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Function to log results
log_result() {
    echo "$1" | tee -a "$RESULTS_FILE"
}

# Function to build WASM
build_wasm() {
    local example_dir="$1"
    local example_name=$(basename "$example_dir")
    
    echo -e "${BLUE}Building WASM for: $example_name${NC}"
    
    cd "$example_dir"
    
    # Attempt WASM build
    if cargo build --target wasm32-unknown-unknown --release 2>&1; then
        echo -e "${GREEN}✅ WASM build successful: $example_name${NC}"
        WASM_SUCCESSES+=("$example_name")
        ((WASM_SUCCESS++))
        
        # Get WASM file info
        local wasm_file="target/wasm32-unknown-unknown/release/${example_name//-/_}.wasm"
        if [[ -f "$wasm_file" ]]; then
            local wasm_size=$(stat -c%s "$wasm_file" 2>/dev/null || echo "unknown")
            log_result "✅ $example_name: WASM build successful (${wasm_size} bytes)"
            
            # Copy WASM to build directory
            cp "$wasm_file" "$BUILD_DIR/${example_name}.wasm" 2>/dev/null || true
            return 0
        else
            log_result "⚠️  $example_name: WASM build reported success but file not found"
            return 1
        fi
    else
        echo -e "${RED}❌ WASM build failed: $example_name${NC}"
        WASM_FAILURES+=("$example_name")
        ((WASM_FAILED++))
        
        # Capture error
        local error_output=$(cargo build --target wasm32-unknown-unknown --release 2>&1 || true)
        BUILD_ERRORS+=("$example_name: $error_output")
        log_result "❌ $example_name: WASM build failed"
        return 1
    fi
}

# Function to build NEF
build_nef() {
    local example_name="$1"
    local wasm_file="$BUILD_DIR/${example_name}.wasm"
    
    if [[ ! -f "$wasm_file" ]]; then
        echo -e "${YELLOW}⚠️  Skipping NEF build for $example_name (no WASM file)${NC}"
        return 1
    fi
    
    echo -e "${BLUE}Building NEF for: $example_name${NC}"
    
    cd "$BUILD_DIR"
    
    # Attempt NEF compilation
    if /home/neo/git/neo-contract-rs/target/release/neo-compiler compile "$wasm_file" 2>&1; then
        echo -e "${GREEN}✅ NEF build successful: $example_name${NC}"
        NEF_SUCCESSES+=("$example_name")
        ((NEF_SUCCESS++))
        
        # Get NEF file info
        local nef_file="${example_name}.nef"
        if [[ -f "$nef_file" ]]; then
            local nef_size=$(stat -c%s "$nef_file" 2>/dev/null || echo "unknown")
            log_result "✅ $example_name: NEF build successful (${nef_size} bytes)"
            
            # Verify NEF file
            if /home/neo/git/neo-contract-rs/target/release/neo-compiler verify "$nef_file" 2>&1; then
                log_result "✅ $example_name: NEF verification successful"
            else
                log_result "⚠️  $example_name: NEF verification failed"
            fi
            return 0
        else
            log_result "⚠️  $example_name: NEF build reported success but file not found"
            return 1
        fi
    else
        echo -e "${RED}❌ NEF build failed: $example_name${NC}"
        NEF_FAILURES+=("$example_name")
        ((NEF_FAILED++))
        
        # Capture error
        local error_output=$(/home/neo/git/neo-contract-rs/target/release/neo-compiler compile "$wasm_file" 2>&1 || true)
        log_result "❌ $example_name: NEF build failed - $error_output"
        return 1
    fi
}

# Main compilation loop
echo -e "${BLUE}Starting compilation of all Neo N3 smart contract examples...${NC}"
echo ""

# List of all example directories
EXAMPLES=(
    "/home/neo/git/neo-contract-rs/examples/01-hello-world"
    "/home/neo/git/neo-contract-rs/examples/01-hello-world-solana-style"
    "/home/neo/git/neo-contract-rs/examples/01-hello-world-solana-style-simple"
    "/home/neo/git/neo-contract-rs/examples/02-simple-storage"
    "/home/neo/git/neo-contract-rs/examples/02-simple-token"
    "/home/neo/git/neo-contract-rs/examples/03-counter"
    "/home/neo/git/neo-contract-rs/examples/04-nep17-token"
    "/home/neo/git/neo-contract-rs/examples/04-nep17-token-solana-style"
    "/home/neo/git/neo-contract-rs/examples/05-nep11-nft"
    "/home/neo/git/neo-contract-rs/examples/06-nep24-royalty-nft"
    "/home/neo/git/neo-contract-rs/examples/07-crowdfunding"
    "/home/neo/git/neo-contract-rs/examples/08-staking"
    "/home/neo/git/neo-contract-rs/examples/09-simple-dex"
    "/home/neo/git/neo-contract-rs/examples/10-multisig-wallet"
    "/home/neo/git/neo-contract-rs/examples/11-governance"
    "/home/neo/git/neo-contract-rs/examples/12-oracle-price-feed"
    "/home/neo/git/neo-contract-rs/examples/13-nft-marketplace"
    "/home/neo/git/neo-contract-rs/examples/14-neo-features-showcase"
    "/home/neo/git/neo-contract-rs/examples/15-neo-complete-features"
    "/home/neo/git/neo-contract-rs/examples/defi/aave-flashloan"
    "/home/neo/git/neo-contract-rs/examples/defi/compound-lending"
    "/home/neo/git/neo-contract-rs/examples/defi/real-aave-flash"
    "/home/neo/git/neo-contract-rs/examples/defi/real-compound-lending"
    "/home/neo/git/neo-contract-rs/examples/defi/real-nep17-token"
    "/home/neo/git/neo-contract-rs/examples/defi/real-uniswap-amm"
    "/home/neo/git/neo-contract-rs/examples/defi/test-tokens"
    "/home/neo/git/neo-contract-rs/examples/defi/uniswap-v2-amm"
)

TOTAL_EXAMPLES=${#EXAMPLES[@]}

# Phase 1: Build all WASM files
echo -e "${YELLOW}Phase 1: Building WASM files for all examples${NC}"
echo ""

for example_dir in "${EXAMPLES[@]}"; do
    if [[ -d "$example_dir" ]]; then
        build_wasm "$example_dir"
    else
        echo -e "${RED}⚠️  Example directory not found: $example_dir${NC}"
    fi
    echo ""
done

# Phase 2: Build NEF files for successful WASM builds
echo -e "${YELLOW}Phase 2: Building NEF files for successful WASM builds${NC}"
echo ""

for example_name in "${WASM_SUCCESSES[@]}"; do
    build_nef "$example_name"
    echo ""
done

# Generate final report
echo "" >> "$RESULTS_FILE"
log_result "==============================================="
log_result "COMPILATION SUMMARY"
log_result "==============================================="
log_result "Total Examples: $TOTAL_EXAMPLES"
log_result ""
log_result "WASM Compilation:"
log_result "  Successful: $WASM_SUCCESS"
log_result "  Failed: $WASM_FAILED"
log_result "  Success Rate: $(echo "scale=1; $WASM_SUCCESS * 100 / $TOTAL_EXAMPLES" | bc -l)%"
log_result ""
log_result "NEF Compilation:"
log_result "  Successful: $NEF_SUCCESS"
log_result "  Failed: $NEF_FAILED"
log_result "  Success Rate: $(echo "scale=1; $NEF_SUCCESS * 100 / $WASM_SUCCESS" | bc -l)% (of successful WASM builds)"
log_result ""

if [[ ${#WASM_SUCCESSES[@]} -gt 0 ]]; then
    log_result "WASM Build Successes:"
    for success in "${WASM_SUCCESSES[@]}"; do
        log_result "  ✅ $success"
    done
    log_result ""
fi

if [[ ${#WASM_FAILURES[@]} -gt 0 ]]; then
    log_result "WASM Build Failures:"
    for failure in "${WASM_FAILURES[@]}"; do
        log_result "  ❌ $failure"
    done
    log_result ""
fi

if [[ ${#NEF_SUCCESSES[@]} -gt 0 ]]; then
    log_result "NEF Build Successes:"
    for success in "${NEF_SUCCESSES[@]}"; do
        log_result "  ✅ $success"
    done
    log_result ""
fi

if [[ ${#NEF_FAILURES[@]} -gt 0 ]]; then
    log_result "NEF Build Failures:"
    for failure in "${NEF_FAILURES[@]}"; do
        log_result "  ❌ $failure"
    done
    log_result ""
fi

# Final summary
echo -e "${BLUE}===============================================${NC}"
echo -e "${BLUE}FINAL COMPILATION SUMMARY${NC}"
echo -e "${BLUE}===============================================${NC}"
echo -e "Total Examples: ${YELLOW}$TOTAL_EXAMPLES${NC}"
echo -e "WASM Success: ${GREEN}$WASM_SUCCESS${NC} | Failed: ${RED}$WASM_FAILED${NC}"
echo -e "NEF Success: ${GREEN}$NEF_SUCCESS${NC} | Failed: ${RED}$NEF_FAILED${NC}"
echo -e "Overall Success Rate: ${YELLOW}$(echo "scale=1; $NEF_SUCCESS * 100 / $TOTAL_EXAMPLES" | bc -l)%${NC}"
echo ""
echo -e "Detailed results saved to: ${BLUE}$RESULTS_FILE${NC}"
echo -e "Built files available in: ${BLUE}$BUILD_DIR${NC}"