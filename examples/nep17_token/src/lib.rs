// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use neo_contract::{
    contract::{nep17::*, SmartContract},
    contract_event, contract_method, smart_contract, static_value, storage_map,
    types::*,
    utils::address,
};

pub struct Token;

#[smart_contract]
impl Token {
    // Static values
    static_value!(static OWNER: H160 = "0x13a83e059c2eedd5157b766d3357bc826810905e";);
    
    // Storage maps
    storage_map!(BalanceMap: H160 => Int256);
    
    // Events
    contract_event!(fn Transfer(from: Option<H160>, to: Option<H160>, amount: Int256));
    
    // Methods
    contract_method!(pub fn name() -> ByteString {
        ByteString::new("Example Token")
    });
    
    contract_method!(pub fn symbol() -> ByteString {
        ByteString::new("EXT")
    });
    
    contract_method!(pub fn decimals() -> u32 {
        8
    });
    
    contract_method!(pub fn total_supply() -> Int256 {
        Int256::from(100_000_000_00000000i64)
    });
    
    contract_method!(pub fn balance_of(account: H160) -> Int256 {
        BalanceMap::new().get(&account).unwrap_or_else(Int256::zero)
    });
    
    contract_method!(pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
        // Check if the caller is the owner of the tokens
        if !runtime::check_witness(from) {
            return false;
        }
        
        // Check if the amount is positive
        if amount <= Int256::zero() {
            return false;
        }
        
        // Get the balance of the sender
        let balance = Self::balance_of(from);
        
        // Check if the sender has enough tokens
        if balance < amount {
            return false;
        }
        
        // Update balances
        let balance_map = BalanceMap::new();
        
        // Subtract from sender
        let new_balance = balance - amount;
        if new_balance.is_zero() {
            balance_map.delete(&from);
        } else {
            balance_map.put(&from, &new_balance);
        }
        
        // Add to receiver
        let to_balance = Self::balance_of(to);
        balance_map.put(&to, &(to_balance + amount));
        
        // Emit transfer event
        Transfer(Some(from), Some(to), amount);
        
        true
    });
    
    // Contract lifecycle methods
    pub fn deploy(data: bool) -> bool {
        // Initialize the token
        if data {
            // Mint initial supply to owner
            let balance_map = BalanceMap::new();
            balance_map.put(&OWNER, &Self::total_supply());
            
            // Emit transfer event
            Transfer(None, Some(OWNER), Self::total_supply());
        }
        
        true
    }
    
    pub fn initialize() -> bool {
        true
    }
}

impl SmartContract for Token {}
impl TokenContract for Token {}
