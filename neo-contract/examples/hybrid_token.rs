#![no_std]
#![cfg_attr(target_arch = "wasm32", feature(core_intrinsics, lang_items))]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use neo_contract::prelude::*;

//! # Hybrid Token Contract for Neo N3
//! 
//! This example demonstrates a hybrid token that implements both NEP-17 (Fungible Token)
//! and NEP-11 (Non-Fungible Token) standards in a single contract.
//!
//! ## Features:
//! - Full NEP-17 functionality (transfers, balances, supply)
//! - Full NEP-11 functionality (token ownership, transfers, properties)
//! - Shared storage architecture
//! - Comprehensive event system
//!
//! ## Event Handling
//! This contract uses the standardized Neo N3 event pattern:
//! - Events are defined as structs with the `#[event]` attribute
//! - Event parameters that need to be indexed for efficient filtering use the `#[index]` attribute
//! - Events are emitted using the `EventName::emit(params)` method
//!
//! The helper methods `emit_ft_transfer` and `emit_nft_transfer` demonstrate how to properly
//! emit events following Neo N3 standards.

/// A hybrid token implementation that combines NEP-17 and NEP-11 functionality
/// to demonstrate how to create advanced token contracts on Neo N3
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Hybrid Token (FT+NFT) for Neo N3")]
mod hybrid_token {
    use super::*;
    
    // Define events for both NEP-17 and NEP-11 standards
    /// Transfer event following NEP-17 standard
    #[event]
    pub struct Transfer {
        #[index]
        from: Option<H160>,
        #[index]
        to: Option<H160>,
        amount: Int256,
    }
    
    /// Transfer event following NEP-11 standard with token ID
    #[event]
    pub struct NFTTransfer {
        #[index]
        from: Option<H160>,
        #[index]
        to: Option<H160>,
        #[index]
        token_id: ByteString,
        amount: Int256,
    }
    
    // Storage keys
    const OWNER_KEY: &[u8] = b"owner";
    const NAME_KEY: &[u8] = b"name";
    const SYMBOL_KEY: &[u8] = b"symbol";
    const DECIMALS_KEY: &[u8] = b"decimals";
    const FT_BALANCE_PREFIX: &[u8] = b"ft_balance:";
    const NFT_BALANCE_PREFIX: &[u8] = b"nft_balance:";
    const TOTAL_SUPPLY_KEY: &[u8] = b"total_supply";
    const TOTAL_NFT_KEY: &[u8] = b"total_nft";
    const TOKEN_PREFIX: &[u8] = b"token:";
    const OWNER_OF_PREFIX: &[u8] = b"owner_of:";
    const TOKENS_OF_PREFIX: &[u8] = b"tokens_of:";
    const PROPERTIES_PREFIX: &[u8] = b"props:";
    
    // Token struct for storing NFT data
    struct TokenData {
        owner: H160,
        name: String,
        description: String,
        image: String,
        properties: Map,
    }
    
    #[storage]
    struct HybridToken {
        // Contract metadata
        owner: StorageItem<H160>,
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        decimals: StorageItem<u8>,
        
        // Fungible token data
        ft_total_supply: StorageItem<Int256>,
        ft_balances: StorageMap<H160, Int256>,
        
        // Non-fungible token data
        nft_total_supply: StorageItem<Int256>,
        nft_tokens: StorageMap<ByteString, TokenData>,
        nft_owner_of: StorageMap<ByteString, H160>,
        nft_balances: StorageMap<H160, Int256>,
        nft_tokens_of: StorageMap<H160, Array<ByteString>>,
    }
    
    impl HybridToken {
        /// Initialize a new hybrid token contract
        #[constructor]
        pub fn new(owner: H160, name: String, symbol: String, decimals: u8) -> Self {
            Self {
                owner: StorageItem::new(OWNER_KEY).with_data(owner),
                name: StorageItem::new(NAME_KEY).with_data(name),
                symbol: StorageItem::new(SYMBOL_KEY).with_data(symbol),
                decimals: StorageItem::new(DECIMALS_KEY).with_data(decimals),
                
                ft_total_supply: StorageItem::new(TOTAL_SUPPLY_KEY).with_data(Int256::zero()),
                ft_balances: StorageMap::new(FT_BALANCE_PREFIX),
                
                nft_total_supply: StorageItem::new(TOTAL_NFT_KEY).with_data(Int256::zero()),
                nft_tokens: StorageMap::new(TOKEN_PREFIX),
                nft_owner_of: StorageMap::new(OWNER_OF_PREFIX),
                nft_balances: StorageMap::new(NFT_BALANCE_PREFIX),
                nft_tokens_of: StorageMap::new(TOKENS_OF_PREFIX),
            }
        }
        
        /// Get contract owner - safe method (read-only)
        #[safe]
        pub fn get_owner(&self) -> H160 {
            self.owner.get().unwrap_or_default()
        }
        
        /// Update contract owner - requires witness check
        #[method]
        pub fn update_owner(&mut self, new_owner: H160) -> bool {
            // Check if the caller is the current owner
            let current_owner = self.get_owner();
            if !Runtime::check_witness(&current_owner) {
                return false;
            }
            
            self.owner.set(new_owner);
            true
        }
        
        //======== Token Metadata Methods ========//
        
        /// Get token name - safe method (read-only)
        #[safe]
        pub fn name(&self) -> String {
            self.name.get().unwrap_or_else(|| "Hybrid Token".to_string())
        }
        
        /// Get token symbol - safe method (read-only)
        #[safe]
        pub fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_else(|| "HYBRID".to_string())
        }
        
        /// Get token decimals - safe method (read-only)
        #[safe]
        pub fn decimals(&self) -> u8 {
            self.decimals.get().unwrap_or(8)
        }
        
        //======== NEP-17 Methods ========//
        
        /// Get total supply of fungible tokens - safe method (read-only)
        #[safe]
        pub fn ft_total_supply(&self) -> Int256 {
            self.ft_total_supply.get().unwrap_or_else(Int256::zero)
        }
        
        /// Get fungible token balance of an address - safe method (read-only)
        #[safe]
        pub fn ft_balance_of(&self, owner: H160) -> Int256 {
            self.ft_balances.get(&owner).unwrap_or_else(Int256::zero)
        }
        
        /// Transfer fungible tokens from one address to another
        #[method]
        pub fn ft_transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            // Validate parameters
            if amount <= Int256::zero() {
                return false;
            }
            
            if to == H160::zero() {
                return false;
            }
            
            // Check if the sender is authorized
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check if the sender has enough balance
            let from_balance = self.ft_balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            let new_from_balance = from_balance - amount;
            if new_from_balance.is_zero() {
                self.ft_balances.delete(&from);
            } else {
                self.ft_balances.insert(&from, &new_from_balance);
            }
            
            let to_balance = self.ft_balance_of(to);
            let new_to_balance = to_balance + amount;
            self.ft_balances.insert(&to, &new_to_balance);
            
            // Emit transfer event
            self.emit_ft_transfer(Some(from), Some(to), amount);
            
            true
        }
        
        /// Mint fungible tokens - only contract owner can mint
        #[method]
        pub fn ft_mint(&mut self, to: H160, amount: Int256) -> bool {
            // Check if caller is the contract owner
            let owner = self.get_owner();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Validate parameters
            if amount <= Int256::zero() {
                return false;
            }
            
            if to == H160::zero() {
                return false;
            }
            
            // Update balance
            let to_balance = self.ft_balance_of(to);
            let new_to_balance = to_balance + amount;
            self.ft_balances.insert(&to, &new_to_balance);
            
            // Update total supply
            let current_supply = self.ft_total_supply();
            let new_supply = current_supply + amount;
            self.ft_total_supply.set(new_supply);
            
            // Emit transfer event (mint is a transfer from null address)
            self.emit_ft_transfer(None, Some(to), amount);
            
            true
        }
        
        /// Burn fungible tokens - tokens must be owned by the caller
        #[method]
        pub fn ft_burn(&mut self, from: H160, amount: Int256) -> bool {
            // Validate parameters
            if amount <= Int256::zero() {
                return false;
            }
            
            // Check if the caller is authorized
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check if the caller has enough balance
            let from_balance = self.ft_balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            // Update balance
            let new_from_balance = from_balance - amount;
            if new_from_balance.is_zero() {
                self.ft_balances.delete(&from);
            } else {
                self.ft_balances.insert(&from, &new_from_balance);
            }
            
            // Update total supply
            let current_supply = self.ft_total_supply();
            let new_supply = current_supply - amount;
            self.ft_total_supply.set(new_supply);
            
            // Emit transfer event (burn is a transfer to null address)
            self.emit_ft_transfer(Some(from), None, amount);
            
            true
        }
        
        /// Emit a NEP-17 Transfer event following Neo N3 standards
        #[safe]
        fn emit_ft_transfer(&self, from: Option<H160>, to: Option<H160>, amount: Int256) {
            // Emit event using the Transfer struct
            Transfer::emit(from, to, amount);
        }
        
        //======== NEP-11 Methods ========//
        
        /// Get total supply of non-fungible tokens - safe method (read-only)
        #[safe]
        pub fn nft_total_supply(&self) -> Int256 {
            self.nft_total_supply.get().unwrap_or_else(Int256::zero)
        }
        
        /// Get non-fungible token balance of an address - safe method (read-only)
        #[safe]
        pub fn nft_balance_of(&self, owner: H160) -> Int256 {
            self.nft_balances.get(&owner).unwrap_or_else(Int256::zero)
        }
        
        /// Get owner of a non-fungible token - safe method (read-only)
        #[safe]
        pub fn nft_owner_of(&self, token_id: ByteString) -> Option<H160> {
            self.nft_owner_of.get(&token_id)
        }
        
        /// Get tokens of an owner - safe method (read-only)
        #[safe]
        pub fn nft_tokens_of(&self, owner: H160) -> Array<ByteString> {
            self.nft_tokens_of.get(&owner).unwrap_or_else(Array::new)
        }
        
        /// Get token data - safe method (read-only)
        #[safe]
        pub fn nft_token_data(&self, token_id: ByteString) -> Option<Map> {
            let token = self.nft_tokens.get(&token_id)?;
            
            let mut data = Map::new();
            data.set("name", token.name);
            data.set("description", token.description);
            data.set("image", token.image);
            data.set("owner", token.owner);
            
            // Add any custom properties
            if let Some(props) = token.properties {
                data.set("properties", props);
            }
            
            Some(data)
        }
        
        /// Get properties of a token - safe method (read-only)
        #[safe]
        pub fn nft_properties(&self, token_id: ByteString) -> Option<Map> {
            let token = self.nft_tokens.get(&token_id)?;
            token.properties
        }
        
        /// Transfer a non-fungible token from one address to another
        #[method]
        pub fn nft_transfer(&mut self, from: H160, to: H160, token_id: ByteString) -> bool {
            // Get current owner
            let token_owner = match self.nft_owner_of(token_id.clone()) {
                Some(owner) => owner,
                None => return false, // Token doesn't exist
            };
            
            // Check if from is the current owner
            if token_owner != from {
                return false;
            }
            
            // Check if the caller is authorized
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check if the recipient is a valid address
            if to == H160::zero() {
                return false;
            }
            
            // Remove token from current owner's collection
            let mut from_tokens = self.nft_tokens_of(from);
            let mut new_from_tokens = Array::<ByteString>::new();
            let mut found = false;
            
            for token in from_tokens.iter() {
                if token == &token_id {
                    found = true;
                } else {
                    new_from_tokens.push(token.clone());
                }
            }
            
            if !found {
                return false;
            }
            
            self.nft_tokens_of.insert(&from, &new_from_tokens);
            
            // Update balances
            let from_balance = self.nft_balance_of(from);
            let new_from_balance = from_balance - Int256::one();
            
            if new_from_balance.is_zero() {
                self.nft_balances.delete(&from);
            } else {
                self.nft_balances.insert(&from, &new_from_balance);
            }
            
            let to_balance = self.nft_balance_of(to);
            let new_to_balance = to_balance + Int256::one();
            self.nft_balances.insert(&to, &new_to_balance);
            
            // Add token to new owner's collection
            let mut to_tokens = self.nft_tokens_of(to);
            to_tokens.push(token_id.clone());
            self.nft_tokens_of.insert(&to, &to_tokens);
            
            // Update token ownership
            self.nft_owner_of.insert(&token_id, &to);
            
            // Update token data
            if let Some(mut token_data) = self.nft_tokens.get(&token_id) {
                token_data.owner = to;
                self.nft_tokens.insert(&token_id, &token_data);
            }
            
            // Emit transfer event
            self.emit_nft_transfer(Some(from), Some(to), token_id, Int256::one());
            
            true
        }
        
        /// Mint a new non-fungible token - only contract owner can mint
        #[method]
        pub fn nft_mint(&mut self, to: H160, token_id: ByteString, name: String, description: String, image: String, properties: Option<Map>) -> bool {
            // Check if caller is the contract owner
            let owner = self.get_owner();
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Check if the token already exists
            if self.nft_owner_of(token_id.clone()).is_some() {
                return false;
            }
            
            // Create token data
            let token = TokenData {
                owner: to,
                name,
                description,
                image,
                properties: properties.unwrap_or_else(Map::new),
            };
            
            // Store token data
            self.nft_tokens.insert(&token_id, &token);
            
            // Update ownership
            self.nft_owner_of.insert(&token_id, &to);
            
            // Update token collection for the owner
            let mut tokens = self.nft_tokens_of(to);
            tokens.push(token_id.clone());
            self.nft_tokens_of.insert(&to, &tokens);
            
            // Update balances
            let balance = self.nft_balance_of(to);
            let new_balance = balance + Int256::one();
            self.nft_balances.insert(&to, &new_balance);
            
            // Update total supply
            let current_supply = self.nft_total_supply();
            let new_supply = current_supply + Int256::one();
            self.nft_total_supply.set(new_supply);
            
            // Emit transfer event (mint is a transfer from null address)
            self.emit_nft_transfer(None, Some(to), token_id, Int256::one());
            
            true
        }
        
        /// Burn a non-fungible token - only token owner can burn
        #[method]
        pub fn nft_burn(&mut self, token_id: ByteString) -> bool {
            // Get current owner
            let owner = match self.nft_owner_of(token_id.clone()) {
                Some(o) => o,
                None => return false, // Token doesn't exist
            };
            
            // Check if the caller is the token owner
            if !Runtime::check_witness(&owner) {
                return false;
            }
            
            // Remove token from owner's collection
            let mut tokens = self.nft_tokens_of(owner);
            let mut new_tokens = Array::<ByteString>::new();
            let mut found = false;
            
            for token in tokens.iter() {
                if token == &token_id {
                    found = true;
                } else {
                    new_tokens.push(token.clone());
                }
            }
            
            if !found {
                return false;
            }
            
            self.nft_tokens_of.insert(&owner, &new_tokens);
            
            // Update balances
            let balance = self.nft_balance_of(owner);
            let new_balance = balance - Int256::one();
            
            if new_balance.is_zero() {
                self.nft_balances.delete(&owner);
            } else {
                self.nft_balances.insert(&owner, &new_balance);
            }
            
            // Remove ownership record
            self.nft_owner_of.delete(&token_id);
            
            // Remove token data
            self.nft_tokens.delete(&token_id);
            
            // Update total supply
            let current_supply = self.nft_total_supply();
            let new_supply = current_supply - Int256::one();
            self.nft_total_supply.set(new_supply);
            
            // Emit transfer event (burn is a transfer to null address)
            self.emit_nft_transfer(Some(owner), None, token_id, Int256::one());
            
            true
        }
        
        /// Emit a NEP-11 Transfer event following Neo N3 standards
        #[safe]
        fn emit_nft_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString, amount: Int256) {
            // Emit event using the NFTTransfer struct
            NFTTransfer::emit(from, to, token_id, amount);
        }
        
        //======== Hybrid Operations ========//
        
        /// Convert fungible tokens to a non-fungible token
        /// This demonstrates how FT and NFT can interact in a hybrid contract
        #[method]
        pub fn convert_ft_to_nft(&mut self, from: H160, token_id: ByteString, name: String, description: String, image: String, ft_amount: Int256) -> bool {
            // Check authorization
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check if token ID already exists
            if self.nft_owner_of(token_id.clone()).is_some() {
                return false;
            }
            
            // Check if user has enough fungible tokens
            let from_balance = self.ft_balance_of(from);
            if from_balance < ft_amount {
                return false;
            }
            
            // Burn fungible tokens
            if !self.ft_burn(from, ft_amount) {
                return false;
            }
            
            // Create NFT properties with record of conversion
            let mut properties = Map::new();
            properties.set("converted_from_ft", ft_amount);
            properties.set("conversion_timestamp", Runtime::get_time());
            
            // Mint the NFT
            if !self.nft_mint(from, token_id, name, description, image, Some(properties)) {
                // This should never happen if the code is correct, but we handle it anyway
                // We would need to refund the FT in a production contract
                return false;
            }
            
            true
        }
        
        /// Fractionalize an NFT into fungible tokens
        /// This demonstrates another hybrid interaction between NFT and FT
        #[method]
        pub fn fractionalize_nft(&mut self, from: H160, token_id: ByteString, ft_amount: Int256) -> bool {
            // Check if token exists and is owned by from
            match self.nft_owner_of(token_id.clone()) {
                Some(owner) if owner == from => {},
                _ => return false,
            }
            
            // Check authorization
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check amount is positive
            if ft_amount <= Int256::zero() {
                return false;
            }
            
            // Burn the NFT
            if !self.nft_burn(token_id.clone()) {
                return false;
            }
            
            // Mint equivalent fungible tokens
            if !self.ft_mint(from, ft_amount) {
                // This should never happen if the code is correct
                return false;
            }
            
            true
        }
    }
    
    // Interface implementation for NEP-17 standard
    #[neo_contract]
    impl NEP17 for HybridToken {
        #[method]
        fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            self.ft_transfer(from, to, amount)
        }
        
        #[safe]
        fn balance_of(&self, owner: H160) -> Int256 {
            self.ft_balance_of(owner)
        }
        
        #[safe]
        fn total_supply(&self) -> Int256 {
            self.ft_total_supply()
        }
    }
    
    // Interface implementation for NEP-11 standard
    #[neo_contract]
    impl NEP11 for HybridToken {
        #[method]
        fn transfer(&mut self, to: H160, token_id: ByteString, data: Option<Any>) -> bool {
            // If data is provided, it could be handled here for the receiving contract
            // We ignore it in this example
            if let Some(token_owner) = self.nft_owner_of(token_id.clone()) {
                self.nft_transfer(token_owner, to, token_id)
            } else {
                false
            }
        }
        
        #[safe]
        fn balance_of(&self, owner: H160) -> Int256 {
            self.nft_balance_of(owner)
        }
        
        #[safe]
        fn owner_of(&self, token_id: ByteString) -> Option<H160> {
            self.nft_owner_of(token_id)
        }
        
        #[safe]
        fn tokens_of(&self, owner: H160) -> Array<ByteString> {
            self.nft_tokens_of(owner)
        }
    }
}

// Implement the necessary methods for deployment
#[cfg(target_arch = "wasm32")]
mod deployment {
    use super::*;
    
    // Needed for no_std compatibility with wasm32 target
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        core::intrinsics::abort()
    }
    
    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! {
        core::intrinsics::abort()
    }
    
    #[no_mangle]
    pub extern "C" fn _start() {}
    
    #[lang = "eh_personality"]
    fn eh_personality() {}
}
