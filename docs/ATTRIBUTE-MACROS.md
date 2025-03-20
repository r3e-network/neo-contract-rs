# NEO Contract Attribute Macros Guide

This document provides a detailed guide to the attribute macros used in NEO Contract Rust for smart contract development.

## Overview

NEO Contract Rust uses attribute macros to define contract structure, events, storage, and methods. These macros provide a clean, declarative way to define your contract's components.

The following attribute macros are available:

- `#[neo_contract::contract]` - Marks a struct or module as a NEO smart contract
- `#[contract_author]` - Specifies the contract author
- `#[contract_description]` - Provides a description of the contract
- `#[contract_version]` - Specifies the contract version
- `#[supported_standards]` - Lists supported NEO standards (e.g., "NEP-17")
- `#[neo_contract::event]` - Defines an event that can be emitted
- `#[index]` - Marks event fields that should be indexed
- `#[storage]` - Defines storage fields in the contract struct
- `#[constructor]` - Marks a method as the contract constructor
- `#[method]` - Marks a method as a public contract method
- `#[safe]` - Marks a method as read-only (no state changes)
- `#[no_reentry]` - Protects methods from reentrancy attacks

## Contract Structure

### Basic Contract Structure

```rust
#[neo_contract::contract]
#[contract_author("Your Name")]
#[contract_description("A description of your contract")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct MyContract {
    // Contract storage fields
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
}

// OR

#[neo_contract::contract]
mod my_contract {
    // Contract code goes here
}
```

### Events

Events allow your contract to notify external systems about state changes. Events can have indexed fields for efficient searching.

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}

// Usage:
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: 100
}.notify();
```

### Storage

The storage attribute defines the persistent state of your contract.

```rust
#[neo_contract::contract]
pub struct MyContract {
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
    #[storage]
    owner: StorageItem<H160>,
}

impl MyContract {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        let mut instance = Self {
            balances: StorageMap::new(b"balances"),
            total_supply: StorageItem::new(b"total_supply"),
            owner: StorageItem::new(b"owner"),
        };
        
        instance.owner.set(&owner);
        instance
    }
}
```

### Methods

Methods define the contract's functionality. There are several types:

#### Constructor
```rust
#[constructor]
pub fn new(owner: H160, initial_supply: u64) -> Self {
    // Initialize contract state
    let mut instance = Self {
        balances: StorageMap::new(b"balances"),
        total_supply: StorageItem::new(b"total_supply"),
        owner: StorageItem::new(b"owner"),
    };
    
    // Initialize state
    instance.total_supply.set(&initial_supply);
    instance.owner.set(&owner);
    instance.balances.insert(owner, initial_supply);
    
    instance
}
```

#### Public Methods
```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Implementation
    true
}
```

#### Read-Only Methods
```rust
#[safe]
pub fn balance_of(&self, address: H160) -> u64 {
    self.balances.get(&address).unwrap_or(0)
}
```

#### Protected Methods (Reentrancy Protection)
```rust
#[method]
#[no_reentry]
pub fn withdraw(&mut self, account: H160, amount: u64) -> bool {
    // Protected from reentrancy attacks
    true
}
```

## Complete Example

Here's a complete example of a NEP-17 compatible token using attribute macros:

```rust
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use neo_contract::prelude::*;

// Define events
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}

// Define the contract with storage
#[neo_contract::contract]
pub struct SampleToken {
    // Storage fields
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
    #[storage]
    name: StorageItem<ByteString>,
    #[storage]
    symbol: StorageItem<ByteString>,
    #[storage]
    decimals: StorageItem<u8>,
    #[storage]
    owner: StorageItem<H160>,
}

impl SampleToken {
    // Constructor
    #[constructor]
    pub fn new(owner: H160, initial_supply: u64) -> Self {
        let mut instance = Self {
            balances: StorageMap::new(b"balances"),
            total_supply: StorageItem::new(b"total_supply"),
            name: StorageItem::new(b"name"),
            symbol: StorageItem::new(b"symbol"),
            decimals: StorageItem::new(b"decimals"),
            owner: StorageItem::new(b"owner"),
        };
        
        // Initialize contract state
        instance.name.set(&ByteString::from("Sample Token"));
        instance.symbol.set(&ByteString::from("SMPL"));
        instance.decimals.set(&8);
        instance.total_supply.set(&initial_supply);
        instance.owner.set(&owner);
        
        // Mint initial supply to owner
        instance.balances.insert(owner, initial_supply);
        
        // Emit transfer event for minting
        Transfer {
            from: None,
            to: Some(owner),
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // Standard NEP-17 methods
    
    // Read-only methods
    #[safe]
    pub fn name(&self) -> ByteString {
        self.name.get().unwrap_or_default()
    }
    
    #[safe]
    pub fn symbol(&self) -> ByteString {
        self.symbol.get().unwrap_or_default()
    }
    
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals.get().unwrap_or(0)
    }
    
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }
    
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    // State-changing methods with reentrancy protection
    
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
        // Check authorization
        assert!(Runtime::check_witness(&from), "Not authorized");
        
        // Check for valid to address
        assert!(to != H160::zero(), "Invalid to address");
        
        // Check amount
        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        if amount > 0 {
            // Subtract from sender
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Add to recipient
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + amount);
            
            // Emit transfer event
            Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount
            }.notify();
        }
        
        true
    }
}
```

## Best Practices

1. **Group Related Methods**: Keep related functionality together for better readability.
2. **Document Your Code**: Add comments explaining the purpose of each method and parameter.
3. **Use Appropriate Attribute Macros**: Choose the right attribute for each component of your contract.
4. **Check Authorization**: Always verify the caller's identity for state-changing methods.
5. **Protect Against Reentrancy**: Use `#[no_reentry]` for methods that modify state and make external calls.
6. **Emit Events**: Notify external systems about important state changes.
7. **Initialize All Storage Fields**: Always initialize your storage in the constructor.
8. **Handle Edge Cases**: Include proper error handling and assertions in your methods.
9. **Follow NEP Standards**: Adhere to NEO Enhancement Proposals for compatibility. 