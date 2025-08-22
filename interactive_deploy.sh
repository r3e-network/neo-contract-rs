#!/bin/bash

# Interactive Neo Express deployment
echo "🚀 Neo N3 Interactive Deployment Test"
echo "====================================="

export PATH="$PATH:$HOME/.dotnet/tools"

echo "📦 Resetting and starting Neo Express..."
~/.dotnet/tools/neoxp reset --force
~/.dotnet/tools/neoxp run --seconds-per-block 1 &
NEO_EXPRESS_PID=$!

sleep 15

echo "💰 Checking wallet info..."
~/.dotnet/tools/neoxp wallet list

echo ""
echo "💰 Trying balance with empty password..."
echo "" | ~/.dotnet/tools/neoxp show balance GAS owner || echo "Balance check failed"

echo ""
echo "📋 Deploying Hello World with empty password..."
echo "" | ~/.dotnet/tools/neoxp contract deploy build/examples/01-hello-world/hello_world_example.nef owner || echo "Deployment failed"

echo ""
echo "📋 Contract list after deployment attempt..."
~/.dotnet/tools/neoxp contract list

# Cleanup
kill $NEO_EXPRESS_PID 2>/dev/null || true