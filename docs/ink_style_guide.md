# Neo N3 Smart Contracts with ink! Style

This guide explains how to write Neo N3 smart contracts using the ink! style, which offers a more ergonomic and declarative approach to smart contract development.

## Overview

Neo Contract RS now supports the ink! style attribute system for smart contracts. This approach uses Rust attributes to define contract elements in a more intuitive way, similar to the style used in Parity's ink! framework for Substrate.

## Basic Structure

Here's the basic structure of an ink! style Neo N3 contract:

```rust
use neo_contract::prelude::ink_style::*;

#[contract]
#[contract_author("Your Name")]
#[contract_description("A description of your contract")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]
mod token_contract {
    use super::*;

    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map<H160, Int256>,
        symbol: ByteString,
        decimals: Int256,
    }

    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = Map::new();
            let owner = Runtime::calling_script_hash();
            
            balances.put(owner.clone(), initial_supply.clone());
            
            Self {
                total_supply: initial_supply,
                balances,
                symbol: ByteString::from("TKN"),
                decimals: 8.into(),
            }
        }
        
        #[message]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }
        
        #[message]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
        }
        
        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            // Check if the sender has enough balance
            if let Some(from_balance) = self.balances.get(&from) {
                if from_balance < &amount {
                    return false;
                }
            } else {
                return false;
            }
            
            // Update balances
            if let Some(from_balance) = self.balances.get_mut(&from) {
                *from_balance -= amount.clone();
            }
            
            let to_balance = self.balances.get_mut(&to).unwrap_or(&mut Int256::zero());
            *to_balance += amount.clone();
            
            // Emit transfer event
            Self::transfer_event(from, to, amount);
            
            true
        }
        
        #[event]
        pub fn transfer_event(from: H160, to: H160, amount: Int256) {}
    }
}
```

## Key Attributes

The ink! style attributes provide a clear and intuitive way to define contract elements:

### Contract Attributes

- `#[contract]`: Marks a module as a Neo N3 smart contract
- `#[contract_author("Author Name")]`: Specifies the contract author
- `#[contract_email("author@example.com")]`: Specifies contact email
- `#[contract_description("Description")]`: Provides a contract description
- `#[contract_version("1.0.0")]`: Specifies the contract version
- `#[supported_standards("NEP-17")]`: Declares supported standards

### Storage and Implementation

- `#[storage]`: Marks a struct as the contract's storage
- `#[constructor]`: Marks a method as a contract constructor
- `#[message]`: Marks a method as a contract message (callable from outside)
- `#[event]`: Marks a method as a contract event

### Security Attributes

- `#[safe]`: Marks a function as safe
- `#[no_reentrant]`: Prevents reentrancy attacks on a contract
- `#[no_reentrant_method]`: Prevents reentrancy attacks on a method

## Importing Attributes

You can import all the ink! style attributes and common types in one go:

```rust
use neo_contract::prelude::ink_style::*;
```

Or import specific attributes:

```rust
use neo_contract::{
    contract, storage, constructor, message, event,
    builtin::{H160, Int256, ByteString, Map},
};
```

## Comparison with Traditional Neo N3 Contracts

Here's how the ink! style compares to the traditional approach:

### Traditional Approach

```rust
use neo_contract::{
    builtin::{H160, Int256, ByteString, Map},
    smart_contract,
};

pub struct TokenStorage {
    total_supply: Int256,
    balances: Map<H160, Int256>,
}

#[smart_contract]
impl TokenStorage {
    pub fn new(initial_supply: Int256) -> Self {
        // Implementation...
    }
    
    pub fn total_supply(&self) -> Int256 {
        // Implementation...
    }
    
    // More methods...
}
```

### ink! Style Approach

```rust
use neo_contract::prelude::ink_style::*;

#[contract]
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
        pub fn total_supply(&self) -> Int256 {
            // Implementation...
        }
        
        // More methods...
    }
}
```

The ink! style is more declarative and clearly separates the contract's storage from its methods, making it easier to understand the contract's structure at a glance.

## Examples

See the following examples for ink! style contracts:

- [ink_style_token](../examples/ink_style_token/src/lib.rs): A basic token example
- [transfer](../examples/transfer/src/lib.rs): Simple transfer functionality
- [neoburger](../examples/neoburger/src/lib.rs): More complex contract with multiple functions

## Best Practices

1. Always use the `#[storage]` attribute for your contract's storage struct
2. Use `#[constructor]` for initialization methods
3. Mark all public methods with `#[message]`
4. Define events with `#[event]`
5. Keep your contract modules concise and focused
6. Add appropriate metadata attributes
7. Use the Map type's get() method for cleaner code

## Migration Guide

If you're migrating from the traditional approach:

1. Wrap your contract in a module with the `#[contract]` attribute
2. Mark your storage struct with `#[storage]`
3. Mark constructor methods with `#[constructor]`
4. Mark public methods with `#[message]`
5. Convert events to use the `#[event]` attribute
6. Add metadata attributes as needed

The framework is designed to support both approaches, so you can gradually migrate your contracts to the ink! style.
