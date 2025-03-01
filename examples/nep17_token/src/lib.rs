// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Array, Any},
    contract_event, contract_method, smart_contract,
    nep17::*,
    Runtime,
};
use alloc::vec::Vec;

pub struct Token;

// Define a constant for the owner address
const OWNER_ADDRESS: &str = "0x13a83e059c2eedd5157b766d3357bc826810905e";

#[smart_contract]
impl Token {
    // Events
    contract_event!(fn Transfer(from: Option<H160>, to: Option<H160>, amount: Int256));
    
    // Methods
    contract_method!(pub fn name() -> ByteString {
        ByteString::from("Example Token")
    });
    
    contract_method!(pub fn symbol() -> ByteString {
        ByteString::from("EXT")
    });
    
    contract_method!(pub fn decimals() -> u32 {
        8
    });
    
    contract_method!(pub fn total_supply() -> Int256 {
        Int256::from(100_000_000_00000000i64)
    });
    
    contract_method!(pub fn balance_of(account: H160) -> Int256 {
        // Create a storage key for the balance map
        let prefix = b"balance";
        
        // Get the balance from storage
        let key = [prefix, account.as_bytes()].concat();
        let storage_context = Runtime::storage_context();
        let value = Runtime::storage_get(storage_context, &key);
        
        match value {
            Some(bytes) => {
                // Convert bytes to Int256
                if bytes.len() >= 8 {
                    let mut data = [0u8; 8];
                    data.copy_from_slice(&bytes[0..8]);
                    Int256::from(i64::from_le_bytes(data))
                } else {
                    Int256::zero()
                }
            },
            None => Int256::zero()
        }
    });
    
    contract_method!(pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        // Check if the caller is the owner of the tokens
        if !Runtime::check_witness(from.clone()) {
            return false;
        }
        
        // Check if the amount is positive
        if amount <= Int256::zero() {
            return false;
        }
        
        // Get the balance of the sender
        let balance = Self::balance_of(from.clone());
        
        // Check if the sender has enough tokens
        if balance < amount {
            return false;
        }
        
        // Get storage context
        let storage_context = Runtime::storage_context();
        let prefix = b"balance";
        
        // Update balances
        // Subtract from sender
        let new_balance = balance - amount.clone();
        let from_key = [prefix, from.as_bytes()].concat();
        
        if new_balance.is_zero() {
            Runtime::storage_delete(storage_context, &from_key);
        } else {
            // Convert Int256 to bytes
            let bytes = new_balance.to_i64().to_le_bytes().to_vec();
            Runtime::storage_put(storage_context, &from_key, &bytes);
        }
        
        // Add to receiver
        let to_balance = Self::balance_of(to.clone());
        let to_key = [prefix, to.as_bytes()].concat();
        let new_to_balance = to_balance + amount.clone();
        
        // Convert Int256 to bytes
        let bytes = new_to_balance.to_i64().to_le_bytes().to_vec();
        Runtime::storage_put(storage_context, &to_key, &bytes);
        
        // Emit transfer event
        Self::Transfer(Some(from), Some(to), amount);
        
        true
    });
    
    // Contract lifecycle methods
    pub fn deploy(data: bool) -> bool {
        // Initialize the token
        if data {
            // Parse owner address
            let owner = H160::hex_decode(OWNER_ADDRESS).unwrap_or(H160::zero());
            
            // Get storage context
            let storage_context = Runtime::storage_context();
            let prefix = b"balance";
            
            // Mint initial supply to owner
            let total_supply = Self::total_supply();
            let owner_key = [prefix, owner.as_bytes()].concat();
            
            // Convert Int256 to bytes
            let bytes = total_supply.to_i64().to_le_bytes().to_vec();
            Runtime::storage_put(storage_context, &owner_key, &bytes);
            
            // Emit transfer event
            Self::Transfer(None, Some(owner), total_supply);
        }
        
        true
    }
    
    pub fn initialize() -> bool {
        true
    }
}

impl NEP17 for Token {}
