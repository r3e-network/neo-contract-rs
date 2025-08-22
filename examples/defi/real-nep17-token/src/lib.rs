#![no_std]
#![no_main]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ops::{Add, Sub};

// Global allocator for no_std
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for no_std
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Storage keys
const SUPPLY_KEY: &[u8] = b"totalSupply";
const BALANCE_PREFIX: &[u8] = b"balance_";
const ALLOWANCE_PREFIX: &[u8] = b"allowance_";
const OWNER_KEY: &[u8] = b"owner";
const PAUSED_KEY: &[u8] = b"paused";
const NAME_KEY: &[u8] = b"name";
const SYMBOL_KEY: &[u8] = b"symbol";
const DECIMALS_KEY: &[u8] = b"decimals";

// Token configuration
const TOKEN_NAME: &str = "Neo DeFi Token";
const TOKEN_SYMBOL: &str = "NDT";
const TOKEN_DECIMALS: u8 = 8;
const INITIAL_SUPPLY: u64 = 100_000_000_00000000; // 100M tokens with 8 decimals

// Storage abstraction
mod storage {
    use alloc::vec::Vec;
    
    extern "C" {
        fn storage_get(key: *const u8, key_len: u32, value: *mut u8, value_len: u32) -> u32;
        fn storage_put(key: *const u8, key_len: u32, value: *const u8, value_len: u32);
        fn storage_delete(key: *const u8, key_len: u32);
    }
    
    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        let mut buffer = Vec::with_capacity(256);
        unsafe {
            buffer.set_len(256);
            let actual_len = storage_get(
                key.as_ptr(),
                key.len() as u32,
                buffer.as_mut_ptr(),
                buffer.len() as u32
            );
            if actual_len > 0 {
                buffer.truncate(actual_len as usize);
                Some(buffer)
            } else {
                None
            }
        }
    }
    
    pub fn put(key: &[u8], value: &[u8]) {
        unsafe {
            storage_put(
                key.as_ptr(),
                key.len() as u32,
                value.as_ptr(),
                value.len() as u32
            );
        }
    }
    
    pub fn delete(key: &[u8]) {
        unsafe {
            storage_delete(key.as_ptr(), key.len() as u32);
        }
    }
}

// Runtime functions
mod runtime {
    extern "C" {
        fn runtime_check_witness(addr: *const u8) -> bool;
        fn runtime_notify(event: *const u8, event_len: u32);
        fn runtime_get_caller() -> *const u8;
        fn runtime_get_time() -> u64;
    }
    
    pub fn check_witness(address: &[u8; 20]) -> bool {
        unsafe { runtime_check_witness(address.as_ptr()) }
    }
    
    pub fn notify(event_data: &[u8]) {
        unsafe {
            runtime_notify(event_data.as_ptr(), event_data.len() as u32);
        }
    }
    
    pub fn get_caller() -> [u8; 20] {
        let mut address = [0u8; 20];
        unsafe {
            let ptr = runtime_get_caller();
            core::ptr::copy_nonoverlapping(ptr, address.as_mut_ptr(), 20);
        }
        address
    }
    
    pub fn get_time() -> u64 {
        unsafe { runtime_get_time() }
    }
}

// Helper functions
fn make_balance_key(address: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(BALANCE_PREFIX.len() + 20);
    key.extend_from_slice(BALANCE_PREFIX);
    key.extend_from_slice(address);
    key
}

fn make_allowance_key(owner: &[u8; 20], spender: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(ALLOWANCE_PREFIX.len() + 40);
    key.extend_from_slice(ALLOWANCE_PREFIX);
    key.extend_from_slice(owner);
    key.extend_from_slice(spender);
    key
}

fn u64_to_bytes(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

fn bytes_to_u64(bytes: &[u8]) -> u64 {
    if bytes.len() >= 8 {
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[..8]);
        u64::from_le_bytes(arr)
    } else {
        0
    }
}

fn get_balance(address: &[u8; 20]) -> u64 {
    let key = make_balance_key(address);
    storage::get(&key)
        .map(|bytes| bytes_to_u64(&bytes))
        .unwrap_or(0)
}

fn set_balance(address: &[u8; 20], amount: u64) {
    let key = make_balance_key(address);
    if amount == 0 {
        storage::delete(&key);
    } else {
        storage::put(&key, &u64_to_bytes(amount));
    }
}

fn get_allowance(owner: &[u8; 20], spender: &[u8; 20]) -> u64 {
    let key = make_allowance_key(owner, spender);
    storage::get(&key)
        .map(|bytes| bytes_to_u64(&bytes))
        .unwrap_or(0)
}

fn set_allowance(owner: &[u8; 20], spender: &[u8; 20], amount: u64) {
    let key = make_allowance_key(owner, spender);
    if amount == 0 {
        storage::delete(&key);
    } else {
        storage::put(&key, &u64_to_bytes(amount));
    }
}

fn get_total_supply() -> u64 {
    storage::get(SUPPLY_KEY)
        .map(|bytes| bytes_to_u64(&bytes))
        .unwrap_or(0)
}

fn set_total_supply(amount: u64) {
    storage::put(SUPPLY_KEY, &u64_to_bytes(amount));
}

fn is_paused() -> bool {
    storage::get(PAUSED_KEY)
        .map(|bytes| bytes[0] != 0)
        .unwrap_or(false)
}

fn get_owner() -> Option<[u8; 20]> {
    storage::get(OWNER_KEY).and_then(|bytes| {
        if bytes.len() >= 20 {
            let mut addr = [0u8; 20];
            addr.copy_from_slice(&bytes[..20]);
            Some(addr)
        } else {
            None
        }
    })
}

fn emit_transfer_event(from: &[u8; 20], to: &[u8; 20], amount: u64) {
    // Create event data: "Transfer" + from + to + amount
    let mut event_data = Vec::with_capacity(8 + 20 + 20 + 8);
    event_data.extend_from_slice(b"Transfer");
    event_data.extend_from_slice(from);
    event_data.extend_from_slice(to);
    event_data.extend_from_slice(&u64_to_bytes(amount));
    runtime::notify(&event_data);
}

fn emit_approval_event(owner: &[u8; 20], spender: &[u8; 20], amount: u64) {
    let mut event_data = Vec::with_capacity(8 + 20 + 20 + 8);
    event_data.extend_from_slice(b"Approval");
    event_data.extend_from_slice(owner);
    event_data.extend_from_slice(spender);
    event_data.extend_from_slice(&u64_to_bytes(amount));
    runtime::notify(&event_data);
}

// NEP-17 standard methods

#[no_mangle]
pub extern "C" fn symbol() -> *const u8 {
    TOKEN_SYMBOL.as_ptr()
}

#[no_mangle]
pub extern "C" fn symbol_len() -> u32 {
    TOKEN_SYMBOL.len() as u32
}

#[no_mangle]
pub extern "C" fn decimals() -> u8 {
    TOKEN_DECIMALS
}

#[no_mangle]
pub extern "C" fn totalSupply() -> u64 {
    get_total_supply()
}

#[no_mangle]
pub extern "C" fn balanceOf(account: *const u8) -> u64 {
    let mut address = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(account, address.as_mut_ptr(), 20);
    }
    get_balance(&address)
}

#[no_mangle]
pub extern "C" fn transfer(from: *const u8, to: *const u8, amount: u64, _data: *const u8) -> bool {
    // Check if paused
    if is_paused() {
        return false;
    }
    
    // Parse addresses
    let mut from_addr = [0u8; 20];
    let mut to_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(from, from_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(to, to_addr.as_mut_ptr(), 20);
    }
    
    // Check witness (authorization)
    if !runtime::check_witness(&from_addr) {
        return false;
    }
    
    // Check balance
    let from_balance = get_balance(&from_addr);
    if from_balance < amount {
        return false;
    }
    
    // Prevent self-transfer
    if from_addr == to_addr {
        return true; // No-op but valid
    }
    
    // Update balances
    let to_balance = get_balance(&to_addr);
    
    // Check for overflow
    if to_balance.checked_add(amount).is_none() {
        return false;
    }
    
    // Perform transfer
    set_balance(&from_addr, from_balance - amount);
    set_balance(&to_addr, to_balance + amount);
    
    // Emit event
    emit_transfer_event(&from_addr, &to_addr, amount);
    
    true
}

// Extended functionality

#[no_mangle]
pub extern "C" fn approve(owner: *const u8, spender: *const u8, amount: u64) -> bool {
    let mut owner_addr = [0u8; 20];
    let mut spender_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(owner, owner_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(spender, spender_addr.as_mut_ptr(), 20);
    }
    
    // Check witness
    if !runtime::check_witness(&owner_addr) {
        return false;
    }
    
    // Set allowance
    set_allowance(&owner_addr, &spender_addr, amount);
    
    // Emit event
    emit_approval_event(&owner_addr, &spender_addr, amount);
    
    true
}

#[no_mangle]
pub extern "C" fn allowance(owner: *const u8, spender: *const u8) -> u64 {
    let mut owner_addr = [0u8; 20];
    let mut spender_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(owner, owner_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(spender, spender_addr.as_mut_ptr(), 20);
    }
    
    get_allowance(&owner_addr, &spender_addr)
}

#[no_mangle]
pub extern "C" fn transferFrom(from: *const u8, to: *const u8, amount: u64) -> bool {
    if is_paused() {
        return false;
    }
    
    let mut from_addr = [0u8; 20];
    let mut to_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(from, from_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(to, to_addr.as_mut_ptr(), 20);
    }
    
    let caller = runtime::get_caller();
    
    // Check allowance
    let current_allowance = get_allowance(&from_addr, &caller);
    if current_allowance < amount {
        return false;
    }
    
    // Check balance
    let from_balance = get_balance(&from_addr);
    if from_balance < amount {
        return false;
    }
    
    // Update allowance
    set_allowance(&from_addr, &caller, current_allowance - amount);
    
    // Update balances
    let to_balance = get_balance(&to_addr);
    if to_balance.checked_add(amount).is_none() {
        return false;
    }
    
    set_balance(&from_addr, from_balance - amount);
    set_balance(&to_addr, to_balance + amount);
    
    emit_transfer_event(&from_addr, &to_addr, amount);
    
    true
}

#[no_mangle]
pub extern "C" fn mint(to: *const u8, amount: u64) -> bool {
    // Only owner can mint
    let caller = runtime::get_caller();
    
    if let Some(owner) = get_owner() {
        if caller != owner {
            return false;
        }
    } else {
        return false;
    }
    
    let mut to_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(to, to_addr.as_mut_ptr(), 20);
    }
    
    let current_supply = get_total_supply();
    let to_balance = get_balance(&to_addr);
    
    // Check for overflow
    if current_supply.checked_add(amount).is_none() ||
       to_balance.checked_add(amount).is_none() {
        return false;
    }
    
    // Update supply and balance
    set_total_supply(current_supply + amount);
    set_balance(&to_addr, to_balance + amount);
    
    // Emit transfer from zero address
    let zero_addr = [0u8; 20];
    emit_transfer_event(&zero_addr, &to_addr, amount);
    
    true
}

#[no_mangle]
pub extern "C" fn burn(amount: u64) -> bool {
    let caller = runtime::get_caller();
    let balance = get_balance(&caller);
    
    if balance < amount {
        return false;
    }
    
    let current_supply = get_total_supply();
    
    // Update balance and supply
    set_balance(&caller, balance - amount);
    set_total_supply(current_supply - amount);
    
    // Emit transfer to zero address
    let zero_addr = [0u8; 20];
    emit_transfer_event(&caller, &zero_addr, amount);
    
    true
}

#[no_mangle]
pub extern "C" fn pause() -> bool {
    let caller = runtime::get_caller();
    
    if let Some(owner) = get_owner() {
        if caller != owner {
            return false;
        }
    } else {
        return false;
    }
    
    storage::put(PAUSED_KEY, &[1u8]);
    true
}

#[no_mangle]
pub extern "C" fn unpause() -> bool {
    let caller = runtime::get_caller();
    
    if let Some(owner) = get_owner() {
        if caller != owner {
            return false;
        }
    } else {
        return false;
    }
    
    storage::put(PAUSED_KEY, &[0u8]);
    true
}

#[no_mangle]
pub extern "C" fn initialize(owner: *const u8) -> bool {
    // Check if already initialized
    if get_total_supply() > 0 {
        return false;
    }
    
    let mut owner_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(owner, owner_addr.as_mut_ptr(), 20);
    }
    
    // Set initial state
    storage::put(OWNER_KEY, &owner_addr);
    storage::put(NAME_KEY, TOKEN_NAME.as_bytes());
    storage::put(SYMBOL_KEY, TOKEN_SYMBOL.as_bytes());
    storage::put(DECIMALS_KEY, &[TOKEN_DECIMALS]);
    storage::put(PAUSED_KEY, &[0u8]);
    
    // Mint initial supply to owner
    set_total_supply(INITIAL_SUPPLY);
    set_balance(&owner_addr, INITIAL_SUPPLY);
    
    // Emit initial mint event
    let zero_addr = [0u8; 20];
    emit_transfer_event(&zero_addr, &owner_addr, INITIAL_SUPPLY);
    
    true
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() {}