# Neo Rust Contract Syntax Guide

This document provides a comprehensive guide to the correct syntax for Neo Rust smart contract development, based on the latest implementation of the framework.

## Core Annotations

### Contract Structure

Define a contract using the `#[neo_contract::contract]` annotation:

```rust
#[neo_contract::contract]
pub struct MyToken {
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
}
```

### Contract Metadata

You can add metadata to your contract using the following annotations:

```rust
#[neo_contract::contract]
#[contract_author("Your Name or Organization")]
#[contract_email("contact@example.com")]
#[contract_description("Description of what your contract does")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]
pub struct MyToken {
    // Contract storage fields
}
```

These metadata annotations provide important information about your contract:
- `#[contract_author]`: The name of the author or organization behind the contract
- `#[contract_email]`: Contact email for the contract developer or maintainer
- `#[contract_description]`: A brief description of the contract's purpose and functionality
- `#[contract_version]`: The version number of the contract
- `#[supported_standards]`: Neo ecosystem standards supported by the contract (e.g., "NEP-17", "NEP-11")

### Event Definition

Define events using the `#[neo_contract::event]` annotation:

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

### Method Types

#### Constructor

```rust
#[constructor]
pub fn new(owner: H160, initial_supply: u64) -> Self {
    let mut instance = Self {
        balances: StorageMap::new(b"balances"),
        total_supply: StorageItem::new(b"total_supply"),
    };
    
    // Initialize contract state
    instance.total_supply.set(&initial_supply);
    instance.balances.insert(owner, initial_supply);
    
    instance
}
```

#### Read-only Methods

```rust
#[safe]
pub fn balance_of(&self, account: H160) -> u64 {
    self.balances.get(&account).unwrap_or(0)
}
```

#### State-changing Methods

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
    // Implementation
    true
}
```

#### Protected Methods (with Reentrancy Protection)

```rust
#[method]
#[no_reentry]
pub fn withdraw(&mut self, account: H160, amount: u64) -> bool {
    // Implementation with reentrancy protection
    true
}
```

## Event Emission

Emit events using the `.notify()` method:

```rust
Transfer {
    from: Some(sender),
    to: Some(receiver),
    amount: amount
}.notify();
```

## Storage Definitions

Define storage fields using the `#[storage]` annotation within the contract struct:

```rust
#[neo_contract::contract]
pub struct MyToken {
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
}
```

## Neo-specific Types

- `H160`: 20-byte hash, used for addresses and contract identifiers
- `ByteString`: String type for Neo VM
- `StorageMap<K, V>`: Key-value storage collection
- `StorageItem<T>`: Single item storage
- `Array<T>`: Array type for Neo VM
- `Map<K, V>`: Map type for Neo VM
- `Any`: Dynamic type for Neo VM

## Complete Example

Here's a complete example of a simple NEP-17 token contract:

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
#[contract_author("Example Author")]
#[contract_description("Example NEP-17 Token")]
#[contract_version("1.0.0")]
#[supported_standards("NEP-17")]
pub struct SimpleToken {
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

impl SimpleToken {
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
        instance.name.set(&ByteString::from("Simple Token"));
        instance.symbol.set(&ByteString::from("SPL"));
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
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
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

## Further Resources

For more detailed examples and documentation, see:

- [Annotation Reference](ANNOTATIONS.md): Complete guide to all annotations
- [Attribute Macros Guide](ATTRIBUTE-MACROS.md): Details on all attribute macros
- [Events Guide](events_guide.md): Guide to defining and emitting events
- [NEP-17 Guide](neo_n3_nep17_guide.md): Guide to implementing NEP-17 tokens
- [Example: Documentation Example](../examples/documentation_example/src/lib.rs): Comprehensive example contract

## Changes from Previous Versions

The key changes from previous documentation versions are:

1. Use `#[neo_contract::event]` instead of `#[event]` for event definitions
2. Use `#[neo_contract::contract]` instead of `#[contract]` for contract definitions
3. Use `.notify()` instead of `.emit()` for event emission
4. All event fields should have `pub` visibility
5. Storage fields now use the `#[storage]` attribute inside the contract struct
6. Contract metadata using `#[contract_author]`, `#[contract_description]`, etc.

## Best Practices

1. **Group Related Methods**: Keep related functionality together for better readability.
2. **Document Your Code**: Add comments explaining the purpose of each method and parameter.
3. **Use Appropriate Annotations**: Choose the right annotation for each component of your contract.
4. **Check Authorization**: Always verify the caller's identity for state-changing methods.
5. **Protect Against Reentrancy**: Use `#[no_reentry]` for methods that modify state and make external calls.
6. **Emit Events**: Notify external systems about important state changes.
7. **Initialize All Storage Fields**: Always initialize your storage in the constructor.
8. **Handle Edge Cases**: Include proper error handling and assertions in your methods.
9. **Include Contract Metadata**: Provide appropriate authorship and description information.