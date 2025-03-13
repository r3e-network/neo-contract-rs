# Getting Started with Neo N3 Contract Development in Rust

This guide will help you set up your development environment and create your first Neo N3 smart contract using the Rust framework.

## Prerequisites

- Rust toolchain (1.60+ recommended)
- Cargo and cargo-make
- WebAssembly target support
- Basic familiarity with Rust and blockchain concepts

## Installation

1. Install Rust and required tools:

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install additional tools
cargo install cargo-make
cargo install wasm-strip
```

2. Clone the neo-contract-rs repository:

```bash
git clone https://github.com/neo-project/neo-contract-rs
cd neo-contract-rs
```

3. Build the framework components:

```bash
cargo build --release
```

## Creating Your First Contract

### 1. Set up a new project

```bash
# Create a new Rust project
cargo new --lib my-neo-contract
cd my-neo-contract
```

### 2. Configure your project

Add the following to your `Cargo.toml`:

```toml
[package]
name = "my-neo-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { path = "../path/to/neo-contract-rs/neo-contract" }

[profile.release]
lto = true
opt-level = "z"
codegen-units = 1
panic = "abort"
strip = true
```

### 3. Write a simple token contract

Create a basic NEP-17 token in `src/lib.rs`:

```rust
#![no_std]

use neo_contract::prelude::*;

#[contract]
mod token {
    use super::*;
    
    // Define storage structure
    #[storage]
    pub struct TokenContract {
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    // Implementation with contract methods
    impl TokenContract {
        // Initialize the contract
        #[initialize]
        pub fn new() -> Self {
            let mut instance = Self {
                total_supply: StorageItem::new(b"total_supply"),
                balances: StorageMap::new(b"balances"),
            };
            
            // Set initial supply
            let initial_supply: u64 = 1_000_000_000; // 1 billion tokens
            instance.total_supply.set(&initial_supply);
            
            // Assign all tokens to contract owner
            let owner = Runtime::get_executing_script_hash();
            instance.balances.insert(&owner, &initial_supply);
            
            // Emit transfer event
            instance.emit_transfer(None, Some(owner), initial_supply);
            
            instance
        }
        
        // Get token name
        #[safe]
        pub fn name(&self) -> String {
            "My Neo Token".into()
        }
        
        // Get token symbol
        #[safe]
        pub fn symbol(&self) -> String {
            "MNT".into()
        }
        
        // Get token decimals
        #[safe]
        pub fn decimals(&self) -> u8 {
            8
        }
        
        // Get total supply
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        // Get balance of address
        #[safe]
        pub fn balance_of(&self, account: &Address) -> u64 {
            self.balances.get(account).unwrap_or_default()
        }
        
        // Transfer tokens
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Ensure positive amount
            if amount == 0 {
                return false;
            }
            
            // Check balance
            let from_balance = self.balance_of(&from);
            if from_balance < amount {
                return false;
            }
            
            // Check if sender is authorized
            if from != Runtime::check_witness() {
                return false;
            }
            
            // Update balances
            if from_balance == amount {
                self.balances.remove(&from);
            } else {
                self.balances.insert(&from, &(from_balance - amount));
            }
            
            let to_balance = self.balance_of(&to);
            self.balances.insert(&to, &(to_balance + amount));
            
            // Emit transfer event
            self.emit_transfer(Some(from), Some(to), amount);
            
            true
        }
        
        // Helper to emit Transfer event in NEP-17 format
        fn emit_transfer(&self, from: Option<Address>, to: Option<Address>, amount: u64) {
            let event_name = ByteString::from("Transfer");
            let mut event_data = Array::<Any>::new();
            
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }
            
            event_data.push(Any::from(amount));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
}
```

### 4. Build your contract

```bash
# Build the WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Strip the WebAssembly binary (optional but recommended)
wasm-strip target/wasm32-unknown-unknown/release/my_neo_contract.wasm

# Convert to Neo VM format using neo-compiler
neo-compiler target/wasm32-unknown-unknown/release/my_neo_contract.wasm
```

This will generate a NEF file and manifest that can be deployed to a Neo N3 blockchain.

## Testing Your Contract

The framework provides utilities for testing your contracts:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract::testing::*;
    
    #[test]
    fn test_token_basics() {
        // Create a test environment
        let mut env = TestEnvironment::new();
        
        // Deploy the contract
        let contract = env.deploy_contract::<token::TokenContract>();
        
        // Test basic properties
        assert_eq!(contract.name(), "My Neo Token");
        assert_eq!(contract.symbol(), "MNT");
        assert_eq!(contract.decimals(), 8);
        
        // Test total supply
        assert_eq!(contract.total_supply(), 1_000_000_000);
        
        // More tests can be added here
    }
}
```

## Next Steps

- Explore the [examples](examples.md) to learn more contract patterns
- Read the [API reference](api/README.md) for detailed documentation
- Learn about [best practices](best-practices.md) for Neo N3 contract development
- Contribute to the framework by following our [contribution guidelines](contributing.md)
