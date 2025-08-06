# Neo N3 Smart Contract Deployment Guide

## Overview

This guide explains how to compile, deploy, and test Neo N3 smart contracts using the integrated neo-compiler and Neo Express.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Compilation](#compilation)
4. [Deployment with Neo Express](#deployment-with-neo-express)
5. [Testing Deployed Contracts](#testing-deployed-contracts)
6. [MainNet/TestNet Deployment](#mainnet-testnet-deployment)
7. [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools

1. **Rust with WASM target**:
```bash
rustup target add wasm32-unknown-unknown
rustup component add rustfmt clippy
```

2. **Neo Express** (for local testing):
```bash
# Install .NET SDK 6.0+ first
dotnet tool install Neo.Express -g
```

3. **Neo CLI** (optional, for MainNet/TestNet):
```bash
dotnet tool install Neo.CLI -g
```

## Quick Start

Deploy all contracts to Neo Express in one command:

```bash
# Clean, build, compile, and deploy everything
make clean all deploy
```

## Compilation

### Building WASM Files

All examples use a standardized build process:

```bash
# Build all examples
make build-all

# Build specific example
make 04-nep17-token

# Build with debug symbols
make BUILD_MODE=debug build-all
```

### Compiling to NEF

The neo-compiler converts WASM files to Neo Executable Format (NEF):

```bash
# Compile all WASM files to NEF
make compile

# Compile specific WASM file
cargo run -p neo-compiler -- compile path/to/contract.wasm --output build/

# Verify NEF file
cargo run -p neo-compiler -- verify build/contract.nef
```

### Compilation Output

For each contract, the compiler generates:
- `.nef` - Neo Executable Format file (bytecode)
- `.manifest.json` - Contract manifest with metadata and ABI

## Deployment with Neo Express

### 1. Initialize Neo Express

```bash
# Start Neo Express (single node)
make neo-express

# Or manually with custom config
neoxp create -f
neoxp wallet create alice
neoxp transfer 1000 GAS genesis alice
```

### 2. Deploy Contracts

#### Deploy All Contracts
```bash
make deploy
```

#### Deploy Specific Contract
```bash
./scripts/deploy.sh deploy build/nep17_token.nef
```

#### Manual Deployment
```bash
# Deploy with Neo Express CLI
neoxp contract deploy build/contract.nef alice

# Get contract hash from deployment
neoxp contract list
```

### 3. Deployment Configuration

The deployment script (`scripts/deploy.sh`) provides:
- Automatic wallet creation and funding
- Sequential deployment of all contracts
- Deployment receipts saved to `build/*.deployment.json`
- Error handling and retry logic

## Testing Deployed Contracts

### Using Neo Express

#### Invoke Contract Methods

```bash
# Call without parameters
neoxp contract invoke <contract-hash> <method> alice

# Call with parameters
neoxp contract invoke <contract-hash> transfer alice -- \
  @address:alice @address:bob @integer:100

# View-only call (no transaction)
neoxp contract invoke <contract-hash> balanceOf alice \
  --witness-scope None -- @address:alice
```

#### Using the Test Script

```bash
# Test specific method
./scripts/deploy.sh test <contract-hash> <method> [params...]

# Example: Test NEP-17 transfer
./scripts/deploy.sh test 0x123...abc transfer \
  alice bob 100
```

### Contract-Specific Tests

#### Hello World
```bash
CONTRACT_HASH=<hash>
neoxp contract invoke $CONTRACT_HASH hello alice
neoxp contract invoke $CONTRACT_HASH greet alice -- @string:"Neo Developer"
```

#### NEP-17 Token
```bash
CONTRACT_HASH=<hash>
# Get balance
neoxp contract invoke $CONTRACT_HASH balanceOf alice -- @address:alice

# Transfer tokens
neoxp contract invoke $CONTRACT_HASH transfer alice -- \
  @address:alice @address:bob @integer:1000000 @null
```

#### Counter
```bash
CONTRACT_HASH=<hash>
# Increment counter
neoxp contract invoke $CONTRACT_HASH increment alice

# Get current value
neoxp contract invoke $CONTRACT_HASH get_value alice
```

## MainNet/TestNet Deployment

### 1. Prepare for Network Deployment

```bash
# Build optimized contracts
make BUILD_MODE=release build-all compile

# Verify all NEF files
make verify
```

### 2. TestNet Deployment

```bash
# Create TestNet wallet
neo-cli wallet create testnet-wallet.json

# Request TestNet GAS from faucet
# https://neofaucet.io/

# Deploy to TestNet
neo-cli contract deploy build/contract.nef \
  --wallet testnet-wallet.json \
  --network testnet
```

### 3. MainNet Deployment

```bash
# IMPORTANT: Test thoroughly on TestNet first!

# Deploy to MainNet
neo-cli contract deploy build/contract.nef \
  --wallet mainnet-wallet.json \
  --network mainnet \
  --gas 10
```

### 4. Verify Deployment

```bash
# Check contract on explorer
# TestNet: https://testnet.neotube.io/
# MainNet: https://neotube.io/

# Verify with Neo CLI
neo-cli contract show <contract-hash> --network testnet
```

## Contract Management

### Update Contract

```bash
# Deploy new version (creates new contract)
neoxp contract deploy build/contract_v2.nef alice

# Migrate storage if needed
neoxp contract invoke <old-hash> migrate alice -- @hash160:<new-hash>
```

### Monitor Contract

```bash
# View contract storage
neoxp contract storage <contract-hash>

# Get contract info
neoxp contract get <contract-hash>

# View recent invocations
neoxp show transactions --contract <contract-hash>
```

## Solana-Style Contracts

For Solana-style contracts, the deployment process is the same:

```bash
# Build Solana-style example
cd examples/01-hello-world-solana-style-simple
cargo build --target wasm32-unknown-unknown --release

# Compile (auto-detects Solana style)
cargo run -p neo-compiler -- compile \
  target/wasm32-unknown-unknown/release/*.wasm

# Deploy
neoxp contract deploy *.nef alice
```

The compiler automatically:
- Detects `#[program]` modules
- Generates appropriate manifest
- Maps Solana patterns to Neo N3

## Troubleshooting

### Common Issues

#### 1. WASM Build Fails
```bash
# Ensure correct RUSTFLAGS
export RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152"

# Clean and rebuild
cargo clean
cargo build --target wasm32-unknown-unknown --release
```

#### 2. NEF Compilation Fails
```bash
# Check WASM file validity
wasm-objdump -x contract.wasm

# Compile with debug output
cargo run -p neo-compiler -- compile contract.wasm --debug
```

#### 3. Deployment Fails
```bash
# Check Neo Express is running
ps aux | grep neo-express

# Check wallet has GAS
neoxp wallet show alice

# Check contract size (max ~1MB)
ls -lh build/*.nef
```

#### 4. Invocation Fails
```bash
# Check method name and parameters
neoxp contract get <contract-hash> | jq '.manifest.abi.methods'

# Test with minimal parameters first
neoxp contract invoke <contract-hash> <method> alice

# Enable verbose output
NEO_EXPRESS_DEBUG=1 neoxp contract invoke ...
```

### Debug Tools

#### Disassemble NEF
```rust
use neo_compiler::debug::Debugger;

let nef = neo_compiler::nef::Nef3::from_file("contract.nef")?;
let debugger = Debugger::new(&nef);
println!("{}", debugger.disassemble());
```

#### Step Debugger
```rust
let mut debugger = Debugger::new(&nef);
debugger.add_breakpoint(0x10);

while let Ok(step) = debugger.step() {
    match step {
        StepResult::Normal { opcode, .. } => println!("Executing: {:?}", opcode),
        StepResult::Breakpoint(_) => println!("Hit breakpoint!"),
        StepResult::End => break,
    }
}
```

## Best Practices

1. **Always test on Neo Express first** before deploying to TestNet/MainNet
2. **Keep NEF files under 1MB** for optimal performance
3. **Use view methods** (marked as `safe` in manifest) for read operations
4. **Implement proper access control** in your contracts
5. **Test with various parameter combinations** to ensure robustness
6. **Monitor gas consumption** during testing
7. **Keep deployment receipts** for contract management
8. **Version your contracts** and maintain upgrade paths

## Resources

- [Neo Developer Portal](https://developers.neo.org/)
- [Neo Express Documentation](https://github.com/neo-project/neo-express)
- [Neo Smart Contract Examples](../examples/)
- [Solana-Style Development Guide](SOLANA_STYLE_GUIDE.md)

## Support

For deployment issues:
- GitHub Issues: [neo-contract-rs](https://github.com/r3e-network/neo-contract-rs)
- Neo Discord: [discord.neo.org](https://discord.neo.org)
- Documentation: [docs.neo.org](https://docs.neo.org)