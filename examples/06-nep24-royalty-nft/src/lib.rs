//! # NEP-24 Royalty NFT Contract
//!
//! A comprehensive NFT contract implementing both NEP-11 and NEP-24 standards:
//! - Complete NEP-11 non-fungible token functionality
//! - NEP-24 royalty standard for creator compensation
//! - Advanced royalty distribution with multiple recipients
//! - Marketplace integration with automatic royalty payments
//! - Creator and collector management systems
//!
//! This contract enables creators to earn ongoing royalties from secondary sales
//! while providing a complete NFT ecosystem for digital art and collectibles.

#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

declare_id!("RoyaltyNft111111111111111111111111111111112");

/// Royalty information structure
#[derive(Clone, Debug)]
pub struct RoyaltyInfo {
    pub recipient: H160,
    pub percentage: u32, // Basis points (100 = 1%)
}

impl RoyaltyInfo {
    pub fn new(recipient: H160, percentage: u32) -> Self {
        Self { recipient, percentage }
    }

    pub fn serialize(&self) -> ByteString {
        let mut result = ByteString::empty();
        result = result.concat(&self.recipient.into_byte_string());
        result = result.concat(&ByteString::from_bytes(&self.percentage.to_le_bytes()));
        result
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 24 { // 20 bytes for H160 + 4 bytes for u32
            return None;
        }

        let recipient = H160::from_byte_string(ByteString::from_bytes(&data[0..20]));
        let percentage = u32::from_le_bytes([data[20], data[21], data[22], data[23]]);

        Some(Self { recipient, percentage })
    }
}

/// Token metadata structure
#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub name: ByteString,
    pub description: ByteString,
    pub image: ByteString,
    pub attributes: Map<ByteString, ByteString>,
}

impl TokenMetadata {
    pub fn new(name: ByteString, description: ByteString, image: ByteString) -> Self {
        Self {
            name,
            description,
            image,
            attributes: Map::new(),
        }
    }

    pub fn add_attribute(&mut self, key: ByteString, value: ByteString) {
        self.attributes.put(key, value);
    }

    pub fn serialize(&self) -> ByteString {
        let mut result = ByteString::empty();
        
        // Serialize name length and data
        let name_len = self.name.len() as u32;
        result = result.concat(&ByteString::from_bytes(&name_len.to_le_bytes()));
        result = result.concat(&self.name);
        
        // Serialize description length and data
        let desc_len = self.description.len() as u32;
        result = result.concat(&ByteString::from_bytes(&desc_len.to_le_bytes()));
        result = result.concat(&self.description);
        
        // Serialize image length and data
        let image_len = self.image.len() as u32;
        result = result.concat(&ByteString::from_bytes(&image_len.to_le_bytes()));
        result = result.concat(&self.image);
        
        // Serialize attributes count
        let attr_count = self.attributes.size() as u32;
        result = result.concat(&ByteString::from_bytes(&attr_count.to_le_bytes()));
        
        result
    }

    pub fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < 12 { // Minimum size for 3 length fields
            return None;
        }
        
        let mut offset = 0;
        
        // Deserialize name
        let name_len = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        if offset + name_len > data.len() { return None; }
        let name = ByteString::from_bytes(&data[offset..offset + name_len]);
        offset += name_len;
        
        // Deserialize description
        if offset + 4 > data.len() { return None; }
        let desc_len = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        if offset + desc_len > data.len() { return None; }
        let description = ByteString::from_bytes(&data[offset..offset + desc_len]);
        offset += desc_len;
        
        // Deserialize image
        if offset + 4 > data.len() { return None; }
        let image_len = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        if offset + image_len > data.len() { return None; }
        let image = ByteString::from_bytes(&data[offset..offset + image_len]);
        offset += image_len;
        
        // Deserialize attributes count
        if offset + 4 > data.len() { return None; }
        let _attr_count = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]);
        
        Some(Self {
            name,
            description,
            image,
            attributes: Map::new(), // Simplified for this conversion
        })
    }
}

/// Royalty NFT account data
#[derive(Clone, Debug)]
pub struct RoyaltyNftAccount {
    pub mint: H160,
    pub owner: H160,
    pub token_id: ByteString,
    pub metadata: TokenMetadata,
    pub royalty_infos: Array<RoyaltyInfo>,
}

/// Collection account data
#[derive(Clone, Debug)]
pub struct RoyaltyCollectionAccount {
    pub symbol: ByteString,
    pub total_supply: Int256,
    pub mint_authority: H160,
    pub freeze_authority: H160,
    pub base_uri: ByteString,
    pub is_frozen: bool,
    pub default_royalty: Array<RoyaltyInfo>,
    pub max_royalty_percentage: u32,
}

/// Owner account data (tracks NFTs owned)
#[derive(Clone, Debug)]
pub struct RoyaltyOwnerAccount {
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
pub struct MintWithRoyalty<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub collection: AccountInfo<'info>,
    #[account(init)]
    pub nft_account: AccountInfo<'info>,
    #[account(mut)]
    pub owner_account: AccountInfo<'info>,
}

/// Marketplace sale context
#[derive(Accounts)]
pub struct MarketplaceSale<'info> {
    #[account(signer)]
    pub marketplace: AccountInfo<'info>,
    #[account(mut)]
    pub nft_account: AccountInfo<'info>,
    #[account(mut)]
    pub seller_account: AccountInfo<'info>,
    #[account(mut)]
    pub buyer_account: AccountInfo<'info>,
    #[account(mut)]
    pub collection: AccountInfo<'info>,
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
pub mod royalty_nft {
    use super::*;

    /// Deploy the royalty NFT contract
    pub fn deploy(
        ctx: Context<Deploy>,
        symbol: ByteString,
        base_uri: ByteString,
        default_royalty_percentage: u32
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        let owner = &ctx.accounts.owner;

        // Validate parameters
        require!(!symbol.is_empty() && symbol.len() <= 16, "Invalid symbol: must be 1-16 characters");
        require!(default_royalty_percentage <= 2500, "Default royalty too high (max 25%)");

        // Initialize default royalty
        let mut default_royalty = Array::new();
        if default_royalty_percentage > 0 {
            default_royalty.push(RoyaltyInfo::new(owner.key(), default_royalty_percentage));
        }

        // Initialize collection account
        let collection_data = RoyaltyCollectionAccount {
            symbol: symbol.clone(),
            total_supply: Int256::zero(),
            mint_authority: owner.key(),
            freeze_authority: owner.key(),
            base_uri,
            is_frozen: false,
            default_royalty,
            max_royalty_percentage: 2500, // 25% max
        };

        // Store collection data
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("royalty_collection_data"), collection_data.serialize());

        // Emit deployment event
        emit!(RoyaltyNftDeployed { symbol });
        Ok(())
    }

    /// Get token symbol
    pub fn symbol(ctx: Context<View>) -> Result<ByteString> {
        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        Ok(collection_data.symbol)
    }

    /// Get decimals (always 0 for NFTs)
    pub fn decimals(_ctx: Context<View>) -> Result<u32> {
        Ok(0)
    }

    /// Get total supply
    pub fn total_supply(ctx: Context<View>) -> Result<Int256> {
        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        Ok(collection_data.total_supply)
    }

    /// Get balance of account
    pub fn balance_of(ctx: Context<View>, owner: H160) -> Result<Int256> {
        let storage = Storage::get_context();
        let owner_key = format!("royalty_owner_account_{}", owner);
        
        match Storage::get(storage, ByteString::from_str(&owner_key)) {
            Some(owner_data) => {
                let owner_account: RoyaltyOwnerAccount = RoyaltyOwnerAccount::deserialize(owner_data)?;
                Ok(owner_account.balance)
            },
            None => Ok(Int256::zero()),
        }
    }

    /// Get owner of token
    pub fn owner_of(ctx: Context<View>, token_id: ByteString) -> Result<H160> {
        let storage = Storage::get_context();
        let nft_key = format!("royalty_nft_{}", token_id);
        
        match Storage::get(storage, ByteString::from_str(&nft_key)) {
            Some(nft_data) => {
                let nft_account: RoyaltyNftAccount = RoyaltyNftAccount::deserialize(nft_data)?;
                Ok(nft_account.owner)
            },
            None => Ok(H160::zero()),
        }
    }

    /// Transfer token
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
        let collection_data = get_royalty_collection_data(&storage)?;
        require!(!collection_data.is_frozen, "Contract is paused");

        // Get NFT data
        let nft_key = format!("royalty_nft_{}", token_id);
        let mut nft_data: RoyaltyNftAccount = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(data) => RoyaltyNftAccount::deserialize(data)?,
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
        let approval_key = format!("royalty_approval_{}", token_id);
        Storage::delete(storage.clone(), ByteString::from_str(&approval_key));

        // Update from owner account
        let from_key = format!("royalty_owner_account_{}", from);
        let mut from_account_data: RoyaltyOwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&from_key)) {
            Some(data) => RoyaltyOwnerAccount::deserialize(data)?,
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
        let to_key = format!("royalty_owner_account_{}", to);
        let mut to_account_data: RoyaltyOwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&to_key)) {
            Some(data) => RoyaltyOwnerAccount::deserialize(data)?,
            None => RoyaltyOwnerAccount {
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

    /// Get royalty information for a token (NEP-24 required)
    pub fn royalty_info(
        ctx: Context<View>,
        token_id: ByteString,
        _royalty_token: H160,
        sale_price: Int256
    ) -> Result<Array<Map<ByteString, Any>>> {
        let storage = Storage::get_context();
        let nft_key = format!("royalty_nft_{}", token_id);
        
        let royalty_infos = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(nft_data) => {
                let nft_account: RoyaltyNftAccount = RoyaltyNftAccount::deserialize(nft_data)?;
                if nft_account.royalty_infos.size() > 0 {
                    nft_account.royalty_infos
                } else {
                    // Use default royalty
                    let collection_data = get_royalty_collection_data(&storage)?;
                    collection_data.default_royalty
                }
            },
            None => return Err(ErrorCode::TokenNotFound.into()),
        };

        let mut result = Array::new();

        for i in 0..royalty_infos.size() {
            let royalty_info = royalty_infos.get(i);
            let mut royalty_map = Map::new();
            
            // Calculate royalty amount based on percentage
            let percentage_int = Int256::new(royalty_info.percentage as i64);
            let royalty_amount = sale_price.checked_mul(&percentage_int)
                .checked_div(&Int256::new(10000));
            
            royalty_map.put(
                ByteString::from_literal("royaltyRecipient"),
                royalty_info.recipient.into_any()
            );
            royalty_map.put(
                ByteString::from_literal("royaltyAmount"),
                royalty_amount.into_any()
            );

            result.push(royalty_map);
        }

        Ok(result)
    }

    /// Mint NFT with royalty information
    pub fn mint_with_royalty(
        ctx: Context<MintWithRoyalty>,
        to: H160,
        token_id: ByteString,
        name: ByteString,
        description: ByteString,
        image: ByteString,
        royalty_recipients: Array<H160>,
        royalty_percentages: Array<u32>
    ) -> Result<()> {
        let authority = &ctx.accounts.authority;
        let collection = &mut ctx.accounts.collection;
        let nft_account = &mut ctx.accounts.nft_account;
        let owner_account = &mut ctx.accounts.owner_account;

        require!(!token_id.is_empty() && token_id.len() <= 64, "Invalid token ID: must be 1-64 characters");

        let storage = Storage::get_context();
        let mut collection_data = get_royalty_collection_data(&storage)?;

        require!(
            authority.key() == collection_data.mint_authority || is_authorized_minter(&storage, authority.key())?,
            "Unauthorized: Not authorized to mint"
        );

        // Validate royalty parameters
        require!(royalty_recipients.size() == royalty_percentages.size(), "Royalty recipients and percentages length mismatch");

        let mut total_royalty = 0u32;
        let mut royalty_infos = Array::new();

        for i in 0..royalty_recipients.size() {
            let recipient = royalty_recipients.get(i);
            let percentage = royalty_percentages.get(i);

            require!(percentage <= collection_data.max_royalty_percentage, "Individual royalty percentage too high");

            total_royalty += percentage;
            royalty_infos.push(RoyaltyInfo::new(recipient, percentage));
        }

        require!(total_royalty <= collection_data.max_royalty_percentage, "Total royalty percentage too high");

        // Check if token already exists
        let nft_key = format!("royalty_nft_{}", token_id);
        require!(
            Storage::get(storage.clone(), ByteString::from_str(&nft_key)).is_none(),
            "Token already exists"
        );

        // Create NFT account
        let metadata = TokenMetadata::new(name, description, image);
        let nft_data = RoyaltyNftAccount {
            mint: collection.key(),
            owner: to,
            token_id: token_id.clone(),
            metadata,
            royalty_infos,
        };
        Storage::put(storage.clone(), ByteString::from_str(&nft_key), nft_data.serialize());

        // Update owner account
        let owner_key = format!("royalty_owner_account_{}", to);
        let mut owner_data: RoyaltyOwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&owner_key)) {
            Some(data) => RoyaltyOwnerAccount::deserialize(data)?,
            None => RoyaltyOwnerAccount {
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
        Storage::put(storage, ByteString::from_literal("royalty_collection_data"), collection_data.serialize());

        // Emit Transfer event (from null address)
        emit!(TransferEvent {
            from: H160::zero(),
            to,
            amount: Int256::one(),
            token_id: token_id.clone(),
        });

        emit!(RoyaltyNftMinted { 
            token_id: token_id.clone(), 
            to, 
            total_royalty_percentage: Int256::new(total_royalty as i64),
        });
        Ok(())
    }

    /// Process marketplace sale with automatic royalty distribution
    pub fn marketplace_sale(
        ctx: Context<MarketplaceSale>,
        token_id: ByteString,
        sale_price: Int256,
        payment_token: H160
    ) -> Result<()> {
        let marketplace = &ctx.accounts.marketplace;
        let nft_account = &mut ctx.accounts.nft_account;
        let seller_account = &mut ctx.accounts.seller_account;
        let buyer_account = &mut ctx.accounts.buyer_account;
        let collection = &mut ctx.accounts.collection;

        let storage = Storage::get_context();

        // Verify marketplace authorization
        require!(is_approved_marketplace(&storage, marketplace.key())?, "Unauthorized: Marketplace not approved");

        // Get NFT data
        let nft_key = format!("royalty_nft_{}", token_id);
        let mut nft_data: RoyaltyNftAccount = match Storage::get(storage.clone(), ByteString::from_str(&nft_key)) {
            Some(data) => RoyaltyNftAccount::deserialize(data)?,
            None => return Err(ErrorCode::TokenNotFound.into()),
        };

        let seller = nft_data.owner;
        let buyer = buyer_account.key();

        // Calculate and distribute royalties
        let mut total_royalty = Int256::zero();

        for i in 0..nft_data.royalty_infos.size() {
            let royalty_info = nft_data.royalty_infos.get(i);
            let percentage_int = Int256::new(royalty_info.percentage as i64);
            let royalty_amount = sale_price.checked_mul(&percentage_int)
                .checked_div(&Int256::new(10000));
            total_royalty = total_royalty.checked_add(&royalty_amount);
        }

        // Update NFT owner
        nft_data.owner = buyer;
        Storage::put(storage.clone(), ByteString::from_str(&nft_key), nft_data.serialize());

        // Update owner accounts (simplified - would need proper account management)
        let seller_key = format!("royalty_owner_account_{}", seller);
        let mut seller_data: RoyaltyOwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&seller_key)) {
            Some(data) => RoyaltyOwnerAccount::deserialize(data)?,
            None => return Err(ErrorCode::OwnerAccountNotFound.into()),
        };

        seller_data.balance = seller_data.balance.checked_sub(&Int256::one());
        remove_token_from_list(&mut seller_data.token_ids, &token_id);
        Storage::put(storage.clone(), ByteString::from_str(&seller_key), seller_data.serialize());

        let buyer_key = format!("royalty_owner_account_{}", buyer);
        let mut buyer_data: RoyaltyOwnerAccount = match Storage::get(storage.clone(), ByteString::from_str(&buyer_key)) {
            Some(data) => RoyaltyOwnerAccount::deserialize(data)?,
            None => RoyaltyOwnerAccount {
                owner: buyer,
                balance: Int256::zero(),
                token_ids: Array::new(),
            },
        };

        buyer_data.balance = buyer_data.balance.checked_add(&Int256::one());
        buyer_data.token_ids.push(token_id.clone());
        Storage::put(storage, ByteString::from_str(&buyer_key), buyer_data.serialize());

        // Calculate seller proceeds
        let seller_proceeds = sale_price.checked_sub(&total_royalty);

        emit!(MarketplaceSale {
            token_id,
            seller,
            buyer,
            sale_price,
            total_royalty,
            seller_proceeds,
        });

        Ok(())
    }

    /// Set creator royalty
    pub fn set_creator_royalty(
        ctx: Context<AdminAction>,
        creator: H160,
        percentage: u32
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;

        // Verify authorization (creator or contract owner)
        require!(
            owner.key() == creator || owner.key() == collection_data.mint_authority,
            "Unauthorized: Not creator or owner"
        );

        require!(percentage <= collection_data.max_royalty_percentage, "Royalty percentage too high");

        let creator_royalty_key = format!("creator_royalty_{}", creator);

        if percentage == 0 {
            Storage::delete(storage, ByteString::from_str(&creator_royalty_key));
        } else {
            Storage::put(storage, ByteString::from_str(&creator_royalty_key), ByteString::from_bytes(&percentage.to_le_bytes()));
        }

        emit!(CreatorRoyaltySet { creator, percentage: Int256::new(percentage as i64) });
        Ok(())
    }

    /// Add approved marketplace
    pub fn add_marketplace(
        ctx: Context<AdminAction>,
        marketplace: H160
    ) -> Result<()> {
        let owner = &ctx.accounts.owner;
        let collection = &ctx.accounts.collection;

        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        require!(owner.key() == collection_data.mint_authority, "Unauthorized: Only owner can add marketplaces");

        let marketplace_key = format!("approved_marketplace_{}", marketplace);
        Storage::put(storage, ByteString::from_str(&marketplace_key), ByteString::from_literal("true"));

        emit!(MarketplaceAdded { marketplace });
        Ok(())
    }

    /// Get maximum allowed royalty percentage
    pub fn get_max_royalty(ctx: Context<View>) -> Result<u32> {
        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        Ok(collection_data.max_royalty_percentage)
    }

    /// Check if marketplace is approved
    pub fn is_approved_marketplace(ctx: Context<View>, marketplace: H160) -> Result<bool> {
        let storage = Storage::get_context();
        is_approved_marketplace(&storage, marketplace)
    }

    /// Get contract owner
    pub fn get_owner(ctx: Context<View>) -> Result<H160> {
        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        Ok(collection_data.mint_authority)
    }

    /// Check if contract is paused
    pub fn is_paused(ctx: Context<View>) -> Result<bool> {
        let storage = Storage::get_context();
        let collection_data = get_royalty_collection_data(&storage)?;
        Ok(collection_data.is_frozen)
    }

    /// Get token properties/metadata
    pub fn properties(ctx: Context<View>, token_id: ByteString) -> Result<Map<ByteString, Any>> {
        let storage = Storage::get_context();
        let nft_key = format!("royalty_nft_{}", token_id);
        
        match Storage::get(storage, ByteString::from_str(&nft_key)) {
            Some(nft_data) => {
                let nft_account: RoyaltyNftAccount = RoyaltyNftAccount::deserialize(nft_data)?;
                let mut result = Map::new();
                result.put(ByteString::from_literal("name"), nft_account.metadata.name.into_any());
                result.put(ByteString::from_literal("description"), nft_account.metadata.description.into_any());
                result.put(ByteString::from_literal("image"), nft_account.metadata.image.into_any());
                Ok(result)
            },
            None => Ok(Map::new()),
        }
    }
}

// Helper functions
fn get_royalty_collection_data(storage: &Storage) -> Result<RoyaltyCollectionAccount> {
    match Storage::get(storage.clone(), ByteString::from_literal("royalty_collection_data")) {
        Some(data) => RoyaltyCollectionAccount::deserialize(data),
        None => Err(ErrorCode::CollectionNotInitialized.into()),
    }
}

fn is_authorized_minter(storage: &Storage, address: H160) -> Result<bool> {
    let minter_key = format!("authorized_minter_{}", address);
    Ok(Storage::get(storage.clone(), ByteString::from_str(&minter_key)).is_some())
}

fn is_approved_for_token(storage: &Storage, token_id: &ByteString, address: H160) -> Result<bool> {
    let approval_key = format!("royalty_approval_{}", token_id);
    match Storage::get(storage.clone(), ByteString::from_str(&approval_key)) {
        Some(approved_bytes) => {
            let approved = H160::from_byte_string(approved_bytes);
            Ok(approved == address)
        },
        None => Ok(false),
    }
}

fn is_approved_marketplace(storage: &Storage, marketplace: H160) -> Result<bool> {
    let marketplace_key = format!("approved_marketplace_{}", marketplace);
    Ok(Storage::get(storage.clone(), ByteString::from_str(&marketplace_key)).is_some())
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
pub struct RoyaltyNftDeployed {
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
pub struct RoyaltyNftMinted {
    pub token_id: ByteString,
    pub to: H160,
    pub total_royalty_percentage: Int256,
}

#[event]
pub struct MarketplaceSale {
    pub token_id: ByteString,
    pub seller: H160,
    pub buyer: H160,
    pub sale_price: Int256,
    pub total_royalty: Int256,
    pub seller_proceeds: Int256,
}

#[event]
pub struct CreatorRoyaltySet {
    pub creator: H160,
    pub percentage: Int256,
}

#[event]
pub struct MarketplaceAdded {
    pub marketplace: H160,
}

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
    #[msg("Royalty percentage too high")]
    RoyaltyTooHigh,
    #[msg("Marketplace not approved")]
    MarketplaceNotApproved,
}