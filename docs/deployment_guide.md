# Neo Smart Contract Deployment Guide

This guide provides detailed instructions for deploying Neo smart contracts developed with the Neo Contract Rust Framework. It covers the entire deployment process from compilation to on-chain deployment across different Neo networks.

## Prerequisites

Before you start, ensure you have:

- A compiled Neo smart contract (NEF file + manifest)
- [Neo CLI](https://docs.neo.org/docs/en-us/node/cli/setup.html) installed
- Sufficient GAS for deployment fees
- A wallet with the necessary permissions

## Compilation Process

Before deployment, compile your Rust contract following these steps:

### 1. Compile to WebAssembly

```bash
# Build for release
cargo build --target wasm32-unknown-unknown --release
```

### 2. Optimize WebAssembly (Optional)

For better performance and lower gas costs, optimize your WebAssembly binary:

```bash
# Install wasm-opt if you haven't already
npm install -g wasm-opt

# Optimize the WebAssembly binary
wasm-opt -Oz -o optimized.wasm target/wasm32-unknown-unknown/release/your_contract.wasm
```

### 3. Convert to Neo Executable Format (NEF)

Use the Neo Compiler to convert your WebAssembly to Neo format:

```bash
# Basic conversion
neo-compiler compile \
    target/wasm32-unknown-unknown/release/your_contract.wasm \
    --output ./build

# With additional metadata
neo-compiler compile \
    target/wasm32-unknown-unknown/release/your_contract.wasm \
    --output ./build \
    --name "YourContractName" \
    --author "Your Name" \
    --description "Contract description" \
    --email "your.email@example.com"
```

This generates two critical files in the `./build` directory:
- `YourContractName.nef`: Contains the Neo VM bytecode
- `YourContractName.manifest.json`: Describes the contract API, permissions, and other metadata

## Understanding the Manifest

The manifest file is crucial for deployment and contains:

```json
{
  "name": "YourContractName",
  "groups": [],
  "abi": {
    "methods": [
      {
        "name": "constructor",
        "parameters": [],
        "returntype": "Void",
        "offset": 0,
        "safe": false
      },
      {
        "name": "someMethod",
        "parameters": [
          {
            "name": "param1",
            "type": "String"
          }
        ],
        "returntype": "Integer",
        "offset": 42,
        "safe": true
      }
    ],
    "events": [
      {
        "name": "SomeEvent",
        "parameters": [
          {
            "name": "param1",
            "type": "String"
          }
        ]
      }
    ]
  },
  "permissions": [
    {
      "contract": "*",
      "methods": "*"
    }
  ],
  "trusts": [],
  "features": {},
  "supportedstandards": ["NEP-17"],
  "extra": {
    "Author": "Your Name",
    "Email": "your.email@example.com",
    "Description": "Contract description"
  }
}
```

### Important Manifest Fields

- **name**: The contract's name
- **abi**: Defines the contract's public interface
  - **methods**: Contract's callable methods with parameters and return types
  - **events**: Events the contract can emit
- **permissions**: Defines which contracts your contract can call
- **trusts**: Contracts that your contract explicitly trusts
- **supportedstandards**: Lists implemented NEP standards (e.g., NEP-17 for tokens)

## Deployment Networks

Neo offers multiple networks for deployment:

1. **Private Network**: For development and initial testing
2. **Testnet**: For testing in a real-world environment before mainnet deployment
3. **Mainnet**: The production Neo blockchain

### Network Configuration

Configure Neo CLI for the desired network:

```bash
# For private network
neo-cli config network private

# For testnet
neo-cli config network testnet

# For mainnet
neo-cli config network mainnet
```

## Deployment Process

### 1. Open a Wallet

```bash
# Open your wallet
neo-cli wallet open path/to/your/wallet.json

# Unlock the wallet
neo-cli wallet unlock
```

You'll be prompted to enter your password.

### 2. Check GAS Balance

Deploying contracts requires GAS:

```bash
neo-cli wallet balance
```

Ensure you have sufficient GAS for deployment.

### 3. Deploy the Contract

```bash
neo-cli deploy path/to/your/contract.nef path/to/your/contract.manifest.json
```

Neo CLI will:
1. Calculate the deployment cost
2. Ask for confirmation
3. Create and send the deployment transaction
4. Provide the contract hash on successful deployment

### 4. Verify Deployment

After deployment, verify your contract:

```bash
# Check if contract exists
neo-cli contract get hash <script-hash>

# Test a read-only method
neo-cli invoke <script-hash> someReadMethod []
```

## Deployment Costs

Contract deployment on Neo requires GAS to cover system fees:

1. **Storage Fee**: Based on the contract size (NEF + manifest)
2. **Execution Fee**: Cost of running the deployment code
3. **Network Fee**: Fee for processing the transaction

### Estimating Deployment Cost

To estimate deployment costs before committing:

```bash
neo-cli deploy path/to/your/contract.nef path/to/your/contract.manifest.json --estimate-only
```

## Advanced Deployment

### Multi-signature Deployment

For contracts requiring multiple signers:

```bash
# Create multi-signature address
neo-cli wallet create-multisig-address <minimum-signatures> <public-key1> <public-key2> ...

# Deploy with multi-signature
neo-cli deploy path/to/your/contract.nef path/to/your/contract.manifest.json --from-addr <multi-sig-address>
```

### Deploying with Contract Parameters

Some contracts require parameters during deployment:

```bash
neo-cli deploy path/to/your/contract.nef path/to/your/contract.manifest.json --parameters <parameters>
```

Parameters format: Array of JSON objects representing parameter values.

### Contract Updates

To update an existing contract:

```bash
neo-cli contract update <script-hash> path/to/new/contract.nef path/to/new/contract.manifest.json
```

Important notes for contract updates:
- The contract must have an `update` method or allow dynamic invocations
- Storage data from the previous version is preserved
- Maintain storage compatibility between versions

## Network-Specific Considerations

### Private Network

For private networks:
- Minimal deployment costs
- Rapid confirmation times
- Reset the chain as needed for testing

### Testnet

For testnet deployment:
- Get testnet GAS from faucets
- Test real network conditions
- Verify integration with external services

### Mainnet

For mainnet deployment:
- Thoroughly test on testnet first
- Ensure sufficient GAS for deployment
- Consider having extra GAS for contract operations
- Verify contract security and reliability
- Consider a professional audit for high-value contracts

## Contract Invocation After Deployment

### Basic Invocation

```bash
# For read-only methods
neo-cli invoke <script-hash> methodName [<parameters>]

# For methods that modify state
neo-cli invoke <script-hash> methodName [<parameters>] --signers <address:CalledByEntry>
```

Parameter format: Array of JSON objects representing parameters.

### Transaction Options

Additional transaction options:

```bash
# Set gas limit
neo-cli invoke <script-hash> methodName [<parameters>] --gas <amount>

# Add additional signatures
neo-cli invoke <script-hash> methodName [<parameters>] --signers <address1:CalledByEntry> <address2:CalledByEntry>
```

## Troubleshooting

### Common Deployment Issues

1. **Insufficient GAS**:
   ```
   Error: Insufficient funds
   ```
   Solution: Ensure your wallet has enough GAS for deployment.

2. **Invalid Manifest**:
   ```
   Error: Invalid manifest format
   ```
   Solution: Verify your manifest JSON format is correct.

3. **Permission Issues**:
   ```
   Error: No permission to deploy contract
   ```
   Solution: Ensure your wallet has the necessary permissions.

4. **Execution Failure**:
   ```
   Error: Execution failed with reason: ...
   ```
   Solution: Debug the contract initialization code.

### Debugging Deployment

For detailed error information:

```bash
# Enable verbose mode
neo-cli settings set log-level debug

# Deploy with debugger
neo-cli deploy path/to/your/contract.nef path/to/your/contract.manifest.json --debug
```

## Best Practices

### Security Considerations

1. **Deployment Wallets**:
   - Use dedicated wallets for contract deployment
   - Secure deployment private keys
   - Consider multi-signature for high-value contracts

2. **Deployment Process**:
   - Test thoroughly on private networks and testnet
   - Follow a deployment checklist
   - Perform dry runs to estimate costs

3. **Post-Deployment**:
   - Verify all contract functionality after deployment
   - Monitor initial transactions
   - Have a contingency plan for issues

### Continuous Deployment

For teams with regular updates:

1. **Deployment Pipeline**:
   - Automate build and NEF generation
   - Maintain separate environments (dev, test, prod)
   - Version control your NEF and manifest files

2. **Testing Strategy**:
   - Automated tests before deployment
   - Integration tests on testnet
   - Simulation of production workflows

### Documentation

Maintain comprehensive documentation for your deployed contracts:

1. **Contract Details**:
   - Script hash and contract address
   - Deployment date and network
   - ABI details and examples

2. **Version History**:
   - Track all deployed versions
   - Document changes between versions
   - Migration paths for users/integrators

## Contract Verification

Neo allows contract verification to prove the NEF file matches the source code:

```bash
# Publish source and verification info
neo-cli contract publish-source <script-hash> --source path/to/source --compiler-info <compiler-info>
```

This allows users to verify your contract's source code, enhancing trust and transparency.

## Conclusion

Deploying Neo smart contracts is a multi-step process that requires careful planning and execution. By following this guide and best practices, you can successfully deploy your Rust-based Neo contracts to any Neo network with confidence.

For further assistance:
- Consult the [Neo documentation](https://docs.neo.org/)
- Join the [Neo Discord community](https://discord.gg/tpUNQNy)
- Explore the [Neo GitHub repositories](https://github.com/neo-project/)

Happy deploying!
