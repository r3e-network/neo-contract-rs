#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
// Solana-style Neo N3 Hello World Contract
pub struct HelloWorld {
    greeting: ByteString,
}

#[contract]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            greeting: ByteString::from_literal("Hello, Neo N3!"),
        }
    }

    #[method]
    pub fn initialize(&self) -> bool {
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("greeting"),
            ByteString::from_literal("Hello, Neo N3!")
        );
        
        Runtime::log(ByteString::from_literal("Contract initialized"));
        true
    }
    
    #[method]
    #[safe]
    pub fn say_hello(&self, name: ByteString) -> ByteString {
        let context = Storage::get_context();
        let greeting = Storage::get(context, ByteString::from_literal("greeting"))
            .unwrap_or(ByteString::from_literal("Hello"));
        
        Runtime::log(ByteString::from_literal("Say hello called"));
        Runtime::notify(ByteString::from_literal("HelloEvent"), Array::new());
        
        greeting
    }
    
    #[method]
    pub fn set_greeting(&self, new_greeting: ByteString) -> bool {
        // Check authorization
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }
        
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("greeting"),
            new_greeting
        );
        
        Runtime::log(ByteString::from_literal("Greeting updated"));
        true
    }
}

