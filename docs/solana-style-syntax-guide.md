# Solana-Style Syntax for Neo N3 Smart Contracts

This guide explains how to write Neo N3 smart contracts using Solana-style syntax, providing a familiar development experience for Solana developers while targeting the Neo N3 blockchain.

## Table of Contents
1. [Overview](#overview)
2. [Key Concepts](#key-concepts)
3. [Program Structure](#program-structure)
4. [Account Validation](#account-validation)
5. [Error Handling](#error-handling)
6. [Events](#events)
7. [Complete Examples](#complete-examples)
8. [Migration Guide](#migration-guide)

## Overview

The Solana-style syntax for Neo N3 brings the ergonomic and type-safe patterns from Solana's Anchor framework to Neo N3 smart contract development. This allows developers familiar with Solana to quickly start building on Neo N3 while maintaining the same mental model and development patterns.

### Benefits

- **Type Safety**: Compile-time validation of account structures and constraints
- **Explicit Account Declaration**: Clear definition of required accounts for each instruction
- **Automatic Validation**: Built-in security checks through account constraints
- **Familiar Patterns**: Solana developers can immediately start building on Neo N3
- **Better Security**: Explicit account requirements prevent common vulnerabilities

## Key Concepts

### Program Module

Instead of implementing traits directly on structs, Solana-style contracts use a `#[program]` module:

```rust
#[program]
pub mod my_contract {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Instruction logic
        Ok(())
    }
}
```

### Context Pattern

Every instruction handler receives a `Context<T>` parameter that provides access to validated accounts:

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let from = &mut ctx.accounts.from_account;
    let to = &mut ctx.accounts.to_account;
    // Transfer logic
    Ok(())
}
```

### Account Validation

Accounts are validated using the `#[derive(Accounts)]` macro with constraints:

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

## Program Structure

### Basic Program Declaration

```rust
use neo_contract::prelude::*;

// Declare the program ID (Neo N3 contract address)
declare_id!("NeoContractAddress123456789");

#[program]
pub mod my_program {
    use super::*;
    
    // Initialization function
    pub fn initialize(ctx: Context<Initialize>, config: Config) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.config = config;
        state.is_initialized = true;
        Ok(())
    }
    
    // Read-only function (marked with #[safe])
    #[safe]
    pub fn get_config(ctx: Context<GetConfig>) -> Result<Config> {
        Ok(ctx.accounts.state.config.clone())
    }
    
    // State-changing function
    pub fn update_config(ctx: Context<UpdateConfig>, new_config: Config) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let authority = &ctx.accounts.authority;
        
        require_keys_eq!(
            state.authority,
            authority.key(),
            MyError::Unauthorized
        );
        
        state.config = new_config;
        Ok(())
    }
}
```

## Account Validation

### Account Types

1. **Account<'info, T>**: Validated account containing data of type T
2. **Signer<'info>**: Account that must sign the transaction
3. **SystemAccount<'info>**: System-owned account
4. **Program<'info, T>**: Program account

### Common Constraints

```rust
#[derive(Accounts)]
pub struct Example<'info> {
    // Initialize a new account
    #[account(init, payer = user, space = 8 + 64)]
    pub new_account: Account<'info, MyData>,
    
    // Mutable account
    #[account(mut)]
    pub mutable_account: Account<'info, MyData>,
    
    // Account with relationship validation
    #[account(mut, has_one = owner)]
    pub owned_account: Account<'info, MyData>,
    
    // Program-derived address (PDA)
    #[account(
        seeds = [b"config", owner.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, Config>,
    
    // Signer who pays for account creation
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Required system program
    pub system_program: Program<'info, System>,
}
```

### Account Constraints Reference

| Constraint | Description | Example |
|------------|-------------|---------|
| `init` | Create new account | `#[account(init)]` |
| `mut` | Account must be mutable | `#[account(mut)]` |
| `has_one` | Validate relationship | `#[account(has_one = owner)]` |
| `seeds` | PDA validation | `#[account(seeds = [b"vault"])]` |
| `bump` | PDA bump seed | `#[account(bump)]` |
| `payer` | Who pays for creation | `#[account(payer = user)]` |
| `space` | Account size | `#[account(space = 8 + 64)]` |
| `init_if_needed` | Create if doesn't exist | `#[account(init_if_needed)]` |

## Error Handling

### Custom Error Types

Define custom errors using the `#[derive(ErrorCode)]` macro:

```rust
#[derive(ErrorCode)]
pub enum MyError {
    #[msg("The account is not authorized to perform this operation")]
    Unauthorized,
    
    #[msg("The provided amount is invalid")]
    InvalidAmount,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("The account has insufficient funds")]
    InsufficientFunds,
}
```

### Error Handling Macros

```rust
// Require a condition to be true
require!(amount > 0, MyError::InvalidAmount);

// Require equality
require_eq!(owner, expected_owner, MyError::Unauthorized);

// Require inequality
require_neq!(state, State::Locked, MyError::StateLocked);

// Require greater than
require_gt!(balance, amount, MyError::InsufficientFunds);

// Require greater than or equal
require_gte!(available, required, MyError::InsufficientResources);

// Require keys equality
require_keys_eq!(account.owner, signer.key(), MyError::Unauthorized);
```

## Events

### Event Declaration

```rust
#[event]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct StateUpdated {
    pub old_value: u64,
    pub new_value: u64,
    pub updater: Pubkey,
}
```

### Emitting Events

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    // ... transfer logic ...
    
    emit!(TokenTransfer {
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}
```

## Complete Examples

### Example 1: Simple Counter

```rust
#![no_std]
#![no_main]

use neo_contract::prelude::*;

declare_id!("NeoCounterContract123");

#[program]
pub mod counter {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.authority.key();
        Ok(())
    }
    
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter.count
            .checked_add(1)
            .ok_or(CounterError::Overflow)?;
        
        emit!(CounterUpdated {
            new_count: counter.count,
            updater: ctx.accounts.user.key(),
        });
        
        Ok(())
    }
    
    #[safe]
    pub fn get_count(ctx: Context<GetCount>) -> Result<u64> {
        Ok(ctx.accounts.counter.count)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCount<'info> {
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

#[derive(ErrorCode)]
pub enum CounterError {
    #[msg("Counter overflow")]
    Overflow,
}

#[event]
pub struct CounterUpdated {
    pub new_count: u64,
    pub updater: Pubkey,
}
```

### Example 2: Token with Multiple Features

```rust
#[program]
pub mod advanced_token {
    use super::*;
    
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        mint.name = name;
        mint.symbol = symbol;
        mint.decimals = decimals;
        mint.supply = 0;
        mint.mint_authority = ctx.accounts.mint_authority.key();
        mint.freeze_authority = Some(ctx.accounts.mint_authority.key());
        Ok(())
    }
    
    pub fn mint_to(
        ctx: Context<MintTo>,
        amount: u64,
    ) -> Result<()> {
        let mint = &mut ctx.accounts.mint;
        let to_account = &mut ctx.accounts.to_account;
        
        require_keys_eq!(
            mint.mint_authority,
            ctx.accounts.mint_authority.key(),
            TokenError::InvalidMintAuthority
        );
        
        mint.supply = mint.supply
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;
        
        to_account.amount = to_account.amount
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;
        
        emit!(TokenMinted {
            mint: ctx.accounts.mint.key(),
            to: ctx.accounts.to.key(),
            amount,
        });
        
        Ok(())
    }
    
    pub fn transfer(
        ctx: Context<Transfer>,
        amount: u64,
    ) -> Result<()> {
        let from_account = &mut ctx.accounts.from_account;
        let to_account = &mut ctx.accounts.to_account;
        
        require!(
            !from_account.is_frozen,
            TokenError::AccountFrozen
        );
        
        require_gte!(
            from_account.amount,
            amount,
            TokenError::InsufficientFunds
        );
        
        from_account.amount -= amount;
        to_account.amount += amount;
        
        emit!(TokenTransfer {
            from: ctx.accounts.from.key(),
            to: ctx.accounts.to.key(),
            amount,
        });
        
        Ok(())
    }
}
```

## Migration Guide

### From Traditional Neo N3 to Solana-Style

#### Before (Traditional Neo N3):
```rust
#[contract_impl]
impl MyContract {
    pub fn init() -> Self {
        Self { /* fields */ }
    }
    
    #[method]
    pub fn do_something(&self, param: u64) -> bool {
        // Logic
        true
    }
}
```

#### After (Solana-Style):
```rust
#[program]
pub mod my_contract {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialization logic
        Ok(())
    }
    
    pub fn do_something(ctx: Context<DoSomething>, param: u64) -> Result<()> {
        // Logic
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### Key Differences

1. **Module-based**: Use `#[program]` module instead of impl blocks
2. **Context Parameter**: All functions receive `Context<T>` as first parameter
3. **Explicit Accounts**: Define account requirements with `#[derive(Accounts)]`
4. **Result Types**: Return `Result<T>` instead of raw types
5. **Error Handling**: Use custom error enums with `#[derive(ErrorCode)]`
6. **Events**: Define events with `#[event]` and emit with `emit!` macro

## Best Practices

1. **Always Validate Inputs**: Use `require!` macros to validate parameters
2. **Check Arithmetic**: Use checked arithmetic operations to prevent overflows
3. **Explicit Authority Checks**: Always verify account ownership and permissions
4. **Comprehensive Error Messages**: Provide clear error messages for debugging
5. **Event Emission**: Emit events for all state changes for off-chain monitoring
6. **Account Size Calculation**: Accurately calculate account sizes to prevent issues
7. **PDA Usage**: Use PDAs for program-controlled accounts when appropriate

## Conclusion

The Solana-style syntax for Neo N3 provides a modern, type-safe approach to smart contract development. It combines the best practices from Solana's ecosystem with Neo N3's powerful features, enabling developers to write secure and maintainable smart contracts with familiar patterns.