// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use crate::builtin::{H160, ByteString, Int256, Array, Any};
use crate::Runtime;

/// GAS native contract
pub struct Gas;

/// GAS script hash
pub const SCRIPT_HASH: H160 = H160([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);

impl Gas {
    /// Get the GAS contract hash
    pub fn hash() -> H160 {
        SCRIPT_HASH
    }
    
    /// Transfer GAS from one account to another
    pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<ByteString>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        
        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        if let Some(data_bs) = data {
            args.push(Any::from(data_bs));
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the symbol of the GAS token
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match ByteString::try_from(result) {
            Ok(symbol) => symbol,
            Err(_) => ByteString::from("GAS"),
        }
    }
    
    /// Get the decimals of the GAS token
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match u8::try_from(result) {
            Ok(decimals) => decimals,
            Err(_) => 8, // GAS has 8 decimals
        }
    }
    
    /// Get the total supply of the GAS token
    pub fn total_supply() -> Int256 {
        let method = ByteString::from("totalSupply");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(total_supply) => total_supply,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Get the balance of GAS for an account
    pub fn balance_of(account: H160) -> Int256 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account));
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(balance) => balance,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Get the name of the GAS token
    pub fn name() -> ByteString {
        let method = ByteString::from("name");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Gas::hash(),
            method,
            args
        );
        
        match ByteString::try_from(result) {
            Ok(name) => name,
            Err(_) => ByteString::from("GAS"),
        }
    }
}
