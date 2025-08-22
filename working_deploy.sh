#!/bin/bash

# Working Neo Express deployment using existing wallets

echo "🚀 Neo N3 Working Deployment Test"
echo "================================="

export PATH="$PATH:$HOME/.dotnet/tools"

echo "📦 Resetting and starting Neo Express..."
~/.dotnet/tools/neoxp reset --force
~/.dotnet/tools/neoxp run --seconds-per-block 1 &
NEO_EXPRESS_PID=$!

sleep 15

echo "💰 Checking wallet info..."
~/.dotnet/tools/neoxp wallet list

echo ""
echo "💰 Checking genesis wallet balance..."
~/.dotnet/tools/neoxp show balance NEO genesis || echo "NEO balance check failed"
~/.dotnet/tools/neoxp show balance GAS genesis || echo "GAS balance check failed"

echo ""
echo "📋 Attempting deployment with genesis account..."

# Try to deploy with password=""
echo "" | timeout 30 ~/.dotnet/tools/neoxp contract deploy build/examples/01-hello-world/hello_world_example.nef genesis --password "" 2>&1 || echo "Deploy with empty password failed"

echo ""
echo "📋 Attempting deployment without password flag..."
timeout 30 ~/.dotnet/tools/neoxp contract deploy build/examples/01-hello-world/hello_world_example.nef genesis --force 2>&1 || echo "Deploy without password failed"

echo ""
echo "📋 Final contract list..."
~/.dotnet/tools/neoxp contract list

# Cleanup
kill $NEO_EXPRESS_PID 2>/dev/null || true