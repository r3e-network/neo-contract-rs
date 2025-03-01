// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use neo_contract::{
    contract::{call, SmartContract, CallFlags},
    contract_method, smart_contract, static_value,
    types::*,
};

pub struct ContractCall;

#[smart_contract]
impl ContractCall {
    // Static values
    static_value!(static TARGET_CONTRACT: H160 = "0x13a83e059c2eedd5157b766d3357bc826810905e";);
    
    // Methods
    contract_method!(pub fn on_nep17_payment(from: H160, amount: Int256, data: Int256) -> bool {
        // Check if the data is valid
        if data != Int256::from(123i32) {
            return false;
        }
        
        // Get the executing script hash
        let this = runtime::executing_script_hash();
        
        // Get the calling script hash (token contract)
        let token_hash = runtime::calling_script_hash();
        
        // Call the token contract to get the balance
        let args = Array::new();
        args.push(this.into());
        
        let balance_of = call(
            token_hash,
            ByteString::new("balanceOf"),
            CallFlags::All,
            args
        );
        
        // Convert the result to Int256
        let balance = match balance_of.as_int256() {
            Some(b) => b,
            None => return false,
        };
        
        // Call the target contract
        let args = Array::new();
        args.push(this.into());
        args.push(token_hash.into());
        args.push(balance.into());
        
        call(
            TARGET_CONTRACT,
            ByteString::new("dummyMethod"),
            CallFlags::All,
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

impl SmartContract for ContractCall {}
