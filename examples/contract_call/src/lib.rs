// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Array, Any},
    Runtime,
    contract, contract_author, contract_description,
    contract_version,
    storage, constructor, message,
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

// Define the target contract hash as a constant
const TARGET_CONTRACT: &str = "0x13a83e059c2eedd5157b766d3357bc826810905e";

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Contract Call Example")]
#[contract_version("0.1.0")]
mod contract_call {
    use super::*;
    
    #[storage]
    pub struct ContractCall {
        // Storage for last received payment data
        last_sender: Option<H160>,
        last_amount: Int256,
    }
    
    impl ContractCall {
        #[constructor]
        pub fn new() -> Self {
            Self {
                last_sender: None,
                last_amount: Int256::zero(),
            }
        }
        
        #[message]
        pub fn on_nep17_payment(&mut self, _from: H160, _amount: Int256, data: Int256) -> bool {
            // Check if the data is valid
            if data != Int256::from(123i32) {
                return false;
            }
            
            // Get the executing script hash
            let this = Runtime::executing_script_hash();
            
            // Get the calling script hash (token contract)
            let token_hash = Runtime::calling_script_hash();
            
            // Parse the target contract hash
            let target_contract = H160::from_hex_string(TARGET_CONTRACT);
            
            // Call the token contract to get the balance
            let mut args = Array::new();
            args.push(Any::from(this));
            
            let balance_of = Runtime::call_contract(
                token_hash,
                ByteString::from("balanceOf"),
                args
            );
            
            // Convert the result to Int256
            let balance = match balance_of.as_int256() {
                Some(b) => b,
                None => return false,
            };
            
            // Store the payment information
            self.last_sender = Some(token_hash);
            self.last_amount = balance.clone();
            
            // Call the target contract with a real method name (transfer instead of dummyMethod)
            let mut args = Array::new();
            args.push(Any::from(this));
            args.push(Any::from(target_contract)); // Send tokens to target_contract
            args.push(Any::from(Int256::from(1i32))); // Send a small amount (1 token)
            args.push(Any::from(ByteString::from("example data"))); // Data parameter
            
            Runtime::call_contract(
                token_hash,
                ByteString::from("transfer"),
                args
            );
            
            true
        }
        
        // New method to get the last payment info
        #[message]
        #[safe]
        pub fn get_last_payment(&self) -> (Option<H160>, Int256) {
            (self.last_sender, self.last_amount.clone())
        }
    }
}
