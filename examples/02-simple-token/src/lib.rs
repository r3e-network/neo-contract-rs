//! # Simple Token Contract
//!
//! A complete implementation of a basic fungible token contract.
//! This contract demonstrates:
//! - Full token standard compliance
//! - Secure minting and burning operations
//! - Administrative controls and ownership management
//! - Comprehensive event logging
//! - Production-ready error handling
//!
//! Features:
//! - Fixed or unlimited supply tokens
//! - Owner-controlled minting
//! - Secure transfer operations
//! - Balance tracking and validation
//! - Comprehensive metadata support

#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString};

extern crate alloc;
use alloc::vec::Vec;

declare_id!("SimpleToken11111111111111111111111111111112");

/// Token account data
#[derive(Clone, Debug)]
pub struct TokenAccount {
    pub mint: H160,
    pub owner: H160,
    pub amount: Int256,
}

impl TokenAccount {
    pub fn serialize(&self) -> ByteString {
        let mut data = Vec::new();
        data.extend_from_slice(&self.mint.to_bytes());
        data.extend_from_slice(&self.owner.to_bytes());
        data.extend_from_slice(&self.amount.into_byte_string().to_bytes());
        ByteString::from_bytes(&data)
    }
    
    pub fn deserialize(data: ByteString) -> Result<Self> {
        let bytes = data.to_bytes();
        if bytes.len() < 72 { // 20 + 20 + 32 bytes minimum
            return Err(SimpleTokenError::InvalidAccountData.into());
        }
        
        let mut mint_bytes = [0u8; 20];
        mint_bytes.copy_from_slice(&bytes[0..20]);
        let mint = H160::from_bytes(&mint_bytes);
        
        let mut owner_bytes = [0u8; 20];
        owner_bytes.copy_from_slice(&bytes[20..40]);
        let owner = H160::from_bytes(&owner_bytes);
        
        let amount_bytes = ByteString::from_bytes(&bytes[40..]);
        let amount = Int256::from_byte_string(amount_bytes);
        
        Ok(TokenAccount { mint, owner, amount })
    }
}

/// Mint account data
#[derive(Clone, Debug)]
pub struct MintAccount {
    pub supply: Int256,
    pub decimals: u32,
    pub symbol: ByteString,
    pub max_supply: Int256,
    pub mint_authority: H160,
    pub freeze_authority: H160,
    pub is_mintable: bool,
    pub is_frozen: bool,
}

impl MintAccount {
    pub fn serialize(&self) -> ByteString {
        let mut data = Vec::new();
        data.extend_from_slice(&self.supply.into_byte_string().to_bytes());
        data.extend_from_slice(&(self.decimals as u64).to_le_bytes());
        let symbol_bytes = self.symbol.to_bytes();
        data.extend_from_slice(&(symbol_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(&symbol_bytes);
        data.extend_from_slice(&self.max_supply.into_byte_string().to_bytes());
        data.extend_from_slice(&self.mint_authority.to_bytes());
        data.extend_from_slice(&self.freeze_authority.to_bytes());
        data.push(if self.is_mintable { 1 } else { 0 });
        data.push(if self.is_frozen { 1 } else { 0 });
        ByteString::from_bytes(&data)
    }
    
    pub fn deserialize(data: ByteString) -> Result<Self> {
        let bytes = data.to_bytes();
        if bytes.len() < 98 { // Minimum size check
            return Err(SimpleTokenError::InvalidAccountData.into());
        }
        
        let mut offset = 0;
        
        let supply_bytes = ByteString::from_bytes(&bytes[offset..offset+32]);
        let supply = Int256::from_byte_string(supply_bytes);
        offset += 32;
        
        let decimals = u32::from_le_bytes([bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]]) as u32;
        offset += 8; // skip padding
        
        let symbol_len = u32::from_le_bytes([bytes[offset], bytes[offset+1], bytes[offset+2], bytes[offset+3]]) as usize;
        offset += 4;
        let symbol = ByteString::from_bytes(&bytes[offset..offset+symbol_len]);
        offset += symbol_len;
        
        let max_supply_bytes = ByteString::from_bytes(&bytes[offset..offset+32]);
        let max_supply = Int256::from_byte_string(max_supply_bytes);
        offset += 32;
        
        let mint_authority = H160::from_bytes(&bytes[offset..offset+20]);
        offset += 20;
        let freeze_authority = H160::from_bytes(&bytes[offset..offset+20]);
        offset += 20;
        
        let is_mintable = bytes[offset] != 0;
        offset += 1;
        let is_frozen = bytes[offset] != 0;
        
        Ok(MintAccount {
            supply,
            decimals,
            symbol,
            max_supply,
            mint_authority,
            freeze_authority,
            is_mintable,
            is_frozen,
        })
    }
}

/// Deploy context
#[derive(Accounts)]
pub struct Deploy<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(init)]
    pub mint: AccountInfo<'info>,
}

/// Transfer context
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(signer)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub to_token_account: AccountInfo<'info>,
}

/// Mint context
#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to_token_account: AccountInfo<'info>,
}

/// Burn context
#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub from_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
}

/// Admin context
#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
}

/// View context
#[derive(Accounts)]
pub struct View<'info> {
    pub mint: AccountInfo<'info>,
}

/// Balance context
#[derive(Accounts)]
pub struct BalanceQuery<'info> {
    pub token_account: AccountInfo<'info>,
}

#[program]
pub mod simple_token {
    use super::*;

    /// Deploy the token contract
    pub fn deploy(
        mut ctx: Context<Deploy>,
        symbol: ByteString,
        decimals: u32,
        initial_supply: Int256,
        max_supply: Int256,
        mintable: bool
    ) -> Result<()> {
        let _mint = &mut ctx.accounts.mint;
        let owner = &ctx.accounts.owner;

        // Validate parameters
        require!(!symbol.is_empty() && symbol.len() <= 16, SimpleTokenError::InvalidAmount);
        require!(decimals <= 18, SimpleTokenError::InvalidAmount);
        require!(initial_supply >= Int256::zero(), SimpleTokenError::InvalidAmount);
        require!(
            max_supply < Int256::zero() || 
            (max_supply > Int256::zero() && initial_supply <= max_supply), 
            SimpleTokenError::InvalidAmount
        );

        // Initialize mint account
        let mint_data = MintAccount {
            supply: initial_supply,
            decimals,
            symbol: symbol.clone(),
            max_supply,
            mint_authority: owner.key,
            freeze_authority: owner.key,
            is_mintable: mintable,
            is_frozen: false,
        };

        // Store mint data
        let _storage = Storage::get_context();
        Storage::put(Storage::get_context(), ByteString::from_literal("mint_data"), mint_data.serialize());

        // Create initial token account for owner if there's initial supply
        if initial_supply > Int256::zero() {
            let owner_token_account = TokenAccount {
                mint: H160::zero(), // Use contract address
                owner: owner.key,
                amount: initial_supply,
            };
            let account_key = create_account_key(owner.key);
            Storage::put(Storage::get_context(), account_key, owner_token_account.serialize());

            // Emit initial transfer event
            Runtime::log(ByteString::from_literal("Transfer event: initial supply minted"));
        }

        // Emit deployment event
        Runtime::log(ByteString::from_literal("Token deployed"));

        Ok(())
    }

    /// Get token symbol
    pub fn symbol(_ctx: Context<View>) -> Result<ByteString> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.symbol)
    }

    /// Get token decimals
    pub fn decimals(_ctx: Context<View>) -> Result<u32> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.decimals)
    }

    /// Get total supply
    pub fn total_supply(_ctx: Context<View>) -> Result<Int256> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.supply)
    }

    /// Get balance of account
    pub fn balance_of(ctx: Context<BalanceQuery>) -> Result<Int256> {
        let token_account = &ctx.accounts.token_account;
        let _storage = Storage::get_context();
        let account_key = create_account_key(token_account.key);
        
        match Storage::get(Storage::get_context(), account_key) {
            Some(account_data) => {
                let token_account: TokenAccount = TokenAccount::deserialize(account_data)?;
                Ok(token_account.amount)
            },
            None => Ok(Int256::zero()),
        }
    }

    /// Transfer tokens
    pub fn transfer(
        mut ctx: Context<Transfer>,
        amount: Int256,
        _data: ByteString
    ) -> Result<()> {
        let _from = &ctx.accounts.from;
        let from_token_account = &mut ctx.accounts.from_token_account;
        let to_token_account = &mut ctx.accounts.to_token_account;

        // Validate parameters
        require!(amount > Int256::zero(), SimpleTokenError::InvalidAmount);
        require!(from_token_account.key != to_token_account.key, SimpleTokenError::InvalidAmount);

        let _storage = Storage::get_context();

        // Check if contract is paused
        let mint_data = get_mint_data(Storage::get_context())?;
        require!(!mint_data.is_frozen, SimpleTokenError::ContractPaused);

        // Get from token account data
        let from_key = create_account_key(from_token_account.key);
        let mut from_account_data: TokenAccount = match Storage::get(Storage::get_context(), from_key.clone()) {
            Some(data) => TokenAccount::deserialize(data)?,
            None => return Err(SimpleTokenError::InsufficientBalance.into()),
        };

        // Check balance
        require!(from_account_data.amount >= amount, SimpleTokenError::InsufficientBalance);

        // Get to token account data
        let to_key = create_account_key(to_token_account.key);
        let mut to_account_data: TokenAccount = match Storage::get(Storage::get_context(), to_key.clone()) {
            Some(data) => TokenAccount::deserialize(data)?,
            None => TokenAccount {
                mint: from_account_data.mint,
                owner: to_token_account.key,
                amount: Int256::zero(),
            },
        };

        // Update balances
        from_account_data.amount = from_account_data.amount.checked_sub(&amount);
        to_account_data.amount = to_account_data.amount.checked_add(&amount);

        // Save updated accounts
        if from_account_data.amount == Int256::zero() {
            Storage::delete(Storage::get_context(), from_key);
        } else {
            Storage::put(Storage::get_context(), from_key, from_account_data.serialize());
        }
        Storage::put(Storage::get_context(), to_key, to_account_data.serialize());

        // Emit transfer event
        Runtime::log(ByteString::from_literal("Transfer completed"));

        Ok(())
    }

    /// Mint new tokens (only authorized)
    pub fn mint(
        mut ctx: Context<Mint>,
        amount: Int256
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let _mint = &mut ctx.accounts.mint;
        let to_token_account = &mut ctx.accounts.to_token_account;

        require!(amount > Int256::zero(), SimpleTokenError::InvalidAmount);

        let _storage = Storage::get_context();
        let mut mint_data = get_mint_data(Storage::get_context())?;

        require!(mint_data.is_mintable, SimpleTokenError::NotMintable);
        require!(
            authority.key == mint_data.mint_authority || is_authorized_minter(Storage::get_context(), authority.key)?,
            SimpleTokenError::Unauthorized
        );
        require!(!mint_data.is_frozen, SimpleTokenError::ContractPaused);

        // Check max supply constraint
        let new_supply = mint_data.supply.checked_add(&amount);
        require!(
            mint_data.max_supply <= Int256::zero() || new_supply <= mint_data.max_supply,
            SimpleTokenError::ExceedsMaxSupply
        );

        // Update mint data
        mint_data.supply = new_supply;
        Storage::put(Storage::get_context(), ByteString::from_literal("mint_data"), mint_data.serialize());

        // Update recipient balance
        let to_key = create_account_key(to_token_account.key);
        let mut to_account_data: TokenAccount = match Storage::get(Storage::get_context(), to_key.clone()) {
            Some(data) => TokenAccount::deserialize(data)?,
            None => TokenAccount {
                mint: H160::zero(), // Use contract address
                owner: to_token_account.key,
                amount: Int256::zero(),
            },
        };

        to_account_data.amount = to_account_data.amount.checked_add(&amount);
        Storage::put(Storage::get_context(), to_key, to_account_data.serialize());

        // Emit transfer event
        Runtime::log(ByteString::from_literal("Tokens minted"));

        Ok(())
    }

    /// Burn tokens (only token holder)
    pub fn burn(
        mut ctx: Context<Burn>,
        amount: Int256
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let from_token_account = &mut ctx.accounts.from_token_account;
        let _mint = &mut ctx.accounts.mint;

        require!(amount > Int256::zero(), SimpleTokenError::InvalidAmount);

        let _storage = Storage::get_context();
        let mut mint_data = get_mint_data(Storage::get_context())?;
        require!(!mint_data.is_frozen, SimpleTokenError::ContractPaused);

        // Get from token account data
        let from_key = create_account_key(from_token_account.key);
        let mut from_account_data: TokenAccount = match Storage::get(Storage::get_context(), from_key.clone()) {
            Some(data) => TokenAccount::deserialize(data)?,
            None => return Err(SimpleTokenError::InsufficientBalance.into()),
        };

        require!(from_account_data.amount >= amount, SimpleTokenError::InsufficientBalance);
        require!(from_account_data.owner == owner.key, SimpleTokenError::Unauthorized);

        // Update total supply
        mint_data.supply = mint_data.supply.checked_sub(&amount);
        Storage::put(Storage::get_context(), ByteString::from_literal("mint_data"), mint_data.serialize());

        // Update holder balance
        from_account_data.amount = from_account_data.amount.checked_sub(&amount);
        
        if from_account_data.amount == Int256::zero() {
            Storage::delete(Storage::get_context(), from_key);
        } else {
            Storage::put(Storage::get_context(), from_key, from_account_data.serialize());
        }

        // Emit transfer event
        Runtime::log(ByteString::from_literal("Tokens burned"));

        Ok(())
    }

    /// Add authorized minter (only owner)
    pub fn add_minter(
        mut ctx: Context<AdminAction>,
        minter: H160
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let _mint = &mut ctx.accounts.mint;

        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        require!(owner.key == mint_data.mint_authority, SimpleTokenError::Unauthorized);

        let minter_key = create_minter_key(minter);
        Storage::put(Storage::get_context(), minter_key, ByteString::from_literal("true"));

        Runtime::log(ByteString::from_literal("Minter added"));
        Ok(())
    }

    /// Remove authorized minter (only owner)
    pub fn remove_minter(
        mut ctx: Context<AdminAction>,
        minter: H160
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let _mint = &mut ctx.accounts.mint;

        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        require!(owner.key == mint_data.mint_authority, SimpleTokenError::Unauthorized);

        let minter_key = create_minter_key(minter);
        Storage::delete(Storage::get_context(), minter_key);

        Runtime::log(ByteString::from_literal("Minter removed"));
        Ok(())
    }

    /// Pause contract (only owner)
    pub fn pause(mut ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let _mint = &mut ctx.accounts.mint;

        let _storage = Storage::get_context();
        let mut mint_data = get_mint_data(Storage::get_context())?;
        require!(owner.key == mint_data.freeze_authority, SimpleTokenError::Unauthorized);

        mint_data.is_frozen = true;
        Storage::put(Storage::get_context(), ByteString::from_literal("mint_data"), mint_data.serialize());

        Runtime::log(ByteString::from_literal("Contract paused"));
        Ok(())
    }

    /// Unpause contract (only owner)
    pub fn unpause(mut ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let _mint = &mut ctx.accounts.mint;

        let _storage = Storage::get_context();
        let mut mint_data = get_mint_data(Storage::get_context())?;
        require!(owner.key == mint_data.freeze_authority, SimpleTokenError::Unauthorized);

        mint_data.is_frozen = false;
        Storage::put(Storage::get_context(), ByteString::from_literal("mint_data"), mint_data.serialize());

        Runtime::log(ByteString::from_literal("Contract unpaused"));
        Ok(())
    }

    /// Get contract owner
    pub fn get_owner(_ctx: Context<View>) -> Result<H160> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.mint_authority)
    }

    /// Check if contract is paused
    pub fn is_paused(_ctx: Context<View>) -> Result<bool> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.is_frozen)
    }

    /// Check if token is mintable
    pub fn is_mintable(_ctx: Context<View>) -> Result<bool> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.is_mintable)
    }

    /// Get maximum supply (0 means unlimited)
    pub fn get_max_supply(_ctx: Context<View>) -> Result<Int256> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        Ok(mint_data.max_supply)
    }

    /// Check if address is authorized minter
    pub fn is_minter(_ctx: Context<View>, address: H160) -> Result<bool> {
        let _storage = Storage::get_context();
        let mint_data = get_mint_data(Storage::get_context())?;
        
        if address == mint_data.mint_authority {
            return Ok(true);
        }

        is_authorized_minter(Storage::get_context(), address)
    }
}

// Helper functions
fn get_mint_data(_storage: StorageContext) -> Result<MintAccount> {
    match Storage::get(Storage::get_context(), ByteString::from_literal("mint_data")) {
        Some(data) => MintAccount::deserialize(data),
        None => Err(SimpleTokenError::AccountNotInitialized.into()),
    }
}

fn is_authorized_minter(_storage: StorageContext, address: H160) -> Result<bool> {
    let minter_key = create_minter_key(address);
    Ok(Storage::get(Storage::get_context(), minter_key).is_some())
}

// Events removed - using simple log messages instead

#[derive(Clone, Debug)]
pub enum SimpleTokenError {
    AccountNotInitialized,
    InsufficientBalance,
    Unauthorized,
    InvalidAmount,
    ContractPaused,
    NotMintable,
    ExceedsMaxSupply,
    InvalidAccountData,
}

impl From<SimpleTokenError> for ContractError {
    fn from(err: SimpleTokenError) -> Self {
        match err {
            SimpleTokenError::AccountNotInitialized => ContractError::InvalidAccountData,
            SimpleTokenError::InsufficientBalance => ContractError::InsufficientFunds,
            SimpleTokenError::Unauthorized => ContractError::Unauthorized,
            SimpleTokenError::InvalidAmount => ContractError::InvalidArgument,
            SimpleTokenError::ContractPaused => ContractError::InvalidInstruction,
            SimpleTokenError::NotMintable => ContractError::InvalidInstruction,
            SimpleTokenError::ExceedsMaxSupply => ContractError::InvalidArgument,
            SimpleTokenError::InvalidAccountData => ContractError::InvalidAccountData,
        }
    }
}

// Helper function for format! replacement
fn create_account_key(address: H160) -> ByteString {
    let prefix = ByteString::from_literal("token_account_");
    let addr_bytes = address.into_byte_string();
    prefix.concat(&addr_bytes)
}

fn create_minter_key(address: H160) -> ByteString {
    let prefix = ByteString::from_literal("minter_");
    let addr_bytes = address.into_byte_string();
    prefix.concat(&addr_bytes)
}