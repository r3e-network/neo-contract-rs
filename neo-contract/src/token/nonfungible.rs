// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Non-fungible token implementation for the Neo blockchain

use alloc::string::String;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::array::Array;
use crate::types::builtin::any::Any;
use crate::storage::map::Map as StorageMap;
use crate::env;
use crate::error::{Error, ErrorCode, Result};

// Local runtime module that imports the necessary functions
mod runtime {
    use super::*;
    use crate::env;
    
    pub fn check_witness(hash: &H160) -> bool {
        env::runtime::check_witness(hash)
    }
    
    pub fn calling_script_hash() -> H160 {
        env::runtime::calling_script_hash()
    }
    
    pub fn notify(event_name: &str, args: &[u8]) {
        env::runtime::notify(event_name, args)
    }
    
    // Placeholder functions that would need to be implemented properly
    pub fn is_contract(_hash: &H160) -> bool {
        // This is a placeholder - in a real implementation, this would check if the hash is a contract
        true
    }
    
    pub fn call_contract(_hash: H160, _method: ByteString, _args: Array) -> bool {
        // This is a placeholder - in a real implementation, this would call the contract
        true
    }
    
    pub fn update(_script: ByteString, _manifest: ByteString, _data: Any) -> bool {
        // This is a placeholder - in a real implementation, this would update the contract
        true
    }
}

/// Standard non-fungible token interface
pub trait NonFungibleToken {
    /// Get the name of the token collection
    fn name(&self) -> ByteString;
    
    /// Get the symbol of the token collection
    fn symbol(&self) -> ByteString;
    
    /// Get the total supply of tokens in the collection
    fn total_supply(&self) -> Int256;
    
    /// Get the owner of a specific token
    fn owner_of(&self, token_id: ByteString) -> H160;
    
    /// Get all tokens owned by an account
    fn tokens_of(&self, owner: H160) -> Array;
    
    /// Get token metadata
    fn token_metadata(&self, token_id: ByteString) -> ByteString;
    
    /// Transfer a token from the sender to a recipient
    fn transfer(&self, to: H160, token_id: ByteString) -> bool;
    
    /// Transfer a token from one account to another
    fn transfer_from(&self, from: H160, to: H160, token_id: ByteString) -> bool;
}

/// Events emitted by non-fungible token contracts
pub trait NonFungibleTokenEvents {
    /// Emit a transfer event
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString);
}

/// Implementation of NFT events
impl NonFungibleTokenEvents for () {
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString) {
        let event_name = "Transfer";
        let mut event_data = Array::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::null()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::null()),
        }
        
        event_data.push(Any::from(token_id));
        
        // In a complete implementation, this would call a proper notify function
        let _ = env::runtime::notify(event_name, &[]);
    }
}

/// NFT token implementation
pub struct NFT {
    /// Token metadata
    metadata: NFTMetadata,
    /// Token owners storage map (token_id -> owner)
    token_owners: StorageMap<ByteString, H160>,
    /// Token metadata storage map (token_id -> metadata)
    token_metadata: StorageMap<ByteString, ByteString>,
    /// Owner tokens storage map (owner -> token_ids)
    owner_tokens: StorageMap<H160, Array>,
    /// Owner of the token contract
    owner: StorageMap<ByteString, H160>,
    /// Total supply of tokens
    total_supply: StorageMap<ByteString, Int256>,
}

/// NFT metadata
pub struct NFTMetadata {
    /// Name of the token collection
    pub name: ByteString,
    /// Symbol of the token collection
    pub symbol: ByteString,
}

impl NFT {
    /// Create a new NFT contract
    pub fn new(
        name: &str,
        symbol: &str,
    ) -> Self {
        let name_bytes = ByteString::from(name);
        let symbol_bytes = ByteString::from(symbol);
        
        Self {
            metadata: NFTMetadata {
                name: name_bytes,
                symbol: symbol_bytes,
            },
            token_owners: StorageMap::<ByteString, H160>::new(b"tokenOwners"),
            token_metadata: StorageMap::<ByteString, ByteString>::new(b"tokenMetadata"),
            owner_tokens: StorageMap::<H160, Array>::new(b"ownerTokens"),
            owner: StorageMap::<ByteString, H160>::new(b"owner"),
            total_supply: StorageMap::<ByteString, Int256>::new(b"totalSupply"),
        }
    }
    
    /// Initialize the NFT contract with an owner
    pub fn initialize(&self, owner: H160) -> Result<()> {
        // Check if already initialized
        if self.owner.has(&ByteString::from("owner")) {
            return Err(Error::with_message(
                ErrorCode::InvalidState,
                "Token already initialized"
            ));
        }
        
        // Set owner
        self.owner.set(&ByteString::from("owner"), &owner)?;
        
        // Initialize total supply
        self.total_supply.set(&ByteString::from("totalSupply"), &Int256::from(0))?;
        
        Ok(())
    }
    
    /// Mint a new NFT token
    pub fn mint(&self, to: &H160, token_id: ByteString, token_metadata: ByteString) -> Result<()> {
        // Check if caller is owner
        let owner = match self.owner.get(&ByteString::from("owner"))? {
            Some(o) => o,
            None => return Err(Error::with_message(
                ErrorCode::NotFound,
                "Contract not initialized"
            ))
        };
        
        // Ensure caller is the owner
        if !runtime::check_witness(&owner) {
            return Err(Error::with_message(
                ErrorCode::Unauthorized,
                "Only owner can mint tokens"
            ));
        }
        
        // Check token doesn't already exist
        if self.token_owners.has(&token_id) {
            return Err(Error::with_message(
                ErrorCode::InvalidState,
                "Token already exists"
            ));
        }
        
        // Set token owner
        self.token_owners.set(&token_id, to)?;
        
        // Set token metadata
        self.token_metadata.set(&token_id, &token_metadata)?;
        
        // Add token to owner's tokens
        let mut owner_tokens = match self.owner_tokens.get(to)? {
            Some(tokens) => tokens,
            None => Array::new()
        };
        owner_tokens.push(token_id.clone());
        self.owner_tokens.set(to, &owner_tokens)?;
        
        // Update total supply
        let total_supply = self.total_supply();
        self.total_supply.set(&ByteString::from("value"), &(total_supply + Int256::from(1)))?;
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), None, Some(to.clone()), token_id);
        
        Ok(())
    }
    
    /// Burn a token
    pub fn burn(&self, token_id: ByteString) -> Result<()> {
        // Get token owner
        let owner = match self.token_owners.get(&token_id)? {
            Some(owner) => owner,
            None => {
                return Err(Error::with_message(
                    ErrorCode::NotFound,
                    "Token does not exist"
                ));
            }
        };
        
        // Check if caller is the owner
        if !runtime::check_witness(&owner) {
            return Err(Error::with_message(
                ErrorCode::Unauthorized,
                "Only token owner can burn"
            ));
        }
        
        // Remove token owner
        self.token_owners.delete(&token_id)?;
        
        // Remove token metadata
        self.token_metadata.delete(&token_id)?;
        
        // Remove token from owner's tokens
        let mut owner_tokens = match self.owner_tokens.get(&owner)? {
            Some(tokens) => tokens,
            None => Array::new()
        };
        
        // Find and remove the token from the owner's tokens
        let mut index = 0;
        let mut found = false;
        
        while index < owner_tokens.0.len() {
            if let Some(Any::ByteString(token)) = owner_tokens.0.get(index) {
                if token == &token_id {
                    owner_tokens.0.remove(index);
                    found = true;
                    break;
                }
            }
            index += 1;
        }
        
        if found {
            self.owner_tokens.set(&owner, &owner_tokens)?;
            
            // Update total supply
            let total_supply = self.total_supply();
            self.total_supply.set(&ByteString::from("value"), &(total_supply - Int256::from(1)))?;
            
            // Emit transfer event
            NonFungibleTokenEvents::emit_transfer(&(), Some(owner), None, token_id);
            
            Ok(())
        } else {
            Err(Error::with_message(
                ErrorCode::NotFound,
                "Token not found in owner's tokens"
            ))
        }
    }
    
    /// Update contract parameters
    pub fn update(&self, script: ByteString, manifest: ByteString, data: Any) -> bool {
        // Check only owner can update
        let owner = self.get_owner();
        if !runtime::check_witness(&owner) {
            return false;
        }
        
        runtime::update(script, manifest, data)
    }
    
    /// Get the owner of the NFT contract
    pub fn get_owner(&self) -> H160 {
        match self.owner.get(&ByteString::from("owner")) {
            Ok(Some(owner)) => owner,
            _ => H160::zero(),
        }
    }
    
    /// Set a new owner for the NFT contract
    pub fn set_owner(&self, new_owner: H160) -> bool {
        // Check if caller is current owner
        let current_owner = self.get_owner();
        if !env::runtime::check_witness(&current_owner) {
            return false;
        }
        
        // Set new owner
        if let Err(_) = self.owner.set(&ByteString::from("owner"), &new_owner) {
            return false;
        }
        
        true
    }
}

impl NonFungibleToken for NFT {
    fn name(&self) -> ByteString {
        self.metadata.name.clone()
    }
    
    fn symbol(&self) -> ByteString {
        self.metadata.symbol.clone()
    }
    
    fn total_supply(&self) -> Int256 {
        // Since we've implemented the Codec trait, use unwrap_or with a default value
        match self.total_supply.get(&ByteString::from("value")) {
            Ok(Some(value)) => value,
            _ => Int256::from(0)
        }
    }
    
    fn owner_of(&self, token_id: ByteString) -> H160 {
        // Since we've implemented the Codec trait, use unwrap_or with a default value
        match self.token_owners.get(&token_id) {
            Ok(Some(owner)) => owner,
            _ => H160::zero()
        }
    }
    
    fn tokens_of(&self, owner: H160) -> Array {
        // Since we've implemented the Codec trait, use unwrap_or with a default value
        match self.owner_tokens.get(&owner) {
            Ok(Some(tokens)) => tokens,
            _ => Array::new()
        }
    }
    
    fn token_metadata(&self, token_id: ByteString) -> ByteString {
        // Since we've implemented the Codec trait, use unwrap_or with a default value
        match self.token_metadata.get(&token_id) {
            Ok(Some(metadata)) => metadata,
            _ => ByteString::default()
        }
    }
    
    fn transfer(&self, to: H160, token_id: ByteString) -> bool {
        // Get sender
        let sender = env::runtime::calling_script_hash();
        
        // Check authorization
        if !env::runtime::check_witness(&sender) {
            return false;
        }
        
        // Check token exists and sender is owner
        let owner = match self.token_owners.get(&token_id) {
            Ok(Some(owner)) => owner,
            _ => return false,
        };
        
        if owner != sender {
            return false;
        }
        
        // Handle sender equals to recipient
        if sender == to {
            return true;
        }
        
        // Update token owner
        if let Err(_) = self.token_owners.set(&token_id, &to) {
            return false;
        }
        
        // Remove token from sender's tokens
        let mut sender_tokens = match self.owner_tokens.get(&sender) {
            Ok(Some(tokens)) => tokens,
            _ => Array::new(),
        };
        
        // Find and remove the token from sender's tokens
        let mut index = 0;
        let mut found = false;
        
        while index < sender_tokens.0.len() {
            if let Some(Any::ByteString(token)) = sender_tokens.0.get(index) {
                if token == &token_id {
                    sender_tokens.0.remove(index);
                    found = true;
                    break;
                }
            }
            index += 1;
        }
        
        if found {
            if let Err(_) = self.owner_tokens.set(&sender, &sender_tokens) {
                return false;
            }
        }
        
        // Add token to recipient's tokens
        let mut recipient_tokens = match self.owner_tokens.get(&to) {
            Ok(Some(tokens)) => tokens,
            _ => Array::new(),
        };
        
        recipient_tokens.push(token_id.clone());
        
        // Store updated tokens list
        if let Err(_) = self.owner_tokens.set(&to, &recipient_tokens) {
            return false;
        }
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), Some(sender), Some(to), token_id);
        
        true
    }
    
    fn transfer_from(&self, from: H160, to: H160, token_id: ByteString) -> bool {
        // Get caller
        let caller = env::runtime::calling_script_hash();
        
        // Check authorization
        if !env::runtime::check_witness(&caller) {
            return false;
        }
        
        // Check token exists and from is owner
        let owner = match self.token_owners.get(&token_id) {
            Ok(Some(owner)) => owner,
            _ => return false,
        };
        
        if owner != from {
            return false;
        }
        
        // Handle from equals to recipient
        if from == to {
            return true;
        }
        
        // Update token owner
        if let Err(_) = self.token_owners.set(&token_id, &to) {
            return false;
        }
        
        // Remove token from sender's tokens
        let mut sender_tokens = match self.owner_tokens.get(&from) {
            Ok(Some(tokens)) => tokens,
            _ => Array::new(),
        };
        
        // Find and remove the token from sender's tokens
        let mut index = 0;
        let mut found = false;
        
        while index < sender_tokens.0.len() {
            if let Some(Any::ByteString(token)) = sender_tokens.0.get(index) {
                if token == &token_id {
                    sender_tokens.0.remove(index);
                    found = true;
                    break;
                }
            }
            index += 1;
        }
        
        if found {
            if let Err(_) = self.owner_tokens.set(&from, &sender_tokens) {
                return false;
            }
        }
        
        // Add token to recipient's tokens
        let mut recipient_tokens = match self.owner_tokens.get(&to) {
            Ok(Some(tokens)) => tokens,
            _ => Array::new(),
        };
        
        recipient_tokens.push(token_id.clone());
        
        // Store updated tokens list
        if let Err(_) = self.owner_tokens.set(&to, &recipient_tokens) {
            return false;
        }
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), Some(from), Some(to), token_id);
        
        true
    }
}

impl NFT {
    /// Handle NFT token received notification for contracts
    fn on_nft_received(&self, to: &H160, from: &H160, token_id: &ByteString) -> bool {
        // Check if recipient is a contract
        if runtime::is_contract(to) {
            // Try to call onNFTReceived method on receiving contract
            let method = ByteString::from("onNFTReceived");
            let mut args = Array::new();
            args.push(Any::from(from.clone()));
            args.push(Any::from(token_id.clone()));
            
            // Call the contract, ignoring any errors
            let _ = runtime::call_contract(
                to.clone(),
                method,
                args
            );
        }
        
        true
    }
}
