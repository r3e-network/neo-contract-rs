#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use neo_contract::prelude::*;

// Define events
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}

// Define the contract with storage
#[neo_contract::contract]
pub struct SampleToken {
    // Storage fields
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>,
    #[storage]
    name: StorageItem<ByteString>,
    #[storage]
    symbol: StorageItem<ByteString>,
    #[storage]
    decimals: StorageItem<u8>,
    #[storage]
    owner: StorageItem<H160>,
}

impl SampleToken {
    // Constructor
    #[constructor]
    pub fn new(owner: H160, initial_supply: u64) -> Self {
        let mut instance = Self {
            balances: StorageMap::new(b"balances"),
            total_supply: StorageItem::new(b"total_supply"),
            name: StorageItem::new(b"name"),
            symbol: StorageItem::new(b"symbol"),
            decimals: StorageItem::new(b"decimals"),
            owner: StorageItem::new(b"owner"),
        };
        
        // Initialize contract state
        instance.name.set(&ByteString::from("Sample Token"));
        instance.symbol.set(&ByteString::from("SMPL"));
        instance.decimals.set(&8);
        instance.total_supply.set(&initial_supply);
        instance.owner.set(&owner);
        
        // Mint initial supply to owner
        instance.balances.insert(owner, initial_supply);
        
        // Emit transfer event for minting
        Transfer {
            from: None,
            to: Some(owner),
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // Standard NEP-17 methods
    
    // Read-only methods
    #[method]
    #[safe]
    pub fn name(&self) -> ByteString {
        self.name.get().unwrap_or_default()
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        self.symbol.get().unwrap_or_default()
    }
    
    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals.get().unwrap_or(0)
    }
    
    #[method]
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }
    
    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    // State-changing methods with reentrancy protection
    
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64, data: Vec<u8>) -> bool {
        // Check authorization
        assert!(Runtime::check_witness(&from), "Not authorized");
        
        // Check for valid to address
        assert!(to != H160::zero(), "Invalid to address");
        
        // Check amount
        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balances
        if amount > 0 {
            // Subtract from sender
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Add to recipient
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + amount);
            
            // Emit transfer event
            Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount
            }.notify();
        }
        
        // Optional: Handle data if provided
        if !data.is_empty() {
            // Process data parameter (e.g., call recipient contract)
        }
        
        true
    }
    
    // Admin functions
    
    #[method]
    #[no_reentry]
    pub fn mint(&mut self, to: H160, amount: u64) -> bool {
        // Only owner can mint
        let owner = self.owner.get().unwrap();
        assert!(Runtime::check_witness(&owner), "Not authorized");
        
        // Update recipient balance
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + amount);
        
        // Update total supply
        let total = self.total_supply();
        self.total_supply.set(&(total + amount));
        
        // Emit transfer event
        Transfer {
            from: None,
            to: Some(to),
            amount: amount
        }.notify();
        
        true
    }
    
    #[method]
    #[no_reentry]
    pub fn burn(&mut self, from: H160, amount: u64) -> bool {
        // Check authorization
        assert!(Runtime::check_witness(&from), "Not authorized");
        
        // Check balance
        let from_balance = self.balance_of(from);
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update balance
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(from, new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        // Update total supply
        let total = self.total_supply();
        self.total_supply.set(&(total - amount));
        
        // Emit transfer event
        Transfer {
            from: Some(from),
            to: None,
            amount: amount
        }.notify();
        
        true
    }
} 