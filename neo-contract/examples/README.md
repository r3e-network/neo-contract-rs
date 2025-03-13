# Neo N3 Contract Examples

This directory contains example smart contracts demonstrating how to use the Neo N3 contract development framework in Rust.

## Available Examples

### 1. Event Demo (`event_demo.rs`)

A basic example showing how to define and emit events in a Neo N3 smart contract, including:
- Event definition using the `#[event]` macro
- Proper event emission using `Runtime::notify`
- Handling of optional parameters with `Any::new()`

### 2. NEP-11 NFT Implementation (`nep11_nft.rs`)

A complete implementation of the NEP-11 Non-Fungible Token standard for Neo N3, showcasing:
- Full NEP-11 standard compliance
- Token ownership management
- Token metadata handling
- Safe and non-safe method marking
- Proper event emission
- Storage patterns with prefix-based keys

### 3. NEP-17 Token (`nep17_token.rs`)

A standard implementation of the NEP-17 Fungible Token standard for Neo N3, featuring:
- Complete NEP-17 standard compliance
- Token minting and burning functionality
- Ownership management with witness checks
- Safe method marking for gas optimization
- Proper event emission using Neo N3 standards

### 4. Hybrid Token (`hybrid_token.rs`)

A combined implementation of both NEP-17 (fungible token) and NEP-11 (non-fungible token) standards in a single contract, demonstrating:
- Dual standard compliance (NEP-17 and NEP-11)
- Token conversion mechanisms (FT to NFT and vice versa)
- Advanced event handling for multiple token types
- Complex storage patterns for managing multiple token systems
- Interface implementations for both standards

## Using These Examples

### Prerequisites

- Rust toolchain with nightly features enabled
- Neo N3 compiler (`neo-compiler` crate)
- Neo N3 VM or simulator for testing

### Building an Example

To build an example contract, use the following command:

```bash
cargo build --target wasm32-unknown-unknown --release -p neo-contract --example nep11_nft
```

This will compile the Rust code to WebAssembly, which can then be processed by the Neo compiler.

### Compiling for Neo N3

After building the WebAssembly file, use the Neo compiler to generate Neo N3 contract files (NEF and manifest):

```bash
neo-compiler compile -i ./target/wasm32-unknown-unknown/release/examples/nep11_nft.wasm -o ./nep11_nft
```

This will generate:
- `nep11_nft.nef`: Neo Executable Format file
- `nep11_nft.manifest.json`: Contract manifest with method definitions and metadata

### Deploying the Contract

Use the Neo N3 CLI or SDK to deploy the contract:

```bash
neo-cli deploy nep11_nft.nef nep11_nft.manifest.json
```

## Best Practices

These examples implement the best practices for Neo N3 smart contract development, as outlined in the [best-practices.md](../docs/best-practices.md) documentation. Key considerations:

1. **No Standard Library**: All examples use `no_std` to ensure compatibility with blockchain environments
2. **Event Standards**: Events follow the Neo N3 standards for proper notification
3. **Storage Patterns**: Efficient storage patterns with prefix-based keys
4. **Safe Methods**: Read-only methods are marked with `#[safe]` for gas optimization
5. **Access Control**: Proper witness checks to verify authorization
6. **Error Handling**: Comprehensive error checking and validation

## Testing

Tests for these examples can be written using the `MockStorage` utility provided by the framework:

```rust
use neo_contract::storage::test_utils::MockStorage;

#[test]
fn test_nft_contract() {
    MockStorage::clear();
    
    // Create contract instance and test functionality
    // ...
}
```

For more details on testing, refer to the [storage.md](../docs/api/storage.md) documentation.

## License

These examples are provided under the same license as the Neo N3 contract framework.
