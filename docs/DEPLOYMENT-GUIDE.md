# Deploying NEO Smart Contracts

This guide explains how to deploy your NEO N3 smart contracts using the documentation-first approach.

## Overview

Deploying a smart contract to the NEO N3 blockchain involves several steps:
1. Compiling your Rust contract to WebAssembly (WASM)
2. Converting the WASM to NEO Execution Format (NEF)
3. Testing on a private network
4. Deploying to a testnet
5. Deploying to mainnet

## Documentation-First Deployment Approach

Following our documentation-first philosophy, we recommend:

1. **Document Deployment Requirements First**: Define your contract's deployment needs before implementation
2. **Create Deployment Checklist**: Document all steps needed for a successful deployment
3. **Document Verification Procedures**: Define how to verify your contract is working correctly
4. **Document Monitoring Plans**: Plan how to monitor your contract after deployment

## Compilation Process

### Step 1: Build the WebAssembly Binary

First, compile your Rust contract to WebAssembly:

```bash
# Navigate to your contract directory
cd examples/token_with_annotations

# Build for WebAssembly target
cargo build --release --target wasm32-unknown-unknown
```

This produces a `.wasm` file in `target/wasm32-unknown-unknown/release/`.

### Step 2: Convert to NEO Execution Format (NEF)

Use the NEO compiler to convert the WASM file to NEF:

```bash
# From the project root
neo-compiler compile target/wasm32-unknown-unknown/release/token_with_annotations.wasm --output build/
```

This generates:
- `token_with_annotations.nef`: The NEO Executable Format file
- `token_with_annotations.manifest.json`: The contract manifest
- `token_with_annotations.nefdbgnfo`: Debug information (optional)

## Local Testing with Neo Express

Before deploying to a public network, test your contract in a private development network:

### Step 1: Set Up Neo Express

Install Neo Blockchain Toolkit and create a private network:

```bash
# Install Neo Express (if not already installed)
dotnet tool install -g Neo.Express

# Create a private network with 1 node
neoxp create --count 1

# Start the network
neoxp run
```

### Step 2: Create Wallet and Get GAS

```bash
# Create a wallet for deployment
neoxp wallet create owner

# Check wallet details
neoxp wallet show owner

# Give the wallet some GAS (only on private networks)
neoxp transfer gas 1000 genesis owner
```

### Step 3: Deploy the Contract

```bash
# Deploy the contract using the NEF file
neoxp contract deploy build/token_with_annotations.nef owner
```

### Step 4: Invoke Contract Methods for Testing

```bash
# Get contract hash (copy from deployment output)
CONTRACT_HASH="0x1234567890123456789012345678901234567890"

# Check token symbol
neoxp contract invoke $CONTRACT_HASH symbol [] --account owner

# Check total supply
neoxp contract invoke $CONTRACT_HASH totalSupply [] --account owner

# Transfer tokens
neoxp contract invoke $CONTRACT_HASH transfer '["NZNos2WqTbu5oCgyfss9kUJhwU4nyYL39w", "NhxK8rNVnWjfdczokZkKJK6zYHc5h4cXxw", 100]' --account owner
```

## Testnet Deployment

After local testing, deploy to a public testnet:

### Step 1: Set Up Neo-CLI

Download and set up Neo-CLI:

```bash
# Download Neo-CLI (example path)
wget https://github.com/neo-project/neo-node/releases/download/v3.5.0/neo-cli-linux-x64.zip

# Extract
unzip neo-cli-linux-x64.zip
cd neo-cli

# Start Neo-CLI in testnet mode
./neo-cli -testnet
```

### Step 2: Set Up Wallet

```bash
# Create or open a wallet
wallet create path/to/wallet.json

# Open existing wallet
wallet open path/to/wallet.json

# Get your wallet address
wallet show address

# Import private key (if needed)
wallet import key <private-key>
```

### Step 3: Get Testnet GAS

Obtain testnet GAS from:
- [Neo TestNet Faucet](https://neowish.ngd.network/testnet.html)
- Community resources

### Step 4: Deploy Contract

```bash
# Deploy the contract
contract deploy build/token_with_annotations.nef

# After reviewing information, type 'yes' to confirm
```

### Step 5: Test on Testnet

Invoke your contract methods to verify everything works:

```bash
# Invoke read method
contract invokefunction <contract-hash> symbol []

# Invoke write method (e.g., transfer)
contract invokefunction <contract-hash> transfer [<from-address>,<to-address>,100] --account <your-account>
```

## Mainnet Deployment

Deploying to mainnet follows the same process as testnet, but requires real GAS:

### Step 1: Set Up Neo-CLI for Mainnet

```bash
# Start Neo-CLI in mainnet mode
./neo-cli
```

### Step 2: Prepare a Secure Wallet

```bash
# Create a new secure wallet or use an existing one
wallet create path/to/mainnet-wallet.json

# Back up your wallet and private keys securely
```

### Step 3: Deploy Contract

```bash
# Deploy the contract
contract deploy build/token_with_annotations.nef

# Review carefully before confirming
```

### Deployment Checklist

Before deploying to mainnet, verify:

- [x] Contract code is thoroughly tested on local network
- [x] Contract code is tested on testnet
- [x] Security audit is completed
- [x] All critical functions work as expected
- [x] You have enough GAS for deployment (~10-100 GAS)
- [x] Contract owner account is secure
- [x] All contract parameters are correctly initialized
- [x] Deployment wallet has been backed up securely
- [x] Contract upgrade strategy is in place (if applicable)

## Verifying Deployed Contracts

After deployment, verify your contract is working correctly:

### Step 1: Check Contract State

```bash
# Use NeoScan explorer to view the contract
# Mainnet: https://neoscan.io/
# Testnet: https://testnet.neoscan.io/

# Search for your contract hash
```

### Step 2: Verify Contract Properties

Verify the following properties:
- Contract name and symbol
- Total supply (for tokens)
- Owner address
- Other contract-specific properties

### Step 3: Verify Contract Methods

Test key functions:
- Read-only methods (symbol, decimals, etc.)
- Transfer functionality (if applicable)
- Special permissions (only owner can access certain functions)

## Monitoring and Maintenance

Establishing ongoing monitoring for your contract:

### Contract Monitoring

1. **Activity Monitoring**:
   - Monitor transfer events
   - Track contract invocations
   - Monitor state changes

2. **Performance Monitoring**:
   - Track GAS costs for operations
   - Monitor transaction success rates

3. **Security Monitoring**:
   - Watch for suspicious transactions
   - Monitor ownership changes
   - Check for unexpected events

### Using Neo CLI for Monitoring

```bash
# Track specific events
contract listeners add --contract-hash <contract-hash> --event Transfer
```

### Using Block Explorers

Monitor your contract via:
- [NeoScan](https://neoscan.io/)
- [Dora](https://dora.coz.io/)
- [NeoTube](https://neotube.io/)

## Contract Upgrades

If your contract needs upgrading:

1. **Document Upgrade Strategy**:
   - Does the contract support upgrades?
   - What is the upgrade authorization process?
   - How will state be migrated?

2. **Implement Upgrade Mechanism**:
   - Use contract update feature if available
   - Or deploy a new contract with state migration

3. **Test Upgrade Process**:
   - Test on local network
   - Test on testnet
   - Verify state migration

## Conclusion

Following the documentation-first approach to deployment ensures a systematic and reliable process for getting your Neo contracts onto the blockchain. By documenting your deployment requirements, verification procedures, and monitoring plans before actual deployment, you can minimize risks and ensure a smoother launch.

For detailed examples of contract implementations, see our [examples directory](../examples/). 