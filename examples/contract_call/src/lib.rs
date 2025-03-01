// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use neo_contract::{
    builtin::{H160, Int256, ByteString, Array, Any},
    CallFlags,
    Runtime,
    contract_method, smart_contract,
};
use alloc::vec::Vec;

pub struct ContractCall;

#[smart_contract]
impl ContractCall {
    // Define the target contract hash as a constant
    const TARGET_CONTRACT: &'static str = "0x13a83e059c2eedd5157b766d3357bc826810905e";
    
    // Methods
    contract_method!(pub fn on_nep17_payment(from: H160, amount: Int256, data: Int256) -> bool {
        // Check if the data is valid
        if data != Int256::from(123i32) {
            return false;
        }
        
        // Get the executing script hash
        let this = Runtime::executing_script_hash();
        
        // Get the calling script hash (token contract)
        let token_hash = Runtime::calling_script_hash();
        
        // Parse the target contract hash
        let target_contract = H160::from_hex_string(Self::TARGET_CONTRACT);
        
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
        
        // Call the target contract
        let mut args = Array::new();
        args.push(Any::from(this));
        args.push(Any::from(token_hash));
        args.push(Any::from(balance));
        
        Runtime::call_contract(
            target_contract,
            ByteString::from("dummyMethod"),
            args
        );
        
        true
    });
    
    // Contract lifecycle methods
    pub fn deploy(_data: bool) -> bool {
        true
    }
    
    pub fn initialize() -> bool {
        true
    }
}
