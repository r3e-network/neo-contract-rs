# Neo Smart Contract Annotations

This document provides a complete reference for the annotation system used in Neo Rust smart contracts. These annotations simplify contract development by automatically generating boilerplate code and ensuring your contract adheres to Neo N3 standards.

## Core Annotations

### `#[neo_contract::contract]`

Marks a struct or module as a Neo smart contract. This is the main entry point for your contract.

```rust
#[neo_contract::contract]
pub struct MyToken {
    // Contract storage fields
}

// OR

#[neo_contract::contract]
mod my_contract {
    // Contract implementation
}
```

This annotation:
- Generates Neo VM entry points (`_deploy`, `main`)
- Registers the contract for manifest generation
- Sets up the default dispatch mechanism

### `#[constructor]`

Identifies the initialization method called when the contract is deployed.

```rust
#[constructor]
pub fn new(owner: H160) -> Self {
    // Initialize contract state
    Self {
        // ...
    }
}
```

Only one constructor per contract is allowed. The constructor:
- Is automatically called during contract deployment
- Should return an instance of the contract
- Is used to initialize contract storage

### `#[method]`

Exposes a function as a callable contract method that can be invoked externally.

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Method implementation
}
```

Methods are:
- Automatically registered in the contract manifest
- Accessible via Neo VM script invocation
- Typically return serializable types (bool, numbers, ByteString, H160, etc.)

### `#[safe]`

Marks a method as read-only, optimizing gas costs for calls that don't modify state.

```rust
#[safe]
pub fn balance_of(&self, account: H160) -> u64 {
    // Read-only implementation
}
```

Safe methods:
- Cannot modify contract storage
- Have lower gas costs
- Are flagged as safe in the contract manifest

### `#[no_reentry]`

Prevents reentrancy attacks by blocking recursive calls to the method.

```rust
#[method]
#[no_reentry]
pub fn withdraw(&mut self, account: H160, amount: u64) -> bool {
    // Protected from reentrancy
}
```

This annotation:
- Adds storage-based reentrancy protection
- Reverts transactions if a reentrancy attack is detected
- Should be used on any state-changing method that makes external calls

## Event Annotations

### `#[neo_contract::event]`

Defines a structured event that can be emitted and indexed by the blockchain.

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}
```

Events:
- Are registered in the contract manifest
- Can be queried efficiently by applications
- Support optional indexing of parameters

### `#[index]`

Marks an event parameter to be indexed, which allows efficient filtering when querying events.

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]  // This field can be queried efficiently
    pub from: H160,
    // ...
}
```

Indexed parameters:
- Allow fast filtering of events
- Are limited to 2-3 per event (blockchain dependent)
- Should be used for parameters you need to search by

## Storage Annotations

### `#[storage]`

Marks fields in your contract struct as persistent storage.

```rust
#[neo_contract::contract]
pub struct MyContract {
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
}
```

Storage fields:
- Are automatically initialized with appropriate prefixes
- Persist contract state between calls
- Support various types through storage adapters

## Best Practices

1. **Use Neo Types**: Always use Neo-specific types like `H160`, `ByteString`, and `StorageMap` for better integration.

2. **Event Structure**: Keep events simple and focused on one type of notification. Use the `#[index]` annotation for key fields.

3. **Method Organization**: Group methods by functionality and add clear comments explaining their purpose.

4. **Safe Methods**: Mark all read-only methods with `#[safe]` to optimize gas costs.

5. **Reentrancy Protection**: Use `#[no_reentry]` on any method that modifies state and makes external calls.

6. **Constructor Initialization**: Always initialize all storage fields in the constructor to avoid null values.

## Example Contract

Here's a complete example showing all annotations in action:

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
pub struct TokenContract {
    // Storage fields
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
    #[storage]
    owner: StorageItem<H160>,
}

impl TokenContract {
    // Constructor
    #[constructor]
    pub fn new(owner: H160, initial_supply: u64) -> Self {
        let mut instance = Self {
            balances: StorageMap::new(b"balances"),
            total_supply: StorageItem::new(b"total_supply"),
            owner: StorageItem::new(b"owner"),
        };
        
        instance.total_supply.set(&initial_supply);
        instance.owner.set(&owner);
        instance.balances.insert(owner, initial_supply);
        
        // Emit transfer event for minting
        Transfer {
            from: None,
            to: Some(owner),
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // Read-only methods
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

## Compilation

Contracts using these annotations are compiled using the Neo Rust compiler:

```bash
cargo build --target wasm32-unknown-unknown --release
neo-compiler compile target/wasm32-unknown-unknown/release/my_contract.wasm
```

The compiler automatically processes the annotations to generate:
1. NEF (Neo Executable Format) file
2. Contract manifest with methods, events and permissions
3. Debug information (if enabled)

## Further Resources

- [NEP-17 Reference Implementation](../examples/nep17-token/src/lib.rs)
- [Neo Developer Documentation](https://developers.neo.org/)
- [Neo Contract API Reference](../neo-contract/docs/API.md) 