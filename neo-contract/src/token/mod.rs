// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Token interfaces and implementations for the Neo blockchain

use alloc::string::String;
use crate::builtin::{H160, ByteString, Int256, Array, Any};
use crate::Runtime;
use crate::env;

/// Standard token interface
pub trait Token {
    /// Get the name of the token
    fn name(&self) -> ByteString;
    
    /// Get the symbol of the token
    fn symbol(&self) -> ByteString;
    
    /// Get the number of decimals for the token
    fn decimals(&self) -> u8;
    
    /// Get the total supply of the token
    fn total_supply(&self) -> Int256;
    
    /// Get the balance of an account
    fn balance_of(&self, account: &H160) -> Int256;
    
    /// Transfer tokens from the sender to a recipient
    fn transfer(&self, to: &H160, amount: Int256) -> bool;
    
    /// Transfer tokens from one account to another
    fn transfer_from(&self, from: &H160, to: &H160, amount: Int256) -> bool;
}

/// Standard token events
pub trait TokenEvents {
    /// Emit a transfer event
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: Int256);
}

/// Implementation of token events
impl TokenEvents for () {
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: Int256) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        Runtime::notify(&event_name, &event_data);
    }
}

/// NEP-5 Token interface for legacy Neo token standard
pub trait NEP5Token: Token {
    /// Get the owner of the token contract
    fn get_owner(&self) -> H160;
    
    /// Set a new owner for the token contract
    fn set_owner(&self, new_owner: H160) -> bool;
}

/// NEP-17 Token interface for Neo N3 token standard
pub trait NEP17Token: Token {
    /// Get the owner of the token contract
    fn get_owner(&self) -> H160;
    
    /// Set a new owner for the token contract
    fn set_owner(&self, new_owner: H160) -> bool;
}

pub mod nep17;
pub mod fungible;
pub mod nonfungible;
