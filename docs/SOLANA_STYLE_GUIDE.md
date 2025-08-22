# Neo N3 Smart Contracts with Solana-Style Syntax

## Overview

This guide explains how to write Neo N3 smart contracts using Solana-style syntax with the neo-contract-rs framework. This approach brings familiar and developer-friendly patterns inspired by Solana's development model to Neo N3 blockchain development, using `#[contract_impl]` as the primary pattern for contract implementation.

## Table of Contents

1. [Key Concepts](#key-concepts)
2. [Basic Structure](#basic-structure)
3. [Contract Implementation](#contract-implementation)
4. [Error Handling](#error-handling)
5. [Events and Notifications](#events-and-notifications)
6. [Examples](#examples)
7. [Migration Guide](#migration-guide)

## Key Concepts

### Contract Implementation Pattern

The main contract logic is defined using a struct with the `#[contract_impl]` attribute on the implementation block:

```rust
#[contract_author("My Contract")]
#[contract_version("1.0.0")]
pub struct MyContract {
    // Contract state fields
    owner: H160,
    balance: U256,
}

#[contract_impl]
impl MyContract {
    pub fn init() -> Self {
        Self {
            owner: Runtime::get_calling_script_hash(),
            balance: U256::zero(),
        }
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, amount: U256) -> Result<()> {
        require!(amount > U256::zero(), ContractError::InvalidAmount);
        // Transfer logic
        Ok(())
    }
    
    #[method]
    #[safe]
    pub fn get_balance(&self) -> U256 {
        self.balance
    }
}
```

### Method Attributes

Methods can be marked with specific attributes for Neo N3 integration:

- `#[method]` - Standard contract method (state-changing)
- `#[safe]` - Read-only method (no state changes)
- `#[method] #[safe]` - Read-only contract method

### Direct Parameter Pattern

Unlike traditional Context patterns, Solana-style methods use direct parameters with proper Neo N3 types:

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: U256) -> Result<bool> {
    require!(amount > U256::zero(), ContractError::InvalidAmount);
    require!(Runtime::check_witness(&from), ContractError::InvalidWitness);
    
    // Transfer logic with Neo N3 native types
    let from_balance = self.get_balance_of(&from);
    require!(from_balance >= amount, ContractError::InsufficientBalance);
    
    self.set_balance(&from, from_balance - amount);
    let to_balance = self.get_balance_of(&to);
    self.set_balance(&to, to_balance + amount);
    
    // Emit Neo N3 notification
    notify!("Transfer", from, to, amount);
    Ok(true)
}
```

## Basic Structure

### Contract Template

```rust
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

use neo_contract::prelude::*;

#[contract_author("Your Name")]
#[contract_version("1.0.0")]
#[contract_name("YourContract")]
#[contract_description("A Solana-style Neo N3 contract")]
pub struct YourContract {
    pub owner: H160,
    pub total_supply: U256,
    pub balances: StorageMap<H160, U256>,
}

#[contract_impl]
impl YourContract {
    pub fn init() -> Self {
        let owner = Runtime::get_calling_script_hash();
        let total_supply = U256::from(1_000_000);
        
        let mut contract = Self {
            owner,
            total_supply,
            balances: StorageMap::new(),
        };
        
        // Give initial supply to owner
        contract.balances.put(&owner, &total_supply);
        
        contract
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        ByteString::from_literal("YCT")
    }
    
    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        8
    }
    
    #[method]
    #[safe]
    pub fn total_supply(&self) -> U256 {
        self.total_supply
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> U256 {
        self.balances.get(&account).unwrap_or(U256::zero())
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
        let from = Runtime::get_calling_script_hash();
        self._transfer(from, to, amount)
    }
    
    fn _transfer(&mut self, from: H160, to: H160, amount: U256) -> Result<bool> {
        require!(amount > U256::zero(), ContractError::InvalidAmount);
        require!(Runtime::check_witness(&from), ContractError::InvalidWitness);
        
        let from_balance = self.balance_of(from);
        require!(from_balance >= amount, ContractError::InsufficientBalance);
        
        self.balances.put(&from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(&to, &(to_balance + amount));
        
        notify!("Transfer", from, to, amount);
        Ok(true)
    }
}
```

## Contract Implementation

### State Management

Use Neo N3 native storage types for persistent state:

```rust
pub struct TokenContract {
    pub balances: StorageMap<H160, U256>,
    pub approvals: StorageMap<(H160, H160), U256>,
    pub total_supply: U256,
}

#[contract_impl]
impl TokenContract {
    #[method]
    pub fn approve(&mut self, spender: H160, amount: U256) -> Result<bool> {
        let owner = Runtime::get_calling_script_hash();
        require!(Runtime::check_witness(&owner), ContractError::InvalidWitness);
        
        self.approvals.put(&(owner, spender), &amount);
        notify!("Approval", owner, spender, amount);
        Ok(true)
    }
    
    #[method]
    #[safe]
    pub fn allowance(&self, owner: H160, spender: H160) -> U256 {
        self.approvals.get(&(owner, spender)).unwrap_or(U256::zero())
    }
}
```

### Access Control

Implement proper access control using Neo N3's witness checking:

```rust
#[contract_impl]
impl MyContract {
    #[method]
    pub fn admin_only_function(&mut self, new_value: U256) -> Result<()> {
        require!(Runtime::check_witness(&self.owner), ContractError::Unauthorized);
        self.admin_value = new_value;
        Ok(())
    }
    
    #[method]
    pub fn multi_sig_function(&mut self) -> Result<()> {
        require!(
            Runtime::check_witness(&self.owner) && 
            Runtime::check_witness(&self.co_owner),
            ContractError::InsufficientSignatures
        );
        // Admin function logic
        Ok(())
    }
}
```

## Error Handling

### Error Definition

Define custom error types for your contract:

```rust
#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidAmount,
    InsufficientBalance,
    Unauthorized,
    AccountFrozen,
    TransferToSelf,
}

impl Into<ContractError> for TokenError {
    fn into(self) -> ContractError {
        match self {
            TokenError::InvalidAmount => ContractError::InvalidArgument,
            TokenError::InsufficientBalance => ContractError::InsufficientBalance,
            TokenError::Unauthorized => ContractError::Unauthorized,
            TokenError::AccountFrozen => ContractError::Forbidden,
            TokenError::TransferToSelf => ContractError::InvalidArgument,
        }
    }
}
```

### Using Validation Macros

```rust
#[method]
pub fn transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
    let from = Runtime::get_calling_script_hash();
    
    // Basic requirement
    require!(amount > U256::zero(), TokenError::InvalidAmount);
    
    // Authorization check
    require!(Runtime::check_witness(&from), TokenError::Unauthorized);
    
    // Balance check
    let balance = self.balance_of(from);
    require!(balance >= amount, TokenError::InsufficientBalance);
    
    // Prevent self-transfer
    require!(from != to, TokenError::TransferToSelf);
    
    // Execute transfer
    self._execute_transfer(from, to, amount)
}
```

## Events and Notifications

### Neo N3 Notifications

Use Neo N3's native notification system:

```rust
#[method]
pub fn transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
    let from = Runtime::get_calling_script_hash();
    
    // Transfer logic...
    
    // Emit Neo N3 notification
    notify!("Transfer", from, to, amount);
    
    // Optional: emit multiple related notifications
    notify!("BalanceChanged", from, self.balance_of(from));
    notify!("BalanceChanged", to, self.balance_of(to));
    
    Ok(true)
}
```

### Event Helpers

Create helper functions for complex events:

```rust
#[contract_impl]
impl MyContract {
    fn emit_transfer_event(&self, from: H160, to: H160, amount: U256) {
        let timestamp = Runtime::get_time();
        notify!("Transfer", from, to, amount, timestamp);
    }
    
    fn emit_approval_event(&self, owner: H160, spender: H160, amount: U256) {
        notify!("Approval", owner, spender, amount);
    }
}
```

## Examples

### Hello World Contract

```rust
#[contract_author("Hello World")]
#[contract_version("1.0.0")]
pub struct HelloWorld {
    pub greeting: ByteString,
    pub visitor_count: u64,
}

#[contract_impl]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            greeting: ByteString::from_literal("Hello, World!"),
            visitor_count: 0,
        }
    }
    
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        self.greeting.clone()
    }
    
    #[method]
    pub fn greet(&mut self, name: ByteString) -> ByteString {
        self.visitor_count += 1;
        
        let message = ByteString::from_literal("Hello, ") + name + ByteString::from_literal("!");
        
        notify!("Greeting", name, self.visitor_count);
        
        message
    }
    
    #[method]
    #[safe]
    pub fn get_visitor_count(&self) -> u64 {
        self.visitor_count
    }
}
```

### NEP-17 Token Contract

```rust
#[contract_author("NEP-17 Token")]
#[contract_version("1.0.0")]
#[contract_name("MyToken")]
#[contract_description("A NEP-17 compliant token")]
pub struct Nep17Token {
    pub balances: StorageMap<H160, U256>,
    pub total_supply: U256,
    pub decimals: u8,
    pub symbol: ByteString,
    pub owner: H160,
}

#[contract_impl]
impl Nep17Token {
    pub fn init(owner: H160, total_supply: U256, decimals: u8, symbol: ByteString) -> Self {
        let mut contract = Self {
            balances: StorageMap::new(),
            total_supply,
            decimals,
            symbol,
            owner,
        };
        
        // Give initial supply to owner
        contract.balances.put(&owner, &total_supply);
        notify!("Transfer", H160::zero(), owner, total_supply);
        
        contract
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        self.symbol.clone()
    }
    
    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals
    }
    
    #[method]
    #[safe]
    pub fn total_supply(&self) -> U256 {
        self.total_supply
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> U256 {
        self.balances.get(&account).unwrap_or(U256::zero())
    }
    
    #[method]
    pub fn transfer(&mut self, to: H160, amount: U256, data: Option<ByteString>) -> Result<bool> {
        let from = Runtime::get_calling_script_hash();
        require!(Runtime::check_witness(&from), ContractError::Unauthorized);
        
        self._transfer(from, to, amount, data)
    }
    
    fn _transfer(
        &mut self, 
        from: H160, 
        to: H160, 
        amount: U256, 
        data: Option<ByteString>
    ) -> Result<bool> {
        require!(amount > U256::zero(), ContractError::InvalidArgument);
        require!(from != to, ContractError::InvalidArgument);
        
        let from_balance = self.balance_of(from);
        require!(from_balance >= amount, ContractError::InsufficientBalance);
        
        self.balances.put(&from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(&to, &(to_balance + amount));
        
        notify!("Transfer", from, to, amount);
        
        // Call onNEP17Payment if recipient is a contract
        if Runtime::get_contract(&to).is_some() {
            if let Some(data) = data {
                Runtime::call_contract(&to, "onNEP17Payment", &[from.into_any(), amount.into_any(), data.into_any()]);
            } else {
                Runtime::call_contract(&to, "onNEP17Payment", &[from.into_any(), amount.into_any()]);
            }
        }
        
        Ok(true)
    }
    
    #[method]
    pub fn mint(&mut self, to: H160, amount: U256) -> Result<()> {
        require!(Runtime::check_witness(&self.owner), ContractError::Unauthorized);
        require!(amount > U256::zero(), ContractError::InvalidArgument);
        
        let balance = self.balance_of(to);
        self.balances.put(&to, &(balance + amount));
        self.total_supply += amount;
        
        notify!("Transfer", H160::zero(), to, amount);
        Ok(())
    }
}
```

## Migration Guide

### Converting from Traditional Syntax

#### Before (Traditional):
```rust
#[no_mangle]
pub extern "C" fn transfer(from: H160, to: H160, amount: U128) -> bool {
    let amount = U256::from(amount);
    
    if amount.is_zero() {
        panic!("Invalid amount");
    }
    
    let from_balance = storage::get(&from).unwrap_or(U256::zero());
    if from_balance < amount {
        return false;
    }
    
    storage::put(&from, from_balance - amount);
    storage::put(&to, storage::get(&to).unwrap_or(U256::zero()) + amount);
    
    true
}
```

#### After (Solana-Style with #[contract_impl]):
```rust
#[contract_impl]
impl TokenContract {
    #[method]
    pub fn transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
        let from = Runtime::get_calling_script_hash();
        
        require!(amount > U256::zero(), ContractError::InvalidArgument);
        require!(Runtime::check_witness(&from), ContractError::Unauthorized);
        
        let from_balance = self.balance_of(from);
        require!(from_balance >= amount, ContractError::InsufficientBalance);
        
        self.balances.put(&from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(&to, &(to_balance + amount));
        
        notify!("Transfer", from, to, amount);
        Ok(true)
    }
}
```

### Key Differences

1. **Entry Points**: Methods in `#[contract_impl]` instead of `#[no_mangle] extern "C"`
2. **State Management**: Structured contract state with storage helpers
3. **Error Handling**: `Result<T>` with custom errors instead of `bool` or panic
4. **Parameter Validation**: `require!` macros for clear validation
5. **Type Safety**: Direct Neo N3 types without manual conversion
6. **Events**: `notify!` macro for Neo N3 notifications

## Best Practices

1. **Always validate inputs** using `require!` macros
2. **Use checked arithmetic** with Neo N3's built-in safe math
3. **Define clear error types** for better debugging
4. **Emit notifications** for important state changes
5. **Implement proper access control** with witness checking
6. **Use storage helpers** for efficient state management
7. **Test thoroughly** with unit and integration tests
8. **Follow NEP standards** for interoperability

## Compiler Integration

The neo-wasm compiler automatically detects and processes Solana-style contracts:

1. **Automatic Detection**: Identifies `#[contract_impl]` implementations
2. **Manifest Generation**: Creates Neo N3 manifests from contract attributes
3. **Method Detection**: Identifies safe vs. state-changing methods
4. **Entry Point Mapping**: Converts contract methods to Neo N3 entry points

### Building Solana-Style Contracts

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Compile to Neo N3 bytecode (manifest auto-generated)
neo-wasm translate target/wasm32-unknown-unknown/release/my_contract.wasm

# Deploy to Neo N3 network
neo-cli contract deploy build/my_contract.nef build/my_contract.manifest.json
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let mut contract = MyToken::init(
            H160::from([1u8; 20]),
            U256::from(1000),
            8,
            ByteString::from_literal("TEST"),
        );
        
        let result = contract.transfer(H160::from([2u8; 20]), U256::from(100), None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validation() {
        let mut contract = MyToken::init(
            H160::from([1u8; 20]),
            U256::from(1000),
            8,
            ByteString::from_literal("TEST"),
        );
        
        let result = contract.transfer(H160::from([2u8; 20]), U256::zero(), None);
        assert!(result.is_err());
    }
}
```

### Integration Tests

See `neo-contract/tests/integration_tests.rs` for comprehensive test examples using the Solana-style patterns.

## Resources

- [Neo N3 Documentation](https://docs.neo.org/)
- [Example Contracts](../examples/)
- [Contract Attributes Guide](./contract-attributes.md)
- [Testing Guide](./testing-guide.md)

## Support

For questions and support:
- GitHub Issues: [neo-contract-rs](https://github.com/r3e-network/neo-contract-rs)
- Documentation: [Neo Developer Portal](https://developers.neo.org/)