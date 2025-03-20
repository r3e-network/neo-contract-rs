#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use neo_contract::prelude::*;

// Define token metadata as constants
const TOKEN_NAME: &str = "Simple Token";
const TOKEN_SYMBOL: &str = "SIMPLE";
const TOKEN_DECIMALS: u8 = 8;

// Define transfer event for NEP-17 compliance
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}

// Main contract using Neo Contract annotation syntax
#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_description("Simple NEP-17 Token Example")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct SimpleToken {
    // Total token supply
    #[storage]
    total_supply: StorageItem<u64>,
    // Map of address balances
    #[storage]
    balances: StorageMap<H160, u64>,
    // Owner of the contract
    #[storage]
    owner: StorageItem<H160>,
}

impl SimpleToken {
    // Constructor - called when the contract is deployed
    #[constructor]
    pub fn new(initial_supply: u64) -> Self {
        // Log deployment information
        Runtime::log("Simple Token: Initializing contract");
        
        // Create contract instance
        let mut instance = Self {
            total_supply: StorageItem::new(b"total_supply"),
            balances: StorageMap::new(b"balances"),
            owner: StorageItem::new(b"owner"),
        };
        
        // Set the deployer as the owner of all initial tokens
        let owner = Runtime::calling_script_hash();
        instance.owner.set(&owner);
        
        // Set initial balance and supply
        instance.balances.insert(owner, initial_supply);
        instance.total_supply.set(&initial_supply);
        
        // Emit transfer event (from None to owner)
        Transfer {
            from: None,
            to: Some(owner),
            amount: initial_supply
        }.notify();
        
        instance
    }
    
    // --- NEP-17 Standard Methods ---
    
    // Get the token symbol
    #[safe]
    pub fn symbol(&self) -> String {
        TOKEN_SYMBOL.to_string()
    }
    
    // Get the token name
    #[safe]
    pub fn name(&self) -> String {
        TOKEN_NAME.to_string()
    }
    
    // Get the token decimals
    #[safe]
    pub fn decimals(&self) -> u8 {
        TOKEN_DECIMALS
    }
    
    // Get the total token supply
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }
    
    // Get the balance of an account
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    // Transfer tokens to another account
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, to: H160, amount: u64, _data: Vec<u8>) -> bool {
        let from = Runtime::calling_script_hash();
        
        // Ensure the caller has enough balance
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        self.do_transfer(from, to, amount)
    }
    
    // Mint new tokens (owner only)
    #[method]
    #[no_reentry]
    pub fn mint(&mut self, to: H160, amount: u64) -> bool {
        // Only the owner can mint new tokens
        let sender = Runtime::calling_script_hash();
        let owner = self.owner.get().unwrap_or_default();
        
        if sender != owner || !Runtime::check_witness(&sender) {
            return false;
        }
        
        // Update the recipient's balance
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + amount);
        
        // Update total supply
        let supply = self.total_supply.get().unwrap_or(0);
        self.total_supply.set(&(supply + amount));
        
        // Emit transfer event
        Transfer {
            from: None,
            to: Some(to),
            amount
        }.notify();
        
        true
    }
    
    // Burn tokens from the caller's account
    #[method]
    #[no_reentry]
    pub fn burn(&mut self, amount: u64) -> bool {
        let from = Runtime::calling_script_hash();
        
        // Ensure the caller has enough balance
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Update the caller's balance
        let new_balance = from_balance - amount;
        if new_balance > 0 {
            self.balances.insert(from, new_balance);
        } else {
            self.balances.remove(&from);
        }
        
        // Update total supply
        let supply = self.total_supply.get().unwrap_or(0);
        self.total_supply.set(&(supply - amount));
        
        // Emit transfer event
        Transfer {
            from: Some(from),
            to: None,
            amount
        }.notify();
        
        true
    }
    
    // Internal helper for transferring tokens between accounts
    fn do_transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Check if the sender has enough balance
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        // Update the sender's balance
        let new_from_balance = from_balance - amount;
        if new_from_balance > 0 {
            self.balances.insert(from, new_from_balance);
        } else {
            self.balances.remove(&from);
        }
        
        // Update the recipient's balance
        let to_balance = self.balance_of(to);
        self.balances.insert(to, to_balance + amount);
        
        // Emit the transfer event
        Transfer {
            from: Some(from),
            to: Some(to),
            amount
        }.notify();
        
        true
    }
}