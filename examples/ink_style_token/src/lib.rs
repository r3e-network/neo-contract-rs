// NEP-17 Token Example using ink! style

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]

use neo_contract::prelude::*;

/// Event emitted when tokens are transferred
struct Transfer {}

impl Transfer {
    pub fn emit(from: Option<Address>, to: Option<Address>, amount: u64) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        // Emit the event
        Runtime::notify(&event_name, &event_data);
    }
}

/// A NEP-17 compatible token implementation
#[contract]
#[contract_author("R3E Network")]
#[contract_description("NEP-17 Token Example using ink! style")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub mod token {
    use super::*;
    
    /// Storage for the token contract
    #[storage]
    pub struct Token {
        /// Token name
        name: StorageItem<String>,
        
        /// Token symbol
        symbol: StorageItem<String>,
        
        /// Total token supply
        total_supply: StorageItem<u64>,
        
        /// Number of decimals
        decimals: StorageItem<u8>,
        
        /// Balance mapping for each address
        balances: StorageMap<Address, u64>,
    }
    
    impl Token {
        /// Constructor to initialize the token
        #[constructor]
        pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: Address) -> Self {
            let mut this = Self {
                name: StorageItem::new(),
                symbol: StorageItem::new(),
                total_supply: StorageItem::new(),
                decimals: StorageItem::new(),
                balances: StorageMap::new(),
            };
            
            this.name.set(name);
            this.symbol.set(symbol);
            this.decimals.set(decimals);
            this.total_supply.set(total_supply);
            this.balances.insert(&owner, total_supply);
            
            // Emit transfer event (from None to owner)
            Transfer::emit(None, Some(owner), total_supply);
            
            this
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
                Transfer::emit(Some(from), Some(to), 0);
                return true;
            }
            
            // Update balances
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(&from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.insert(&to, new_to_balance);
            
            // Emit transfer event
            Transfer::emit(Some(from), Some(to), amount);
            
            true
        }
        
        /// Transfer tokens from one address to another (if approved)
        #[method]
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
                Transfer::emit(Some(from), Some(to), 0);
                return true;
            }
            
            // Update balances
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(&from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.insert(&to, new_to_balance);
            
            // Emit transfer event
            Transfer::emit(Some(from), Some(to), amount);
            
            true
        }
    }
}
