#!/bin/bash

# Neo Express Deployment Script
# Deploys compiled NEF contracts to Neo Express

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NEO_EXPRESS_CONFIG="${NEO_EXPRESS_CONFIG:-default.neo-express}"
WALLET_NAME="${WALLET_NAME:-alice}"
BUILD_DIR="${BUILD_DIR:-build}"

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if neo-express is installed
check_neo_express() {
    if ! command -v neoxp &> /dev/null; then
        print_error "Neo Express is not installed"
        print_info "Install with: dotnet tool install Neo.Express -g"
        exit 1
    fi
}

# Initialize Neo Express if needed
init_neo_express() {
    if [ ! -f "$NEO_EXPRESS_CONFIG" ]; then
        print_info "Initializing Neo Express..."
        neoxp create -f
        
        # Create wallets
        print_info "Creating wallets..."
        neoxp wallet create alice
        neoxp wallet create bob
        neoxp wallet create owner
        
        # Fund wallets with GAS
        print_info "Funding wallets..."
        neoxp transfer 1000 GAS genesis alice
        neoxp transfer 1000 GAS genesis bob
        neoxp transfer 1000 GAS genesis owner
    fi
}

# Start Neo Express
start_neo_express() {
    print_info "Starting Neo Express..."
    
    # Check if already running
    if pgrep -f "neo-express" > /dev/null; then
        print_warn "Neo Express is already running"
    else
        neoxp run -s 1 &
        NEO_EXPRESS_PID=$!
        sleep 5  # Wait for Neo Express to start
        print_info "Neo Express started with PID: $NEO_EXPRESS_PID"
    fi
}

# Compile contract
compile_contract() {
    local CONTRACT_PATH=$1
    local CONTRACT_NAME=$(basename "$CONTRACT_PATH")
    
    print_info "Compiling $CONTRACT_NAME..."
    
    # Build WASM
    if [ -d "$CONTRACT_PATH" ]; then
        cd "$CONTRACT_PATH"
        RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152" \
            cargo build --target wasm32-unknown-unknown --release
        cd - > /dev/null
        
        # Find WASM file
        local WASM_FILE=$(find "$CONTRACT_PATH/target/wasm32-unknown-unknown/release" -name "*.wasm" ! -name "*deps*" | head -1)
        
        if [ -f "$WASM_FILE" ]; then
            # Compile to NEF
            print_info "Compiling to NEF..."
            cargo run -p neo-compiler -- compile "$WASM_FILE" --output "$BUILD_DIR"
            return 0
        else
            print_error "WASM file not found for $CONTRACT_NAME"
            return 1
        fi
    else
        print_error "Contract directory not found: $CONTRACT_PATH"
        return 1
    fi
}

# Deploy contract
deploy_contract() {
    local NEF_FILE=$1
    local MANIFEST_FILE="${NEF_FILE%.nef}.manifest.json"
    local CONTRACT_NAME=$(basename "$NEF_FILE" .nef)
    
    if [ ! -f "$NEF_FILE" ]; then
        print_error "NEF file not found: $NEF_FILE"
        return 1
    fi
    
    if [ ! -f "$MANIFEST_FILE" ]; then
        print_error "Manifest file not found: $MANIFEST_FILE"
        return 1
    fi
    
    print_info "Deploying $CONTRACT_NAME..."
    
    # Deploy using Neo Express
    local RESULT=$(neoxp contract deploy "$NEF_FILE" "$WALLET_NAME" --json)
    local CONTRACT_HASH=$(echo "$RESULT" | jq -r '.contractHash')
    
    if [ "$CONTRACT_HASH" != "null" ]; then
        print_info "✅ Contract deployed successfully!"
        print_info "   Contract Hash: $CONTRACT_HASH"
        
        # Save deployment info
        echo "{
    \"name\": \"$CONTRACT_NAME\",
    \"hash\": \"$CONTRACT_HASH\",
    \"deployedAt\": \"$(date -Iseconds)\",
    \"network\": \"neo-express\"
}" > "$BUILD_DIR/${CONTRACT_NAME}.deployment.json"
        
        return 0
    else
        print_error "Failed to deploy contract"
        return 1
    fi
}

# Test contract
test_contract() {
    local CONTRACT_HASH=$1
    local METHOD=$2
    shift 2
    local PARAMS="$@"
    
    print_info "Testing contract $CONTRACT_HASH :: $METHOD..."
    
    # Invoke contract method
    if [ -z "$PARAMS" ]; then
        neoxp contract invoke "$CONTRACT_HASH" "$METHOD" "$WALLET_NAME"
    else
        neoxp contract invoke "$CONTRACT_HASH" "$METHOD" "$WALLET_NAME" -- $PARAMS
    fi
}

# Main deployment function
deploy_all() {
    print_info "Starting deployment process..."
    
    # Create build directory
    mkdir -p "$BUILD_DIR"
    
    # Check prerequisites
    check_neo_express
    init_neo_express
    start_neo_express
    
    # Compile and deploy each example
    local EXAMPLES_DIR="examples"
    local SUCCESS_COUNT=0
    local FAIL_COUNT=0
    
    for CONTRACT_DIR in "$EXAMPLES_DIR"/*; do
        if [ -d "$CONTRACT_DIR" ] && [ -f "$CONTRACT_DIR/Cargo.toml" ]; then
            local CONTRACT_NAME=$(basename "$CONTRACT_DIR")
            
            print_info "Processing $CONTRACT_NAME..."
            
            # Skip Solana-style examples if not ready
            if [[ "$CONTRACT_NAME" == *"solana"* ]]; then
                print_warn "Skipping Solana-style example: $CONTRACT_NAME"
                continue
            fi
            
            # Compile contract
            if compile_contract "$CONTRACT_DIR"; then
                # Find NEF file
                local NEF_FILE="$BUILD_DIR/${CONTRACT_NAME//-/_}.nef"
                
                if [ -f "$NEF_FILE" ]; then
                    # Deploy contract
                    if deploy_contract "$NEF_FILE"; then
                        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
                        print_info "✅ $CONTRACT_NAME deployed successfully"
                    else
                        FAIL_COUNT=$((FAIL_COUNT + 1))
                        print_error "❌ Failed to deploy $CONTRACT_NAME"
                    fi
                else
                    print_error "NEF file not found: $NEF_FILE"
                    FAIL_COUNT=$((FAIL_COUNT + 1))
                fi
            else
                FAIL_COUNT=$((FAIL_COUNT + 1))
            fi
            
            echo ""
        fi
    done
    
    # Summary
    print_info "="
    print_info "Deployment Summary:"
    print_info "  Successful: $SUCCESS_COUNT"
    print_info "  Failed: $FAIL_COUNT"
    print_info "="
    
    if [ $FAIL_COUNT -eq 0 ]; then
        print_info "🎉 All contracts deployed successfully!"
    else
        print_warn "⚠️ Some contracts failed to deploy"
    fi
}

# Parse command line arguments
case "${1:-all}" in
    all)
        deploy_all
        ;;
    contract)
        if [ -z "$2" ]; then
            print_error "Usage: $0 contract <path-to-contract>"
            exit 1
        fi
        compile_contract "$2"
        ;;
    deploy)
        if [ -z "$2" ]; then
            print_error "Usage: $0 deploy <nef-file>"
            exit 1
        fi
        deploy_contract "$2"
        ;;
    test)
        if [ -z "$2" ] || [ -z "$3" ]; then
            print_error "Usage: $0 test <contract-hash> <method> [params...]"
            exit 1
        fi
        test_contract "$2" "$3" "${@:4}"
        ;;
    stop)
        print_info "Stopping Neo Express..."
        pkill -f "neo-express" || true
        ;;
    *)
        echo "Usage: $0 [all|contract|deploy|test|stop]"
        echo ""
        echo "Commands:"
        echo "  all              - Compile and deploy all contracts"
        echo "  contract <path>  - Compile a specific contract"
        echo "  deploy <nef>     - Deploy a compiled NEF file"
        echo "  test <hash> <method> [params] - Test a deployed contract"
        echo "  stop             - Stop Neo Express"
        exit 1
        ;;
esac