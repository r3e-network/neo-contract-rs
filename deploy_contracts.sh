#!/bin/bash

# Neo N3 Rust Framework - Contract Deployment Script
# Deploys all compiled contracts to Neo Express local blockchain

set -e

echo "🚀 Neo N3 Rust Framework - Contract Deployment"
echo "=============================================="

# Set up environment
export PATH="$PATH:$HOME/.dotnet/tools"
NEO_EXPRESS="~/.dotnet/tools/neoxp"

echo "📦 Starting Neo Express blockchain..."
# Start Neo Express in background
$NEO_EXPRESS run --seconds-per-block 1 > neo-express.log 2>&1 &
NEO_EXPRESS_PID=$!

# Wait for blockchain to start
echo "⏳ Waiting for blockchain to initialize..."
sleep 10

# Function to deploy a contract
deploy_contract() {
    local contract_path=$1
    local contract_name=$2
    
    echo "📋 Deploying $contract_name..."
    
    # Extract directory and find NEF and manifest files
    local nef_file="$contract_path.nef"
    local manifest_file="$contract_path.manifest.json"
    
    if [ ! -f "$nef_file" ] || [ ! -f "$manifest_file" ]; then
        echo "  ❌ Files not found: $nef_file or $manifest_file"
        return 1
    fi
    
    echo "  📄 NEF: $nef_file"
    echo "  📄 Manifest: $manifest_file"
    
    # Deploy the contract
    if $NEO_EXPRESS contract deploy "$nef_file" "$manifest_file" owen; then
        echo "  ✅ Deployment successful"
        
        # Get the contract hash
        CONTRACT_HASH=$($NEO_EXPRESS contract get "$contract_name" 2>/dev/null | grep "Contract Hash" | awk '{print $3}' || echo "unknown")
        echo "  🏷️  Contract Hash: $CONTRACT_HASH"
        
        # Store deployment info
        echo "$contract_name,$CONTRACT_HASH,$nef_file,$manifest_file" >> deployed_contracts.csv
        
        return 0
    else
        echo "  ❌ Deployment failed"
        return 1
    fi
}

# Function to invoke a contract method
invoke_contract() {
    local contract_hash=$1
    local method_name=$2
    local params=$3
    
    echo "🔧 Invoking $contract_hash.$method_name($params)"
    
    if [ -z "$params" ]; then
        $NEO_EXPRESS contract invoke "$contract_hash" "$method_name" owen
    else
        $NEO_EXPRESS contract invoke "$contract_hash" "$method_name" $params owen
    fi
}

# Initialize deployment log
echo "contract_name,contract_hash,nef_file,manifest_file" > deployed_contracts.csv

# Counter for statistics
total_deployments=0
successful_deployments=0

echo ""
echo "🚀 Deploying Contracts..."
echo "========================="

# Deploy contracts
contracts=(
    "build/examples/01-hello-world/hello_world_example:HelloWorld"
    "build/examples/04-nep17-token/nep17_token:NEP17Token"
    "build/examples/defi/real-nep17-token/real_nep17_token:RealNEP17"
    "build/examples/defi/real-uniswap-amm/real_uniswap_amm:UniswapAMM"
    "build/examples/defi/real-compound-lending/real_compound_lending:CompoundLending"
    "build/examples/defi/real-aave-flash/real_aave_flash:AaveFlash"
)

for contract_info in "${contracts[@]}"; do
    IFS=':' read -r contract_path contract_name <<< "$contract_info"
    total_deployments=$((total_deployments + 1))
    
    if deploy_contract "$contract_path" "$contract_name"; then
        successful_deployments=$((successful_deployments + 1))
    fi
    echo ""
done

echo "📊 Deployment Results:"
echo "======================"
echo "Total contracts: $total_deployments"
echo "Successful: $successful_deployments"
echo "Failed: $((total_deployments - successful_deployments))"

if [ $successful_deployments -gt 0 ]; then
    echo ""
    echo "🧪 Testing Contract Invocations..."
    echo "=================================="
    
    # Test some basic contract calls
    echo "📋 Deployed contracts:"
    cat deployed_contracts.csv
    
    echo ""
    echo "🔧 Sample contract invocations:"
    
    # Example invocations (will be customized based on deployed contracts)
    # invoke_contract "0x..." "symbol" ""
    # invoke_contract "0x..." "decimals" ""
    # invoke_contract "0x..." "totalSupply" ""
fi

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    if [ ! -z "$NEO_EXPRESS_PID" ]; then
        echo "  🛑 Stopping Neo Express (PID: $NEO_EXPRESS_PID)"
        kill $NEO_EXPRESS_PID 2>/dev/null || true
    fi
    
    echo "📄 Neo Express log saved to: neo-express.log"
    echo "📄 Deployment results saved to: deployed_contracts.csv"
}

# Set trap for cleanup
trap cleanup EXIT

echo ""
echo "💡 Neo Express is running. Press Ctrl+C to stop and cleanup."
echo "   Check neo-express.log for blockchain logs"
echo "   Check deployed_contracts.csv for deployment results"

# Keep script running to maintain blockchain
wait $NEO_EXPRESS_PID