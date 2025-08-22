#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::prelude::*;

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
use neo_contract::types::{IntoByteString, FromByteString, builtin::IntoAny};

// This is a simplified Solana-style example that compiles to Neo N3
// It demonstrates the pattern without using all the complex macros

pub struct HelloWorld {
    greeting: ByteString,
}

#[contract_impl]
impl HelloWorld {
    pub fn init() -> Self {
        Self {
            greeting: ByteString::from_literal("Hello from Solana-style syntax!"),
        }
    }

    // Initialize with custom greeting (Solana-style pattern)
    #[method]
    pub fn initialize(&self, greeting: ByteString) -> bool {
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("greeting"), greeting);
        
        // Emit event (Solana-style)
        let mut event_data = Array::new();
        event_data.push(ByteString::from_literal("Initialized").into_any());
        Runtime::notify(ByteString::from_literal("ProgramInitialized"), event_data);
        
        true
    }

    // Get greeting (marked as safe/read-only)
    #[method]
    #[safe]
    pub fn get_greeting(&self) -> ByteString {
        let storage = Storage::get_context();
        match Storage::get(storage, ByteString::from_literal("greeting")) {
            Some(greeting) => greeting,
            None => self.greeting.clone(),
        }
    }

    // Set greeting (requires authority check)
    #[method]
    pub fn set_greeting(&self, new_greeting: ByteString) -> bool {
        // Check authority (Solana-style pattern)
        let authority = Runtime::get_executing_script_hash();
        if !Runtime::check_witness(authority) {
            return false;
        }
        
        let storage = Storage::get_context();
        Storage::put(storage, ByteString::from_literal("greeting"), new_greeting.clone());
        
        // Emit event
        let mut event_data = Array::new();
        event_data.push(new_greeting.into_any());
        Runtime::notify(ByteString::from_literal("GreetingChanged"), event_data);
        
        true
    }

    // Transfer pattern (common in Solana)
    #[method]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
        // Verify from account has signed
        if !Runtime::check_witness(from) {
            return false;
        }
        
        let storage = Storage::get_context();
        
        // Get balances
        let from_key = ByteString::from_literal("balance:").concat(&from.into_byte_string());
        let to_key = ByteString::from_literal("balance:").concat(&to.into_byte_string());
        
        let from_balance = match Storage::get(storage.clone(), from_key.clone()) {
            Some(b) => Int256::from_byte_string(b),
            None => Int256::zero(),
        };
        
        // Check sufficient balance
        if from_balance < amount {
            return false;
        }
        
        let to_balance = match Storage::get(storage.clone(), to_key.clone()) {
            Some(b) => Int256::from_byte_string(b),
            None => Int256::zero(),
        };
        
        // Update balances
        let new_from_balance = from_balance.checked_sub(&amount);
        let new_to_balance = to_balance.checked_add(&amount);
        
        Storage::put(storage.clone(), from_key, new_from_balance.into_byte_string());
        Storage::put(storage, to_key, new_to_balance.into_byte_string());
        
        // Emit transfer event
        let mut event_data = Array::new();
        event_data.push(from.into_any());
        event_data.push(to.into_any());
        event_data.push(amount.into_any());
        Runtime::notify(ByteString::from_literal("Transfer"), event_data);
        
        true
    }
}