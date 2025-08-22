#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString};
use neo_contract::serialize::NeoSerializable;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
#[cfg(target_arch = "wasm32")]
// Panic handler removed to avoid conflicts during testing

// Solana-style Neo N3 Simple Token Contract
pub struct SimpleToken {
    owner: H160,
    symbol: ByteString,
    decimals: u32,
    total_supply: Int256,
    paused: bool,
}

#[contract_impl]
impl SimpleToken {
    pub fn init() -> Self {
        Self {
            owner: H160::zero(),
            symbol: ByteString::from_literal("STK"),
            decimals: 8,
            total_supply: Int256::zero(),
            paused: false,
        }
    }

    #[method]
    pub fn initialize(&self, owner: H160, symbol: ByteString, initial_supply: Int256) -> bool {
        let context = Storage::get_context();
        
        // Store token metadata
        Storage::put(context.clone(), ByteString::from_literal("owner"), owner.into_byte_string());
        Storage::put(context.clone(), ByteString::from_literal("symbol"), symbol);
        Storage::put(context.clone(), ByteString::from_literal("decimals"), ByteString::from_literal("8"));
        Storage::put(context.clone(), ByteString::from_literal("total_supply"), initial_supply.into_byte_string());
        Storage::put(context.clone(), ByteString::from_literal("paused"), ByteString::from_literal("false"));
        
        // Set initial balance for owner
        let balance_key = ByteString::from_literal("balance:").concat(&owner.into_byte_string());
        Storage::put(context, balance_key, initial_supply.into_byte_string());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(owner.into_any());
        event_data.push(initial_supply.into_any());
        Runtime::notify(ByteString::from_literal("TokenInitialized"), event_data);
        
        Runtime::log(ByteString::from_literal("Simple token initialized"));
        true
    }

    #[method]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
        // Check authorization
        if !Runtime::check_witness(from) {
            return false;
        }
        
        // Check if paused
        let context = Storage::get_context();
        if Storage::get(context.clone(), ByteString::from_literal("paused")).is_some() {
            return false;
        }
        
        // Check and update balances
        let from_key = ByteString::from_literal("balance:").concat(&from.into_byte_string());
        let to_key = ByteString::from_literal("balance:").concat(&to.into_byte_string());
        
        let from_balance = match Storage::get(context.clone(), from_key.clone()) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        };
        
        if from_balance < amount {
            return false;
        }
        
        let to_balance = match Storage::get(context.clone(), to_key.clone()) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        };
        
        // Update balances
        let new_from_balance = from_balance.checked_sub(&amount);
        let new_to_balance = to_balance.checked_add(&amount);
        
        Storage::put(context.clone(), from_key, new_from_balance.into_byte_string());
        Storage::put(context, to_key, new_to_balance.into_byte_string());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), event_data);
        
        true
    }

    #[method]
    #[safe]
    pub fn balance_of(&self, account: H160) -> Int256 {
        let context = Storage::get_context();
        let balance_key = ByteString::from_literal("balance:").concat(&account.into_byte_string());
        match Storage::get(context, balance_key) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        }
    }

    #[method]
    #[safe]
    pub fn total_supply(&self) -> Int256 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("total_supply")) {
            Some(supply_bytes) => Int256::from_byte_string(supply_bytes),
            None => Int256::zero(),
        }
    }

    #[method]
    #[safe]
    pub fn symbol(&self) -> ByteString {
        let context = Storage::get_context();
        Storage::get(context, ByteString::from_literal("symbol"))
            .unwrap_or(ByteString::from_literal("STK"))
    }

    #[method]
    #[safe]
    pub fn decimals(&self) -> u32 {
        8 // Fixed at 8 decimals for simplicity
    }

    #[method]
    pub fn mint(&self, to: H160, amount: Int256) -> bool {
        // Check authorization (only owner can mint)
        let context = Storage::get_context();
        let owner = match Storage::get(context.clone(), ByteString::from_literal("owner")) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => return false,
        };
        
        if !Runtime::check_witness(owner) {
            return false;
        }
        
        // Update recipient balance
        let balance_key = ByteString::from_literal("balance:").concat(&to.into_byte_string());
        let current_balance = match Storage::get(context.clone(), balance_key.clone()) {
            Some(balance_bytes) => Int256::from_byte_string(balance_bytes),
            None => Int256::zero(),
        };
        let new_balance = current_balance.checked_add(&amount);
        Storage::put(context.clone(), balance_key, new_balance.into_byte_string());
        
        // Update total supply
        let current_supply = match Storage::get(context.clone(), ByteString::from_literal("total_supply")) {
            Some(supply_bytes) => Int256::from_byte_string(supply_bytes),
            None => Int256::zero(),
        };
        let new_supply = current_supply.checked_add(&amount);
        Storage::put(context, ByteString::from_literal("total_supply"), new_supply.into_byte_string());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Mint"), event_data);
        
        true
    }

    #[method]
    pub fn pause(&self) -> bool {
        // Check authorization
        let context = Storage::get_context();
        let owner = match Storage::get(context.clone(), ByteString::from_literal("owner")) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => return false,
        };
        
        if !Runtime::check_witness(owner) {
            return false;
        }
        
        Storage::put(context, ByteString::from_literal("paused"), ByteString::from_literal("true"));
        Runtime::log(ByteString::from_literal("Token paused"));
        true
    }

    #[method]
    pub fn unpause(&self) -> bool {
        // Check authorization
        let context = Storage::get_context();
        let owner = match Storage::get(context.clone(), ByteString::from_literal("owner")) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => return false,
        };
        
        if !Runtime::check_witness(owner) {
            return false;
        }
        
        Storage::delete(context, ByteString::from_literal("paused"));
        Runtime::log(ByteString::from_literal("Token unpaused"));
        true
    }
}