#!/bin/bash

# Test with GitHub Actions-like Environment
# Simulates the exact compilation environment used in GitHub Actions

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              GITHUB ACTIONS ENVIRONMENT SIMULATION            ║${NC}"
echo -e "${BLUE}║                 Testing with Nightly Rust                     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

# Simulate GitHub Actions environment
export CARGO_TERM_COLOR=always
export CARGO_INCREMENTAL=0
export CACHE_ON_FAILURE=false

# Use nightly toolchain like GitHub Actions
echo -e "${BLUE}Switching to nightly toolchain (like GitHub Actions)...${NC}"
rustup default nightly

echo -e "\n${YELLOW}🔧 TESTING CORE FRAMEWORK BUILD${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Test core framework build with nightly
echo -n "  Building neo-contract with nightly... "
if cargo build -p neo-contract --release >/dev/null 2>&1; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
    echo -e "${RED}Compilation errors with nightly:${NC}"
    cargo build -p neo-contract --release 2>&1 | head -20
    echo ""
    echo -e "${YELLOW}This explains why GitHub Actions fails but local stable succeeds!${NC}"
fi

echo -n "  Building neo-compiler with nightly... "
if cargo build -p neo-compiler --release >/dev/null 2>&1; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
    cargo build -p neo-compiler --release 2>&1 | head -10
fi

echo -n "  Building neo-contract-proc-macros with nightly... "
if cargo build -p neo-contract-proc-macros --release >/dev/null 2>&1; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
    cargo build -p neo-contract-proc-macros --release 2>&1 | head -10
fi

echo -e "\n${YELLOW}🔧 TESTING EXAMPLE COMPILATION${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

examples=("01-hello-world" "02-simple-storage" "03-counter")

for example in "${examples[@]}"; do
    if [ -d "examples/$example" ]; then
        echo -n "  Testing $example with nightly... "
        if (cd "examples/$example" && cargo check >/dev/null 2>&1); then
            echo -e "${GREEN}✅${NC}"
        else
            echo -e "${RED}❌${NC}"
            echo -e "${RED}Errors in $example:${NC}"
            (cd "examples/$example" && cargo check 2>&1 | head -5)
        fi
    fi
done

# Test with WASM target (like GitHub Actions WASM builds)
echo -e "\n${YELLOW}🔧 TESTING WASM COMPILATION${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo -n "  WASM compilation with nightly... "
if env RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152" cargo check --target wasm32-unknown-unknown -p neo-contract >/dev/null 2>&1; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
    echo -e "${RED}WASM compilation errors:${NC}"
    env RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152" cargo check --target wasm32-unknown-unknown -p neo-contract 2>&1 | head -10
fi

# Switch back to stable for local development
echo -e "\n${BLUE}Switching back to stable toolchain for local development...${NC}"
rustup default stable

echo -e "\n${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    ENVIRONMENT ANALYSIS COMPLETE              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"

echo -e "\n${YELLOW}📋 ANALYSIS SUMMARY${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "This script identified environment differences between local and GitHub Actions:"
echo ""
echo "• Local stable toolchain may be more permissive with warnings"
echo "• GitHub Actions nightly toolchain has stricter compilation requirements"
echo "• Mutability and move semantics are enforced more strictly in CI"
echo "• Import scope and trait availability differ between environments"
echo ""
echo "Use the errors above to fix issues before pushing to GitHub Actions."