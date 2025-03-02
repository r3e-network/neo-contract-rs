# Neo Contract RS Examples

This directory contains examples of Neo N3 smart contracts built with Neo Contract RS. All examples use the modern ink!-style contract definition syntax for a consistent and intuitive development experience.

## Examples Overview

### NEP-17 Token Examples

- **nep17**: A basic implementation of the NEP-17 token standard using ink! style attributes
- **nep17_token**: A complete NEP-17 token with storage handling and all standard methods

### Contract Interaction Examples

- **contract_call**: Demonstrates how to interact with other contracts on the Neo N3 blockchain

### Ink! Style Examples

- **ink_style_token**: A simple token implementation using ink! style attributes
- **ink_style_token_with_attributes**: An enhanced token implementation with complete metadata attributes

### Advanced Examples

- **csharp_features**: Demonstrates C# framework features ported to Rust
- **neoburger**: NeoBurger ecosystem contracts demonstrating advanced Neo N3 functionality
- **transfer**: Example demonstrating token transfer mechanics

## Ink! Style Contract Definition

All examples in this repository follow the ink!-style contract definition approach, which provides a more ergonomic and declarative way to write smart contracts.

### Basic Structure

```rust
#[contract]
#[contract_author("Author Name")]
#[contract_description("Contract Description")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod token_contract {
    use super::*;

    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map<H160, Int256>,
    }

    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            // Implementation...
        }
        
        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            // Implementation...
        }
        
        #[event]
        pub fn transfer_event(from: H160, to: H160, amount: Int256) {}
    }
}
```

### Key Attributes

- `#[contract]`: Marks a module as a Neo N3 smart contract
- `#[storage]`: Marks a struct as the contract's storage
- `#[constructor]`: Marks a method as a contract constructor
- `#[message]`: Marks a method as a contract message (callable from outside)
- `#[safe]`: Marks a method as read-only (cannot modify contract storage)
- `#[event]`: Marks a method as a contract event

### Metadata Attributes

- `#[contract_author("Author Name")]`: Specifies the contract author
- `#[contract_description("Description")]`: Provides a contract description
- `#[contract_version("1.0.0")]`: Specifies the contract version
- `#[supported_standards("NEP-17")]`: Declares supported standards

## Building and Running Examples

To build all examples at once, use the provided script:

```bash
cd /path/to/neo-contract-rs
./scripts/build_examples.sh
```

To build a specific example:

```bash
cd /path/to/neo-contract-rs/examples/<example-name>
cargo build --target wasm32-unknown-unknown --release
```

The compiled WebAssembly binaries will be located in `target/wasm32-unknown-unknown/release/`.
