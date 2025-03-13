# Deploying the Annotation-Based Escrow Contract

This guide provides step-by-step instructions for deploying the Escrow Contract to the Neo N3 blockchain. This contract uses Neo's annotation system for improved security, gas efficiency, and developer experience.

## Neo Contract Annotations Overview

The escrow contract uses modern Neo Contract annotations, which provide several benefits:

1. `#[neo_contract::contract]`: Marks a struct as a smart contract
2. `#[neo_contract::manifest]`: Generates the contract manifest with proper ABI
3. `#[constructor]`: Identifies the initialization method
4. `#[method]`: Exposes methods for external calls
5. `#[safe]`: Marks read-only methods for gas optimization
6. `#[no_reentry]`: Prevents re-entrancy attacks
7. `#[neo_contract::event(...)]`: Defines structured events

These annotations are processed during compilation and ensure the contract adheres to Neo N3 standards.

## Prerequisites

Before deploying the contract, ensure you have:

1. The Neo N3 development environment set up
2. The Neo Contract Rust framework installed (version 3.0 or higher for annotation support)
3. A Neo N3 wallet with sufficient GAS for deployment
4. The Escrow Contract code built and ready for deployment

## Build the Contract

First, compile the contract to a WebAssembly (WASM) file:

```bash
# Navigate to the contract directory
cd examples/ledger_workshop_example

# Build the contract in release mode
cargo build --release
```

This will generate a WASM file at `target/wasm32-unknown-unknown/release/escrow-contract.wasm`.

## Using a WASM Named Import Tool

Because the contract uses annotations, it's recommended to use a WASM named import tool to optimize the binary:

```bash
# Install Neo WASM Tool (if not already installed)
cargo install neo-wasm-tool

# Process the WASM file
neo-wasm-tool escrow-contract.wasm optimized-escrow-contract.wasm
```

## Deploy Using Neo-CLI

The Neo-CLI is a command-line tool that can be used to deploy contracts to the Neo N3 blockchain:

```bash
# Start Neo-CLI
./neo-cli

# Open your wallet
open wallet /path/to/your/wallet.json

# Deploy the optimized contract
deploy /path/to/optimized-escrow-contract.wasm

# Follow the prompts to provide contract details
# - Name: EscrowContract
# - Version: 1.0.0
# - Author: Your Name
# - Email: your.email@example.com
# - Description: A secure escrow contract with time locks, transaction validation, and dispute resolution
```

The deployment process will show a preview of the contract manifest and estimated gas costs. The manifest will include all methods annotated with `#[method]` and all events defined with `#[neo_contract::event]`. Confirm the deployment to proceed.

## Deploy Using NeoLine or Other Wallets

Alternatively, you can deploy using wallet extensions like NeoLine:

1. Open the NeoLine extension
2. Navigate to the "Deploy" section
3. Upload the optimized WASM file and NEF file (if available)
4. Fill in the contract details
5. Confirm the deployment

## Verifying Annotated Methods

After deployment, you can verify that all annotated methods are properly exposed:

```bash
# Using Neo-CLI
contract get metadata <contract-hash>
```

This will return the contract's metadata, including all exposed methods. You should see all methods that have the `#[method]` annotation, categorized as either safe or non-safe based on the presence of the `#[safe]` annotation.

## Initializing with the Constructor

Because the contract uses a `#[constructor]` annotation, you need to properly initialize it with the owner address:

```bash
# Using Neo-CLI
invoke <contract-hash> new [<owner-address>]
```

This will execute the constructor method and set up the contract's initial state. The owner address will be used for administrative operations like resolving disputes.

## Contract Manifest Structure

The generated manifest from the annotations will include:

1. **Safe Methods**: Methods marked with `#[safe]` will be identified as read-only
2. **Methods**: All methods marked with `#[method]` will be included
3. **Events**: All events defined with `#[neo_contract::event]` will be included
4. **Permissions**: Default permissions for contract operations

You can verify the manifest structure:

```bash
# Using Neo-CLI
contract get manifest <contract-hash>
```

## Security Considerations for Annotated Contracts

1. **Re-entrancy Protection**: Methods with `#[no_reentry]` will be protected from re-entrancy attacks automatically.
2. **Safe Methods**: Methods marked with `#[safe]` should never modify state; doing so may cause unexpected behavior.
3. **Constructor Invocation**: The constructor should be called only once during initialization.

## Testing Deployment

After deploying the contract, test its functionality:

```bash
# Test a safe method (read-only)
invoke <contract-hash> get_escrow_details [1]

# Test a state-changing method (requires signing)
invoke <contract-hash> create_escrow [<sender-address>,<recipient-address>,100,3600]
```

Ensure that the contract functions correctly and that all annotated methods are accessible.

## Testnet vs. Mainnet

- **Testnet**: Deploy to testnet first for testing and validation
- **Mainnet**: Only deploy to mainnet after thorough testing

### Testnet Resources:

- TestNet Faucet: https://n3t5.neotube.io/faucet
- TestNet Explorer: https://testnet.explorer.onegate.space/
- TestNet RPC Nodes: `https://n3seed1.ngd.network:40332`

### Mainnet Resources:

- MainNet Explorer: https://explorer.onegate.space/
- MainNet RPC Nodes: `https://n3seed1.ngd.network:10332`

## Contract Upgrade Strategy

When upgrading an annotated contract:

1. Deploy the new contract version with the same annotation structure
2. Migrate data from the old contract to maintain state consistency
3. Update client applications to use the new contract hash

## Additional Resources

- [Neo Contract Annotations Documentation](https://developers.neo.org/docs/n3/develop/write/attributes)
- [Neo Blockchain Toolkit](https://github.com/neo-project/neo-blockchain-toolkit)
- [Neo Debugger](https://github.com/neo-project/neo-debugger)
- [Neo Smart Contract Examples](https://github.com/neo-project/neo-contract-examples)

## Troubleshooting

Common deployment issues with annotated contracts:

1. **Invalid Manifest**: Check that all annotations are properly used
2. **Missing Methods**: Ensure all required methods have the `#[method]` annotation
3. **Constructor Issues**: Verify that the `#[constructor]` method is properly implemented
4. **Gas Limit Exceeded**: Large contracts might require more gas; adjust the gas limit accordingly

## Post-Deployment

After successful deployment:

1. Monitor contract usage and performance
2. Document the contract hash and details
3. Update client applications with the new contract hash
4. Announce the deployment to relevant communities 