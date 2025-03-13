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
struct Transfer {}

// Implementation for properly emitting the Transfer event using Neo N3 standards
impl Transfer {
    // Static method to emit the Transfer event in Neo N3 format
    pub fn emit(from: Option<H160>, to: Option<H160>, amount: u64) {
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

// Main contract using Neo Contract annotation syntax
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Simple NEP-17 Token Example")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct SimpleToken {
    // Total token supply
    total_supply: StorageMap<String, u64>,
    // Map of address balances
    balances: StorageMap<H160, u64>,
    // Owner of the contract
    owner: StorageMap<String, H160>,
}

impl SimpleToken {
    // Constructor - called when the contract is deployed
    #[constructor]
    pub fn new(initial_supply: u64) -> Self {
        // Log deployment information
        Runtime::log("Simple Token: Initializing contract");
        
        // Create contract instance
        let mut instance = Self {
            total_supply: StorageMap::new(b"total_supply"),
            balances: StorageMap::new(b"balances"),
            owner: StorageMap::new(b"owner"),
        };
        
        // Set the deployer as the owner of all initial tokens
        let owner = Runtime::calling_script_hash();
        instance.owner.insert("address", owner);
        
        // Set initial balance and supply
        instance.balances.insert(owner, initial_supply);
        instance.total_supply.insert("value", initial_supply);
        
        // Emit transfer event (from None to owner)
        Transfer::emit(None, Some(owner), initial_supply);
        
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
    
    // Get token decimals
    #[safe]
    pub fn decimals(&self) -> u8 {
        TOKEN_DECIMALS
    }
    
    // Get total supply
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get("value").unwrap_or_default()
    }
    
    // Get balance of an address
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or_default()
    }
    
    // Transfer tokens
    #[method]
    #[no_reentry]
    pub fn transfer(&mut self, to: H160, amount: u64, _data: Vec<u8>) -> bool {
        // Get sender
        let from = Runtime::calling_script_hash();
        
        // Check authorization
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Execute transfer
        self.do_transfer(from, to, amount)
    }
    
    // --- Additional Methods ---
    
    // Mint new tokens (only contract owner)
    #[method]
    #[no_reentry]
    pub fn mint(&mut self, to: H160, amount: u64) -> bool {
        // Check if caller is owner
        let caller = Runtime::calling_script_hash();
        let owner = self.owner.get("address").unwrap_or_default();
        assert!(caller == owner, "Only owner can mint tokens");
        
        // Don't allow minting to null address
        assert!(!to.is_zero(), "Cannot mint to null address");
        
        // Update total supply
        let current_supply = self.total_supply.get("value").unwrap_or_default();
        self.total_supply.insert("value", current_supply + amount);
        
        // Update receiver balance
        let receiver_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, receiver_balance + amount);
        
        // Emit transfer event
        Transfer::emit(None, Some(to), amount);
        
        true
    }
    
    // Burn tokens
    #[method]
    #[no_reentry]
    pub fn burn(&mut self, amount: u64) -> bool {
        // Get sender
        let from = Runtime::calling_script_hash();
        
        // Check authorization
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Get sender balance
        let from_balance = self.balances.get(&from).unwrap_or_default();
        
        // Check if enough balance
        assert!(from_balance >= amount, "Insufficient balance");
        
        // Update sender balance
        let new_from_balance = from_balance - amount;
        
        if new_from_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_from_balance);
        }
        
        // Update total supply
        let current_supply = self.total_supply.get("value").unwrap_or_default();
        self.total_supply.insert("value", current_supply - amount);
        
        // Emit transfer event
        Transfer::emit(Some(from), None, amount);
        
        true
    }
    
    // --- Internal Methods ---
    
    // Internal transfer implementation
    fn do_transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Don't allow transfer to null address
        if to.is_zero() {
            return false;
        }
        
        // Don't allow zero amount transfers
        if amount == 0 {
            return false;
        }
        
        // Check if sender has enough balance
        let from_balance = self.balances.get(&from).unwrap_or_default();
        if from_balance < amount {
            return false;
        }
        
        // Update sender balance
        let new_from_balance = from_balance - amount;
        
        // Remove or update sender balance
        if new_from_balance == 0 {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, new_from_balance);
        }
        
        // Update receiver balance
        let to_balance = self.balances.get(&to).unwrap_or_default();
        self.balances.insert(to, to_balance + amount);
        
        // Emit transfer event
        Transfer::emit(Some(from), Some(to), amount);
        
        true
    }
}