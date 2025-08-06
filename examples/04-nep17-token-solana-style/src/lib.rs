#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::builtin::IntoAny;
use neo_contract::serialize::{Serialize, Deserialize};

declare_id!("NeoNEP17TokenContract123456789");

#[program]
pub mod nep17_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: ByteString,
        symbol: ByteString,
        decimals: u8,
        total_supply: Int256,
    ) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let mint_authority = &ctx.accounts.mint_authority;
        
        require!(
            !token_metadata.is_initialized,
            TokenError::AlreadyInitialized
        );
        
        require_gt!(
            total_supply,
            Int256::zero(),
            TokenError::InvalidSupply
        );
        
        token_metadata.name = name;
        token_metadata.symbol = symbol;
        token_metadata.decimals = decimals;
        token_metadata.total_supply = total_supply;
        token_metadata.mint_authority = mint_authority.key();
        token_metadata.freeze_authority = Some(mint_authority.key());
        token_metadata.is_initialized = true;
        
        let owner_balance = &mut ctx.accounts.owner_balance;
        owner_balance.owner = mint_authority.key();
        owner_balance.amount = total_supply;
        
        emit!(TokenInitialized {
            name: token_metadata.name.clone(),
            symbol: token_metadata.symbol.clone(),
            decimals,
            total_supply,
            mint_authority: mint_authority.key(),
        });
        
        Ok(())
    }
    
    #[safe]
    pub fn symbol(ctx: Context<GetMetadata>) -> Result<ByteString> {
        let token_metadata = &ctx.accounts.token_metadata;
        Ok(token_metadata.symbol.clone())
    }
    
    #[safe]
    pub fn decimals(ctx: Context<GetMetadata>) -> Result<u8> {
        let token_metadata = &ctx.accounts.token_metadata;
        Ok(token_metadata.decimals)
    }
    
    #[safe]
    pub fn total_supply(ctx: Context<GetMetadata>) -> Result<Int256> {
        let token_metadata = &ctx.accounts.token_metadata;
        Ok(token_metadata.total_supply)
    }
    
    #[safe]
    pub fn balance_of(ctx: Context<BalanceOf>) -> Result<Int256> {
        let balance_account = &ctx.accounts.balance_account;
        Ok(balance_account.amount)
    }
    
    pub fn transfer(
        ctx: Context<Transfer>,
        amount: Int256,
    ) -> Result<()> {
        let from_balance = &mut ctx.accounts.from_balance;
        let to_balance = &mut ctx.accounts.to_balance;
        let from = &ctx.accounts.from;
        
        require_keys_eq!(
            from_balance.owner,
            from.key(),
            TokenError::Unauthorized
        );
        
        require_gt!(
            amount,
            Int256::zero(),
            TokenError::InvalidAmount
        );
        
        require_gte!(
            from_balance.amount,
            amount,
            TokenError::InsufficientBalance
        );
        
        from_balance.amount = from_balance.amount
            .checked_sub(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        to_balance.amount = to_balance.amount
            .checked_add(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenTransfer {
            from: from.key(),
            to: to_balance.owner,
            amount,
        });
        
        Ok(())
    }
    
    pub fn approve(
        ctx: Context<Approve>,
        spender: Pubkey,
        amount: Int256,
    ) -> Result<()> {
        let allowance = &mut ctx.accounts.allowance;
        let owner = &ctx.accounts.owner;
        
        allowance.owner = owner.key();
        allowance.spender = spender;
        allowance.amount = amount;
        
        emit!(TokenApproval {
            owner: owner.key(),
            spender,
            amount,
        });
        
        Ok(())
    }
    
    pub fn transfer_from(
        ctx: Context<TransferFrom>,
        amount: Int256,
    ) -> Result<()> {
        let allowance = &mut ctx.accounts.allowance;
        let from_balance = &mut ctx.accounts.from_balance;
        let to_balance = &mut ctx.accounts.to_balance;
        let spender = &ctx.accounts.spender;
        
        require_keys_eq!(
            allowance.spender,
            spender.key(),
            TokenError::Unauthorized
        );
        
        require_gt!(
            amount,
            Int256::zero(),
            TokenError::InvalidAmount
        );
        
        require_gte!(
            allowance.amount,
            amount,
            TokenError::InsufficientAllowance
        );
        
        require_gte!(
            from_balance.amount,
            amount,
            TokenError::InsufficientBalance
        );
        
        allowance.amount = allowance.amount
            .checked_sub(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        from_balance.amount = from_balance.amount
            .checked_sub(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        to_balance.amount = to_balance.amount
            .checked_add(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenTransfer {
            from: from_balance.owner,
            to: to_balance.owner,
            amount,
        });
        
        Ok(())
    }
    
    pub fn mint(
        ctx: Context<Mint>,
        amount: Int256,
    ) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let mint_authority = &ctx.accounts.mint_authority;
        let to_balance = &mut ctx.accounts.to_balance;
        
        require_keys_eq!(
            token_metadata.mint_authority,
            mint_authority.key(),
            TokenError::Unauthorized
        );
        
        require_gt!(
            amount,
            Int256::zero(),
            TokenError::InvalidAmount
        );
        
        token_metadata.total_supply = token_metadata.total_supply
            .checked_add(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        to_balance.amount = to_balance.amount
            .checked_add(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenMinted {
            to: to_balance.owner,
            amount,
            new_supply: token_metadata.total_supply,
        });
        
        Ok(())
    }
    
    pub fn burn(
        ctx: Context<Burn>,
        amount: Int256,
    ) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        let from_balance = &mut ctx.accounts.from_balance;
        let from = &ctx.accounts.from;
        
        require_keys_eq!(
            from_balance.owner,
            from.key(),
            TokenError::Unauthorized
        );
        
        require_gt!(
            amount,
            Int256::zero(),
            TokenError::InvalidAmount
        );
        
        require_gte!(
            from_balance.amount,
            amount,
            TokenError::InsufficientBalance
        );
        
        from_balance.amount = from_balance.amount
            .checked_sub(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        token_metadata.total_supply = token_metadata.total_supply
            .checked_sub(&amount)
            .ok_or(TokenError::ArithmeticError)?;
        
        emit!(TokenBurned {
            from: from.key(),
            amount,
            new_supply: token_metadata.total_supply,
        });
        
        Ok(())
    }
    
    pub fn freeze_account(
        ctx: Context<FreezeAccount>,
    ) -> Result<()> {
        let token_metadata = &ctx.accounts.token_metadata;
        let freeze_authority = &ctx.accounts.freeze_authority;
        let target_balance = &mut ctx.accounts.target_balance;
        
        require!(
            token_metadata.freeze_authority.is_some(),
            TokenError::NoFreezeAuthority
        );
        
        require_keys_eq!(
            token_metadata.freeze_authority.unwrap(),
            freeze_authority.key(),
            TokenError::Unauthorized
        );
        
        target_balance.is_frozen = true;
        
        emit!(AccountFrozen {
            account: target_balance.owner,
            authority: freeze_authority.key(),
        });
        
        Ok(())
    }
    
    pub fn thaw_account(
        ctx: Context<ThawAccount>,
    ) -> Result<()> {
        let token_metadata = &ctx.accounts.token_metadata;
        let freeze_authority = &ctx.accounts.freeze_authority;
        let target_balance = &mut ctx.accounts.target_balance;
        
        require!(
            token_metadata.freeze_authority.is_some(),
            TokenError::NoFreezeAuthority
        );
        
        require_keys_eq!(
            token_metadata.freeze_authority.unwrap(),
            freeze_authority.key(),
            TokenError::Unauthorized
        );
        
        target_balance.is_frozen = false;
        
        emit!(AccountThawed {
            account: target_balance.owner,
            authority: freeze_authority.key(),
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = mint_authority, space = 8 + TokenMetadata::SIZE)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(init, payer = mint_authority, space = 8 + BalanceAccount::SIZE)]
    pub owner_balance: Account<'info, BalanceAccount>,
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
    pub balance_account: Account<'info, BalanceAccount>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = owner @ TokenError::Unauthorized)]
    pub from_balance: Account<'info, BalanceAccount>,
    #[account(mut)]
    pub to_balance: Account<'info, BalanceAccount>,
    pub from: Signer<'info>,
}

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(init_if_needed, payer = owner, space = 8 + AllowanceAccount::SIZE)]
    pub allowance: Account<'info, AllowanceAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferFrom<'info> {
    #[account(mut)]
    pub allowance: Account<'info, AllowanceAccount>,
    #[account(mut)]
    pub from_balance: Account<'info, BalanceAccount>,
    #[account(mut)]
    pub to_balance: Account<'info, BalanceAccount>,
    pub spender: Signer<'info>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub to_balance: Account<'info, BalanceAccount>,
    pub mint_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(mut)]
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut, has_one = owner @ TokenError::Unauthorized)]
    pub from_balance: Account<'info, BalanceAccount>,
    pub from: Signer<'info>,
}

#[derive(Accounts)]
pub struct FreezeAccount<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub target_balance: Account<'info, BalanceAccount>,
    pub freeze_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ThawAccount<'info> {
    pub token_metadata: Account<'info, TokenMetadata>,
    #[account(mut)]
    pub target_balance: Account<'info, BalanceAccount>,
    pub freeze_authority: Signer<'info>,
}

#[account]
pub struct TokenMetadata {
    pub name: ByteString,
    pub symbol: ByteString,
    pub decimals: u8,
    pub total_supply: Int256,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
    pub is_initialized: bool,
}

impl TokenMetadata {
    pub const SIZE: usize = 100 + 10 + 1 + 32 + 32 + 33 + 1;
}

#[account]
pub struct BalanceAccount {
    pub owner: Pubkey,
    pub amount: Int256,
    pub is_frozen: bool,
}

impl BalanceAccount {
    pub const SIZE: usize = 32 + 32 + 1;
}

#[account]
pub struct AllowanceAccount {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub amount: Int256,
}

impl AllowanceAccount {
    pub const SIZE: usize = 32 + 32 + 32;
}

#[error_code]
pub enum TokenError {
    #[msg("Token already initialized")]
    AlreadyInitialized,
    #[msg("Invalid supply amount")]
    InvalidSupply,
    #[msg("Invalid transfer amount")]
    InvalidAmount,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Insufficient allowance")]
    InsufficientAllowance,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Arithmetic error")]
    ArithmeticError,
    #[msg("No freeze authority set")]
    NoFreezeAuthority,
    #[msg("Account is frozen")]
    AccountFrozen,
}

#[event]
pub struct TokenInitialized {
    pub name: ByteString,
    pub symbol: ByteString,
    pub decimals: u8,
    pub total_supply: Int256,
    pub mint_authority: Pubkey,
}

#[event]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: Int256,
}

#[event]
pub struct TokenApproval {
    pub owner: Pubkey,
    pub spender: Pubkey,
    pub amount: Int256,
}

#[event]
pub struct TokenMinted {
    pub to: Pubkey,
    pub amount: Int256,
    pub new_supply: Int256,
}

#[event]
pub struct TokenBurned {
    pub from: Pubkey,
    pub amount: Int256,
    pub new_supply: Int256,
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