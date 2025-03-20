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

// Define token events
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<Address>,
    #[index]
    pub to: Option<Address>,
    pub amount: u64,
}

#[neo_contract::event]
pub struct OwnershipTransferred {
    #[index]
    pub previous_owner: Address,
    #[index]
    pub new_owner: Address,
}

#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_description("NEP-17 Token Implementation for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct NEP17Token {
    // Token configuration constants
    #[storage]
    balances: StorageMap<Address, u64>,
    
    #[storage]
    total_supply: StorageItem<u64>,
    
    #[storage]
    owner: StorageItem<Address>,
}

impl NEP17Token {
    // Token configuration constants
    const TOKEN_NAME: &'static str = "Sample NEP17 Token";
    const TOKEN_SYMBOL: &'static str = "NEP17";
    const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY: u64 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimals

    /// Initialize the token contract with initial supply and owner
    #[constructor]
    pub fn new(owner: Address) -> Self {
        let mut instance = Self {
            balances: StorageMap::new(b"balances"),
            total_supply: StorageItem::new(b"total_supply"),
            owner: StorageItem::new(b"owner"),
        };
        
        // Set initial values
        instance.total_supply.set(&Self::TOKEN_TOTAL_SUPPLY);
        instance.owner.set(&owner);
        
        // Mint initial supply to owner
        instance.balances.insert(owner, Self::TOKEN_TOTAL_SUPPLY);
        
        // Emit the transfer event (from None to owner)
        Transfer {
            from: None,
            to: Some(owner),
            amount: Self::TOKEN_TOTAL_SUPPLY
        }.notify();
        
        instance
    }
    
    /// Get the name of the token
    #[safe]
    pub fn name(&self) -> String {
        Self::TOKEN_NAME.into()
    }
    
    /// Get the symbol of the token
    #[safe]
    pub fn symbol(&self) -> String {
        Self::TOKEN_SYMBOL.into()
    }
    
    /// Get the number of decimals the token uses
    #[safe]
    pub fn decimals(&self) -> u8 {
        Self::TOKEN_DECIMALS
    }
    
    /// Get the total token supply
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or_default()
    }
    
    /// Get the token balance of the specified address
    #[safe]
    pub fn balance_of(&self, address: Address) -> u64 {
        self.balances.get(&address).unwrap_or_default()
    }
    
    /// Transfer tokens from one address to another
    #[method]
    #[no_reentry]
    pub fn transfer(
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
        Transfer {
            from: Some(from),
            to: Some(to.clone()),
            amount
        }.notify();
        
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
    pub fn mint(&mut self, to: Address, amount: u64) -> bool {
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
        self.total_supply.set(&(current_supply + amount));
        
        // Emit the transfer event (from None to recipient)
        Transfer {
            from: None,
            to: Some(to),
            amount
        }.notify();
        
        true
    }
    
    /// Burn tokens from an address (only owner or token holder can call)
    #[method]
    #[no_reentry]
    pub fn burn(&mut self, from: Address, amount: u64) -> bool {
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
        self.total_supply.set(&(current_supply - amount));
        
        // Emit the transfer event (from address to None)
        Transfer {
            from: Some(from),
            to: None,
            amount
        }.notify();
        
        true
    }
    
    /// Transfer ownership of the contract to a new address (only owner can call)
    #[method]
    #[no_reentry]
    pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
        // Only current owner can transfer ownership
        let current_owner = self.owner.get().unwrap_or_default();
        assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
        
        // New owner cannot be zero address
        assert!(new_owner != Address::zero(), "Cannot transfer to zero address");
        
        // Set new owner
        self.owner.set(&new_owner);
        
        // Emit ownership transfer event
        OwnershipTransferred {
            previous_owner: current_owner,
            new_owner
        }.notify();
        
        true
    }
    
    /// Get the current contract owner
    #[safe]
    pub fn get_owner(&self) -> Address {
        self.owner.get().unwrap_or_default()
    }
}
