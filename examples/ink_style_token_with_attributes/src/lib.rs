#![no_std]

extern crate alloc;

use alloc::string::String;

//! # Ink-Style Token Contract with Attributes
//!
//! An implementation of a fungible token using Neo N3 annotations with an Ink-inspired style
//! This contract implements:
//! - NEP-17 standard token features
//! - Event handling with annotations
//! - Method visibility controls
//! - Storage management

#[neo_contract::contract]
mod ink_token {
    use neo_contract::prelude::*;
    use alloc::string::String;
    
    /// Token transfer event with Neo N3 indexing
    #[event]
    struct Transfer {
        #[index]
        from: Option<Address>,
        #[index]
        to: Option<Address>,
        amount: u64
    }
    
    /// Implementation for properly emitting the Transfer event using Neo N3 standards
    impl Transfer {
        /// Static method to emit the Transfer event in Neo N3 format
        pub fn emit(from: Option<Address>, to: Option<Address>, amount: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("Transfer");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::from(ByteArray::new())), // null for minting
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::from(ByteArray::new())), // null for burning
            }
            
            event_data.push(Any::from(amount));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Event emitted when ownership is transferred
    #[event]
    struct OwnershipTransferred {
        #[index]
        previous_owner: Address,
        #[index]
        new_owner: Address,
    }
    
    /// Implementation for properly emitting the OwnershipTransferred event using Neo N3 standards
    impl OwnershipTransferred {
        /// Static method to emit the OwnershipTransferred event in Neo N3 format
        pub fn emit(previous_owner: Address, new_owner: Address) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("OwnershipTransferred");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(previous_owner));
            event_data.push(Any::from(new_owner));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Contract storage structure using Neo N3 storage annotation
    #[storage]
    struct InkToken {
        /// Token name
        name: Item<String>,
        
        /// Token symbol
        symbol: Item<String>,
        
        /// Number of decimal places
        decimals: Item<u8>,
        
        /// Total supply of tokens
        total_supply: Item<u64>,
        
        /// Token balances mapped by address
        balances: Map<Address, u64>,
        
        /// Contract owner address
        owner: Item<Address>,
    }
    
    impl InkToken {
        /// Initialize a new token contract with the specified parameters
        #[constructor]
        fn new(
            owner: Address,
            name: String,
            symbol: String,
            decimals: u8,
            total_supply: u64
        ) -> Self {
            // Create the storage structure
            let mut instance = Self {
                name: Item::new("name"),
                symbol: Item::new("symbol"),
                decimals: Item::new("decimals"),
                total_supply: Item::new("total_supply"),
                balances: Map::new(),
                owner: Item::new("owner"),
            };
            
            // Initialize token metadata
            instance.name.set(name);
            instance.symbol.set(symbol);
            instance.decimals.set(decimals);
            instance.total_supply.set(total_supply);
            instance.owner.set(owner);
            
            // Initialize the balance of the token owner with the total supply
            instance.balances.insert(owner, total_supply);
            
            // Emit the transfer event for the initial supply (from null address)
            Transfer::emit(None, Some(owner), total_supply);
            
            instance
        }
        
        /// Get the token name
        #[method]
        #[safe]
        fn name(&self) -> String {
            self.name.get().unwrap_or_default()
        }
        
        /// Get the token symbol
        #[method]
        #[safe]
        fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_default()
        }
        
        /// Get the number of decimal places
        #[method]
        #[safe]
        fn decimals(&self) -> u8 {
            self.decimals.get().unwrap_or_default()
        }
        
        /// Get the total token supply
        #[method]
        #[safe]
        fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        /// Get the token balance for a specific account
        #[method]
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        /// Transfer tokens from one account to another
        #[method]
        #[no_reentry]
        fn transfer(&mut self, from: Address, to: Address, amount: u64, data: Option<ByteArray>) -> bool {
            // Verify transaction signature
            assert!(Runtime::check_witness(&from), "Unauthorized");
            
            // Check for valid receiving address
            assert!(to != Address::zero(), "Invalid receiving address");
            
            // Get sender's balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            
            // Ensure sender has enough tokens
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Skip if amount is zero
            if amount == 0 {
                return true;
            }
            
            // Calculate new balances
            let new_from_balance = from_balance - amount;
            
            // Update sender's balance
            if new_from_balance > 0 {
                self.balances.insert(from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Update receiver's balance
            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to.clone(), to_balance + amount);
            
            // Emit transfer event with proper Neo N3 format
            Transfer::emit(Some(from), Some(to.clone()), amount);
            
            // NEP-17 standard: Call onNEP17Payment if the recipient is a contract
            if to != from {
                let contract_called = Runtime::calling_script_hash();
                
                // Only allow notification to receiving contract when it's not the caller
                if contract_called != to {
                    let on_nep17_payment_method = "onNEP17Payment";
                    
                    // Prepare arguments
                    let mut args = Array::<Any>::new();
                    args.push(Any::from(from));
                    args.push(Any::from(amount));
                    if let Some(data_value) = data {
                        args.push(Any::from(data_value));
                    } else {
                        args.push(Any::from(ByteArray::new())); // empty data
                    }
                    
                    // Call receiver's onNEP17Payment method
                    // Ignore errors to ensure the transfer succeeds regardless
                    let _: Result<(), Error> = Runtime::call_contract(&to, on_nep17_payment_method, &args);
                }
            }
            
            true
        }
        
        /// Mint new tokens (only callable by owner)
        #[method]
        #[no_reentry]
        fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Verify owner authorization
            let owner = self.owner.get().unwrap_or_default();
            assert!(Runtime::check_witness(&owner), "Only owner can mint tokens");
            
            // Amount must be greater than zero
            assert!(amount > 0, "Mint amount must be greater than 0");
            
            // Get current values
            let current_supply = self.total_supply.get().unwrap_or_default();
            let to_balance = self.balances.get(&to).unwrap_or_default();
            
            // Update total supply
            self.total_supply.set(current_supply + amount);
            
            // Update receiver's balance
            self.balances.insert(to, to_balance + amount);
            
            // Emit transfer event (mint = transfer from None) with proper Neo N3 format
            Transfer::emit(None, Some(to), amount);
            
            true
        }
        
        /// Burn tokens
        #[method]
        #[no_reentry]
        fn burn(&mut self, from: Address, amount: u64) -> bool {
            // Verify authorization
            assert!(Runtime::check_witness(&from), "Unauthorized");
            
            // Amount must be greater than zero
            assert!(amount > 0, "Burn amount must be greater than 0");
            
            // Get current balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            
            // Ensure account has enough tokens
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Get current supply
            let current_supply = self.total_supply.get().unwrap_or_default();
            
            // Update total supply
            self.total_supply.set(current_supply - amount);
            
            // Update sender's balance
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Emit transfer event (burn = transfer to None) with proper Neo N3 format
            Transfer::emit(Some(from), None, amount);
            
            true
        }
        
        /// Transfer ownership of the contract
        #[method]
        #[no_reentry]
        fn transfer_ownership(&mut self, new_owner: Address) -> bool {
            // Get current owner
            let current_owner = self.owner.get().unwrap_or_default();
            
            // Verify ownership
            assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
            
            // Ensure new owner is not zero address
            assert!(new_owner != Address::zero(), "Cannot transfer to zero address");
            
            // Update owner
            self.owner.set(new_owner);
            
            // Emit ownership transfer event
            OwnershipTransferred::emit(current_owner, new_owner);
            
            true
        }
        
        /// Get the current owner of the contract
        #[method]
        #[safe]
        fn get_owner(&self) -> Address {
            self.owner.get().unwrap_or_default()
        }
    }
}
