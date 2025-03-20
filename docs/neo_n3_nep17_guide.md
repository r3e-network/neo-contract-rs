# Neo N3 NEP-17 Token Standard Implementation Guide

This guide provides a comprehensive overview of implementing the NEP-17 fungible token standard for Neo N3 using the neo-contract-rs framework.

## Overview

NEP-17 is the fungible token standard for Neo N3, replacing the previous NEP-5 standard used in Neo Legacy. It defines a set of methods and events that a compliant token contract must implement to ensure interoperability with wallets, exchanges, and other smart contracts.

## Required Methods

A NEP-17 compliant token must implement the following methods:

### 1. Symbol

Returns the token's symbol.

```rust
#[safe]
pub fn symbol(&self) -> ByteString {
    self.symbol.get().unwrap_or_default()
}
```

### 2. Decimals

Returns the number of decimal places the token uses.

```rust
#[safe]
pub fn decimals(&self) -> u8 {
    self.decimals.get().unwrap_or_default()
}
```

### 3. TotalSupply

Returns the total token supply.

```rust
#[safe]
pub fn total_supply(&self) -> u64 {
    self.total_supply.get().unwrap_or_default()
}
```

### 4. BalanceOf

Returns the token balance of a specific address.

```rust
#[safe]
pub fn balance_of(&self, account: H160) -> u64 {
    self.balances.get(&account).unwrap_or_default()
}
```

### 5. Transfer

Transfers tokens from one address to another.

```rust
#[method]
#[no_reentry]
pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
    // Verify the transaction sender is authorized
    assert!(Runtime::check_witness(&from), "No authorization");
    
    // Check for valid to address
    assert!(to != H160::zero(), "Cannot transfer to zero address");
    
    // Check if amount is greater than 0
    if amount == 0 {
        return true;
    }
    
    // Check if from has sufficient balance
    let from_balance = self.balance_of(from);
    assert!(from_balance >= amount, "Insufficient balance");
    
    // Update balances
    let new_from_balance = from_balance - amount;
    if new_from_balance > 0 {
        self.balances.insert(from, new_from_balance);
    } else {
        self.balances.remove(&from);
    }
    
    let to_balance = self.balance_of(to);
    self.balances.insert(to, to_balance + amount);
    
    // Emit the transfer event
    Transfer {
        from: Some(from),
        to: Some(to),
        amount: amount
    }.notify();
    
    // If the recipient is a contract, call its onNEP17Payment method
    if data.len() > 0 && ContractManagement::get_contract(&to).is_some() {
        let on_nep17_payment = "onNEP17Payment";
        let mut args = Array::<Any>::new();
        args.push(Any::from(from));
        args.push(Any::from(amount));
        args.push(Any::from(data));
        
        Contract::call(&to, on_nep17_payment, &args);
    }
    
    true
}
```

## Required Events

### Transfer Event

A compliant NEP-17 token must emit a `Transfer` event when tokens are transferred, including when tokens are created (from is null) or destroyed (to is null).

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,  // None for minting operations
    #[index]
    pub to: Option<H160>,    // None for burning operations
    pub amount: u64,
}

// Usage examples:
// For transfers:
Transfer {
    from: Some(sender),
    to: Some(recipient),
    amount: 100
}.notify();

// For minting:
Transfer {
    from: None,
    to: Some(recipient),
    amount: 1000
}.notify();

// For burning:
Transfer {
    from: Some(sender),
    to: None,
    amount: 500
}.notify();
```

## Complete Implementation

Here's a complete implementation of a NEP-17 token contract with all required methods and events:

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
    name: StorageItem<ByteString>,
    #[storage]
    symbol: StorageItem<ByteString>,
    #[storage]
    decimals: StorageItem<u8>,
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
            name: StorageItem::new(b"name"),
            symbol: StorageItem::new(b"symbol"),
            decimals: StorageItem::new(b"decimals"),
            owner: StorageItem::new(b"owner"),
        };
        
        // Initialize contract state
        instance.name.set(&ByteString::from("NEP-17 Token"));
        instance.symbol.set(&ByteString::from("NPT"));
        instance.decimals.set(&8);
        instance.total_supply.set(&initial_supply);
        instance.owner.set(&owner);
        
        // Mint initial supply to owner
        instance.balances.insert(owner, initial_supply);
        
        // Emit transfer event for initial supply
        Transfer {
            from: None,
            to: Some(owner),
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // NEP-17 required methods
    
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
    
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
        // Verify the transaction sender is authorized
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Check for valid to address
        assert!(to != H160::zero(), "Cannot transfer to zero address");
        
        // Check if amount is greater than 0
        if amount == 0 {
            return true;
        }
        
        // Check if from has sufficient balance
        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(from, new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + amount);
        
        // Emit the transfer event
        Transfer {
            from: Some(from),
            to: Some(to),
            amount: amount
        }.notify();
        
        // If the recipient is a contract, call its onNEP17Payment method
        if data.len() > 0 && ContractManagement::get_contract(&to).is_some() {
            let on_nep17_payment = "onNEP17Payment";
            let mut args = Array::<Any>::new();
            args.push(Any::from(from));
            args.push(Any::from(amount));
            args.push(Any::from(data));
            
            Contract::call(&to, on_nep17_payment, &args);
        }
        
        true
    }
    
    // Optional methods for token management
    
    #[method]
    #[no_reentry]
    pub fn mint(&mut self, to: H160, amount: u64) -> bool {
        // Only owner can mint
        let owner = self.owner.get().unwrap();
        assert!(Runtime::check_witness(&owner), "No authorization");
        
        // Check for valid to address
        assert!(to != H160::zero(), "Cannot mint to zero address");
        
        // Update recipient balance
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + amount);
        
        // Update total supply
        let supply = self.total_supply();
        self.total_supply.set(&(supply + amount));
        
        // Emit transfer event for minting
        Transfer {
            from: None,
            to: Some(to),
            amount: amount
        }.notify();
        
        true
    }
    
    #[method]
    #[no_reentry]
    pub fn burn(&mut self, from: H160, amount: u64) -> bool {
        // Verify authorization
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Check balance
        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balance
        let new_balance = from_balance - amount;
        if new_balance > 0 {
            self.balances.insert(from, new_balance);
        } else {
            self.balances.remove(&from);
        }
        
        // Update total supply
        let supply = self.total_supply();
        self.total_supply.set(&(supply - amount));
        
        // Emit transfer event for burning
        Transfer {
            from: Some(from),
            to: None,
            amount: amount
        }.notify();
        
        true
    }
}
```

## Integrating with Other Contracts

NEP-17 tokens can interact with other contracts through the `onNEP17Payment` method. When transferring tokens to a contract, the contract's `onNEP17Payment` method is called if it exists.

### Contract receiving NEP-17 tokens

```rust
#[method]
pub fn on_nep17_payment(&mut self, from: H160, amount: u64, data: Vec<u8>) {
    // Only accept calls from the NEP-17 token contract
    let token_contract = H160::from_str("YOUR_TOKEN_HASH_HERE").unwrap();
    assert!(Runtime::calling_script_hash() == token_contract, "Invalid token");
    
    // Process the received tokens...
}
```

## Testing NEP-17 Tokens

When testing your NEP-17 token contract, make sure to test all the required methods and events:

1. Test token initialization with correct name, symbol, decimals, and initial supply
2. Test balance_of for accounts with and without tokens
3. Test transfers between accounts, including edge cases
4. Test transfer to contract accounts with onNEP17Payment
5. Test event emission for transfers, minting, and burning
6. Test authorization requirements for transfer and management methods

## Security Considerations

1. **Reentrancy Protection**: Use `#[no_reentry]` to prevent reentrancy attacks, especially when calling external contracts.
2. **Integer Overflow**: Ensure all arithmetic operations cannot overflow.
3. **Authorization**: Always verify that the sender is authorized to perform operations.
4. **Zero Address**: Prevent transfers to the zero address.
5. **Gas Optimization**: Optimize gas usage by removing zero balances from storage.

## Compliance Checklist

- [x] Implements all required methods (symbol, decimals, totalSupply, balanceOf, transfer)
- [x] Emits Transfer events for all token movements
- [x] Handles null addresses correctly for minting and burning
- [x] Verifies transaction authorization
- [x] Maintains accurate token balances
- [x] Uses proper indexing for Transfer events
- [x] Implements contract notification via onNEP17Payment

By following this guide, you can create a fully compliant NEP-17 token that integrates smoothly with the Neo N3 ecosystem, including wallets, exchanges, and other smart contracts.

For more information, refer to:
- [Neo N3 Implementation Guide](./neo_n3_implementation_guide.md)
- [Neo N3 Runtime Guide](./neo_n3_runtime_guide.md)
- [Neo N3 Storage Guide](./neo_n3_storage_guide.md)
- [Neo N3 Security Guide](./neo_n3_security_guide.md)
