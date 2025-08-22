#!/bin/bash

# Simple Neo Express contract deployment test

set -e

echo "🚀 Neo N3 Contract Deployment Test"
echo "=================================="

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
~/.dotnet/tools/neoxp show balance owner || echo "Balance check failed"

echo "📋 Deploying Hello World contract..."
CONTRACT_NEF="build/examples/01-hello-world/hello_world_example.nef"
CONTRACT_MANIFEST="build/examples/01-hello-world/hello_world_example.manifest.json"

if [ -f "$CONTRACT_NEF" ] && [ -f "$CONTRACT_MANIFEST" ]; then
    echo "  📄 NEF: $CONTRACT_NEF"
    echo "  📄 Manifest: $CONTRACT_MANIFEST"
    
    # Deploy the contract
    echo "🚀 Deploying contract..."
    if ~/.dotnet/tools/neoxp contract deploy "$CONTRACT_NEF" "$CONTRACT_MANIFEST" owner; then
        echo "✅ Contract deployed successfully!"
        
        # Show deployed contracts
        echo "📋 Showing contract information..."
        ~/.dotnet/tools/neoxp contract list || echo "Contract list failed"
        
    else
        echo "❌ Contract deployment failed"
    fi
else
    echo "❌ Contract files not found: $CONTRACT_NEF or $CONTRACT_MANIFEST"
fi

echo ""
echo "📋 Deploying NEP-17 Token contract..."
TOKEN_NEF="build/examples/04-nep17-token/nep17_token.nef"
TOKEN_MANIFEST="build/examples/04-nep17-token/nep17_token.manifest.json"

if [ -f "$TOKEN_NEF" ] && [ -f "$TOKEN_MANIFEST" ]; then
    echo "  📄 NEF: $TOKEN_NEF"
    echo "  📄 Manifest: $TOKEN_MANIFEST"
    
    # Deploy the contract
    echo "🚀 Deploying NEP-17 token..."
    if ~/.dotnet/tools/neoxp contract deploy "$TOKEN_NEF" "$TOKEN_MANIFEST" owner; then
        echo "✅ NEP-17 Token deployed successfully!"
        
        # Test some token methods
        echo "🔧 Testing token methods..."
        
        # Get the contract hash (we'll need to extract it from output)
        echo "📋 Listing all contracts..."
        ~/.dotnet/tools/neoxp contract list
        
    else
        echo "❌ NEP-17 Token deployment failed"
    fi
else
    echo "❌ Token files not found: $TOKEN_NEF or $TOKEN_MANIFEST"
fi

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    if [ ! -z "$NEO_EXPRESS_PID" ]; then
        echo "  🛑 Stopping Neo Express (PID: $NEO_EXPRESS_PID)"
        kill $NEO_EXPRESS_PID 2>/dev/null || true
        sleep 2
    fi
    
    echo "📄 Neo Express log:"
    echo "=================="
    tail -20 neo-express.log 2>/dev/null || echo "No log file found"
}

# Set trap for cleanup
trap cleanup EXIT

echo ""
echo "✅ Test completed! Check the output above for results."
sleep 5

cleanup