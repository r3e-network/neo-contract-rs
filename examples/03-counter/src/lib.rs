#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString};

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
// Solana-style Neo N3 Counter Contract
pub struct Counter {
    count: Int256,
}

#[contract]
impl Counter {
    pub fn init() -> Self {
        Self {
            count: Int256::zero(),
        }
    }

    #[method]
    pub fn initialize(&self) -> bool {
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("counter"),
            Int256::zero().into_byte_string()
        );
        
        Runtime::log(ByteString::from_literal("Counter initialized"));
        true
    }
    
    #[method]
    pub fn increment(&self) -> Int256 {
        let context = Storage::get_context();
        let current = match Storage::get(context.clone(), ByteString::from_literal("counter")) {
            Some(c) => Int256::from_byte_string(c),
            None => Int256::zero(),
        };
        
        let new_count = current.checked_add(&Int256::one());
        Storage::put(context, ByteString::from_literal("counter"), new_count.into_byte_string());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(new_count.into_any());
        Runtime::notify(ByteString::from_literal("CounterIncremented"), event_data);
        
        new_count
    }
    
    #[method]
    pub fn decrement(&self) -> Int256 {
        let context = Storage::get_context();
        let current = match Storage::get(context.clone(), ByteString::from_literal("counter")) {
            Some(c) => Int256::from_byte_string(c),
            None => Int256::zero(),
        };
        
        let new_count = if current > Int256::zero() {
            current.checked_sub(&Int256::one())
        } else {
            Int256::zero()
        };
        
        Storage::put(context, ByteString::from_literal("counter"), new_count.into_byte_string());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(new_count.into_any());
        Runtime::notify(ByteString::from_literal("CounterDecremented"), event_data);
        
        new_count
    }
    
    #[method]
    #[safe]
    pub fn get_count(&self) -> Int256 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("counter")) {
            Some(c) => Int256::from_byte_string(c),
            None => Int256::zero(),
        }
    }
    
    #[method]
    pub fn reset(&self) -> bool {
        // Check authorization
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }
        
        let context = Storage::get_context();
        Storage::put(context, ByteString::from_literal("counter"), Int256::zero().into_byte_string());
        
        // Emit event
        Runtime::notify(ByteString::from_literal("CounterReset"), Array::new());
        
        true
    }
}