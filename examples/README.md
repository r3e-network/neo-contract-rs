# Neo N3 Smart Contract Examples in Rust

This directory contains various example smart contracts for the Neo N3 blockchain built using the Neo Contract Rust framework. These examples demonstrate different use cases and serve as reference implementations for developers getting started with Neo N3 smart contract development using Rust.

## Modern Neo Contract Annotations

Many examples have been updated to use Neo's modern annotation system, which provides:

- **Enhanced Security**: Protection against re-entrancy attacks with `#[no_reentry]`
- **Gas Optimization**: Better efficiency with properly marked `#[safe]` methods
- **Structured Events**: Typed event definitions with `#[neo_contract::event(...)]`
- **Clear API Definitions**: Explicit method visibility with `#[method]`

For details on the annotation system and how to update existing contracts, see the [Annotation Update Guide](./ANNOTATION_UPDATE_GUIDE.md).

## Example Categories

The examples are organized into the following categories:

### DeFi (Decentralized Finance)

Located in the [defi](./defi) directory, these examples showcase financial applications:

- **DEX (Decentralized Exchange)**: Implements an automated market maker with liquidity pools, swapping, and fee collection.
- **Lending**: Demonstrates a lending platform with deposits, loans, interest rates, and collateralization.
- **Staking**: Implements a staking platform with rewards distribution and time-locked stakes.
- **Secure Vault**: A multi-token secure storage vault with advanced security features including time-locks, multi-signature approvals, and emergency controls. Demonstrates best practices for storage patterns and security.

### NFTs (Non-Fungible Tokens)

Located in the [nft](./nft) directory, these examples demonstrate NFT implementations:

- **Basic NFT**: A simple implementation of the NEP-11 non-fungible token standard.
- **Storage-Optimized NFT**: An advanced implementation of NEP-11 that demonstrates efficient storage patterns and gas optimization techniques.
- **Marketplace**: A comprehensive NFT marketplace with fixed-price listings, auctions, offers, and royalties.

### Gambling and Lotteries

Located in the [gambling](./gambling) directory, these examples showcase random number generation and chance-based contracts:

- **Lottery**: Implements a round-based lottery system with ticket purchases and random winner selection.

### Blockchain Interaction

- **Ledger Example** ([ledger_example](./ledger_example/)): Demonstrates comprehensive use of the Ledger API for blockchain-dependent functionality including time-based vesting, block-based rewards, transaction validation, and rate limiting. Includes extensive testing implementation showcasing the techniques from the [Ledger API Testing Guide](../docs/ledger_api_testing.md).
- **Ledger Workshop Example** ([ledger_workshop_example](./ledger_workshop_example/)): A hands-on implementation of an escrow contract showcasing practical use of blockchain time and block data. Created as part of the [Ledger API Workshop](../docs/ledger_api_workshop.md) for learning blockchain-dependent contract development. Uses modern Neo annotation syntax.
- **Transaction Patterns** ([tx_patterns](./tx_patterns/)): Demonstrates common transaction patterns including confirmation validation, rate limiting, and multi-step transactions.
- **Cross-Contract Communication** ([cross_contract](./cross_contract/)): Examples of contract-to-contract interaction patterns including contract registry and proxy patterns.

### Simple Examples

Located in the [simple](./simple) directory, these are basic examples to get started:

- **Hello World** ([hello_world](./hello_world/)): A minimal smart contract that stores and retrieves a greeting message. Uses modern Neo annotation syntax.
- **Domain Name Service**: Demonstrates a simple name service for registering and resolving domain names.
- **Simple Token** ([simple_token](./simple_token/)): Basic implementation of the NEP-17 fungible token standard. Uses modern Neo annotation syntax.
- **Event Demo** ([event_demo](./event_demo/)): Demonstrates structured events using Neo annotation syntax.
- **NEP-17 Token** ([nep17](./nep17/)): Standard-compliant fungible token with Neo annotation syntax.

## Neo Annotation Examples

The following examples showcase the modern Neo Contract annotation system:

| Example | Features Demonstrated |
|---------|------------------------|
| [Simple Token](./simple_token/) | `#[neo_contract::event]`, `#[method]`, `#[safe]`, basic token functionality |
| [Event Demo](./event_demo/) | Various event types, structured parameters, array events |
| [NEP-17 Token](./nep17/) | Standard-compliant token with all annotations |
| [Ledger Workshop](./ledger_workshop_example/) | Complex contract with security annotations and Ledger API |

## Using the Examples

Each example includes:

1. A README with detailed explanation of the contract's purpose and functionality
2. Source code with comments explaining key concepts
3. Build instructions for both development and production

### Building an Example

Most examples can be built using:

   ```bash
# For development and testing
cargo check -p example-name --features std
cargo build -p example-name --features std

# For production/deployment
cargo build -p example-name --release
```

Replace `example-name` with the actual crate name from the example's Cargo.toml file.

## Storage Patterns and Optimization

Many examples demonstrate efficient storage patterns as described in the [Storage Guide](../docs/storage_guide.md):

- **Composite Key Pattern**: Using structured keys to organize related data
- **Storage Prefix Pattern**: Using prefixes to prevent key collisions
- **Lazy Loading Pattern**: Loading data only when needed
- **Pagination Pattern**: Efficiently handling large collections
- **Batch Operation Pattern**: Grouping operations to reduce gas costs

For dedicated examples showcasing storage optimization, see:
- [Storage-Optimized NFT](./nft/storage_optimized/) - Optimized NFT implementation
- [Secure Vault](./defi/secure_vault/) - Advanced storage patterns with security features

## Ledger API and Blockchain Data

Examples consistently use the Neo N3 Ledger API to access blockchain data:

- `Ledger::current_index()` - Get current block index
- `Ledger::current_hash()` - Get current block hash  
- `Ledger::current_timestamp()` - Get current timestamp
- `Ledger::get_block()` - Get block by index or hash
- `Ledger::get_transaction()` - Get transaction by hash

For a dedicated example showcasing Ledger API usage and comprehensive testing strategies, see the [ledger_example](./ledger_example/) directory. This example demonstrates proper techniques for testing time-dependent and block-dependent smart contract logic, following best practices from the [Ledger API Testing Guide](../docs/ledger_api_testing.md).

## Known Issues

Some examples may experience compilation issues due to ongoing development of the Neo Contract Rust framework. Common issues include:

1. Procedural macro resolution failures with `#[neo_contract::contract]`, `#[method]`, and other annotations
2. Storage trait implementation issues
3. Runtime function signature mismatches

Please refer to the [troubleshooting guide](../docs/troubleshooting.md) for detailed solutions.

## Contributing

We welcome contributions to improve these examples or add new ones. Please follow the project's contribution guidelines when submitting changes or additions.

## License

These examples are provided under the same license as the Neo Contract Rust framework.
