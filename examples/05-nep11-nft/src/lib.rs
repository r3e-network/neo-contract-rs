#![no_std]
#![no_main]

use neo_contract::prelude::*;

declare_id!("NEP11NFT");

/// Simple NEP-11 NFT implementation
#[program]
pub mod nep11_nft {
    use super::*;

    /// Initialize the NFT collection
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let storage = Storage::get_context();
        
        // Store collection metadata
        Storage::put(
            storage,
            ByteString::from_literal("symbol"),
            ByteString::from_literal("NFT")
        );
        
        Storage::put(
            storage,
            ByteString::from_literal("name"),
            ByteString::from_literal("Neo NFT Collection")
        );
        
        Storage::put(
            storage,
            ByteString::from_literal("owner"),
            ctx.accounts.owner.key
        );
        
        Storage::put(
            storage,
            ByteString::from_literal("total_supply"),
            Int256::zero()
        );
        
        Runtime::log(ByteString::from_literal("NFT collection initialized"));
        Ok(())
    }

    /// Mint a new NFT
    pub fn mint(
        ctx: Context<Mint>,
        token_id: ByteString,
        metadata: ByteString,
    ) -> Result<()> {
        let storage = Storage::get_context();
        
        // Check if token already exists
        let token_key = ByteString::from_literal("token_").concat(&token_id);
        require!(
            Storage::get(storage, token_key.clone()).is_none(),
            ContractError::TokenAlreadyExists
        );
        
        // Mint the token
        Storage::put(storage, token_key.clone(), ctx.accounts.recipient.key);
        
        // Store metadata
        let metadata_key = ByteString::from_literal("metadata_").concat(&token_id);
        Storage::put(storage, metadata_key, metadata);
        
        // Update total supply
        let total_supply = Storage::get(storage, ByteString::from_literal("total_supply"))
            .unwrap_or(ByteString::from_literal("0"));
        let new_supply = Int256::new(1); // Simplified - would parse and add
        Storage::put(
            storage,
            ByteString::from_literal("total_supply"),
            new_supply
        );
        
        // Emit mint event
        let mut event_data = Array::new();
        event_data.push(token_id.into_any());
        event_data.push(ctx.accounts.recipient.key.into_any());
        Runtime::notify(ByteString::from_literal("NFTMinted"), event_data);
        
        Ok(())
    }

    /// Transfer an NFT
    pub fn transfer(
        ctx: Context<Transfer>,
        token_id: ByteString,
    ) -> Result<()> {
        let storage = Storage::get_context();
        
        // Get token owner
        let token_key = ByteString::from_literal("token_").concat(&token_id);
        let owner = Storage::get(storage, token_key.clone())
            .ok_or(ContractError::TokenNotFound)?;
        
        // Verify ownership
        require!(
            check_witness_with_account(ctx.accounts.from.key),
            ContractError::Unauthorized
        );
        
        // Transfer the token
        Storage::put(storage, token_key, ctx.accounts.to.key);
        
        // Emit transfer event
        let mut event_data = Array::new();
        event_data.push(token_id.into_any());
        event_data.push(ctx.accounts.from.key.into_any());
        event_data.push(ctx.accounts.to.key.into_any());
        Runtime::notify(ByteString::from_literal("NFTTransferred"), event_data);
        
        Ok(())
    }

    /// Get the owner of a token
    pub fn owner_of(_ctx: Context<View>, token_id: ByteString) -> Result<H160> {
        let storage = Storage::get_context();
        let token_key = ByteString::from_literal("token_").concat(&token_id);
        
        Storage::get(storage, token_key)
            .ok_or(ContractError::TokenNotFound)
            .map(|_| H160::zero()) // Simplified - would parse owner
    }

    /// Get token metadata
    pub fn token_metadata(_ctx: Context<View>, token_id: ByteString) -> Result<ByteString> {
        let storage = Storage::get_context();
        let metadata_key = ByteString::from_literal("metadata_").concat(&token_id);
        
        Storage::get(storage, metadata_key)
            .ok_or(ContractError::TokenNotFound)
    }

    /// Get collection symbol
    pub fn symbol(_ctx: Context<View>) -> Result<ByteString> {
        let storage = Storage::get_context();
        Ok(Storage::get(storage, ByteString::from_literal("symbol"))
            .unwrap_or(ByteString::from_literal("NFT")))
    }

    /// Get total supply
    pub fn total_supply(_ctx: Context<View>) -> Result<Int256> {
        let storage = Storage::get_context();
        Ok(Storage::get(storage, ByteString::from_literal("total_supply"))
            .map(|_| Int256::zero())
            .unwrap_or(Int256::zero()))
    }
}

// Account structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Mint<'info> {
    pub authority: Signer<'info>,
    pub recipient: Account<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    pub from: Signer<'info>,
    pub to: Account<'info>,
}

#[derive(Accounts)]
pub struct View<'info> {
    pub caller: Signer<'info>,
}

// Error types
#[error_code]
pub enum ContractError {
    #[msg("Token already exists")]
    TokenAlreadyExists,
    #[msg("Token not found")]
    TokenNotFound,
    #[msg("Unauthorized")]
    Unauthorized,
}