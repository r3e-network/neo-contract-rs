#![no_std]
#![cfg_attr(target_arch = "wasm32", feature(core_intrinsics, lang_items))]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use neo_contract::prelude::*;

/// NEP-11 compliant Non-Fungible Token implementation for Neo N3
/// This example showcases best practices for Neo N3 contract development in Rust
/// Updated to follow ink! style contract syntax
#[contract]
mod nep11_nft {
    use super::*;

    // Define events
    pub struct Transfer {
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
    const TOKEN_PREFIX: &[u8] = b"token:";
    const OWNER_OF_PREFIX: &[u8] = b"owner_of:";
    const BALANCE_PREFIX: &[u8] = b"balance:";
    const TOKENS_OF_PREFIX: &[u8] = b"tokens_of:";
    const TOTAL_SUPPLY_KEY: &[u8] = b"total_supply";
    const TOTAL_TOKENS_KEY: &[u8] = b"total_tokens";

    // Metadata keys
    const PROPERTIES_PREFIX: &[u8] = b"props:";

    // Token struct for storing NFT data
    struct TokenData {
        owner: H160,
        name: String,
        description: String,
        image: String,
        properties: Map,
    }

    // Contract storage structure
    #[storage]
    struct NFTContract {
        // Contract metadata
        owner: StorageItem<H160>,
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        decimals: StorageItem<u8>,
        total_supply: StorageItem<Int256>,
        total_tokens: StorageItem<Int256>,

        // Token data collections - using StorageMap for complex structures
        tokens: StorageMap<ByteString, TokenData>,

        // Ownership collections
        owner_of: StorageMap<ByteString, H160>,
        balances: StorageMap<H160, Int256>,
        tokens_of: StorageMap<H160, Array<ByteString>>,
    }

    impl NFTContract {
        /// Initialize a new NFT contract
        #[constructor]
        pub fn new(owner: H160, name: String, symbol: String) -> Self {
            Self {
                owner: StorageItem::new(OWNER_KEY).with_data(owner),
                name: StorageItem::new(NAME_KEY).with_data(name),
                symbol: StorageItem::new(SYMBOL_KEY).with_data(symbol),
                decimals: StorageItem::new(DECIMALS_KEY).with_data(0u8), // NFTs use 0 decimals
                total_supply: StorageItem::new(TOTAL_SUPPLY_KEY).with_data(Int256::zero()),
                total_tokens: StorageItem::new(TOTAL_TOKENS_KEY).with_data(Int256::zero()),

                tokens: StorageMap::new(TOKEN_PREFIX),
                owner_of: StorageMap::new(OWNER_OF_PREFIX),
                balances: StorageMap::new(BALANCE_PREFIX),
                tokens_of: StorageMap::new(TOKENS_OF_PREFIX),
            }
        }

        /// Get contract owner - safe method (read-only)
        #[safe]
        pub fn get_owner(&self) -> H160 { self.owner.get().unwrap_or_default() }

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

        /// Get token name - safe method (read-only)
        #[safe]
        pub fn name(&self) -> String { self.name.get().unwrap_or_else(|| "Neo NFT".to_string()) }

        /// Get token symbol - safe method (read-only)
        #[safe]
        pub fn symbol(&self) -> String { self.symbol.get().unwrap_or_else(|| "NNFT".to_string()) }

        /// Get token decimals - safe method (read-only)
        #[safe]
        pub fn decimals(&self) -> u8 { self.decimals.get().unwrap_or(0) }

        /// Get total supply - safe method (read-only)
        #[safe]
        pub fn total_supply(&self) -> Int256 { self.total_supply.get().unwrap_or_else(Int256::zero) }

        /// Get total tokens - safe method (read-only)
        #[safe]
        pub fn total_tokens(&self) -> Int256 { self.total_tokens.get().unwrap_or_else(Int256::zero) }

        /// Get balance of an address - safe method (read-only)
        #[safe]
        pub fn balance_of(&self, owner: H160) -> Int256 { self.balances.get(&owner).unwrap_or_else(Int256::zero) }

        /// Get owner of a token - safe method (read-only)
        #[safe]
        pub fn owner_of(&self, token_id: ByteString) -> Option<H160> { self.owner_of.get(&token_id) }

        /// Get tokens of an owner - safe method (read-only)
        #[safe]
        pub fn tokens_of(&self, owner: H160) -> Array<ByteString> {
            self.tokens_of.get(&owner).unwrap_or_else(Array::new)
        }

        /// Get token data - safe method (read-only)
        #[safe]
        pub fn token_data(&self, token_id: ByteString) -> Option<Map> {
            let token = self.tokens.get(&token_id)?;

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
        pub fn properties(&self, token_id: ByteString) -> Option<Map> {
            let token = self.tokens.get(&token_id)?;
            token.properties
        }

        /// Internal method to emit a Transfer event following Neo N3 standards
        #[safe]
        fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString, amount: Int256) {
            // Create event name as ByteString
            let event_name = ByteString::from("Transfer");

            // Create an Array to hold parameters
            let mut event_data = Array::<Any>::new();

            // Add parameters as Any values, handling null values with Any::new()
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }

            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::new()),
            }

            event_data.push(Any::from(token_id));
            event_data.push(Any::from(amount));

            // Emit the event
            Runtime::notify(&event_name, &event_data);
        }

        /// Internal method to increment a balance
        #[method]
        fn increase_balance(&mut self, address: &H160, amount: Int256) {
            let current = self.balance_of(*address);
            self.balances.insert(address, &(current + amount));
        }

        /// Internal method to decrement a balance
        #[method]
        fn decrease_balance(&mut self, address: &H160, amount: Int256) -> bool {
            let current = self.balance_of(*address);
            if current < amount {
                return false;
            }

            self.balances.insert(address, &(current - amount));
            true
        }

        /// Add a token to the owner's tokens list
        #[method]
        fn add_token_to_owner(&mut self, owner: &H160, token_id: &ByteString) {
            let mut tokens = self.tokens_of(*owner);
            tokens.push(token_id.clone());
            self.tokens_of.insert(owner, &tokens);
        }

        /// Remove a token from the owner's tokens list
        #[method]
        fn remove_token_from_owner(&mut self, owner: &H160, token_id: &ByteString) -> bool {
            let mut tokens = self.tokens_of(*owner);

            // Find and remove the token
            let mut found = false;
            let mut new_tokens = Array::<ByteString>::new();

            for token in tokens.iter() {
                if token == token_id {
                    found = true;
                } else {
                    new_tokens.push(token.clone());
                }
            }

            if found {
                self.tokens_of.insert(owner, &new_tokens);
            }

            found
        }
    }

    // NEP-11 trait definition
    pub trait NEP11 {
        fn transfer(&mut self, to: H160, token_id: ByteString, data: Option<Any>) -> bool;
        fn balance_of(&self, owner: H160) -> Int256;
        fn owner_of(&self, token_id: ByteString) -> Option<H160>;
        fn tokens_of(&self, owner: H160) -> Array<ByteString>;
    }

    // NEP-11 standard methods implementation
    impl NEP11 for NFTContract {
        /// Transfer a token from one address to another
        #[method]
        fn transfer(&mut self, to: H160, token_id: ByteString, data: Option<Any>) -> bool {
            // Get current owner
            let from = match self.owner_of(token_id.clone()) {
                Some(owner) => owner,
                None => return false, // Token doesn't exist
            };

            // Check if the caller is the owner
            if !Runtime::check_witness(&from) {
                return false;
            }

            // Check if the recipient is a valid address
            if to == H160::zero() {
                return false;
            }

            // Remove token from current owner
            if !self.remove_token_from_owner(&from, &token_id) {
                return false;
            }

            // Update balances
            if !self.decrease_balance(&from, Int256::one()) {
                return false;
            }

            self.increase_balance(&to, Int256::one());

            // Add token to new owner
            self.add_token_to_owner(&to, &token_id);

            // Update token ownership
            self.owner_of.insert(&token_id, &to);

            // Update the token data
            if let Some(mut token_data) = self.tokens.get(&token_id) {
                token_data.owner = to;
                self.tokens.insert(&token_id, &token_data);
            }

            // Emit Transfer event
            self.emit_transfer(Some(from), Some(to), token_id, Int256::one());

            // If data is provided, you can handle it for the recipient contract
            // (omitted for simplicity but would be needed for full NEP-11 compliance)

            true
        }

        /// Get token balance for an owner
        #[safe]
        fn balance_of(&self, owner: H160) -> Int256 { self.balance_of(owner) }

        /// Get the owner of a token
        #[safe]
        fn owner_of(&self, token_id: ByteString) -> Option<H160> { self.owner_of(token_id) }

        /// Get all tokens owned by an address
        #[safe]
        fn tokens_of(&self, owner: H160) -> Array<ByteString> { self.tokens_of(owner) }
    }

    // Custom methods for our NFT contract
    impl NFTContract {
        /// Mint a new NFT - only contract owner can mint
        #[method]
        pub fn mint(
            &mut self,
            to: H160,
            token_id: ByteString,
            name: String,
            description: String,
            image: String,
            properties: Option<Map>,
        ) -> bool {
            // Check if the caller is the contract owner
            let owner = self.get_owner();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Check if the token already exists
            if self.owner_of(token_id.clone()).is_some() {
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
            self.tokens.insert(&token_id, &token);

            // Update ownership
            self.owner_of.insert(&token_id, &to);

            // Update token collection for the owner
            self.add_token_to_owner(&to, &token_id);

            // Update balances
            self.increase_balance(&to, Int256::one());

            // Update totals
            let current_supply = self.total_supply();
            self.total_supply.set(current_supply + Int256::one());

            let current_tokens = self.total_tokens();
            self.total_tokens.set(current_tokens + Int256::one());

            // Emit Transfer event (mint is a transfer from null address)
            self.emit_transfer(None, Some(to), token_id, Int256::one());

            true
        }

        /// Burn an NFT - only the owner of the token can burn it
        #[method]
        pub fn burn(&mut self, token_id: ByteString) -> bool {
            // Get current owner
            let owner = match self.owner_of(token_id.clone()) {
                Some(o) => o,
                None => return false, // Token doesn't exist
            };

            // Check if the caller is the token owner
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Remove token from owner's collection
            if !self.remove_token_from_owner(&owner, &token_id) {
                return false;
            }

            // Update balances
            if !self.decrease_balance(&owner, Int256::one()) {
                return false;
            }

            // Remove ownership record
            self.owner_of.delete(&token_id);

            // Remove token data
            self.tokens.delete(&token_id);

            // Update totals
            let current_supply = self.total_supply();
            self.total_supply.set(current_supply - Int256::one());

            // Note: We don't decrease total_tokens as it represents the total minted

            // Emit Transfer event (burn is a transfer to null address)
            self.emit_transfer(Some(owner), None, token_id, Int256::one());

            true
        }
    }
}

// Implement the necessary methods for deployment
#[cfg(target_arch = "wasm32")]
mod deployment {
    use super::*;

    // Needed for no_std compatibility with wasm32 target
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! { core::intrinsics::abort() }

    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! { core::intrinsics::abort() }

    #[no_mangle]
    pub extern "C" fn _start() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}
