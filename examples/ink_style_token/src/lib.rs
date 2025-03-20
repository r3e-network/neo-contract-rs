// NEP-17 Token Example using ink! style

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]

use neo_contract::prelude::*;

/// Event emitted when tokens are transferred
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<Address>,
    #[index]
    pub to: Option<Address>,
    pub amount: u64,
}

/// A NEP-17 compatible token implementation
#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_description("NEP-17 Token Example using ink! style")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct Token {
    /// Token name
    #[storage]
    name: StorageItem<String>,
    
    /// Token symbol
    #[storage]
    symbol: StorageItem<String>,
    
    /// Total token supply
    #[storage]
    total_supply: StorageItem<u64>,
    
    /// Number of decimals
    #[storage]
    decimals: StorageItem<u8>,
    
    /// Balance mapping for each address
    #[storage]
    balances: StorageMap<Address, u64>,
}

impl Token {
    /// Constructor to initialize the token
    #[constructor]
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: Address) -> Self {
        let mut instance = Self {
            name: StorageItem::new(b"name"),
            symbol: StorageItem::new(b"symbol"),
            total_supply: StorageItem::new(b"total_supply"),
            decimals: StorageItem::new(b"decimals"),
            balances: StorageMap::new(b"balances"),
        };
        
        instance.name.set(&name);
        instance.symbol.set(&symbol);
        instance.decimals.set(&decimals);
        instance.total_supply.set(&total_supply);
        instance.balances.insert(owner, total_supply);
        
        // Emit transfer event (from None to owner)
        Transfer {
            from: None,
            to: Some(owner),
            amount: total_supply
        }.notify();
        
        instance
    }
    
    /// Get the token name
    #[safe]
    pub fn name(&self) -> String {
        self.name.get().unwrap_or_default()
    }
    
    /// Get the token symbol
    #[safe]
    pub fn symbol(&self) -> String {
        self.symbol.get().unwrap_or_default()
    }
    
    /// Get the number of decimals
    #[safe]
    pub fn decimals(&self) -> u8 {
        self.decimals.get().unwrap_or(8)
    }
    
    /// Get the total token supply
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or_default()
    }
    
    /// Get the balance of an address
    #[safe]
    pub fn balance_of(&self, account: Address) -> u64 {
        self.balances.get(&account).unwrap_or_default()
    }
    
    /// Transfer tokens from the caller to another address
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, to: Address, amount: u64) -> bool {
        // Get the calling address
        let from = neo_contract::runtime::get_calling_scripthashs().first().unwrap().clone();
        
        // Check if the caller has enough balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Handle the case where amount is 0
        if amount == 0 {
            Transfer {
                from: Some(from),
                to: Some(to),
                amount: 0
            }.notify();
            return true;
        }
        
        // Update balances
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(from, new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        let to_balance = self.balance_of(to);
        let new_to_balance = to_balance + amount;
        self.balances.insert(to, new_to_balance);
        
        // Emit transfer event
        Transfer {
            from: Some(from),
            to: Some(to),
            amount: amount
        }.notify();
        
        true
    }
    
    /// Transfer tokens from one address to another (if approved)
    #[method]
    #[no_reentry]
    pub fn transfer_from(&mut self, from: Address, to: Address, amount: u64) -> bool {
        // In a real implementation, this would check for approval
        // For simplicity, this example assumes the caller is always approved
        
        // Check if the from address has enough balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Handle the case where amount is 0
        if amount == 0 {
            Transfer {
                from: Some(from),
                to: Some(to),
                amount: 0
            }.notify();
            return true;
        }
        
        // Update balances
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(from, new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        let to_balance = self.balance_of(to);
        let new_to_balance = to_balance + amount;
        self.balances.insert(to, new_to_balance);
        
        // Emit transfer event
        Transfer {
            from: Some(from),
            to: Some(to),
            amount: amount
        }.notify();
        
        true
    }
}
