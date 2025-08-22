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
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

// Solana-style Neo N3 Simple Storage Contract
pub struct SimpleStorage {
    owner: H160,
}

#[contract_impl]
impl SimpleStorage {
    pub fn init() -> Self {
        Self {
            owner: H160::zero(),
        }
    }

    #[method]
    pub fn initialize(&self, owner: H160) -> bool {
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("owner"),
            owner.into_byte_string()
        );
        
        // Initialize total items counter
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_items"),
            Int256::zero().into_byte_string()
        );
        
        Runtime::log(ByteString::from_literal("Storage initialized"));
        true
    }
    
    #[method]
    pub fn store_string(&self, key: ByteString, value: ByteString) -> bool {
        // Check authorization
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }
        
        let context = Storage::get_context();
        Storage::put(context, key, value);
        
        // Increment counter
        let total = match Storage::get(Storage::get_context(), ByteString::from_literal("total_items")) {
            Some(t) => Int256::from_byte_string(t).checked_add(&Int256::one()),
            None => Int256::one(),
        };
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_items"),
            total.into_byte_string()
        );
        
        Runtime::log(ByteString::from_literal("String stored"));
        true
    }
    
    #[method]
    #[safe]
    pub fn get_string(&self, key: ByteString) -> Option<ByteString> {
        let context = Storage::get_context();
        Storage::get(context, key)
    }
    
    #[method]
    pub fn delete_string(&self, key: ByteString) -> bool {
        // Check authorization
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }
        
        let context = Storage::get_context();
        Storage::delete(context, key);
        
        Runtime::log(ByteString::from_literal("String deleted"));
        true
    }
    
    #[method]
    #[safe]
    pub fn get_total_items(&self) -> Int256 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("total_items")) {
            Some(total) => Int256::from_byte_string(total),
            None => Int256::zero(),
        }
    }
}