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
// Solana-style Neo N3 Features Showcase Contract
pub struct NeoFeaturesShowcase {
    owner: H160,
}

#[contract]
impl NeoFeaturesShowcase {
    pub fn init() -> Self {
        Self {
            owner: H160::zero(),
        }
    }

    #[method]
    pub fn initialize(&self, owner: H160) -> bool {
        let context = Storage::get_context();
        Storage::put(
            context.clone(),
            ByteString::from_literal("owner"),
            owner.into_byte_string()
        );
        
        Storage::put(
            context,
            ByteString::from_literal("oracle_enabled"),
            ByteString::from_literal("true")
        );
        
        Runtime::log(ByteString::from_literal("Neo Features Showcase initialized"));
        true
    }

    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("owner")) {
            Some(owner_bytes) => H160::from_byte_string(owner_bytes),
            None => H160::zero(),
        }
    }

    #[method]
    pub fn demonstrate_storage(&self, key: ByteString, value: ByteString) -> bool {
        // Check authorization
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }

        let context = Storage::get_context();
        Storage::put(context, key, value);
        
        Runtime::log(ByteString::from_literal("Storage feature demonstrated"));
        true
    }

    #[method]
    #[safe]
    pub fn get_time(&self) -> u64 {
        Runtime::get_time()
    }

    #[method]
    #[safe]
    pub fn get_script_hash(&self) -> H160 {
        Runtime::get_executing_script_hash()
    }
}