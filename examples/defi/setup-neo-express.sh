#!/bin/bash

# Setup Neo Express for DeFi Testing
echo "Setting up Neo Express for DeFi contract testing..."

# Check if neo-express is installed
if ! command -v neo-express &> /dev/null; then
    echo "Installing Neo Express..."
    dotnet tool install Neo.Express -g
fi

# Create Neo Express instance
echo "Creating Neo Express blockchain instance..."
neo-express create defi-testnet -f

# Start Neo Express
echo "Starting Neo Express..."
neo-express run -i defi-testnet.neo-express -s 1 &
NEO_EXPRESS_PID=$!

# Wait for Neo Express to start
sleep 5

# Create test wallets
echo "Creating test wallets..."
neo-express wallet create alice -i defi-testnet.neo-express
neo-express wallet create bob -i defi-testnet.neo-express
neo-express wallet create charlie -i defi-testnet.neo-express
neo-express wallet create defi-deployer -i defi-testnet.neo-express

# Transfer NEO and GAS to wallets
echo "Transferring NEO and GAS to test wallets..."
neo-express transfer 1000 NEO genesis alice -i defi-testnet.neo-express
neo-express transfer 1000 GAS genesis alice -i defi-testnet.neo-express
neo-express transfer 1000 NEO genesis bob -i defi-testnet.neo-express
neo-express transfer 1000 GAS genesis bob -i defi-testnet.neo-express
neo-express transfer 1000 NEO genesis charlie -i defi-testnet.neo-express
neo-express transfer 1000 GAS genesis charlie -i defi-testnet.neo-express
neo-express transfer 10000 NEO genesis defi-deployer -i defi-testnet.neo-express
neo-express transfer 10000 GAS genesis defi-deployer -i defi-testnet.neo-express

echo "Neo Express setup complete!"
echo "Neo Express PID: $NEO_EXPRESS_PID"
echo ""
echo "Wallets created:"
echo "  - alice"
echo "  - bob"
echo "  - charlie"
echo "  - defi-deployer"
echo ""
echo "To stop Neo Express: kill $NEO_EXPRESS_PID"