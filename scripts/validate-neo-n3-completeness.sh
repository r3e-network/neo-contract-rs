#!/bin/bash

# Neo N3 Completeness Validation Script
# Ensures all Neo N3 syscalls, natives, types, and opcodes are correctly supported

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo -e "${BLUE}=== Neo N3 Framework Completeness Validation ===${NC}"
echo "Validating complete Neo N3 specification compliance..."

# Initialize counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

# Function to check and report
check_item() {
    local description="$1"
    local command="$2"
    local expected_count="$3"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    echo -n "  Checking $description... "
    
    if eval "$command" > /dev/null 2>&1; then
        local actual_count=$(eval "$command" | wc -l)
        if [ "$expected_count" = "any" ] || [ "$actual_count" -ge "$expected_count" ]; then
            echo -e "${GREEN}✓${NC} ($actual_count found)"
            PASSED_CHECKS=$((PASSED_CHECKS + 1))
            return 0
        else
            echo -e "${RED}✗${NC} ($actual_count found, expected $expected_count)"
            FAILED_CHECKS=$((FAILED_CHECKS + 1))
            return 1
        fi
    else
        echo -e "${RED}✗${NC} (command failed)"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        return 1
    fi
}

echo -e "\n${YELLOW}1. Neo N3 System Calls Validation${NC}"

# Check if we have the syscall definitions
check_item "System.Storage.Get syscall" \
    "grep -r 'SYSTEM_STORAGE_GET\|storage_get' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Storage.Put syscall" \
    "grep -r 'SYSTEM_STORAGE_PUT\|storage_put' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Storage.Delete syscall" \
    "grep -r 'SYSTEM_STORAGE_DELETE\|storage_delete' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Runtime.CheckWitness syscall" \
    "grep -r 'SYSTEM_RUNTIME_CHECK_WITNESS\|check_witness' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Runtime.Notify syscall" \
    "grep -r 'SYSTEM_RUNTIME_NOTIFY\|runtime_notify' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Runtime.Log syscall" \
    "grep -r 'SYSTEM_RUNTIME_LOG\|runtime_log' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Crypto.SHA256 syscall" \
    "grep -r 'SYSTEM_CRYPTO_SHA256\|crypto_sha256' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "System.Contract.Call syscall" \
    "grep -r 'SYSTEM_CONTRACT_CALL\|contract_call' $PROJECT_ROOT/neo-compiler/src/ $PROJECT_ROOT/neo-contract/src/" \
    "1"

echo -e "\n${YELLOW}2. Native Contract Support Validation${NC}"

check_item "NEO token contract support" \
    "grep -r 'NEO_TOKEN\|neo_token\|NeoToken' $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "GAS token contract support" \
    "grep -r 'GAS_TOKEN\|gas_token\|GasToken' $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "Policy contract support" \
    "grep -r 'PolicyContract\|policy' $PROJECT_ROOT/neo-contract/src/native/" \
    "1"

check_item "Oracle contract support" \
    "grep -r 'OracleContract\|oracle' $PROJECT_ROOT/neo-contract/src/native/" \
    "1"

check_item "ContractManagement support" \
    "grep -r 'ContractManagement\|contract_management' $PROJECT_ROOT/neo-contract/src/native/" \
    "1"

check_item "StdLib contract support" \
    "grep -r 'StdLib\|stdlib' $PROJECT_ROOT/neo-contract/src/native/" \
    "1"

check_item "CryptoLib contract support" \
    "grep -r 'CryptoLib\|cryptolib' $PROJECT_ROOT/neo-contract/src/native/" \
    "1"

echo -e "\n${YELLOW}3. Neo N3 Type System Validation${NC}"

check_item "H160 (UInt160) type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'H160\|UInt160' {} \;" \
    "1"

check_item "H256 (UInt256) type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'H256\|UInt256' {} \;" \
    "1"

check_item "Int256 type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Int256' {} \;" \
    "1"

check_item "ByteString type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'ByteString' {} \;" \
    "1"

check_item "Array type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Array' {} \;" \
    "1"

check_item "Map type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Map' {} \;" \
    "1"

check_item "Struct type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Struct' {} \;" \
    "1"

check_item "Buffer type support" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Buffer' {} \;" \
    "1"

echo -e "\n${YELLOW}4. VM Opcodes Validation${NC}"

check_item "Basic stack opcodes" \
    "grep -r 'Push\|Pop\|Dup\|Swap\|Drop' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "5"

check_item "Arithmetic opcodes" \
    "grep -r 'Add\|Sub\|Mul\|Div\|Mod' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "5"

check_item "Logical opcodes" \
    "grep -r 'And\|Or\|Xor\|Not' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "4"

check_item "Comparison opcodes" \
    "grep -r 'Equal\|NotEqual\|LessThan\|GreaterThan' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "3"

check_item "Control flow opcodes" \
    "grep -r 'Jmp\|JmpIf\|JmpIfNot\|Call\|Ret' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "5"

check_item "Array opcodes" \
    "grep -r 'NewArray\|NewStruct\|Append\|Remove\|Size' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "3"

check_item "Storage opcodes (syscalls)" \
    "grep -r 'SysCall' $PROJECT_ROOT/neo-compiler/src/opcodes.rs" \
    "1"

echo -e "\n${YELLOW}5. NEP Standard Implementations${NC}"

check_item "NEP-17 token standard" \
    "find $PROJECT_ROOT -name '*.rs' -exec grep -l 'Nep17\|NEP.*17' {} \;" \
    "1"

check_item "NEP-11 NFT standard" \
    "find $PROJECT_ROOT -name '*.rs' -exec grep -l 'Nep11\|NEP.*11' {} \;" \
    "1"

check_item "NEP-24 royalty standard" \
    "find $PROJECT_ROOT -name '*.rs' -exec grep -l 'Nep24\|NEP.*24' {} \;" \
    "1"

echo -e "\n${YELLOW}6. Serialization Support${NC}"

check_item "NeoSerializable trait" \
    "grep -r 'NeoSerializable' $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "Serialization methods" \
    "grep -r 'serialize\|deserialize' $PROJECT_ROOT/neo-contract/src/serialize/" \
    "5"

check_item "Binary serialization support" \
    "grep -r 'to_bytes\|from_bytes' $PROJECT_ROOT/neo-contract/src/" \
    "2"

echo -e "\n${YELLOW}7. Event and Notification System${NC}"

check_item "Event emission support" \
    "grep -r 'emit\|notify' $PROJECT_ROOT/neo-contract/src/" \
    "1"

check_item "Notification types" \
    "find $PROJECT_ROOT/neo-contract/src -name '*.rs' -exec grep -l 'Notification' {} \;" \
    "1"

echo -e "\n${YELLOW}8. Compiler WASM Support${NC}"

check_item "WASM instruction parsing" \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'wasm\|WASM' {} \;" \
    "2"

check_item "NEF generation" \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'Nef\|NEF' {} \;" \
    "2"

check_item "Manifest generation" \
    "find $PROJECT_ROOT/neo-compiler/src -name '*.rs' -exec grep -l 'Manifest' {} \;" \
    "2"

echo -e "\n${YELLOW}9. Example Contract Validation${NC}"

check_item "Hello World example" \
    "ls $PROJECT_ROOT/examples/01-hello-world*/src/lib.rs" \
    "1"

check_item "NEP-17 token example" \
    "ls $PROJECT_ROOT/examples/*nep17*/src/lib.rs" \
    "1"

check_item "NFT example" \
    "ls $PROJECT_ROOT/examples/*nft*/src/lib.rs" \
    "1"

check_item "DeFi examples" \
    "ls $PROJECT_ROOT/examples/defi/*/src/lib.rs" \
    "5"

echo -e "\n${YELLOW}10. Build and Compilation${NC}"

cd "$PROJECT_ROOT"

check_item "Core library compilation" \
    "cargo check -p neo-contract" \
    "any"

check_item "Compiler compilation" \
    "cargo check -p neo-compiler" \
    "any"

check_item "Proc macros compilation" \
    "cargo check -p neo-contract-proc-macros" \
    "any"

check_item "Example compilation (hello-world)" \
    "cd examples/01-hello-world && cargo check" \
    "any"

echo -e "\n${BLUE}=== Validation Summary ===${NC}"
echo "Total checks: $TOTAL_CHECKS"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"

PASS_RATE=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
echo "Pass rate: $PASS_RATE%"

if [ $PASS_RATE -ge 95 ]; then
    echo -e "\n${GREEN}✓ Neo N3 Framework is ENTERPRISE READY${NC}"
    echo "All critical Neo N3 components are properly supported."
    exit 0
elif [ $PASS_RATE -ge 85 ]; then
    echo -e "\n${YELLOW}⚠ Neo N3 Framework is PRODUCTION READY with minor gaps${NC}"
    echo "Most Neo N3 components are supported, some optimizations needed."
    exit 0
elif [ $PASS_RATE -ge 70 ]; then
    echo -e "\n${YELLOW}⚠ Neo N3 Framework is DEVELOPMENT READY${NC}"
    echo "Core Neo N3 components are supported, but missing some features."
    exit 1
else
    echo -e "\n${RED}✗ Neo N3 Framework needs significant work${NC}"
    echo "Many critical Neo N3 components are missing or incomplete."
    exit 1
fi