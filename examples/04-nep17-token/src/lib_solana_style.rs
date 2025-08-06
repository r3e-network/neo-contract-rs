//! # NEP-17 Token Contract - Solana Style
//!
//! Full implementation of NEP-17 token standard using Solana-style syntax

#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use neo_contract::prelude::*;

declare_id!("NeoNEP17TokenProgram");

#[program]
pub mod nep17_token {
    use super::*;

    /// Initialize the token with metadata and initial supply
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u128,
    ) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let mint_authority = &ctx.accounts.mint_authority;
        let initial_holder = &mut ctx.accounts.initial_holder;
        
        require!(!token_metadata.is_initialized, TokenError::AlreadyInitialized);
        require!(total_supply > 0, TokenError::InvalidSupply);
        require!(decimals <= 18, TokenError::InvalidDecimals);
        
        // Set token metadata
        token_metadata.name = name.clone();
        token_metadata.symbol = symbol.clone();
        token_metadata.decimals = decimals;
        token_metadata.total_supply = total_supply;
        token_metadata.mint_authority = mint_authority.key();
        token_metadata.freeze_authority = Some(mint_authority.key());
        token_metadata.is_paused = false;
        token_metadata.is_initialized = true;
        
        // Give initial supply to mint authority
        initial_holder.owner = mint_authority.key();
        initial_holder.balance = total_supply;
        initial_holder.is_frozen = false;
        
        emit!(TokenInitialized {
            name,
            symbol,
            decimals,
            total_supply,
            mint_authority: mint_authority.key(),
        });
        
        msg!("Token {} initialized with supply {}", symbol, total_supply);
        Ok(())
    }
    
    /// Get token symbol
    pub fn symbol(ctx: Context<GetMetadata>) -> Result<String> {
        Ok(ctx.accounts.token_metadata.symbol.clone())
    }
    
    /// Get token decimals
    pub fn decimals(ctx: Context<GetMetadata>) -> Result<u8> {
        Ok(ctx.accounts.token_metadata.decimals)
    }
    
    /// Get total supply
    pub fn total_supply(ctx: Context<GetMetadata>) -> Result<u128> {
        Ok(ctx.accounts.token_metadata.total_supply)
    }
    
    /// Get balance of an account
    pub fn balance_of(ctx: Context<BalanceOf>) -> Result<u128> {
        Ok(ctx.accounts.token_account.balance)
    }
    
    /// Transfer tokens between accounts
    pub fn transfer(ctx: Context<Transfer>, amount: u128) -> Result<()> {
        let from_account = &mut ctx.accounts.from_account;
        let to_account = &mut ctx.accounts.to_account;
        let token_metadata = &ctx.accounts.token_metadata;
        
        // Check if token is paused
        require!(!token_metadata.is_paused, TokenError::TokenPaused);
        
        // Check if accounts are frozen
        require!(!from_account.is_frozen, TokenError::AccountFrozen);
        require!(!to_account.is_frozen, TokenError::AccountFrozen);
        
        // Validate amount
        require!(amount > 0, TokenError::InvalidAmount);
        require!(from_account.balance >= amount, TokenError::InsufficientBalance);
        
        // Perform transfer
        from_account.balance = from_account.balance
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
            
        to_account.balance = to_account.balance
            .checked_add(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenTransfer {
            from: from_account.owner,
            to: to_account.owner,
            amount,
        });
        
        msg!("Transferred {} tokens", amount);
        Ok(())
    }
    
    /// Approve spending allowance
    pub fn approve(ctx: Context<Approve>, amount: u128) -> Result<()> {
        let allowance = &mut ctx.accounts.allowance;
        let owner = &ctx.accounts.owner;
        let spender = &ctx.accounts.spender;
        
        allowance.owner = owner.key();
        allowance.spender = spender.key();
        allowance.amount = amount;
        
        emit!(TokenApproval {
            owner: owner.key(),
            spender: spender.key(),
            amount,
        });
        
        Ok(())
    }
    
    /// Get allowance
    pub fn allowance(ctx: Context<GetAllowance>) -> Result<u128> {
        Ok(ctx.accounts.allowance.amount)
    }
    
    /// Transfer from approved allowance
    pub fn transfer_from(ctx: Context<TransferFrom>, amount: u128) -> Result<()> {
        let allowance = &mut ctx.accounts.allowance;
        let from_account = &mut ctx.accounts.from_account;
        let to_account = &mut ctx.accounts.to_account;
        let token_metadata = &ctx.accounts.token_metadata;
        
        // Check if token is paused
        require!(!token_metadata.is_paused, TokenError::TokenPaused);
        
        // Check if accounts are frozen
        require!(!from_account.is_frozen, TokenError::AccountFrozen);
        require!(!to_account.is_frozen, TokenError::AccountFrozen);
        
        // Validate allowance and amount
        require!(amount > 0, TokenError::InvalidAmount);
        require!(allowance.amount >= amount, TokenError::InsufficientAllowance);
        require!(from_account.balance >= amount, TokenError::InsufficientBalance);
        
        // Update allowance
        allowance.amount = allowance.amount
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        // Perform transfer
        from_account.balance = from_account.balance
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
            
        to_account.balance = to_account.balance
            .checked_add(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenTransfer {
            from: from_account.owner,
            to: to_account.owner,
            amount,
        });
        
        Ok(())
    }
    
    /// Mint new tokens (authority only)
    pub fn mint(ctx: Context<Mint>, amount: u128) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let to_account = &mut ctx.accounts.to_account;
        
        require!(amount > 0, TokenError::InvalidAmount);
        require!(!token_metadata.is_paused, TokenError::TokenPaused);
        require!(!to_account.is_frozen, TokenError::AccountFrozen);
        
        // Update total supply
        token_metadata.total_supply = token_metadata.total_supply
            .checked_add(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        // Add to recipient balance
        to_account.balance = to_account.balance
            .checked_add(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenMinted {
            to: to_account.owner,
            amount,
            new_supply: token_metadata.total_supply,
        });
        
        Ok(())
    }
    
    /// Burn tokens
    pub fn burn(ctx: Context<Burn>, amount: u128) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let from_account = &mut ctx.accounts.from_account;
        
        require!(amount > 0, TokenError::InvalidAmount);
        require!(from_account.balance >= amount, TokenError::InsufficientBalance);
        require!(!token_metadata.is_paused, TokenError::TokenPaused);
        require!(!from_account.is_frozen, TokenError::AccountFrozen);
        
        // Update balances
        from_account.balance = from_account.balance
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        token_metadata.total_supply = token_metadata.total_supply
            .checked_sub(amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenBurned {
            from: from_account.owner,
            amount,
            new_supply: token_metadata.total_supply,
        });
        
        Ok(())
    }
    
    /// Freeze an account
    pub fn freeze_account(ctx: Context<FreezeAccount>) -> Result<()> {
        let target_account = &mut ctx.accounts.target_account;
        let token_metadata = &ctx.accounts.token_metadata;
        
        require!(
            token_metadata.freeze_authority.is_some(),
            TokenError::NoFreezeAuthority
        );
        
        target_account.is_frozen = true;
        
        emit!(AccountFrozen {
            account: target_account.owner,
            authority: ctx.accounts.freeze_authority.key(),
        });
        
        Ok(())
    }
    
    /// Thaw a frozen account
    pub fn thaw_account(ctx: Context<ThawAccount>) -> Result<()> {
        let target_account = &mut ctx.accounts.target_account;
        let token_metadata = &ctx.accounts.token_metadata;
        
        require!(
            token_metadata.freeze_authority.is_some(),
            TokenError::NoFreezeAuthority
        );
        
        target_account.is_frozen = false;
        
        emit!(AccountThawed {
            account: target_account.owner,
            authority: ctx.accounts.freeze_authority.key(),
        });
        
        Ok(())
    }
    
    /// Pause all token operations
    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        
        token_metadata.is_paused = true;
        
        emit!(TokenPaused {
            authority: ctx.accounts.mint_authority.key(),
        });
        
        Ok(())
    }
    
    /// Resume token operations
    pub fn unpause(ctx: Context<Unpause>) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        
        token_metadata.is_paused = false;
        
        emit!(TokenUnpaused {
            authority: ctx.accounts.mint_authority.key(),
        });
        
        Ok(())
    }
}

// ===== Account Validation Structures =====

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = mint_authority, space = 8 + TokenMetadata::SIZE)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(init, payer = mint_authority, space = 8 + TokenAccount::SIZE)]
    pub initial_holder: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetMetadata<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
}

#[derive(Accounts)]
pub struct BalanceOf<'info> {
    pub token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut, has_one = owner)]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(init_if_needed, payer = owner, space = 8 + AllowanceAccount::SIZE)]
    pub allowance: Account<'info, AllowanceAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: Spender can be any valid account
    pub spender: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetAllowance<'info> {
    pub allowance: Account<'info, AllowanceAccount>,
}

#[derive(Accounts)]
pub struct TransferFrom<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut, has_one = owner, has_one = spender)]
    pub allowance: Account<'info, AllowanceAccount>,
    #[account(mut)]
    pub from_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub spender: Signer<'info>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut, has_one = mint_authority)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut, has_one = owner)]
    pub from_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FreezeAccount<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub target_account: Account<'info, TokenAccount>,
    #[account(constraint = token_metadata.freeze_authority == Some(freeze_authority.key()))]
    pub freeze_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ThawAccount<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub target_account: Account<'info, TokenAccount>,
    #[account(constraint = token_metadata.freeze_authority == Some(freeze_authority.key()))]
    pub freeze_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut, has_one = mint_authority)]
    pub token_metadata: Account<'info, TokenMetadata>,
    pub mint_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Unpause<'info> {
    #[account(mut, has_one = mint_authority)]
    pub token_metadata: Account<'info, TokenMetadata>,
    pub mint_authority: Signer<'info>,
}

// ===== Account Data Structures =====

#[account]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
    pub is_paused: bool,
    pub is_initialized: bool,
}

impl TokenMetadata {
    pub const SIZE: usize = 4 + 100 + // name
        4 + 10 + // symbol
        1 + // decimals
        16 + // total_supply
        32 + // mint_authority
        1 + 32 + // freeze_authority
        1 + // is_paused
        1; // is_initialized
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub balance: u128,
    pub is_frozen: bool,
}

impl TokenAccount {
    pub const SIZE: usize = 32 + 16 + 1;
}

#[account]
pub struct AllowanceAccount {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub amount: u128,
}

impl AllowanceAccount {
    pub const SIZE: usize = 32 + 32 + 16;
}

// ===== Error Codes =====

#[error_code]
pub enum TokenError {
    #[msg("Token already initialized")]
    AlreadyInitialized,
    
    #[msg("Invalid supply amount")]
    InvalidSupply,
    
    #[msg("Invalid decimals (max 18)")]
    InvalidDecimals,
    
    #[msg("Invalid transfer amount")]
    InvalidAmount,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Insufficient allowance")]
    InsufficientAllowance,
    
    #[msg("Arithmetic error")]
    ArithmeticError,
    
    #[msg("No freeze authority set")]
    NoFreezeAuthority,
    
    #[msg("Account is frozen")]
    AccountFrozen,
    
    #[msg("Token operations are paused")]
    TokenPaused,
    
    #[msg("Unauthorized operation")]
    Unauthorized,
}

// ===== Events =====

#[event]
pub struct TokenInitialized {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub mint_authority: Pubkey,
}

#[event]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u128,
}

#[event]
pub struct TokenApproval {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub amount: u128,
}

#[event]
pub struct TokenMinted {
    pub to: Pubkey,
    pub amount: u128,
    pub new_supply: u128,
}

#[event]
pub struct TokenBurned {
    pub from: Pubkey,
    pub amount: u128,
    pub new_supply: u128,
}

#[event]
pub struct AccountFrozen {
    pub account: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct AccountThawed {
    pub account: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct TokenPaused {
    pub authority: Pubkey,
}

#[event]
pub struct TokenUnpaused {
    pub authority: Pubkey,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_metadata_size() {
        assert!(TokenMetadata::SIZE >= 195);
    }

    #[test]
    fn test_token_account_size() {
        assert!(TokenAccount::SIZE >= 49);
    }
    
    #[test]
    fn test_allowance_account_size() {
        assert!(AllowanceAccount::SIZE >= 80);
    }
}