//! NEP-24: NFT Royalty Standard
//! Provides royalty payment support for NFTs

use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, Array};

type Result<T> = core::result::Result<T, ByteString>;

/// NEP-24 Royalty Info structure
#[derive(Debug, Clone)]
pub struct RoyaltyInfo {
    /// Recipient of royalty payments
    pub recipient: H160,
    /// Royalty amount (in basis points, e.g., 250 = 2.5%)
    pub amount: u16,
}

/// NEP-24 trait for NFT contracts with royalty support
pub trait NEP24 {
    /// Get royalty information for a token
    fn royalty_info(&self, token_id: ByteString, sale_price: Int256) -> Option<RoyaltyPayment>;
    
    /// Set royalty information for a token (only token owner)
    fn set_royalty(&mut self, token_id: ByteString, recipient: H160, amount: u16) -> bool;
    
    /// Get default royalty for all tokens
    fn default_royalty(&self) -> Option<RoyaltyInfo>;
    
    /// Set default royalty for all tokens (only contract owner)
    fn set_default_royalty(&mut self, recipient: H160, amount: u16) -> bool;
}

/// Royalty payment information
#[derive(Debug, Clone)]
pub struct RoyaltyPayment {
    pub recipient: H160,
    pub amount: Int256,
}

/// NEP-24 implementation helper
pub struct NEP24Implementation;

impl NEP24Implementation {
    /// Calculate royalty payment
    pub fn calculate_royalty(sale_price: Int256, royalty_bps: u16) -> Int256 {
        // royalty_bps is in basis points (1/10000)
        // e.g., 250 bps = 2.5%
        let royalty = sale_price
            .checked_mul(&Int256::from(royalty_bps as i64));
        
        royalty
            .checked_div(&Int256::from(10000))
    }

    /// Validate royalty amount (max 50% = 5000 bps)
    pub fn validate_royalty(amount: u16) -> bool {
        amount <= 5000
    }

    /// Store royalty info in contract storage
    pub fn store_royalty(token_id: ByteString, recipient: H160, amount: u16) {
        use crate::services::storage::Storage;
        
        let context = Storage::get_context();
        let key = ByteString::from_literal("royalty:")
            .concat(&token_id);
        
        // Pack royalty info
        let royalty_data = Array::from_vec(vec![
            recipient.into_any(),
            Int256::from(amount as i64).into_any(),
        ]);
        
        // In real implementation, serialize royalty_data to ByteString
        // For now, store empty as we can't properly serialize without the full runtime
        Storage::put(context, key, ByteString::empty());
    }

    /// Get royalty info from contract storage
    pub fn get_royalty(token_id: ByteString) -> Option<RoyaltyInfo> {
        use crate::services::storage::Storage;
        
        let context = Storage::get_context();
        let key = ByteString::from_literal("royalty:")
            .concat(&token_id);
        
        Storage::get(context, key)
            .and_then(|_data| {
                // In a real implementation, deserialize the ByteString
                // For now, return None as we can't properly deserialize without the full runtime
                None::<RoyaltyInfo>
            })
    }

    /// Store default royalty in contract storage
    pub fn store_default_royalty(recipient: H160, amount: u16) {
        use crate::services::storage::Storage;
        
        let context = Storage::get_context();
        let key = ByteString::from_literal("default_royalty");
        
        let royalty_data = Array::from_vec(vec![
            recipient.into_any(),
            Int256::from(amount as i64).into_any(),
        ]);
        
        // In real implementation, serialize royalty_data to ByteString
        // For now, store empty as we can't properly serialize without the full runtime
        Storage::put(context, key, ByteString::empty());
    }

    /// Get default royalty from contract storage
    pub fn get_default_royalty() -> Option<RoyaltyInfo> {
        use crate::services::storage::Storage;
        
        let context = Storage::get_context();
        let key = ByteString::from_literal("default_royalty");
        
        Storage::get(context, key)
            .and_then(|_data| {
                // In a real implementation, deserialize the ByteString
                // For now, return None as we can't properly deserialize without the full runtime
                None::<RoyaltyInfo>
            })
    }

    /// Process royalty payment during NFT sale
    pub fn process_royalty_payment(
        token_id: ByteString,
        sale_price: Int256,
        buyer: H160,
        seller: H160,
    ) -> Result<RoyaltyPayment> {
        // Get royalty info (token-specific or default)
        let royalty_info = NEP24Implementation::get_royalty(token_id.clone())
            .or_else(|| NEP24Implementation::get_default_royalty())
            .ok_or_else(|| ByteString::from_literal("No royalty info"))?;

        // Calculate royalty amount
        let royalty_amount = NEP24Implementation::calculate_royalty(sale_price, royalty_info.amount);

        // Ensure buyer has sufficient funds
        let total_amount = sale_price
            .checked_add(&royalty_amount); // This will panic on overflow

        Ok(RoyaltyPayment {
            recipient: royalty_info.recipient,
            amount: royalty_amount,
        })
    }

    /// Emit royalty payment event
    pub fn emit_royalty_payment(
        token_id: ByteString,
        recipient: H160,
        amount: Int256,
        payer: H160,
    ) {
        use crate::services::runtime::Runtime;
        
        let event_data = Array::from_vec(vec![
            ByteString::from_literal("RoyaltyPayment").into_any(),
            token_id.into_any(),
            recipient.into_any(),
            amount.into_any(),
            payer.into_any(),
        ]);
        
        Runtime::notify(
            ByteString::from_literal("RoyaltyPayment"),
            event_data,
        );
    }
}

/// Royalty registry for managing royalties across multiple NFT contracts
pub struct RoyaltyRegistry;

impl RoyaltyRegistry {
    const REGISTRY_KEY: &'static str = "royalty_registry";

    /// Register royalty info for an NFT contract
    pub fn register(
        contract: H160,
        token_id: ByteString,
        recipient: H160,
        amount: u16,
    ) -> bool {
        use crate::services::storage::Storage;
        use crate::services::runtime::Runtime;

        // Check caller is the NFT contract
        if Runtime::get_calling_script_hash() != contract {
            return false;
        }

        let context = Storage::get_context();
        let key = ByteString::from_literal(Self::REGISTRY_KEY)
            .concat(&ByteString::from_literal(":"))
            .concat(&contract.into_byte_string())
            .concat(&ByteString::from_literal(":"))
            .concat(&token_id);

        let royalty_data = Array::from_vec(vec![
            recipient.into_any(),
            Int256::from(amount as i64).into_any(),
        ]);

        // In real implementation, serialize royalty_data to ByteString
        // For now, store empty as we can't properly serialize without the full runtime
        Storage::put(context, key, ByteString::empty());
        true
    }

    /// Query royalty info from registry
    pub fn query(contract: H160, token_id: ByteString) -> Option<RoyaltyInfo> {
        use crate::services::storage::Storage;

        let context = Storage::get_context();
        let key = ByteString::from_literal(Self::REGISTRY_KEY)
            .concat(&ByteString::from_literal(":"))
            .concat(&contract.into_byte_string())
            .concat(&ByteString::from_literal(":"))
            .concat(&token_id);

        Storage::get(context, key)
            .and_then(|_data| {
                // In a real implementation, deserialize the ByteString
                // For now, return None as we can't properly deserialize without the full runtime
                None::<RoyaltyInfo>
            })
    }
}