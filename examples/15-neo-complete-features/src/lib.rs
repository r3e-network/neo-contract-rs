#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Neo N3 Complete Features Contract
pub struct NeoCompleteFeatures {
    owner: H160,
    is_initialized: bool,
}

#[contract]
impl NeoCompleteFeatures {
    pub fn init() -> Self {
        Self {
            owner: H160::zero(),
            is_initialized: false,
        }
    }

    #[method]
    pub fn initialize(&self, owner: H160) -> bool {
        let context = Storage::get_context();
        Storage::put(context.clone(), ByteString::from_literal("owner"), owner.into_byte_string());
        Storage::put(context, ByteString::from_literal("initialized"), ByteString::from_literal("true"));
        
        Runtime::log(ByteString::from_literal("Contract initialized with all Neo N3 features"));
        true
    }

    #[method]
    #[safe]
    pub fn get_owner(&self) -> H160 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("owner")) {
            Some(owner_bytes) => H160::from_bytes(&owner_bytes.to_bytes()),
            None => H160::zero(),
        }
    }

    #[method]
    #[safe]
    pub fn is_initialized(&self) -> bool {
        let context = Storage::get_context();
        Storage::get(context, ByteString::from_literal("initialized")).is_some()
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
        
        Runtime::log(ByteString::from_literal("Storage demonstration complete"));
        true
    }

    #[method]
    #[safe]
    pub fn get_stored_value(&self, key: ByteString) -> Option<ByteString> {
        let context = Storage::get_context();
        Storage::get(context, key)
    }
}