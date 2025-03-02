#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map},
    Runtime,
    contract, contract_author, contract_description,
    contract_version, supported_standards,
    storage, constructor, message, safe,
};
use core::panic::PanicInfo;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Define a panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define a constant for the owner address
const OWNER_ADDRESS: &str = "0x13a83e059c2eedd5157b766d3357bc826810905e";

#[contract]
#[contract_author("R3E Network")]
#[contract_description("NEP-17 Token Example")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod token_contract {
    use super::*;

    #[storage]
    pub struct Token {
        token_supply: Int256,
        balances: Map<H160, Int256>,
        token_name: ByteString,
        token_symbol: ByteString,
        token_decimals: u8,
    }

    impl Token {
        #[constructor]
        pub fn new() -> Self {
            let mut balances = Map::new();
            
            // Parse the owner address
            let owner = H160::hex_decode(OWNER_ADDRESS).unwrap_or(H160::zero());
            
            // Mint initial supply to owner
            let token_supply = Int256::from(100_000_000_00000000i64);
            balances.put(owner.clone(), token_supply.clone());
            
            Self {
                token_supply,
                balances,
                token_name: ByteString::from("Example Token"),
                token_symbol: ByteString::from("EXT"),
                token_decimals: 8,
            }
        }
        
        // ---------- NEP-17 Standard Methods ----------
        
        // Get token name
        #[message]
        #[safe]
        pub fn name(&self) -> ByteString {
            self.token_name.clone()
        }
        
        // Get token symbol
        #[message]
        #[safe]
        pub fn symbol(&self) -> ByteString {
            self.token_symbol.clone()
        }
        
        // Get token decimals
        #[message]
        #[safe]
        pub fn decimals(&self) -> u8 {
            self.token_decimals
        }
        
        // Get total token supply
        #[message]
        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.token_supply.clone()
        }
        
        // Get token balance for an account
        #[message]
        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
        }
        
        // Transfer tokens between accounts
        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, _data: ByteString) -> bool {
            // Check if sender has authorized the transfer
            if !Runtime::check_witness(from) {
                return false;
            }
            
            // Check for valid amount
            if amount <= Int256::zero() {
                return false;
            }
            
            // Get current balance
            let from_balance = match self.balances.get(&from) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            // Check if sender has enough balance
            if from_balance < amount {
                return false;
            }
            
            // Calculate new balances
            let new_from_balance = from_balance - amount.clone();
            
            // Update sender balance
            if new_from_balance > Int256::zero() {
                self.balances.put(from.clone(), new_from_balance);
            } else {
                self.balances.delete(&from);
            }
            
            // Update receiver balance
            let to_balance = match self.balances.get(&to) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            };
            
            let new_to_balance = to_balance + amount.clone();
            self.balances.put(to.clone(), new_to_balance);
            
            true
        }
    }
}

// NEP-17 Trait Implementation
impl neo_contract::nep17::NEP17 for token_contract::Token {
    fn symbol(&self) -> ByteString {
        self.symbol()
    }
    
    fn decimals(&self) -> u8 {
        self.decimals()
    }
    
    fn total_supply(&self) -> Int256 {
        self.total_supply()
    }
    
    fn balance_of(&self, account: H160) -> Int256 {
        self.balance_of(account)
    }
    
    fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: ByteString) -> bool {
        self.transfer(from, to, amount, data)
    }
}
