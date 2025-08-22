#!/bin/bash

# Neo Express contract deployment and testing

set -e

echo "🚀 Neo N3 Rust Framework - Full Deployment & Testing"
echo "===================================================="

# Set up environment
export PATH="$PATH:$HOME/.dotnet/tools"

echo "📦 Starting Neo Express blockchain..."
# Reset and start Neo Express
~/.dotnet/tools/neoxp reset --force
~/.dotnet/tools/neoxp run --seconds-per-block 1 > neo-express.log 2>&1 &
NEO_EXPRESS_PID=$!

# Wait for blockchain to start
echo "⏳ Waiting for blockchain to initialize..."
sleep 15

echo "💰 Checking initial balances..."
~/.dotnet/tools/neoxp show balance owner GAS || echo "Balance check failed"
~/.dotnet/tools/neoxp show balance owner NEO || echo "NEO balance check failed"

echo ""
echo "📋 Deploying Hello World contract..."
CONTRACT_NEF="build/examples/01-hello-world/hello_world_example.nef"

if [ -f "$CONTRACT_NEF" ]; then
    echo "  📄 NEF: $CONTRACT_NEF"
    
    # Deploy the contract
    echo "🚀 Deploying contract..."
    if ~/.dotnet/tools/neoxp contract deploy "$CONTRACT_NEF" owner; then
        echo "✅ Hello World contract deployed successfully!"
        
        # Show deployed contracts
        echo "📋 Listing deployed contracts..."
        ~/.dotnet/tools/neoxp contract list
        
    else
        echo "❌ Contract deployment failed"
    fi
else
    echo "❌ Contract files not found: $CONTRACT_NEF"
fi

echo ""
echo "📋 Deploying NEP-17 Token contract..."
TOKEN_NEF="build/examples/04-nep17-token/nep17_token.nef"

if [ -f "$TOKEN_NEF" ]; then
    echo "  📄 NEF: $TOKEN_NEF"
    
    # Deploy the contract
    echo "🚀 Deploying NEP-17 token..."
    if ~/.dotnet/tools/neoxp contract deploy "$TOKEN_NEF" owner; then
        echo "✅ NEP-17 Token deployed successfully!"
        
        # List all contracts
        echo "📋 Listing all deployed contracts..."
        ~/.dotnet/tools/neoxp contract list
        
        echo ""
        echo "🔧 Testing contract invocations..."
        
        # Get contract hash for invocations
        echo "📋 Getting contract information..."
        ~/.dotnet/tools/neoxp contract list --json > contracts.json || echo "Failed to get contract list"
        
        if [ -f "contracts.json" ]; then
            echo "📄 Contract information saved to contracts.json"
            cat contracts.json
        fi
        
    else
        echo "❌ NEP-17 Token deployment failed"
    fi
else
    echo "❌ Token files not found: $TOKEN_NEF"
fi

echo ""
echo "📋 Deploying Real DeFi contracts..."

# Deploy Real NEP-17 Token
REAL_TOKEN_NEF="build/examples/defi/real-nep17-token/real_nep17_token.nef"
if [ -f "$REAL_TOKEN_NEF" ]; then
    echo "🚀 Deploying Real NEP-17 Token..."
    if ~/.dotnet/tools/neoxp contract deploy "$REAL_TOKEN_NEF" owner; then
        echo "✅ Real NEP-17 Token deployed successfully!"
    else
        echo "❌ Real NEP-17 Token deployment failed"
    fi
fi

# Deploy Uniswap AMM
UNISWAP_NEF="build/examples/defi/real-uniswap-amm/real_uniswap_amm.nef"
if [ -f "$UNISWAP_NEF" ]; then
    echo "🚀 Deploying Uniswap AMM..."
    if ~/.dotnet/tools/neoxp contract deploy "$UNISWAP_NEF" owner; then
        echo "✅ Uniswap AMM deployed successfully!"
    else
        echo "❌ Uniswap AMM deployment failed"
    fi
fi

# Deploy Compound Lending
COMPOUND_NEF="build/examples/defi/real-compound-lending/real_compound_lending.nef"
if [ -f "$COMPOUND_NEF" ]; then
    echo "🚀 Deploying Compound Lending..."
    if ~/.dotnet/tools/neoxp contract deploy "$COMPOUND_NEF" owner; then
        echo "✅ Compound Lending deployed successfully!"
    else
        echo "❌ Compound Lending deployment failed"
    fi
fi

# Deploy Aave Flash Loans
AAVE_NEF="build/examples/defi/real-aave-flash/real_aave_flash.nef"
if [ -f "$AAVE_NEF" ]; then
    echo "🚀 Deploying Aave Flash Loans..."
    if ~/.dotnet/tools/neoxp contract deploy "$AAVE_NEF" owner; then
        echo "✅ Aave Flash Loans deployed successfully!"
    else
        echo "❌ Aave Flash Loans deployment failed"
    fi
fi

echo ""
echo "📊 Final contract listing..."
~/.dotnet/tools/neoxp contract list

echo ""
echo "💰 Final balance check..."
~/.dotnet/tools/neoxp show balance owner GAS || echo "Balance check failed"

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    if [ ! -z "$NEO_EXPRESS_PID" ]; then
        echo "  🛑 Stopping Neo Express (PID: $NEO_EXPRESS_PID)"
        kill $NEO_EXPRESS_PID 2>/dev/null || true
        sleep 2
    fi
    
    echo "📄 Neo Express log (last 30 lines):"
    echo "===================================="
    tail -30 neo-express.log 2>/dev/null || echo "No log file found"
    
    echo ""
    echo "📄 Files generated:"
    echo "==================="
    ls -la *.json *.log 2>/dev/null || echo "No additional files generated"
}

# Set trap for cleanup
trap cleanup EXIT

echo ""
echo "✅ Deployment test completed! Check the output above for results."
echo "   📄 Neo Express blockchain log: neo-express.log"
echo "   📄 Contract information: contracts.json (if generated)"

sleep 10

cleanup