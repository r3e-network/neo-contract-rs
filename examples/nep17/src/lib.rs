#![no_std]

extern crate alloc;

//! # NEP-17 Token Smart Contract for Neo N3
//!
//! A fungible token implementation following the NEP-17 standard on Neo N3 blockchain.
//! This contract demonstrates:
//! - Complete implementation of NEP-17 standard
//! - Neo N3 storage patterns with proper annotations
//! - Event handling with Neo N3 indexing
//! - Method security controls and visibility
//! - Owner management functions

#[neo_contract::contract]
mod nep17_token {
    use neo_contract::prelude::*;
    use alloc::string::String;
    
    // Token configuration constants
    const TOKEN_NAME: &str = "Sample NEP17 Token";
    const TOKEN_SYMBOL: &str = "NEP17";
    const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY: u64 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimals
    
    /// Event emitted when tokens are transferred
    #[event]
    struct Transfer {
        #[index]
        from: Option<Address>,
        #[index]
        to: Option<Address>,
        amount: u64,
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
    
    /// Contract storage
    #[storage]
    struct NEP17Token {
        /// Token balances for each address
        balances: Map<Address, u64>,
        
        /// Total token supply
        total_supply: Item<u64>,
        
        /// Contract owner address
        owner: Item<Address>,
    }
    
    impl NEP17Token {
        /// Initialize the token contract with initial supply and owner
        #[constructor]
        fn new(owner: Address) -> Self {
            let mut instance = Self {
                balances: Map::new(),
                total_supply: Item::new("total_supply"),
                owner: Item::new("owner"),
            };
            
            // Set initial values
            instance.total_supply.set(TOKEN_TOTAL_SUPPLY);
            instance.owner.set(owner);
            
            // Mint initial supply to owner
            instance.balances.insert(owner, TOKEN_TOTAL_SUPPLY);
            
            // Emit the transfer event (from None to owner)
            Transfer::emit(None, Some(owner), TOKEN_TOTAL_SUPPLY);
            
            instance
        }
        
        /// Get the name of the token
        #[method]
        #[safe]
        fn name(&self) -> String {
            TOKEN_NAME.to_string()
        }
        
        /// Get the symbol of the token
        #[method]
        #[safe]
        fn symbol(&self) -> String {
            TOKEN_SYMBOL.to_string()
        }
        
        /// Get the number of decimals the token uses
        #[method]
        #[safe]
        fn decimals(&self) -> u8 {
            TOKEN_DECIMALS
        }
        
        /// Get the total token supply
        #[method]
        #[safe]
        fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        /// Get the token balance of the specified address
        #[method]
        #[safe]
        fn balance_of(&self, address: Address) -> u64 {
            self.balances.get(&address).unwrap_or_default()
        }
        
        /// Transfer tokens from one address to another
        #[method]
        #[no_reentry]
        fn transfer(
            &mut self,
            from: Address, 
            to: Address, 
            amount: u64, 
            data: Option<ByteArray>
        ) -> bool {
            // Amount must be greater than zero
            assert!(amount > 0, "Transfer amount must be greater than 0");
            
            // Validate "from" address has signed the transaction
            assert!(Runtime::check_witness(&from), "No authorization");
            
            // Check if "from" address has sufficient balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update balances
            // Subtract from the sender
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Add to the recipient
            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to.clone(), to_balance + amount);
            
            // Emit the transfer event
            Transfer::emit(Some(from), Some(to.clone()), amount);
            
            // If receiving contract has onNEP17Payment method, call it
            if to != from {
                let contract_called = Runtime::calling_script_hash();
                
                // Only allow notification to receiving contract when it's not the caller
                if contract_called != to {
                    let on_nep17_payment_method = "onNEP17Payment";
                    
                    // Prepare arguments
                    let mut args = Array::<Any>::new();
                    args.push(Any::from(from)); // from address
                    args.push(Any::from(amount)); // amount
                    if let Some(data_value) = data {
                        args.push(Any::from(data_value)); // optional data
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
        
        /// Mint new tokens and assign to an address (only owner can call)
        #[method]
        #[no_reentry]
        fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Only owner can mint
            let owner = self.owner.get().unwrap_or_default();
            assert!(Runtime::check_witness(&owner), "Only owner can mint tokens");
            
            // Amount must be greater than zero
            assert!(amount > 0, "Mint amount must be greater than 0");
            
            // Update recipient balance
            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to.clone(), to_balance + amount);
            
            // Update total supply
            let current_supply = self.total_supply.get().unwrap_or_default();
            self.total_supply.set(current_supply + amount);
            
            // Emit the transfer event (from None to recipient)
            Transfer::emit(None, Some(to), amount);
            
            true
        }
        
        /// Burn tokens from an address (only owner or token holder can call)
        #[method]
        #[no_reentry]
        fn burn(&mut self, from: Address, amount: u64) -> bool {
            // Caller must be either the token holder or the owner
            let owner = self.owner.get().unwrap_or_default();
            assert!(
                Runtime::check_witness(&from) || 
                (Runtime::check_witness(&owner) && owner != from),
                "No authorization to burn"
            );
            
            // Amount must be greater than zero
            assert!(amount > 0, "Burn amount must be greater than 0");
            
            // Check if "from" address has sufficient balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update balance
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(from.clone(), new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Update total supply
            let current_supply = self.total_supply.get().unwrap_or_default();
            self.total_supply.set(current_supply - amount);
            
            // Emit the transfer event (from address to None)
            Transfer::emit(Some(from), None, amount);
            
            true
        }
        
        /// Transfer ownership of the contract to a new address (only owner can call)
        #[method]
        #[no_reentry]
        fn transfer_ownership(&mut self, new_owner: Address) -> bool {
            // Only current owner can transfer ownership
            let current_owner = self.owner.get().unwrap_or_default();
            assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
            
            // New owner cannot be zero address
            assert!(new_owner != Address::zero(), "Cannot transfer to zero address");
            
            // Update owner
            self.owner.set(new_owner);
            
            // Emit event
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
