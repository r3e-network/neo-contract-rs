# Neo Contract RS

A comprehensive Rust framework for developing Neo N3 smart contracts. This framework allows developers to write Neo smart contracts using Rust, targeting the Neo N3 blockchain.

## Features

- **Native Rust Development**: Write Neo N3 smart contracts using Rust
- **Ink!-Style Contract Definition**: Modern, declarative approach to contract development inspired by ink!
- **Extensive Standard Library**: Built-in types and utilities for Neo N3 smart contract development
- **NEP-17 Support**: First-class support for the NEP-17 fungible token standard
- **Contract-to-Contract Calls**: Easy-to-use API for contract interaction
- **Deploy & Test Tools**: Tools for deploying and testing smart contracts (coming soon)

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Wasm target: `rustup target add wasm32-unknown-unknown`

### Installation

Add the framework to your project:

```toml
[dependencies]
neo-contract = { git = "https://github.com/R3E-Network/neo-contract-rs" }
```

### Creating a New Contract

Create a new library project with the cdylib crate type:

```toml
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { git = "https://github.com/R3E-Network/neo-contract-rs" }
wee_alloc = "0.4.5"  # Lightweight memory allocator for WebAssembly
```

## Contract Development Approaches

### Ink!-Style Attribute Macros (Recommended)

Neo Contract RS now fully supports the ink!-style attribute macros for contract definition. This approach provides a more declarative and intuitive way to write smart contracts:

```rust
use neo_contract::prelude::ink_style::*;

#[contract]
#[contract_author("Your Name")]
#[contract_description("A description of your contract")]
mod token_contract {
    use super::*;
    
    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map,
    }
    
    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            // Implementation...
        }
        
        #[message]
        pub fn balance_of(&self, account: H160) -> Int256 {
            // Implementation...
        }
        
        #[event]
        pub fn transfer_event(from: H160, to: H160, amount: Int256) {}
    }
}
```

This approach is similar to the ink! framework for Substrate but tailored for Neo N3 smart contracts. See the [ink! style guide](docs/ink_style_guide.md) for more details.

### Traditional Style (Legacy Support)

The framework also supports a more traditional approach to contract development for backward compatibility:

```rust
#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString},
    Runtime,
    contract_method, smart_contract,
};

// Global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub struct Counter;

#[smart_contract]
impl Counter {
    contract_method!(pub fn increment() -> u32 {
        // Implementation
        42
    });
    
    contract_method!(pub fn get() -> u32 {
        // Implementation
        42
    });
}
```

## Contract Attributes Reference

### Contract Structure Attributes

- `#[neo_contract::contract]` - Defines a Neo N3 smart contract module
- `#[neo(storage)]` - Marks a struct as the contract's storage
- `#[neo(constructor)]` - Marks a method as a contract constructor
- `#[neo(message)]` - Marks a method as a contract message (callable from outside)
- `#[neo(event)]` - Marks a method as a contract event

### Contract Metadata Attributes

- `#[neo_contract::manifest_extra("key", "value")]` - Adds custom metadata to the contract manifest
- `#[neo_contract::contract_author("Author Name")]` - Specifies the contract author
- `#[neo_contract::contract_email("email@example.com")]` - Specifies the author's email
- `#[neo_contract::contract_description("Description")]` - Provides a contract description
- `#[neo_contract::contract_version("1.0.0")]` - Specifies the contract version
- `#[neo_contract::contract_source_code("https://github.com/...")]` - Links to the source code

## Building and Deploying

### Building

Build your contract with the WebAssembly target:

```bash
cargo build --target wasm32-unknown-unknown --release
```

This will generate a WebAssembly binary in `target/wasm32-unknown-unknown/release/`.

### Deploying (Coming Soon)

We are working on deployment tools to make it easy to deploy your contracts to the Neo N3 blockchain.

## Examples

Check the `examples` directory for complete contract examples:

- `ink_style_token_with_attributes` - A token contract using the ink!-style attribute macros
- `nep17_token` - A NEP-17 token implementation
- `contract_call` - Example of contract-to-contract calls
- `csharp_features` - Examples of C# framework features in Rust

## Advanced Features

### Static Field Initialization

```rust
// Initialize a Hash160 with a hex string
#[neo::hash160("0x0123456789abcdef0123456789abcdef01234567")]
static CONTRACT_HASH: H160 = H160::zero();

// Initialize an integer with a value
#[neo::integer("1000000")]
static AMOUNT: Int256 = Int256::zero();
```

### Contract-to-Contract Calls

```rust
// Call another contract
let result = Runtime::call_contract(
    &target_contract_hash,
    "method_name",
    &args,
    CallFlags::All
);
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## NeoBurger Example

The NeoBurger example demonstrates a complex contract system for Neo N3 governance. It consists of three contracts:

1. **BurgerNEO** - Core contract that handles NEO staking and bNEO token issuance
   - Implements NEP-17 token standard
   - Manages NEO deposits and withdrawals
   - Distributes GAS rewards to bNEO holders

2. **BurgerAgent** - Agent contract that handles voting and NEO management
   - Manages voting for consensus nodes
   - Handles NEO transfers on behalf of the core contract
   - Claims GAS rewards and sends them to the core contract

3. **GovernanceToken (NOBUG)** - Governance token contract for the NeoBurger system
   - Implements NEP-17 token standard
   - Provides governance functionality for the NeoBurger ecosystem
   - Allows token holders to submit and execute proposals

### Building the NeoBurger Example

```bash
# Build the core contract
cd examples/neoburger
cargo build --release

# Build the agent contract
cd ../neoburger_agent
cargo build --release

# Build the governance token contract
cd ../neoburger_governance
cargo build --release
```

### Features Demonstrated

- **ink!-style Attribute Macros**: Using the new unified attribute macro system
- **NEP-17 Token Standard**: Implementation of the Neo N3 token standard
- **Storage Management**: Efficient storage of balances, rewards, and governance data
- **Voting Mechanism**: System for voting on Neo consensus nodes
- **Reward Distribution**: GAS reward distribution to token holders
- **Agent Contract System**: Delegation of NEO management to agent contracts
- **Governance Functionality**: Proposal submission and execution system

## Neoburger Example Contracts

The repository includes example contracts for the Neoburger ecosystem:
- **neoburger**: Main contract for the Neoburger platform
- **neoburger_agent**: Agent contract for Neoburger operations
- **neoburger_governance**: Governance contract for Neoburger ecosystem

These examples demonstrate complex contract interactions and governance mechanisms.
