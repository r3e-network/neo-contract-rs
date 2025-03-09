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

// Define token metadata as static constants
#[string]
static TOKEN_NAME: &str = "Simple Token";

#[string]
static TOKEN_SYMBOL: &str = "SIMPLE";

#[integer]
static TOKEN_DECIMALS: u8 = 8;

// Define contract owner
#[hash160]
static OWNER: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";

// Define token events
#[event]
struct Transfer {
    #[index]
    from: Option<H160>,
    #[index]
    to: Option<H160>,
    amount: Int256,
}

// Main contract module using ink!-style syntax
#[contract]
#[contract_permission(storage, *)]
#[manifest_extra(
    name = "Simple Token",
    author = "Neo Contract RS",
    email = "dev@neo.org",
    description = "A simple NEP-17 token example"
)]
#[supported_standards("NEP-17")]
mod token_contract {
    use super::*;

    // Define contract storage
    #[storage]
    pub struct Token {
        // Total token supply
        total_supply: Int256,
        // Map of address balances
        balances: Map<H160, Int256>,
    }

    impl Token {
        // Constructor - called when the contract is deployed
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            // Log deployment information
            runtime::log("Simple Token: Initializing contract".into());
            
            // Set the deployer as the owner of all initial tokens
            let owner: H160 = OWNER.into();
            
            // Create the storage map
            let mut balances = Map::new();
            balances.insert(owner, initial_supply.clone());
            
            // Emit transfer event (from None to owner)
            let event = Transfer {
                from: None,
                to: Some(owner),
                amount: initial_supply.clone(),
            };
            event.emit();
            
            // Return the initialized token
            Self {
                total_supply: initial_supply,
                balances,
            }
        }
        
        // --- NEP-17 Standard Methods ---
        
        // Get the token symbol
        #[message]
        #[safe]
        pub fn symbol(&self) -> ByteString {
            TOKEN_SYMBOL.into()
        }
        
        // Get the token name
        #[message]
        #[safe]
        pub fn name(&self) -> ByteString {
            TOKEN_NAME.into()
        }
        
        // Get token decimals
        #[message]
        #[safe]
        pub fn decimals(&self) -> u8 {
            TOKEN_DECIMALS.into()
        }
        
        // Get total supply
        #[message]
        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }
        
        // Get balance of an address
        #[message]
        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        // Transfer tokens
        #[message]
        pub fn transfer(&mut self, to: H160, amount: Int256) -> bool {
            // Get sender
            let from = runtime::calling_script_hash();
            
            // Check authorization
            assert!(runtime::check_witness(from), "No authorization");
            
            // Execute transfer
            self.do_transfer(from, to, amount)
        }
        
        // --- Additional Methods ---
        
        // Mint new tokens (only contract owner)
        #[message]
        pub fn mint(&mut self, to: H160, amount: Int256) -> bool {
            // Check if caller is owner
            let caller = runtime::calling_script_hash();
            let owner: H160 = OWNER.into();
            assert!(caller == owner, "Only owner can mint tokens");
            
            // Don't allow minting to null address
            assert!(!to.is_zero(), "Cannot mint to null address");
            
            // Update total supply
            self.total_supply += amount.clone();
            
            // Update receiver balance
            let receiver_balance = self.balances.get(&to).unwrap_or_default();
            let new_receiver_balance = receiver_balance + amount.clone();
            self.balances.insert(to, new_receiver_balance);
            
            // Emit transfer event
            let event = Transfer {
                from: None,
                to: Some(to),
                amount: amount.clone(),
            };
            event.emit();
            
            true
        }
        
        // Burn tokens
        #[message]
        pub fn burn(&mut self, amount: Int256) -> bool {
            // Get sender
            let from = runtime::calling_script_hash();
            
            // Check authorization
            assert!(runtime::check_witness(from), "No authorization");
            
            // Get sender balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            
            // Check if enough balance
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update sender balance
            let new_from_balance = from_balance - amount.clone();
            
            if new_from_balance.is_zero() {
                self.balances.remove(&from);
            } else {
                self.balances.insert(from, new_from_balance);
            }
            
            // Update total supply
            self.total_supply -= amount.clone();
            
            // Emit transfer event
            let event = Transfer {
                from: Some(from),
                to: None,
                amount: amount.clone(),
            };
            event.emit();
            
            true
        }
        
        // --- Internal Methods ---
        
        // Internal transfer implementation
        fn do_transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            // Don't allow transfer to null address
            if to.is_zero() {
                return false;
            }
            
            // Don't allow zero amount transfers
            if amount.is_zero() {
                return false;
            }
            
            // Check if amount is negative
            if amount < Int256::zero() {
                return false;
            }
            
            // Check if sender has enough balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            if from_balance < amount {
                return false;
            }
            
            // Update sender balance
            let new_from_balance = from_balance - amount.clone();
            
            // Remove or update sender balance
            if new_from_balance.is_zero() {
                self.balances.remove(&from);
            } else {
                self.balances.insert(from, new_from_balance);
            }
            
            // Update receiver balance
            let to_balance = self.balances.get(&to).unwrap_or_default();
            let new_to_balance = to_balance + amount.clone();
            self.balances.insert(to, new_to_balance);
            
            // Emit transfer event
            let event = Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount.clone(),
            };
            event.emit();
            
            true
        }
    }
}