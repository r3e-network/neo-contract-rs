# Neo N3 Smart Contracts with Solana-Style Syntax

## Overview

This guide explains how to write Neo N3 smart contracts using Solana-style syntax with the neo-contract-rs framework. This approach brings the familiar and developer-friendly patterns from Solana's Anchor framework to Neo N3 blockchain development.

## Table of Contents

1. [Key Concepts](#key-concepts)
2. [Basic Structure](#basic-structure)
3. [Account Validation](#account-validation)
4. [Error Handling](#error-handling)
5. [Events](#events)
6. [Examples](#examples)
7. [Migration Guide](#migration-guide)

## Key Concepts

### Program Module

The main contract logic is defined within a `#[program]` module:

```rust
#[program]
pub mod my_contract {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Implementation
        Ok(())
    }
}
```

### Context Pattern

Every handler function receives a `Context<T>` parameter that provides type-safe access to accounts:

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
    let from = &mut ctx.accounts.from_account;
    let to = &mut ctx.accounts.to_account;
    // Transfer logic
    Ok(())
}
```

### Account Validation

Account constraints are defined using the `#[derive(Accounts)]` macro:

```rust
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = owner)]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}
```

## Basic Structure

### Contract Template

```rust
#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use neo_contract::prelude::*;

declare_id!("YourProgramId");

#[program]
pub mod your_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialization logic
        Ok(())
    }
    
    // Add more handler functions
}

// Account validation structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + StateAccount::SIZE)]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Account data structures
#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub data: u64,
}

impl StateAccount {
    pub const SIZE: usize = 32 + 8; // Pubkey + u64
}

// Error codes
#[error_code]
pub enum MyError {
    #[msg("Custom error message")]
    CustomError,
}

// Events
#[event]
pub struct MyEvent {
    pub user: Pubkey,
    pub value: u64,
}
```

## Account Validation

### Constraint Types

#### init
Creates a new account:
```rust
#[account(init, payer = authority, space = 8 + MyAccount::SIZE)]
pub my_account: Account<'info, MyAccount>,
```

#### mut
Marks account as mutable:
```rust
#[account(mut)]
pub my_account: Account<'info, MyAccount>,
```

#### has_one
Validates account ownership:
```rust
#[account(mut, has_one = owner)]
pub my_account: Account<'info, MyAccount>,
```

#### seeds & bump
Program Derived Addresses (PDAs):
```rust
#[account(
    init,
    seeds = [b"state", user.key().as_ref()],
    bump,
    payer = user,
    space = 8 + UserState::SIZE
)]
pub user_state: Account<'info, UserState>,
```

#### constraint
Custom validation:
```rust
#[account(
    mut,
    constraint = state.is_active @ MyError::InactiveState
)]
pub state: Account<'info, StateAccount>,
```

## Error Handling

### Error Definition

```rust
#[error_code]
pub enum TokenError {
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Account frozen")]
    AccountFrozen,
}
```

### Using Validation Macros

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
    let from = &mut ctx.accounts.from_account;
    
    // Basic requirement
    require!(amount > 0, TokenError::InvalidAmount);
    
    // Balance check
    require!(from.balance >= amount, TokenError::InsufficientBalance);
    
    // Equality check
    require_eq!(from.owner, ctx.accounts.signer.key(), TokenError::Unauthorized);
    
    // Greater than check
    require_gt!(from.balance, 0, TokenError::EmptyAccount);
    
    Ok(())
}
```

## Events

### Event Definition

```rust
#[event]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u128,
    pub timestamp: i64,
}
```

### Emitting Events

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
    // Transfer logic...
    
    emit!(TokenTransfer {
        from: ctx.accounts.from_account.owner,
        to: ctx.accounts.to_account.owner,
        amount,
        timestamp: Runtime::get_time(),
    });
    
    Ok(())
}
```

## Examples

### Hello World

```rust
#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.greeting = greeting;
        state.visitor_count = 0;
        
        msg!("Hello World contract initialized!");
        Ok(())
    }
    
    pub fn greet(ctx: Context<Greet>, name: String) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.visitor_count += 1;
        
        msg!("Hello, {}! You are visitor #{}", name, state.visitor_count);
        
        emit!(GreetingEvent {
            visitor: ctx.accounts.visitor.key(),
            name,
            count: state.visitor_count,
        });
        
        Ok(())
    }
}
```

### NEP-17 Token

```rust
#[program]
pub mod nep17_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u128,
    ) -> Result<()> {
        let metadata = &mut ctx.accounts.token_metadata;
        
        require!(total_supply > 0, TokenError::InvalidSupply);
        
        metadata.name = name;
        metadata.symbol = symbol;
        metadata.decimals = decimals;
        metadata.total_supply = total_supply;
        
        // Give initial supply to authority
        let holder = &mut ctx.accounts.initial_holder;
        holder.balance = total_supply;
        
        Ok(())
    }
    
    pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
        let from = &mut ctx.accounts.from_account;
        let to = &mut ctx.accounts.to_account;
        
        require!(amount > 0, TokenError::InvalidAmount);
        require!(from.balance >= amount, TokenError::InsufficientBalance);
        
        from.balance -= amount;
        to.balance += amount;
        
        emit!(TokenTransfer {
            from: from.owner,
            to: to.owner,
            amount,
        });
        
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
    let amount = amount.into();
    
    if amount == 0 {
        env::panic_str("Invalid amount");
    }
    
    let from_balance = storage::get(&from).unwrap_or(0);
    if from_balance < amount {
        return false;
    }
    
    storage::put(&from, from_balance - amount);
    storage::put(&to, storage::get(&to).unwrap_or(0) + amount);
    
    true
}
```

#### After (Solana-Style):
```rust
#[program]
pub mod token {
    use super::*;
    
    pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
        let from = &mut ctx.accounts.from_account;
        let to = &mut ctx.accounts.to_account;
        
        require!(amount > 0, TokenError::InvalidAmount);
        require!(from.balance >= amount, TokenError::InsufficientBalance);
        
        from.balance = from.balance
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
            
        to.balance = to.balance
            .checked_add(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenTransfer {
            from: from.owner,
            to: to.owner,
            amount,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = owner)]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}
```

### Key Differences

1. **Entry Points**: Functions in `#[program]` module instead of `#[no_mangle] extern "C"`
2. **Account Access**: Type-safe `Context<T>` instead of raw parameters
3. **Error Handling**: `Result<()>` with custom errors instead of `bool` or panic
4. **State Management**: Account structs with `#[account]` instead of raw storage
5. **Validation**: Declarative constraints instead of manual checks
6. **Events**: Structured events with `emit!` macro

## Best Practices

1. **Always validate inputs** using `require!` macros
2. **Use checked arithmetic** to prevent overflows
3. **Define clear error messages** for better debugging
4. **Emit events** for important state changes
5. **Use PDAs** for deterministic account addresses
6. **Implement proper access control** with signers and constraints
7. **Test thoroughly** with unit and integration tests

## Compiler Integration

The neo-wasm compiler has been enhanced to automatically detect and process Solana-style contracts:

1. **Automatic Detection**: Identifies `#[program]` modules and Solana patterns
2. **Manifest Generation**: Creates Neo N3 manifests from Solana-style code
3. **Read-only Methods**: Detects view functions from naming patterns
4. **Entry Point Mapping**: Converts Solana handlers to Neo N3 entry points

### Building Solana-Style Contracts

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Compile to Neo N3 bytecode (manifest auto-generated if needed)
neo-wasm translate target/wasm32-unknown-unknown/release/my_contract.wasm

# Or with explicit manifest
neo-wasm translate target/wasm32-unknown-unknown/release/my_contract.wasm \
    --manifest my_contract.manifest.json
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        // Test transfer logic
    }
    
    #[test]
    fn test_validation() {
        // Test constraint validation
    }
}
```

### Integration Tests

See `neo-contract/tests/solana_contracts_tests.rs` for comprehensive test examples.

## Resources

- [Neo N3 Documentation](https://docs.neo.org/)
- [Solana Anchor Framework](https://www.anchor-lang.com/)
- [Example Contracts](../examples/)
- [Migration Scripts](../scripts/migrate/)

## Support

For questions and support:
- GitHub Issues: [neo-contract-rs](https://github.com/r3e-network/neo-contract-rs)
- Documentation: [Neo Developer Portal](https://developers.neo.org/)