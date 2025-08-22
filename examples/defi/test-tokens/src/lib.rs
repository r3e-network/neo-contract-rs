#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;

// Global allocator for no_std
use wee_alloc::WeeAlloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// Panic handler for no_std
// Panic handler removed to avoid conflicts during testing

// NEP-17 Token Implementation
// This is a simplified version that will compile

// Main entry point - required for WASM
#[no_mangle]
pub extern "C" fn _start() {
    // Entry point
}

// Token name
#[no_mangle]
pub extern "C" fn name() -> i32 {
    // Return constant for "TEST"
    1
}

// Token symbol  
#[no_mangle]
pub extern "C" fn symbol() -> i32 {
    // Return constant for "TST"
    2
}

// Token decimals
#[no_mangle]
pub extern "C" fn decimals() -> i32 {
    8
}

// Total supply
#[no_mangle]
pub extern "C" fn totalSupply() -> i64 {
    1000000000
}

// Balance of account
#[no_mangle]
pub extern "C" fn balanceOf(_account: i32) -> i64 {
    // Simplified - return fixed balance
    1000000
}

// Transfer tokens
#[no_mangle]
pub extern "C" fn transfer(_from: i32, _to: i32, _amount: i64, _data: i32) -> bool {
    // Simplified transfer - always succeed
    true
}

// Initialize
#[no_mangle]
pub extern "C" fn initialize() {
    // Initialization logic
}

// Deploy contract
#[no_mangle]
pub extern "C" fn _deploy(_data: i32, _update: bool) {
    // Deploy logic
}