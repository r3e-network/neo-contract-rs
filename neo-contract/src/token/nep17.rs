// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! NEP-17 token implementation for the Neo N3 blockchain

use alloc::vec::Vec;
use alloc::string::String;

// Update imports to use prelude
use crate::prelude::{H160, ByteString, Int256, Array, Any, StorageMap};
use crate::policy::voting::Storable;  // Import Storable from the voting module where we defined it
use crate::runtime::Runtime;
use crate::error::{Error, Result};  // Remove ErrorCode since we're using the enum variants directly
use crate::token::{Token, TokenEvents, NEP17Token};

/// NEP-17 token implementation
pub struct NEP17TokenContract {
    /// Token metadata
    metadata: NEP17TokenMetadata,
    /// Balances storage map
    balances: StorageMap<H160, Int256>,
    /// Owner of the token contract
    owner: StorageMap<ByteString, H160>,
}

/// NEP-17 token metadata
pub struct NEP17TokenMetadata {
    /// Name of the token
    pub name: ByteString,
    /// Symbol of the token
    pub symbol: ByteString,
    /// Number of decimals for the token
    pub decimals: u8,
    /// Total supply of the token
    pub total_supply: StorageMap<ByteString, Int256>,
}

impl NEP17TokenContract {
    /// Create a new NEP-17 token
    pub fn new(
        name: &str,
        symbol: &str,
        decimals: u8,
    ) -> Self {
        let name_bytes = ByteString::from(name);
        let symbol_bytes = ByteString::from(symbol);
        
        Self {
            metadata: NEP17TokenMetadata {
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
        if self.owner.get(&ByteString::from("owner")).is_some() {
            return Err(Error::AlreadyExists("Token already initialized"));
        }
        
        // Set owner
        self.owner.put(&ByteString::from("owner"), &owner);
        
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
            return Err(Error::InvalidArgument("Amount must be positive"));
        }
        
        // Check only owner can mint
        let owner = self.get_owner();
        if !Runtime::check_witness(&owner) {
            return Err(Error::Unauthorized("Only owner can mint"));
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
            return Err(Error::InvalidArgument("Amount must be positive"));
        }
        
        // Check owner or self authorization
        if !Runtime::check_witness(from) {
            return Err(Error::Unauthorized("Not authorized to burn"));
        }
        
        // Check balance
        let balance = self.balance_of(from);
        if balance < amount {
            return Err(Error::InsufficientFunds("Insufficient balance for burn"));
        }
        
        // Update balance
        self.balances.put(from, &(balance - amount));
        
        // Update total supply
        let total_supply = self.total_supply();
        self.metadata.total_supply.put(&ByteString::from("value"), &(total_supply - amount));
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), Some(from.clone()), None, amount);
        
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
}

impl Token for NEP17TokenContract {
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
        self.metadata.total_supply.get(&ByteString::from("value")).unwrap_or_else(Int256::zero)
    }
    
    fn balance_of(&self, account: &H160) -> Int256 {
        self.balances.get(account).unwrap_or_else(Int256::zero)
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
        
        // Handle sender equals to recipient
        if sender == *to {
            return true;
        }
        
        // Update balances
        self.balances.put(&sender, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(to, &(to_balance + amount));
        
        // Handle neo domain verification
        self.on_nep17_payment(to, amount);
        
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
        
        // Handle from equals to recipient
        if *from == *to {
            return true;
        }
        
        // Update balances
        self.balances.put(from, &(from_balance - amount));
        let to_balance = self.balance_of(to);
        self.balances.put(to, &(to_balance + amount));
        
        // Handle neo domain verification
        self.on_nep17_payment(to, amount);
        
        // Emit transfer event
        TokenEvents::emit_transfer(&(), Some(from.clone()), Some(to.clone()), amount);
        
        true
    }
}

impl NEP17Token for NEP17TokenContract {
    fn get_owner(&self) -> H160 {
        self.owner.get(&ByteString::from("owner")).unwrap_or_else(H160::zero)
    }
    
    fn set_owner(&self, new_owner: H160) -> bool {
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

impl NEP17TokenContract {
    /// Handle NEP-17 payment
    fn on_nep17_payment(&self, to: &H160, amount: Int256) -> bool {
        // Check if recipient is a contract
        if Runtime::is_contract(to) {
            // Try to call onNEP17Payment method on receiving contract
            let method = ByteString::from("onNEP17Payment");
            let mut args = Array::<Any>::new();
            
            args.push(Any::from(Runtime::executing_script_hash()));
            args.push(Any::from(amount));
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
