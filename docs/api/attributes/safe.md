# Safe Methods in Neo N3 Contracts

This document explains the concept of safe methods in Neo N3 smart contracts and how to implement them in the Rust framework.

## Overview

In the Neo N3 smart contract framework, methods can be classified as either "safe" (read-only) or "non-safe" (state-modifying). This distinction is important for optimizing contract execution and ensuring proper access control.

Safe methods are marked with the `#[safe]` attribute and are represented in the contract manifest with `"safe": true`.

## Definition

A **safe method** is a contract method that:
- Does not modify the contract's state
- Does not write to storage
- Does not transfer assets
- Does not make state-changing calls to other contracts
- Has no side effects beyond returning a value

## Usage

To mark a method as safe, use the `#[safe]` attribute:

```rust
use neo_contract::prelude::*;

#[contract]
mod token {
    use super::*;
    
    #[storage]
    pub struct TokenContract {
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    impl TokenContract {
        // Safe method - only reads from storage
        #[safe]
        pub fn name(&self) -> String {
            "My Token".into()
        }
        
        // Safe method - only reads from storage
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        // Safe method - only reads from storage
        #[safe]
        pub fn balance_of(&self, account: &Address) -> u64 {
            self.balances.get(account).unwrap_or_default()
        }
        
        // Non-safe method - modifies state
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Implementation that modifies state
            // ...
        }
    }
}
```

## Benefits

Marking methods as safe provides several benefits:

1. **Performance**: Safe methods can be executed more efficiently by the Neo N3 VM
2. **Security**: Clearly distinguishes which methods can modify state
3. **Read-Only Access**: Allows external systems to safely query contract state
4. **Gas Optimization**: Safe methods typically consume less GAS
5. **Static Analysis**: Enables better static analysis of contract behavior

## Contract Manifest

When a method is marked as safe, it will be reflected in the generated contract manifest:

```json
{
  "name": "Token",
  "methods": [
    {
      "name": "name",
      "parameters": [],
      "returntype": "String",
      "safe": true
    },
    {
      "name": "total_supply",
      "parameters": [],
      "returntype": "Integer",
      "safe": true
    },
    {
      "name": "balance_of",
      "parameters": [
        {
          "name": "account",
          "type": "Hash160"
        }
      ],
      "returntype": "Integer",
      "safe": true
    },
    {
      "name": "transfer",
      "parameters": [
        {
          "name": "from",
          "type": "Hash160"
        },
        {
          "name": "to",
          "type": "Hash160"
        },
        {
          "name": "amount",
          "type": "Integer"
        }
      ],
      "returntype": "Boolean",
      "safe": false
    }
  ]
}
```

## Restrictions

Safe methods have the following restrictions:

1. Cannot modify storage (write operations)
2. Cannot transfer assets
3. Cannot call non-safe methods of other contracts
4. Cannot emit events (though this is relaxed in some implementations)

## Verification

The Neo N3 VM enforces the safety of methods at runtime. If a safe method attempts to perform a state-modifying operation, the VM will raise an exception.

## Best Practices

1. **Mark All Read-Only Methods**: Any method that only reads from storage should be marked as safe
2. **Self References**: Safe methods should typically take `&self` rather than `&mut self`
3. **Return Values**: Prefer returning values over mutable references
4. **Documentation**: Clearly document which methods are safe and which are not
5. **Testing**: Test safe methods to ensure they don't inadvertently modify state

## Example: NEP-17 Token Standard

In the NEP-17 token standard, the following methods should be marked as safe:

```rust
#[safe]
pub fn name(&self) -> String;

#[safe]
pub fn symbol(&self) -> String;

#[safe]
pub fn decimals(&self) -> u8;

#[safe]
pub fn total_supply(&self) -> u64;

#[safe]
pub fn balance_of(&self, account: &Address) -> u64;
```

While the transfer method should not be marked as safe:

```rust
pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool;
```
