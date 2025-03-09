// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Fungible token implementation for the Neo blockchain

use alloc::vec::Vec;
use alloc::string::String;

// Update to use prelude
use crate::prelude::{H160, ByteString, Int256, Array, Any, StorageMap};
use crate::policy::voting::Storable;  // Import Storable from the voting module
use crate::runtime::Runtime;
use crate::error::{Error, ErrorCode, Result};
use crate::token::{Token, TokenEvents};

/// Standard fungible token implementation
pub struct FungibleToken {
    /// Token metadata
    metadata: FungibleTokenMetadata,
    /// Balances storage map
    balances: StorageMap<H160, Int256>,
    /// Owner of the token contract
    owner: StorageMap<ByteString, H160>,
}

/// Token metadata
pub struct FungibleTokenMetadata {
    /// Name of the token
    pub name: ByteString,
    /// Symbol of the token
    pub symbol: ByteString,
    /// Number of decimals for the token
    pub decimals: u8,
    /// Total supply of the token
    pub total_supply: StorageMap<ByteString, Int256>,
}

impl FungibleToken {
    /// Create a new fungible token
    pub fn new(
        name: &str,
        symbol: &str,
        decimals: u8,
    ) -> Self {
        let name_bytes = ByteString::from(name);
        let symbol_bytes = ByteString::from(symbol);
        
        Self {
            metadata: FungibleTokenMetadata {
                name: name_bytes,
                symbol: symbol_bytes,
                decimals,
                total_supply: StorageMap::<ByteString, Int256>::new(b"totalSupply"),
            },
            balances: StorageMap::<H160, Int256>::new(b"balances"),
            owner: StorageMap::<ByteString, H160>::new(b"owner"),
        }
    }
    
    /// Initialize the token with an initial supply and owner
    pub fn initialize(&self, owner: H160, initial_supply: Int256) -> Result<()> {
        // Check if already initialized
        if let Ok(Some(_)) = self.owner.get(&ByteString::from("owner")) {
            return Err(Error::with_message(ErrorCode::InvalidState, "Token already initialized"));
        }
        
        // Set owner
        self.owner.set(&ByteString::from("owner"), &owner)?;
        
        // Mint initial supply to owner
        if initial_supply > Int256::zero() {
            self.mint(&owner, initial_supply)?;
        }
        
        Ok(())
    }
    
    /// Mint new tokens to an account
    pub fn mint(&self, to: &H160, amount: Int256) -> Result<()> {
        // Check amount
        if amount <= Int256::zero() {
            return Err(Error::with_message(ErrorCode::InvalidArgument, "Amount must be positive"));
        }
        
        // Check only owner can mint
        let owner = self.get_owner();
        if !Runtime::check_witness(&owner) {
            return Err(Error::with_message(ErrorCode::Unauthorized, "Only owner can mint"));
        }
        
        // Update balance
        let balance = self.balance_of(to);
        self.balances.put(to, &(balance + amount));
        
        // Update total supply
        let total_supply = self.total_supply();
        self.metadata.total_supply.put(&ByteString::from("value"), &(total_supply + amount));
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), None, Some(to.clone()), amount);
        
        Ok(())
    }
    
    /// Burn tokens from an account
    pub fn burn(&self, from: &H160, amount: Int256) -> Result<()> {
        // Check amount
        if amount <= Int256::zero() {
            return Err(Error::with_message(ErrorCode::InvalidArgument, "Amount must be positive"));
        }
        
        // Check owner or self authorization
        if !Runtime::check_witness(from) {
            return Err(Error::with_message(ErrorCode::Unauthorized, "Not authorized to burn"));
        }
        
        // Check balance
        let balance = self.balance_of(from);
        if balance < amount {
            return Err(Error::with_message(ErrorCode::InsufficientFunds, "Insufficient balance for burn"));
        }
        
        // Update balance
        self.balances.set(from, &(balance - amount))?;
        
        // Update total supply
        let total_supply = self.total_supply();
        self.metadata.total_supply.set(&ByteString::from("value"), &(total_supply - amount))?;
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), Some(from.clone()), None, amount);
        
        Ok(())
    }
    
    /// Get the owner of the token contract
    pub fn get_owner(&self) -> H160 {
        match self.owner.get(&ByteString::from("owner")) {
            Ok(Some(owner)) => owner,
            _ => H160::zero(),
        }
    }
    
    /// Set a new owner for the token contract
    pub fn set_owner(&self, new_owner: H160) -> bool {
        // Check if caller is current owner
        let current_owner = self.get_owner();
        if !Runtime::check_witness(&current_owner) {
            return false;
        }
        
        // Set new owner
        if let Err(_) = self.owner.set(&ByteString::from("owner"), &new_owner) {
            return false;
        }
        
        true
    }
}

impl Token for FungibleToken {
    fn name(&self) -> ByteString {
        self.metadata.name.clone()
    }
    
    fn symbol(&self) -> ByteString {
        self.metadata.symbol.clone()
    }
    
    fn decimals(&self) -> u8 {
        self.metadata.decimals
    }
    
    fn total_supply(&self) -> Int256 {
        match self.metadata.total_supply.get(&ByteString::from("value")) {
            Ok(Some(value)) => value,
            _ => Int256::zero(),
        }
    }
    
    fn balance_of(&self, account: &H160) -> Int256 {
        match self.balances.get(account) {
            Ok(Some(balance)) => balance,
            _ => Int256::zero(),
        }
    }
    
    fn transfer(&self, to: &H160, amount: Int256) -> bool {
        // Get sender
        let sender = Runtime::calling_script_hash();
        
        // Check authorization
        if !Runtime::check_witness(&sender) {
            return false;
        }
        
        // Check amount
        if amount <= Int256::zero() {
            return false;
        }
        
        // Check balance
        let from_balance = self.balance_of(&sender);
        if from_balance < amount {
            return false;
        }
        
        // Update balances
        self.balances.put(&sender, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(to, &(to_balance + amount));
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), Some(sender), Some(to.clone()), amount);
        
        true
    }
    
    fn transfer_from(&self, from: &H160, to: &H160, amount: Int256) -> bool {
        // Check authorization
        if !Runtime::check_witness(from) {
            return false;
        }
        
        // Check amount
        if amount <= Int256::zero() {
            return false;
        }
        
        // Check balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Update balances
        self.balances.put(from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(to, &(to_balance + amount));
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), Some(from.clone()), Some(to.clone()), amount);
        
        true
    }
}
