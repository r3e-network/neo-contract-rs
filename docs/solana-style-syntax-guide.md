# Solana-Style Syntax for Neo N3 Smart Contracts

This guide explains how to write Neo N3 smart contracts using Solana-style syntax, providing a familiar development experience for Solana developers while targeting the Neo N3 blockchain, using `#[contract_impl]` as the primary pattern.

## Table of Contents
1. [Overview](#overview)
2. [Key Concepts](#key-concepts)
3. [Contract Structure](#contract-structure)
4. [Method Implementation](#method-implementation)
5. [Error Handling](#error-handling)
6. [Events and Notifications](#events-and-notifications)
7. [Complete Examples](#complete-examples)
8. [Migration Guide](#migration-guide)

## Overview

The Solana-style syntax for Neo N3 brings ergonomic and type-safe patterns inspired by Solana's development model to Neo N3 smart contract development. This allows developers familiar with Solana concepts to quickly start building on Neo N3 while maintaining familiar mental models and development patterns.

### Benefits

- **Type Safety**: Compile-time validation of contract structures and method signatures
- **Explicit State Management**: Clear definition of contract state and storage patterns
- **Automatic Validation**: Built-in security checks through parameter validation
- **Familiar Patterns**: Solana-style development experience adapted for Neo N3
- **Better Security**: Explicit parameter requirements prevent common vulnerabilities
- **Neo N3 Native**: Direct integration with Neo N3 blockchain features

## Key Concepts

### Contract Implementation Pattern

Instead of using program modules, Solana-style Neo N3 contracts use struct-based contracts with `#[contract_impl]`:

```rust
#[contract_author("My Contract")]
#[contract_version("1.0.0")]
pub struct MyContract {
    owner: H160,
    value: U256,
}

#[contract_impl]
impl MyContract {
    pub fn init() -> Self {
        Self {
            owner: Runtime::get_calling_script_hash(),
            value: U256::zero(),
        }
    }
    
    #[method]
    pub fn set_value(&mut self, new_value: U256) -> Result<()> {
        require!(Runtime::check_witness(&self.owner), ContractError::Unauthorized);
        self.value = new_value;
        notify!("ValueChanged", self.value);
        Ok(())
    }
}
```

### Direct Parameter Pattern

Unlike traditional Context patterns, Solana-style methods use direct parameters with proper Neo N3 types:

```rust
#[method]
pub fn transfer(&mut self, from: H160, to: H160, amount: U256) -> Result<bool> {
    require!(amount > U256::zero(), ContractError::InvalidAmount);
    require!(Runtime::check_witness(&from), ContractError::Unauthorized);
    
    // Direct parameter access with Neo N3 native types
    let from_balance = self.balance_of(from);
    require!(from_balance >= amount, ContractError::InsufficientBalance);
    
    self.update_balance(from, from_balance - amount);
    let to_balance = self.balance_of(to);
    self.update_balance(to, to_balance + amount);
    
    notify!("Transfer", from, to, amount);
    Ok(true)
}
```

## Contract Structure

### Basic Template

```rust
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

use neo_contract::prelude::*;

#[contract_author("Your Name")]
#[contract_version("1.0.0")]
#[contract_name("MyContract")]
#[contract_description("A Solana-style Neo N3 contract")]
pub struct MyContract {
    // Contract state fields
    pub owner: H160,
    pub initialized: bool,
    pub data: StorageMap<H160, U256>,
}

#[contract_impl]
impl MyContract {
    // Constructor - called once during deployment
    pub fn init() -> Self {
        Self {
            owner: Runtime::get_calling_script_hash(),
            initialized: false,
            data: StorageMap::new(),
        }
    }
    
    // Public methods (entry points)
    #[method]
    pub fn initialize(&mut self, initial_data: U256) -> Result<()> {
        require!(!self.initialized, ContractError::AlreadyInitialized);
        require!(Runtime::check_witness(&self.owner), ContractError::Unauthorized);
        
        self.initialized = true;
        self.data.put(&self.owner, &initial_data);
        
        notify!("Initialized", self.owner, initial_data);
        Ok(())
    }
    
    // Read-only methods
    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        self.owner
    }
    
    #[method]
    #[safe]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    #[method]
    #[safe]
    pub fn get_data(&self, key: H160) -> U256 {
        self.data.get(&key).unwrap_or(U256::zero())
    }
    
    // Private helper methods
    fn validate_caller(&self) -> Result<()> {
        require!(Runtime::check_witness(&self.owner), ContractError::Unauthorized);
        Ok(())
    }
}
```

### Contract Attributes

Use attributes to provide contract metadata:

```rust
#[contract_author("Developer Name")]        // Contract author
#[contract_version("1.0.0")]               // Semantic version
#[contract_name("TokenContract")]           // Contract name
#[contract_description("A NEP-17 token")]  // Description
#[contract_email("dev@example.com")]        // Contact email (optional)
#[contract_website("https://example.com")]  // Website (optional)
pub struct TokenContract {
    // Contract fields
}
```

## Method Implementation

### Method Types

#### State-Changing Methods
```rust
#[method]
pub fn update_state(&mut self, value: U256) -> Result<()> {
    // Can modify contract state
    self.internal_value = value;
    notify!("StateUpdated", value);
    Ok(())
}
```

#### Read-Only Methods
```rust
#[method]
#[safe]
pub fn get_value(&self) -> U256 {
    // Cannot modify state, marked as safe
    self.internal_value
}
```

#### Private Helper Methods
```rust
fn internal_helper(&self, param: U256) -> U256 {
    // Private methods don't need #[method] attribute
    param * 2
}
```

### Parameter Validation

Use validation macros for secure parameter checking:

```rust
#[method]
pub fn secure_transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
    // Basic validation
    require!(amount > U256::zero(), ContractError::InvalidAmount);
    
    // Authorization validation
    let caller = Runtime::get_calling_script_hash();
    require!(Runtime::check_witness(&caller), ContractError::Unauthorized);
    
    // Business logic validation
    let balance = self.get_balance(caller);
    require!(balance >= amount, ContractError::InsufficientBalance);
    
    // Equality check
    require_eq!(self.get_status(), Status::Active, ContractError::ContractPaused);
    
    // Greater than check
    require_gt!(amount, U256::from(100), ContractError::AmountTooSmall);
    
    // Execute transfer
    self.execute_transfer(caller, to, amount)
}
```

## Error Handling

### Custom Error Types

Define domain-specific errors for better debugging:

```rust
#[derive(Debug, PartialEq)]
pub enum TokenError {
    InvalidAmount,
    InsufficientBalance,
    Unauthorized,
    TokenPaused,
    TransferToSelf,
    ExceedsAllowance,
}

impl Into<ContractError> for TokenError {
    fn into(self) -> ContractError {
        match self {
            TokenError::InvalidAmount => ContractError::InvalidArgument,
            TokenError::InsufficientBalance => ContractError::InsufficientBalance,
            TokenError::Unauthorized => ContractError::Unauthorized,
            TokenError::TokenPaused => ContractError::Forbidden,
            TokenError::TransferToSelf => ContractError::InvalidArgument,
            TokenError::ExceedsAllowance => ContractError::Forbidden,
        }
    }
}
```

### Error Handling Patterns

```rust
#[method]
pub fn complex_operation(&mut self, param: U256) -> Result<U256> {
    // Validate input
    require!(param > U256::zero(), TokenError::InvalidAmount);
    
    // Check state
    require!(self.is_active, TokenError::TokenPaused);
    
    // Perform operation that might fail
    let result = self.risky_calculation(param)
        .map_err(|_| TokenError::InvalidAmount)?;
    
    // Update state
    self.last_result = result;
    
    notify!("OperationComplete", param, result);
    Ok(result)
}
```

## Events and Notifications

### Notification Patterns

Use Neo N3's native notification system:

```rust
#[method]
pub fn transfer_with_events(&mut self, to: H160, amount: U256) -> Result<bool> {
    let from = Runtime::get_calling_script_hash();
    
    // Pre-transfer notification
    notify!("TransferStarted", from, to, amount);
    
    // Perform transfer
    let success = self.execute_transfer(from, to, amount)?;
    
    if success {
        // Success notification with detailed info
        notify!("Transfer", from, to, amount);
        notify!("BalanceChanged", from, self.balance_of(from));
        notify!("BalanceChanged", to, self.balance_of(to));
        
        // Timestamp for analytics
        let timestamp = Runtime::get_time();
        notify!("TransferComplete", from, to, amount, timestamp);
    }
    
    Ok(success)
}
```

### Event Helper Functions

Create reusable event emitters:

```rust
#[contract_impl]
impl MyContract {
    fn emit_transfer(&self, from: H160, to: H160, amount: U256) {
        notify!("Transfer", from, to, amount);
    }
    
    fn emit_approval(&self, owner: H160, spender: H160, amount: U256) {
        notify!("Approval", owner, spender, amount);
    }
    
    fn emit_state_change(&self, field: ByteString, old_value: U256, new_value: U256) {
        notify!("StateChanged", field, old_value, new_value);
    }
}
```

## Complete Examples

### Hello World Contract

```rust
#![no_std]
#![no_main]

use neo_contract::prelude::*;

#[contract_author("Hello World Example")]
#[contract_version("1.0.0")]
pub struct HelloWorld {
    pub greeting: ByteString,
    pub visitor_count: u64,
    pub visitors: StorageMap<H160, ByteString>,
}

#[contract_impl]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            greeting: ByteString::from_literal("Hello, World!"),
            visitor_count: 0,
            visitors: StorageMap::new(),
        }
    }
    
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        self.greeting.clone()
    }
    
    #[method]
    pub fn greet(&mut self, name: ByteString) -> ByteString {
        let caller = Runtime::get_calling_script_hash();
        
        // Update visitor info
        self.visitors.put(&caller, &name);
        self.visitor_count += 1;
        
        // Create personalized greeting
        let message = ByteString::from_literal("Hello, ") + name + 
                     ByteString::from_literal("! You are visitor #") +
                     ByteString::from(&self.visitor_count.to_string());
        
        // Emit event
        notify!("Greeting", caller, name, self.visitor_count);
        
        message
    }
    
    #[method]
    #[safe]
    pub fn get_visitor_count(&self) -> u64 {
        self.visitor_count
    }
    
    #[method]
    #[safe]
    pub fn get_visitor_name(&self, visitor: H160) -> Option<ByteString> {
        self.visitors.get(&visitor)
    }
}
```

### NEP-17 Token Contract

```rust
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;

use neo_contract::prelude::*;

#[contract_author("NEP-17 Token")]
#[contract_version("1.0.0")]
#[contract_name("MyToken")]
#[contract_description("A full-featured NEP-17 token")]
pub struct MyToken {
    pub balances: StorageMap<H160, U256>,
    pub allowances: StorageMap<(H160, H160), U256>,
    pub total_supply: U256,
    pub decimals: u8,
    pub symbol: ByteString,
    pub name: ByteString,
    pub owner: H160,
}

#[contract_impl]
impl MyToken {
    pub fn init(
        name: ByteString,
        symbol: ByteString,
        decimals: u8,
        total_supply: U256,
    ) -> Self {
        let owner = Runtime::get_calling_script_hash();
        
        let mut contract = Self {
            balances: StorageMap::new(),
            allowances: StorageMap::new(),
            total_supply,
            decimals,
            symbol,
            name,
            owner,
        };
        
        // Give initial supply to owner
        contract.balances.put(&owner, &total_supply);
        notify!("Transfer", H160::zero(), owner, total_supply);
        
        contract
    }
    
    // NEP-17 Standard Methods
    
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
        
        self.do_transfer(from, to, amount, data)
    }
    
    // Extended Functionality
    
    #[method]
    pub fn approve(&mut self, spender: H160, amount: U256) -> Result<bool> {
        let owner = Runtime::get_calling_script_hash();
        require!(Runtime::check_witness(&owner), ContractError::Unauthorized);
        
        self.allowances.put(&(owner, spender), &amount);
        notify!("Approval", owner, spender, amount);
        
        Ok(true)
    }
    
    #[method]
    #[safe]
    pub fn allowance(&self, owner: H160, spender: H160) -> U256 {
        self.allowances.get(&(owner, spender)).unwrap_or(U256::zero())
    }
    
    #[method]
    pub fn transfer_from(
        &mut self, 
        owner: H160, 
        to: H160, 
        amount: U256,
        data: Option<ByteString>
    ) -> Result<bool> {
        let spender = Runtime::get_calling_script_hash();
        require!(Runtime::check_witness(&spender), ContractError::Unauthorized);
        
        let allowance = self.allowance(owner, spender);
        require!(allowance >= amount, ContractError::InsufficientBalance);
        
        // Update allowance
        self.allowances.put(&(owner, spender), &(allowance - amount));
        
        // Execute transfer
        self.do_transfer(owner, to, amount, data)
    }
    
    // Administrative Functions
    
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
    
    #[method]
    pub fn burn(&mut self, amount: U256) -> Result<()> {
        let caller = Runtime::get_calling_script_hash();
        require!(Runtime::check_witness(&caller), ContractError::Unauthorized);
        
        let balance = self.balance_of(caller);
        require!(balance >= amount, ContractError::InsufficientBalance);
        
        self.balances.put(&caller, &(balance - amount));
        self.total_supply -= amount;
        
        notify!("Transfer", caller, H160::zero(), amount);
        Ok(())
    }
    
    // Private Helper Methods
    
    fn do_transfer(
        &mut self,
        from: H160,
        to: H160,
        amount: U256,
        data: Option<ByteString>,
    ) -> Result<bool> {
        require!(amount > U256::zero(), ContractError::InvalidArgument);
        require!(from != to, ContractError::InvalidArgument);
        
        let from_balance = self.balance_of(from);
        require!(from_balance >= amount, ContractError::InsufficientBalance);
        
        // Update balances
        self.balances.put(&from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(&to, &(to_balance + amount));
        
        // Emit transfer event
        notify!("Transfer", from, to, amount);
        
        // Call onNEP17Payment if recipient is a contract
        if Runtime::get_contract(&to).is_some() {
            let mut args = vec![from.into_any(), amount.into_any()];
            if let Some(data) = data {
                args.push(data.into_any());
            }
            
            Runtime::call_contract(&to, "onNEP17Payment", &args);
        }
        
        Ok(true)
    }
}
```

## Migration Guide

### From Traditional Neo N3 Contracts

#### Before (Traditional):
```rust
#[no_mangle]
pub extern "C" fn transfer(from: H160, to: H160, amount: U128) -> bool {
    let amount = U256::from(amount);
    
    if !Runtime::check_witness(&from) {
        return false;
    }
    
    if amount.is_zero() {
        return false;
    }
    
    let from_balance = storage::get(&from).unwrap_or(U256::zero());
    if from_balance < amount {
        return false;
    }
    
    storage::put(&from, from_balance - amount);
    storage::put(&to, storage::get(&to).unwrap_or(U256::zero()) + amount);
    
    Runtime::notify("Transfer", &[from.into(), to.into(), amount.into()]);
    true
}
```

#### After (Solana-Style):
```rust
#[contract_impl]
impl MyToken {
    #[method]
    pub fn transfer(&mut self, to: H160, amount: U256) -> Result<bool> {
        let from = Runtime::get_calling_script_hash();
        
        require!(Runtime::check_witness(&from), ContractError::Unauthorized);
        require!(amount > U256::zero(), ContractError::InvalidAmount);
        
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

### Migration Steps

1. **Convert Entry Points**: Replace `#[no_mangle] extern "C"` functions with `#[method]` methods
2. **Add Contract Structure**: Define contract state using a struct
3. **Use Result Types**: Return `Result<T>` instead of primitive types or panic
4. **Add Validation**: Use `require!` macros for parameter validation
5. **Structured Storage**: Use `StorageMap` and other storage helpers
6. **Proper Error Handling**: Define custom error types and proper error propagation

### Key Differences

1. **Entry Points**: Methods in `#[contract_impl]` instead of `#[no_mangle] extern "C"`
2. **State Management**: Structured contract state with typed storage
3. **Error Handling**: `Result<T>` with descriptive errors instead of boolean returns
4. **Parameter Validation**: Declarative validation with `require!` macros
5. **Type Safety**: Compile-time validation of method signatures
6. **Storage Access**: Type-safe storage operations through contract fields

## Best Practices

1. **Always validate inputs** using `require!` macros
2. **Use descriptive error types** for better debugging
3. **Implement proper access control** with witness checking
4. **Emit events** for important state changes
5. **Use storage helpers** for efficient data management
6. **Test thoroughly** with unit and integration tests
7. **Follow NEP standards** for interoperability
8. **Document your contract** with clear comments

## Resources

- [Complete Examples](../examples/)
- [Neo N3 Documentation](https://docs.neo.org/)
- [Contract Testing Guide](./testing-guide.md)
- [NEP Standards](https://github.com/neo-project/proposals)