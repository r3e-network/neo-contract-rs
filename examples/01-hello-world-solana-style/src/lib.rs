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
// Pure Solana-style Neo N3 Hello World Contract
pub struct HelloWorld {
    authority: H160,
    greeting: ByteString,
    visitor_count: Int256,
    is_initialized: bool,
}

#[contract]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            authority: H160::zero(),
            greeting: ByteString::from_literal("Hello, Neo N3!"),
            visitor_count: Int256::zero(),
            is_initialized: false,
        }
    }

    #[method]
    pub fn initialize(&self, authority: H160, greeting: ByteString) -> bool {
        let context = Storage::get_context();
        
        // Store authority
        Storage::put(context.clone(), ByteString::from_literal("authority"), authority.into_byte_string());
        
        // Store greeting
        Storage::put(context.clone(), ByteString::from_literal("greeting"), greeting.clone());
        
        // Initialize visitor count
        Storage::put(context.clone(), ByteString::from_literal("visitor_count"), Int256::zero().into_byte_string());
        
        // Mark as initialized
        Storage::put(context, ByteString::from_literal("is_initialized"), ByteString::from_literal("true"));
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(authority.into_any());
        event_data.push(greeting.into_any());
        Runtime::notify(ByteString::from_literal("ContractInitialized"), event_data);
        
        Runtime::log(ByteString::from_literal("Hello World contract initialized"));
        true
    }

    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        let context = Storage::get_context();
        Storage::get(context, ByteString::from_literal("greeting"))
            .unwrap_or(ByteString::from_literal("Hello, Neo N3!"))
    }

    #[method]
    pub fn set_greeting(&self, new_greeting: ByteString) -> bool {
        // Check authorization
        let context = Storage::get_context();
        let stored_authority = match Storage::get(context.clone(), ByteString::from_literal("authority")) {
            Some(auth_bytes) => H160::from_byte_string(auth_bytes),
            None => return false,
        };
        
        if !Runtime::check_witness(stored_authority) {
            return false;
        }
        
        // Update greeting
        Storage::put(context, ByteString::from_literal("greeting"), new_greeting.clone());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(new_greeting.into_any());
        Runtime::notify(ByteString::from_literal("GreetingUpdated"), event_data);
        
        Runtime::log(ByteString::from_literal("Greeting updated"));
        true
    }

    #[method]
    pub fn say_hello(&self, visitor: H160) -> ByteString {
        let context = Storage::get_context();
        
        // Increment visitor count
        let current_count = match Storage::get(context.clone(), ByteString::from_literal("visitor_count")) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        };
        let new_count = current_count.checked_add(&Int256::one());
        Storage::put(context.clone(), ByteString::from_literal("visitor_count"), new_count.into_byte_string());
        
        // Get greeting
        let greeting = Storage::get(context, ByteString::from_literal("greeting"))
            .unwrap_or(ByteString::from_literal("Hello, Neo N3!"));
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(visitor.into_any());
        event_data.push(new_count.into_any());
        Runtime::notify(ByteString::from_literal("VisitorGreeted"), event_data);
        
        Runtime::log(ByteString::from_literal("Visitor greeted"));
        greeting
    }

    #[method]
    #[safe]
    pub fn get_visitor_count(&self) -> Int256 {
        let context = Storage::get_context();
        match Storage::get(context, ByteString::from_literal("visitor_count")) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        }
    }

    #[method]
    #[safe]
    pub fn is_initialized(&self) -> bool {
        let context = Storage::get_context();
        Storage::get(context, ByteString::from_literal("is_initialized")).is_some()
    }
}