# Neo Contract Rust Examples

This directory contains example smart contracts written using the Neo Contract Rust framework. These examples demonstrate various features and patterns for developing Neo N3 smart contracts with Rust.

## Examples Overview

### Basic Examples

- **[hello_world](./hello_world/)**: A simple contract that demonstrates basic storage and method definition.
- **[transfer](./transfer/)**: Shows how to handle NEP-17 token transfers within a contract.

### Token Standards

- **[nep17](./nep17/)**: A basic implementation of the NEP-17 fungible token standard.
- **[nep17_token](./nep17_token/)**: An extended NEP-17 token implementation with additional features.

### Ink! Style Examples

- **[ink_style_token](./ink_style_token/)**: NEP-17 token implementation using the ink! style syntax.
- **[ink_style_token_with_attributes](./ink_style_token_with_attributes/)**: Similar to the above, but with attribute macros.
- **[ink_style_complete](./ink_style_complete/)**: A comprehensive example showing the full power of ink! style syntax.

### Advanced Patterns

- **[contract_call](./contract_call/)**: Demonstrates how to make calls between contracts.
- **[csharp_features](./csharp_features/)**: Shows how to implement features commonly found in C# contracts.

### Real-world Applications

- **[neoburger](./neoburger/)**: Implementation of the NeoBurger protocol.
- **[neoburger_agent](./neoburger_agent/)**: Agent contract for the NeoBurger protocol.
- **[neoburger_governance](./neoburger_governance/)**: Governance contract for the NeoBurger protocol.

## Building Examples

Each example directory contains its own code and potentially a Makefile for building. To build an example:

1. Navigate to the example directory:
   ```
   cd examples/hello_world
   ```

2. If a Makefile is present, build using:
   ```
   make
   ```
   or
   ```
   make BUILD_MODE=release
   ```

3. If no Makefile is present, build manually:
   ```bash
   # Compile to WASM
   cargo build --target wasm32-unknown-unknown

   # Convert to NEF
   ../../bin/neo-wasm -input target/wasm32-unknown-unknown/debug/hello_world.wasm -output build -name hello_world
   ```

## Example Contract: Hello World

Here's a simple example of a Hello World contract:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use neo_contract::prelude::*;

#[neo_contract]
pub mod contract {
    use super::*;

    #[neo_storage]
    struct HelloWorld {
        message: StorageItem<String>,
    }

    impl HelloWorld {
        pub fn new() -> Self {
            Self {
                message: StorageItem::new(b"message"),
            }
        }

        #[neo_method(return_value = String)]
        #[safe]
        pub fn get_message(&self) -> String {
            self.message.get().unwrap_or_else(|| "Hello, Neo!".to_string())
        }

        #[neo_method]
        pub fn set_message(&mut self, message: String) {
            self.message.set(&message);
        }
    }
}
```

## Example Contract: NEP-17 Token

Here's a simplified example of a NEP-17 token contract:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use neo_contract::prelude::*;

#[neo_contract]
pub mod token {
    use super::*;

    #[neo_storage]
    struct Token {
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }

    impl Token {
        pub fn new() -> Self {
            Self {
                total_supply: StorageItem::new(b"total_supply"),
                balances: StorageMap::new(b"balances"),
            }
        }

        #[neo_method(return_value = u64)]
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or(0)
        }

        #[neo_method(return_value = u64)]
        #[safe]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or(0)
        }

        #[neo_method(return_value = bool)]
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64, data: Vec<u8>) -> bool {
            // Verify sender
            assert!(runtime::check_witness(&from), "No authorization");
            
            // Get balances
            let from_balance = self.balance_of(from);
            let to_balance = self.balance_of(to);
            
            // Check sufficient funds
            assert!(from_balance >= amount, "Insufficient funds");
            
            // Handle zero transfer
            if amount == 0 {
                events::transfer(from, to, 0);
                return true;
            }
            
            // Update balances
            if from_balance == amount {
                self.balances.delete(&from);
            } else {
                self.balances.set(&from, &(from_balance - amount));
            }
            
            self.balances.set(&to, &(to_balance + amount));
            
            // Emit event
            events::transfer(from, to, amount);
            
            // Post transfer hook
            if to.is_contract() {
                contract::call(
                    &to,
                    "onNEP17Payment",
                    &[from, amount, data],
                    CallFlags::ALL,
                );
            }
            
            true
        }
    }
}
```

Check the individual example directories for more detailed code and documentation.
