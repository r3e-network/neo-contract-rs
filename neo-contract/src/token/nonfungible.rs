// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Non-fungible token implementation for the Neo blockchain

use alloc::string::String;
use crate::builtin::{H160, ByteString, Int256, Array, Any};
use crate::storage::{StorageMap, Storable};
use crate::Runtime;
use crate::error::{Error, ErrorCode, Result};

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
    fn tokens_of(&self, owner: H160) -> Array<ByteString>;
    
    /// Get token metadata
    fn token_metadata(&self, token_id: ByteString) -> ByteString;
    
    /// Transfer a token from the sender to a recipient
    fn transfer(&self, to: H160, token_id: ByteString) -> bool;
    
    /// Transfer a token from one account to another
    fn transfer_from(&self, from: H160, to: H160, token_id: ByteString) -> bool;
}

/// NFT events
pub trait NonFungibleTokenEvents {
    /// Emit a transfer event
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString);
}

/// Implementation of NFT events
impl NonFungibleTokenEvents for () {
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, token_id: ByteString) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(token_id));
        
        Runtime::notify(&event_name, &event_data);
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
    owner_tokens: StorageMap<H160, Array<ByteString>>,
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
            owner_tokens: StorageMap::<H160, Array<ByteString>>::new(b"ownerTokens"),
            owner: StorageMap::<ByteString, H160>::new(b"owner"),
            total_supply: StorageMap::<ByteString, Int256>::new(b"totalSupply"),
        }
    }
    
    /// Initialize the NFT contract with an owner
    pub fn initialize(&self, owner: H160) -> Result<()> {
        // Check if already initialized
        if self.owner.get(&ByteString::from("owner")).is_some() {
            return Err(Error::new(
                ErrorCode::AlreadyExists,
                "NFT already initialized"
            ));
        }
        
        // Set owner
        self.owner.put(&ByteString::from("owner"), &owner);
        
        // Initialize total supply
        self.total_supply.put(&ByteString::from("value"), &Int256::zero());
        
        Ok(())
    }
    
    /// Mint a new token to an account
    pub fn mint(&self, to: &H160, token_id: ByteString, token_metadata: ByteString) -> Result<()> {
        // Check only owner can mint
        let owner = self.get_owner();
        if !Runtime::check_witness(&owner) {
            return Err(Error::new(
                ErrorCode::Unauthorized,
                "Only owner can mint"
            ));
        }
        
        // Check token doesn't already exist
        if self.token_owners.get(&token_id).is_some() {
            return Err(Error::new(
                ErrorCode::AlreadyExists,
                "Token already exists"
            ));
        }
        
        // Set token owner
        self.token_owners.put(&token_id, to);
        
        // Set token metadata
        self.token_metadata.put(&token_id, &token_metadata);
        
        // Add token to owner's tokens
        let mut owner_tokens = self.owner_tokens.get(to).unwrap_or_else(Array::<ByteString>::new);
        owner_tokens.push(token_id.clone());
        self.owner_tokens.put(to, &owner_tokens);
        
        // Update total supply
        let total_supply = self.total_supply();
        self.total_supply.put(&ByteString::from("value"), &(total_supply + Int256::one()));
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), None, Some(to.clone()), token_id);
        
        Ok(())
    }
    
    /// Burn a token
    pub fn burn(&self, token_id: ByteString) -> Result<()> {
        // Get token owner
        let owner = match self.token_owners.get(&token_id) {
            Some(owner) => owner,
            None => {
                return Err(Error::new(
                    ErrorCode::NotFound,
                    "Token not found"
                ));
            }
        };
        
        // Check authorization
        if !Runtime::check_witness(&owner) {
            return Err(Error::new(
                ErrorCode::Unauthorized,
                "Not authorized to burn"
            ));
        }
        
        // Remove token owner
        self.token_owners.delete(&token_id);
        
        // Remove token metadata
        self.token_metadata.delete(&token_id);
        
        // Remove token from owner's tokens
        let mut owner_tokens = self.owner_tokens.get(&owner).unwrap_or_else(Array::<ByteString>::new);
        let mut index = 0;
        while index < owner_tokens.len() {
            if owner_tokens.get(index).unwrap() == token_id {
                owner_tokens.remove(index);
                break;
            }
            index += 1;
        }
        self.owner_tokens.put(&owner, &owner_tokens);
        
        // Update total supply
        let total_supply = self.total_supply();
        self.total_supply.put(&ByteString::from("value"), &(total_supply - Int256::one()));
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), Some(owner), None, token_id);
        
        Ok(())
    }
    
    /// Update contract parameters
    pub fn update(&self, script: ByteString, manifest: ByteString, data: Any) -> bool {
        // Check only owner can update
        let owner = self.get_owner();
        if !Runtime::check_witness(&owner) {
            return false;
        }
        
        Runtime::update(script, manifest, data)
    }
    
    /// Get the owner of the NFT contract
    pub fn get_owner(&self) -> H160 {
        self.owner.get(&ByteString::from("owner")).unwrap_or_else(H160::zero)
    }
    
    /// Set a new owner for the NFT contract
    pub fn set_owner(&self, new_owner: H160) -> bool {
        // Check if caller is current owner
        let current_owner = self.get_owner();
        if !Runtime::check_witness(&current_owner) {
            return false;
        }
        
        // Set new owner
        self.owner.put(&ByteString::from("owner"), &new_owner);
        
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
        self.total_supply.get(&ByteString::from("value")).unwrap_or_else(Int256::zero)
    }
    
    fn owner_of(&self, token_id: ByteString) -> H160 {
        self.token_owners.get(&token_id).unwrap_or_else(H160::zero)
    }
    
    fn tokens_of(&self, owner: H160) -> Array<ByteString> {
        self.owner_tokens.get(&owner).unwrap_or_else(Array::<ByteString>::new)
    }
    
    fn token_metadata(&self, token_id: ByteString) -> ByteString {
        self.token_metadata.get(&token_id).unwrap_or_else(ByteString::default)
    }
    
    fn transfer(&self, to: H160, token_id: ByteString) -> bool {
        // Get sender
        let sender = Runtime::calling_script_hash();
        
        // Check authorization
        if !Runtime::check_witness(&sender) {
            return false;
        }
        
        // Check token exists and sender is owner
        let owner = match self.token_owners.get(&token_id) {
            Some(owner) => owner,
            None => return false,
        };
        
        if owner != sender {
            return false;
        }
        
        // Handle sender equals to recipient
        if sender == to {
            return true;
        }
        
        // Update token owner
        self.token_owners.put(&token_id, &to);
        
        // Remove token from sender's tokens
        let mut sender_tokens = self.owner_tokens.get(&sender).unwrap_or_else(Array::<ByteString>::new);
        let mut index = 0;
        while index < sender_tokens.len() {
            if sender_tokens.get(index).unwrap() == token_id {
                sender_tokens.remove(index);
                break;
            }
            index += 1;
        }
        self.owner_tokens.put(&sender, &sender_tokens);
        
        // Add token to recipient's tokens
        let mut recipient_tokens = self.owner_tokens.get(&to).unwrap_or_else(Array::<ByteString>::new);
        recipient_tokens.push(token_id.clone());
        self.owner_tokens.put(&to, &recipient_tokens);
        
        // Handle on token received notification for contracts
        self.on_nft_received(&to, &sender, &token_id);
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), Some(sender), Some(to), token_id);
        
        true
    }
    
    fn transfer_from(&self, from: H160, to: H160, token_id: ByteString) -> bool {
        // Check authorization
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        // Check token exists and from is owner
        let owner = match self.token_owners.get(&token_id) {
            Some(owner) => owner,
            None => return false,
        };
        
        if owner != from {
            return false;
        }
        
        // Handle from equals to recipient
        if from == to {
            return true;
        }
        
        // Update token owner
        self.token_owners.put(&token_id, &to);
        
        // Remove token from sender's tokens
        let mut sender_tokens = self.owner_tokens.get(&from).unwrap_or_else(Array::<ByteString>::new);
        let mut index = 0;
        while index < sender_tokens.len() {
            if sender_tokens.get(index).unwrap() == token_id {
                sender_tokens.remove(index);
                break;
            }
            index += 1;
        }
        self.owner_tokens.put(&from, &sender_tokens);
        
        // Add token to recipient's tokens
        let mut recipient_tokens = self.owner_tokens.get(&to).unwrap_or_else(Array::<ByteString>::new);
        recipient_tokens.push(token_id.clone());
        self.owner_tokens.put(&to, &recipient_tokens);
        
        // Handle on token received notification for contracts
        self.on_nft_received(&to, &from, &token_id);
        
        // Emit transfer event
        NonFungibleTokenEvents::emit_transfer(&(), Some(from), Some(to), token_id);
        
        true
    }
}

impl NFT {
    /// Handle NFT token received notification for contracts
    fn on_nft_received(&self, to: &H160, from: &H160, token_id: &ByteString) -> bool {
        // Check if recipient is a contract
        if Runtime::is_contract(to) {
            // Try to call onNFTReceived method on receiving contract
            let method = ByteString::from("onNFTReceived");
            let mut args = Array::<Any>::new();
            
            args.push(Any::from(from.clone()));
            args.push(Any::from(token_id.clone()));
            args.push(Any::from(Any::new()));  // data parameter
            
            // Call the contract, ignoring any errors
            let _ = Runtime::call_contract(
                to.clone(),
                method,
                args
            );
        }
        
        true
    }
}
