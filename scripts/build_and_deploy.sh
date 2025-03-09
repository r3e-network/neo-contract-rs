#!/bin/bash
# Build and deploy script for Neo Contract Rust
# This script helps build and deploy smart contracts to Neo blockchain

set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
BIN_DIR="$ROOT_DIR/bin"
NEO_WASM="$BIN_DIR/neo-wasm"

# Default values
CONTRACT_PATH=""
NETWORK="express"
BUILD_MODE="debug"
WALLET="owner"
METHOD=""
PARAMS=""
OPERATION="build"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Print colored message
print_message() {
    echo -e "${GREEN}==>${NC} $1"
}

print_error() {
    echo -e "${RED}==>${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}==>${NC} $1"
}

# Print usage
print_usage() {
    echo "Neo Contract Rust Build and Deploy Script"
    echo ""
    echo "Usage: $0 [options] [contract_path]"
    echo ""
    echo "Operations:"
    echo "  -b, --build          Build contract (default)"
    echo "  -d, --deploy         Build and deploy contract"
    echo "  -i, --invoke         Invoke a contract method"
    echo "  -c, --clean          Clean build artifacts"
    echo ""
    echo "Options:"
    echo "  -p, --path PATH      Path to contract (required)"
    echo "  -n, --network NET    Network to deploy to (express, testnet, mainnet) (default: express)"
    echo "  -m, --mode MODE      Build mode (debug, release) (default: debug)"
    echo "  -w, --wallet WALLET  Wallet to use for deployment (default: owner)"
    echo "  --method METHOD      Method to invoke (required for --invoke)"
    echo "  --params PARAMS      Parameters for method invocation (JSON array string)"
    echo "  -h, --help           Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --path examples/hello_world                   # Build hello_world contract"
    echo "  $0 --path examples/hello_world --mode release    # Build in release mode"
    echo "  $0 --deploy --path examples/hello_world          # Deploy to Neo Express"
    echo "  $0 --invoke --path examples/hello_world --method get_message  # Invoke get_message"
}

# Check if neo-wasm is available
check_neo_wasm() {
    if [ ! -f "$NEO_WASM" ]; then
        print_error "neo-wasm tool not found at $NEO_WASM"
        print_message "Building neo-wasm tool..."
        
        # Build neo-wasm
        if [ -f "$SCRIPT_DIR/build/build_neo_wasm.sh" ]; then
            bash "$SCRIPT_DIR/build/build_neo_wasm.sh"
        else
            print_error "build_neo_wasm.sh script not found"
            exit 1
        fi
    fi
}

# Check if contract path has a Makefile
has_makefile() {
    [ -f "$CONTRACT_PATH/Makefile" ]
}

# Build contract
build_contract() {
    print_message "Building contract: $CONTRACT_PATH (${BUILD_MODE})"
    
    # Check if directory exists
    if [ ! -d "$CONTRACT_PATH" ]; then
        print_error "Contract directory not found: $CONTRACT_PATH"
        exit 1
    fi
    
    # Create build directory
    mkdir -p "$CONTRACT_PATH/build"
    
    # Change to contract directory
    pushd "$CONTRACT_PATH" > /dev/null
    
    # Check if Makefile exists and use it
    if has_makefile; then
        print_message "Using Makefile"
        make BUILD_MODE="$BUILD_MODE"
    else
        # Get contract name from directory name if not specified
        CONTRACT_NAME=$(basename "$CONTRACT_PATH")
        print_message "Building using cargo and neo-wasm for $CONTRACT_NAME"
        
        # Build with cargo
        if [ "$BUILD_MODE" = "release" ]; then
            cargo build --target wasm32-unknown-unknown --release
            WASM_FILE="target/wasm32-unknown-unknown/release/$CONTRACT_NAME.wasm"
            NEO_WASM_FLAGS="-release"
        else
            cargo build --target wasm32-unknown-unknown
            WASM_FILE="target/wasm32-unknown-unknown/debug/$CONTRACT_NAME.wasm"
            NEO_WASM_FLAGS=""
        fi
        
        # Check if WASM file exists
        if [ ! -f "$WASM_FILE" ]; then
            print_error "WASM file not found: $WASM_FILE"
            popd > /dev/null
            exit 1
        fi
        
        # Convert WASM to NEF
        "$NEO_WASM" -input "$WASM_FILE" -output "build" -name "$CONTRACT_NAME" $NEO_WASM_FLAGS
    fi
    
    # Check build results
    CONTRACT_NAME=$(basename "$CONTRACT_PATH")
    if [ ! -f "build/$CONTRACT_NAME.nef" ] || [ ! -f "build/$CONTRACT_NAME.manifest.json" ]; then
        print_error "Build failed for $CONTRACT_NAME"
        popd > /dev/null
        exit 1
    fi
    
    print_message "Successfully built $CONTRACT_NAME"
    print_message "  - build/$CONTRACT_NAME.nef"
    print_message "  - build/$CONTRACT_NAME.manifest.json"
    
    popd > /dev/null
}

# Deploy contract
deploy_contract() {
    CONTRACT_NAME=$(basename "$CONTRACT_PATH")
    print_message "Deploying contract: $CONTRACT_NAME to $NETWORK"
    
    # Build contract first
    build_contract
    
    # Deploy based on network
    case "$NETWORK" in
        express|dev)
            # Deploy to Neo Express
            print_message "Deploying to Neo Express"
            if has_makefile; then
                pushd "$CONTRACT_PATH" > /dev/null
                make deploy NEOXP_WALLET="$WALLET"
                popd > /dev/null
            else
                neoxp contract deploy "$CONTRACT_PATH/build/$CONTRACT_NAME.nef" "$WALLET.wallet.json" --password "$WALLET"
            fi
            ;;
        testnet)
            # Deploy to TestNet
            print_message "Deploying to TestNet"
            print_warning "TestNet deployment currently requires neo-cli or NeoLine."
            print_message "NEF file: $CONTRACT_PATH/build/$CONTRACT_NAME.nef"
            print_message "Manifest file: $CONTRACT_PATH/build/$CONTRACT_NAME.manifest.json"
            ;;
        mainnet)
            # Deploy to MainNet
            print_message "Deploying to MainNet"
            print_warning "MainNet deployment currently requires neo-cli or NeoLine."
            print_message "NEF file: $CONTRACT_PATH/build/$CONTRACT_NAME.nef"
            print_message "Manifest file: $CONTRACT_PATH/build/$CONTRACT_NAME.manifest.json"
            ;;
        *)
            print_error "Unknown network: $NETWORK"
            exit 1
            ;;
    esac
}

# Invoke contract method
invoke_contract() {
    CONTRACT_NAME=$(basename "$CONTRACT_PATH")
    print_message "Invoking method: $METHOD on contract $CONTRACT_NAME"
    
    # Check if method is specified
    if [ -z "$METHOD" ]; then
        print_error "No method specified. Use --method to specify the method to invoke."
        exit 1
    fi
    
    # Invoke based on network
    case "$NETWORK" in
        express|dev)
            # Invoke on Neo Express
            if has_makefile; then
                pushd "$CONTRACT_PATH" > /dev/null
                make invoke METHOD="$METHOD" PARAMS="$PARAMS" NEOXP_WALLET="$WALLET"
                popd > /dev/null
            else
                if [ -z "$PARAMS" ]; then
                    neoxp contract invoke "$CONTRACT_NAME" "$METHOD" --account "$WALLET" --password "$WALLET"
                else
                    neoxp contract invoke "$CONTRACT_NAME" "$METHOD" "$PARAMS" --account "$WALLET" --password "$WALLET"
                fi
            fi
            ;;
        testnet|mainnet)
            print_warning "$NETWORK invocation currently requires neo-cli or NeoLine."
            print_message "Contract: $CONTRACT_NAME"
            print_message "Method: $METHOD"
            print_message "Parameters: $PARAMS"
            ;;
        *)
            print_error "Unknown network: $NETWORK"
            exit 1
            ;;
    esac
}

# Clean contract
clean_contract() {
    CONTRACT_NAME=$(basename "$CONTRACT_PATH")
    print_message "Cleaning contract: $CONTRACT_NAME"
    
    if has_makefile; then
        pushd "$CONTRACT_PATH" > /dev/null
        make clean
        popd > /dev/null
    else
        rm -rf "$CONTRACT_PATH/build"
        rm -rf "$CONTRACT_PATH/target"
    fi
}

# Parse command-line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -b|--build)
                OPERATION="build"
                shift
                ;;
            -d|--deploy)
                OPERATION="deploy"
                shift
                ;;
            -i|--invoke)
                OPERATION="invoke"
                shift
                ;;
            -c|--clean)
                OPERATION="clean"
                shift
                ;;
            -p|--path)
                CONTRACT_PATH="$2"
                shift 2
                ;;
            -n|--network)
                NETWORK="$2"
                shift 2
                ;;
            -m|--mode)
                BUILD_MODE="$2"
                shift 2
                ;;
            -w|--wallet)
                WALLET="$2"
                shift 2
                ;;
            --method)
                METHOD="$2"
                shift 2
                ;;
            --params)
                PARAMS="$2"
                shift 2
                ;;
            -h|--help)
                print_usage
                exit 0
                ;;
            *)
                # If it's a path without the --path flag
                if [ -d "$1" ]; then
                    CONTRACT_PATH="$1"
                else
                    print_error "Unknown option: $1"
                    print_usage
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # Check if contract path is specified
    if [ -z "$CONTRACT_PATH" ]; then
        print_error "No contract path specified. Use --path to specify the contract path."
        print_usage
        exit 1
    fi
}

# Main function
main() {
    # Parse command-line arguments
    parse_args "$@"
    
    # Check neo-wasm
    check_neo_wasm
    
    # Perform operation
    case "$OPERATION" in
        build)
            build_contract
            ;;
        deploy)
            deploy_contract
            ;;
        invoke)
            invoke_contract
            ;;
        clean)
            clean_contract
            ;;
        *)
            print_error "Unknown operation: $OPERATION"
            print_usage
            exit 1
            ;;
    esac
}

# Run main function
main "$@"