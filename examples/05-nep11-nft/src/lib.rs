//! # NEP-11 Non-Fungible Token Contract
//!
//! A complete implementation of the NEP-11 non-fungible token standard.
//! This contract demonstrates:
//! - Full NEP-11 compliance with all required methods
//! - Unique token creation and management
//! - Token metadata and properties system
//! - Secure transfer mechanics
//! - Enumeration capabilities
//! - Administrative controls
//! - Event emission for all operations
//!
//! This is a production-ready NFT contract suitable for digital collectibles,
//! art, gaming assets, and other unique digital items.

#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

declare_id!("Nep11Token11111111111111111111111111111112");

/// NFT account data
#[derive(Clone, Debug)]
pub struct NftAccount {
    pub mint: H160,
    pub owner: H160,
    pub token_id: ByteString,
    pub properties: Map<ByteString, Any>,
}

/// Collection account data
#[derive(Clone, Debug)]
pub struct CollectionAccount {
    pub symbol: ByteString,
    pub total_supply: Int256,
    pub mint_authority: H160,
    pub freeze_authority: H160,
    pub base_uri: ByteString,
    pub is_frozen: bool,
}

/// Owner account data (tracks NFTs owned)
#[derive(Clone, Debug)]
pub struct OwnerAccount {
    pub owner: H160,
    pub balance: Int256,
    pub token_ids: Array<ByteString>,
}

/// Deploy context
#[derive(Accounts)]
pub struct Deploy<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(init)]
    pub collection: AccountInfo<'info>,
}

/// Transfer context
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(signer)]
    pub owner_or_approved: AccountInfo<'info>,
    #[account(mut)]
    pub nft_account: AccountInfo<'info>,
    #[account(mut)]
    pub from_owner_account: AccountInfo<'info>,
    #[account(mut)]
    pub to_owner_account: AccountInfo<'info>,
}

/// Mint context
#[derive(Accounts)]
pub struct Mint<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub collection: AccountInfo<'info>,
    #[account(init)]
    pub nft_account: AccountInfo<'info>,
    #[account(mut)]
    pub owner_account: AccountInfo<'info>,
}

/// Burn context
#[derive(Accounts)]
pub struct Burn<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub nft_account: AccountInfo<'info>,
    #[account(mut)]
    pub owner_account: AccountInfo<'info>,
    #[account(mut)]
    pub collection: AccountInfo<'info>,
}

/// Approve context
#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub nft_account: AccountInfo<'info>,
}

/// Admin context
#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    pub collection: AccountInfo<'info>,
}

/// View context
#[derive(Accounts)]
pub struct View<'info> {
    pub account: AccountInfo<'info>,
}

#[program]
pub mod nep11_token {
    use super::*;

    /// Deploy the NFT contract with initial parameters
    pub fn deploy(
        ctx: Context<Deploy>,
        symbol: ByteString,
        base_uri: ByteString
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        let owner = &ctx.accounts.owner;

        // Validate parameters
        require!(!symbol.is_empty() && symbol.len() <= 16, "Invalid symbol: must be 1-16 characters");

        // Initialize collection account
        let collection_data = CollectionAccount {
            symbol: symbol.clone(),
            total_supply: Int256::zero(),
            mint_authority: owner.key(),
            freeze_authority: owner.key(),
            base_uri,
            is_frozen: false,
        };

        // Store collection data
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        // Emit deployment event
        emit!(ContractDeployed { symbol });
        Ok(())
    }

    /// Get token symbol (NEP-11 required)
    pub fn symbol(ctx: Context<View>) -> Result<ByteString> {
        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        Ok(collection_data.symbol)
    }

    /// Get number of decimals (always 0 for NFTs) (NEP-11 required)
    pub fn decimals(_ctx: Context<View>) -> Result<u32> {
        Ok(0) // NFTs are non-divisible
    }

    /// Get total token supply (NEP-11 required)
    pub fn total_supply(ctx: Context<View>) -> Result<Int256> {
        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        Ok(collection_data.total_supply)
    }

    /// Get balance of an account (number of tokens owned) (NEP-11 required)
    pub fn balance_of(ctx: Context<View>, owner: H160) -> Result<Int256> {
        let storage = Storage::get_context();
        let owner_key = format!("owner_account_{}", owner);
        
        match Storage::get(storage, ByteString::from_str(&owner_key)) {
            Some(owner_data) => {
                let owner_account: OwnerAccount = OwnerAccount::deserialize(owner_data)?;
                Ok(owner_account.balance)
            },
            None => Ok(Int256::zero()),
        }
    }

    /// Get owner of a specific token (NEP-11 required)
    pub fn owner_of(ctx: Context<View>, token_id: ByteString) -> Result<H160> {
        let storage = Storage::get_context();
        let nft_key = format!("nft_{}", token_id);
        
        match Storage::get(storage, ByteString::from_str(&nft_key)) {
            Some(nft_data) => {
                let nft_account: NftAccount = NftAccount::deserialize(nft_data)?;
                Ok(nft_account.owner)
            },
            None => Ok(H160::zero()),
        }
    }

    /// Get tokens owned by an account (NEP-11 required)
    pub fn tokens_of(ctx: Context<View>, owner: H160) -> Result<Array<ByteString>> {
        let storage = Storage::get_context();
        let owner_key = format!("owner_account_{}", owner);
        
        match Storage::get(storage, ByteString::from_str(&owner_key)) {
            Some(owner_data) => {
                let owner_account: OwnerAccount = OwnerAccount::deserialize(owner_data)?;
                Ok(owner_account.token_ids)
            },
            None => Ok(Array::new()),
        }
    }

    /// Transfer a token (NEP-11 required)
    pub fn transfer(
        ctx: Context<Transfer>,
        to: H160,
        token_id: ByteString,
        _data: Vec<u8>
    ) -> Result<()> {
        let owner_or_approved = &ctx.accounts.owner_or_approved;
        let nft_account = &mut ctx.accounts.nft_account;
        let from_owner_account = &mut ctx.accounts.from_owner_account;
        let to_owner_account = &mut ctx.accounts.to_owner_account;

        let storage = Storage::get_context();

        // Check if contract is paused
        let collection_data = get_collection_data(&storage)?;
        require!(!collection_data.is_frozen, "Contract is paused");

        // Get NFT data
        let nft_key = format!("nft_{}", token_id);
        let mut nft_data: NftAccount = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(data) => NftAccount::deserialize(data)?,
            None => return Err(ErrorCode::TokenNotFound.into()),
        };

        let from = nft_data.owner;

        // Check authorization (owner or approved)
        require!(
            owner_or_approved.key() == from || is_approved_for_token(&storage, &token_id, owner_or_approved.key())?,
            "Unauthorized: Not owner or approved"
        );

        // Update NFT owner
        nft_data.owner = to;
        Storage::put(storage.clone(), ByteString::from_str(&nft_key), nft_data.serialize());

        // Clear approval
        let approval_key = format!("approval_{}", token_id);
        Storage::delete(storage.clone(), ByteString::from_str(&approval_key));

        // Update from owner account
        let from_key = format!("owner_account_{}", from);
        let mut from_account_data: OwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&from_key)) {
            Some(data) => OwnerAccount::deserialize(data)?,
            None => return Err(ErrorCode::OwnerAccountNotFound.into()),
        };

        from_account_data.balance = from_account_data.balance.checked_sub(&Int256::one());
        remove_token_from_list(&mut from_account_data.token_ids, &token_id);

        if from_account_data.balance == Int256::zero() {
            Storage::delete(storage.clone(), ByteString::from_str(&from_key));
        } else {
            Storage::put(storage.clone(), ByteString::from_str(&from_key), from_account_data.serialize());
        }

        // Update to owner account
        let to_key = format!("owner_account_{}", to);
        let mut to_account_data: OwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&to_key)) {
            Some(data) => OwnerAccount::deserialize(data)?,
            None => OwnerAccount {
                owner: to,
                balance: Int256::zero(),
                token_ids: Array::new(),
            },
        };

        to_account_data.balance = to_account_data.balance.checked_add(&Int256::one());
        to_account_data.token_ids.push(token_id.clone());
        Storage::put(storage, ByteString::from_str(&to_key), to_account_data.serialize());

        // Emit Transfer event
        emit!(TransferEvent {
            from,
            to,
            amount: Int256::one(),
            token_id: token_id.clone(),
        });

        Ok(())
    }

    /// Get properties of a token (NEP-11 optional)
    pub fn properties(ctx: Context<View>, token_id: ByteString) -> Result<Map<ByteString, Any>> {
        let storage = Storage::get_context();
        let nft_key = format!("nft_{}", token_id);
        
        match Storage::get(storage, ByteString::from_str(&nft_key)) {
            Some(nft_data) => {
                let nft_account: NftAccount = NftAccount::deserialize(nft_data)?;
                Ok(nft_account.properties)
            },
            None => Ok(Map::new()),
        }
    }

    /// Approve another address to transfer a specific token
    pub fn approve(
        ctx: Context<Approve>,
        to: H160,
        token_id: ByteString
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let nft_account = &ctx.accounts.nft_account;

        let storage = Storage::get_context();

        // Verify token exists and get owner
        let nft_key = format!("nft_{}", token_id);
        let nft_data: NftAccount = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(data) => NftAccount::deserialize(data)?,
            None => return Err(ErrorCode::TokenNotFound.into()),
        };

        require!(nft_data.owner == owner.key(), "Unauthorized: Not token owner");

        // Set or clear approval
        let approval_key = format!("approval_{}", token_id);
        if to == H160::zero() {
            Storage::delete(storage, ByteString::from_str(&approval_key));
        } else {
            Storage::put(storage, ByteString::from_str(&approval_key), to.into_byte_string());
        }

        // Emit Approval event
        emit!(ApprovalEvent {
            owner: nft_data.owner,
            approved: to,
            token_id,
        });

        Ok(())
    }

    /// Get approved address for a token
    pub fn get_approved(ctx: Context<View>, token_id: ByteString) -> Result<H160> {
        let storage = Storage::get_context();
        let approval_key = format!("approval_{}", token_id);
        
        match Storage::get(storage, ByteString::from_str(&approval_key)) {
            Some(approved_bytes) => Ok(H160::from_byte_string(approved_bytes)),
            None => Ok(H160::zero()),
        }
    }

    /// Mint a new token (authorized minter only)
    pub fn mint(
        ctx: Context<Mint>,
        to: H160,
        token_id: ByteString,
        properties: Map<ByteString, Any>
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let collection = &mut ctx.accounts.collection;
        let nft_account = &mut ctx.accounts.nft_account;
        let owner_account = &mut ctx.accounts.owner_account;

        require!(!token_id.is_empty() && token_id.len() <= 64, "Invalid token ID: must be 1-64 characters");

        let storage = Storage::get_context();
        let mut collection_data = get_collection_data(&storage)?;

        require!(
            authority.key() == collection_data.mint_authority || is_authorized_minter(&storage, authority.key())?,
            "Unauthorized: Not authorized to mint"
        );

        // Check if token already exists
        let nft_key = format!("nft_{}", token_id);
        require!(
            Storage::get(storage.clone(), ByteString::from_str(&nft_key)).is_none(),
            "Token already exists"
        );

        // Create NFT account
        let nft_data = NftAccount {
            mint: collection.key(),
            owner: to,
            token_id: token_id.clone(),
            properties,
        };
        Storage::put(storage.clone(), ByteString::from_str(&nft_key), nft_data.serialize());

        // Update owner account
        let owner_key = format!("owner_account_{}", to);
        let mut owner_data: OwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&owner_key)) {
            Some(data) => OwnerAccount::deserialize(data)?,
            None => OwnerAccount {
                owner: to,
                balance: Int256::zero(),
                token_ids: Array::new(),
            },
        };

        owner_data.balance = owner_data.balance.checked_add(&Int256::one());
        owner_data.token_ids.push(token_id.clone());
        Storage::put(storage.clone(), ByteString::from_str(&owner_key), owner_data.serialize());

        // Update collection
        collection_data.total_supply = collection_data.total_supply.checked_add(&Int256::one());
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        // Emit Transfer event (from null address)
        emit!(TransferEvent {
            from: H160::zero(),
            to,
            amount: Int256::one(),
            token_id: token_id.clone(),
        });

        emit!(TokenMinted { token_id });
        Ok(())
    }

    /// Burn a token (token owner only)
    pub fn burn(
        ctx: Context<Burn>,
        token_id: ByteString
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let nft_account = &mut ctx.accounts.nft_account;
        let owner_account = &mut ctx.accounts.owner_account;
        let collection = &mut ctx.accounts.collection;

        let storage = Storage::get_context();

        // Get NFT data
        let nft_key = format!("nft_{}", token_id);
        let nft_data: NftAccount = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(data) => NftAccount::deserialize(data)?,
            None => return Err(ErrorCode::TokenNotFound.into()),
        };

        require!(nft_data.owner == owner.key(), "Unauthorized: Not token owner");

        // Remove NFT account
        Storage::delete(storage.clone(), ByteString::from_str(&nft_key));

        // Remove approval if exists
        let approval_key = format!("approval_{}", token_id);
        Storage::delete(storage.clone(), ByteString::from_str(&approval_key));

        // Update owner account
        let owner_key = format!("owner_account_{}", nft_data.owner);
        let mut owner_data: OwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&owner_key)) {
            Some(data) => OwnerAccount::deserialize(data)?,
            None => return Err(ErrorCode::OwnerAccountNotFound.into()),
        };

        owner_data.balance = owner_data.balance.checked_sub(&Int256::one());
        remove_token_from_list(&mut owner_data.token_ids, &token_id);

        if owner_data.balance == Int256::zero() {
            Storage::delete(storage.clone(), ByteString::from_str(&owner_key));
        } else {
            Storage::put(storage.clone(), ByteString::from_str(&owner_key), owner_data.serialize());
        }

        // Update collection
        let mut collection_data = get_collection_data(&storage)?;
        collection_data.total_supply = collection_data.total_supply.checked_sub(&Int256::one());
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        // Emit Transfer event (to null address)
        emit!(TransferEvent {
            from: nft_data.owner,
            to: H160::zero(),
            amount: Int256::one(),
            token_id: token_id.clone(),
        });

        emit!(TokenBurned { token_id });
        Ok(())
    }

    /// Get contract owner
    pub fn get_owner(ctx: Context<View>) -> Result<H160> {
        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        Ok(collection_data.mint_authority)
    }

    /// Add authorized minter (owner only)
    pub fn add_minter(
        ctx: Context<AdminAction>,
        minter: H160
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        require!(owner.key() == collection_data.mint_authority, "Unauthorized: Only owner can add minters");

        let minter_key = format!("minter_{}", minter);
        Storage::put(storage, ByteString::from_str(&minter_key), ByteString::from_literal("true"));

        emit!(MinterAdded { minter });
        Ok(())
    }

    /// Set base URI for token metadata (owner only)
    pub fn set_base_uri(
        ctx: Context<AdminAction>,
        base_uri: ByteString
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let mut collection_data = get_collection_data(&storage)?;
        require!(owner.key() == collection_data.mint_authority, "Unauthorized: Only owner can set base URI");

        collection_data.base_uri = base_uri.clone();
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        emit!(BaseURISet { base_uri });
        Ok(())
    }

    /// Get base URI
    pub fn get_base_uri(ctx: Context<View>) -> Result<ByteString> {
        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        Ok(collection_data.base_uri)
    }

    /// Get token URI (base_uri + token_id)
    pub fn token_uri(ctx: Context<View>, token_id: ByteString) -> Result<ByteString> {
        let storage = Storage::get_context();

        // Check if token exists
        let nft_key = format!("nft_{}", token_id);
        require!(
            Storage::get(storage.clone(), ByteString::from_str(&nft_key)).is_some(),
            "Token does not exist"
        );

        let collection_data = get_collection_data(&storage)?;
        if collection_data.base_uri.is_empty() {
            Ok(token_id)
        } else {
            Ok(collection_data.base_uri.concat(&token_id))
        }
    }

    /// Pause contract (owner only)
    pub fn pause(ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let mut collection_data = get_collection_data(&storage)?;
        require!(owner.key() == collection_data.freeze_authority, "Unauthorized: Only owner can pause");

        collection_data.is_frozen = true;
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        emit!(ContractPaused {});
        Ok(())
    }

    /// Unpause contract (owner only)
    pub fn unpause(ctx: Context<AdminAction>) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let mut collection_data = get_collection_data(&storage)?;
        require!(owner.key() == collection_data.freeze_authority, "Unauthorized: Only owner can unpause");

        collection_data.is_frozen = false;
        Storage::put(storage, ByteString::from_literal("collection_data"), collection_data.serialize());

        emit!(ContractUnpaused {});
        Ok(())
    }

    /// Check if contract is paused
    pub fn is_paused(ctx: Context<View>) -> Result<bool> {
        let storage = Storage::get_context();
        let collection_data = get_collection_data(&storage)?;
        Ok(collection_data.is_frozen)
    }
}

// Helper functions
fn get_collection_data(storage: &Storage) -> Result<CollectionAccount> {
    match Storage::get(storage.clone(), ByteString::from_literal("collection_data")) {
        Some(data) => CollectionAccount::deserialize(data),
        None => Err(ErrorCode::CollectionNotInitialized.into()),
    }
}

fn is_authorized_minter(storage: &Storage, address: H160) -> Result<bool> {
    let minter_key = format!("minter_{}", address);
    Ok(Storage::get(storage.clone(), ByteString::from_str(&minter_key)).is_some())
}

fn is_approved_for_token(storage: &Storage, token_id: &ByteString, address: H160) -> Result<bool> {
    let approval_key = format!("approval_{}", token_id);
    match Storage::get(storage.clone(), ByteString::from_str(&approval_key)) {
        Some(approved_bytes) => {
            let approved = H160::from_byte_string(approved_bytes);
            Ok(approved == address)
        },
        None => Ok(false),
    }
}

fn remove_token_from_list(token_ids: &mut Array<ByteString>, token_id: &ByteString) {
    let mut new_tokens = Array::new();
    for i in 0..token_ids.size() {
        let token = token_ids.get(i).clone();
        if token != *token_id {
            new_tokens.push(token);
        }
    }
    *token_ids = new_tokens;
}

// Events
#[event]
pub struct ContractDeployed {
    pub symbol: ByteString,
}

#[event]
pub struct TransferEvent {
    pub from: H160,
    pub to: H160,
    pub amount: Int256,
    pub token_id: ByteString,
}

#[event]
pub struct ApprovalEvent {
    pub owner: H160,
    pub approved: H160,
    pub token_id: ByteString,
}

#[event]
pub struct TokenMinted {
    pub token_id: ByteString,
}

#[event]
pub struct TokenBurned {
    pub token_id: ByteString,
}

#[event]
pub struct MinterAdded {
    pub minter: H160,
}

#[event]
pub struct BaseURISet {
    pub base_uri: ByteString,
}

#[event]
pub struct ContractPaused {}

#[event]
pub struct ContractUnpaused {}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Collection not initialized")]
    CollectionNotInitialized,
    #[msg("Token not found")]
    TokenNotFound,
    #[msg("Owner account not found")]
    OwnerAccountNotFound,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Token already exists")]
    TokenAlreadyExists,
    #[msg("Contract is paused")]
    ContractPaused,
    #[msg("Invalid token ID")]
    InvalidTokenId,
}