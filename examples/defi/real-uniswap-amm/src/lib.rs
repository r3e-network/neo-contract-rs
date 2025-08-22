#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use neo_contract::prelude::*;
use core::cmp::min;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler removed to avoid conflicts during testing

// Storage keys
const RESERVE0_KEY: &[u8] = b"reserve0";
const RESERVE1_KEY: &[u8] = b"reserve1";
const TOTAL_SUPPLY_KEY: &[u8] = b"totalSupply";
const TOKEN0_KEY: &[u8] = b"token0";
const TOKEN1_KEY: &[u8] = b"token1";
const K_LAST_KEY: &[u8] = b"kLast";
const FEE_TO_KEY: &[u8] = b"feeTo";
const LIQUIDITY_PREFIX: &[u8] = b"liq_";
const MINIMUM_LIQUIDITY: u64 = 1000;
const FEE_RATE: u64 = 3; // 0.3% fee

// Storage module
mod storage {
    use neo_contract::prelude::*;
    
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

// Runtime module
mod runtime {
    extern "C" {
        fn runtime_check_witness(addr: *const u8) -> bool;
        fn runtime_notify(event: *const u8, event_len: u32);
        fn runtime_get_caller() -> *const u8;
        fn contract_call(contract: *const u8, method: *const u8, method_len: u32, 
                         args: *const u8, args_len: u32) -> bool;
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
    
    pub fn call_contract(contract: &[u8; 20], method: &[u8], args: &[u8]) -> bool {
        unsafe {
            contract_call(
                contract.as_ptr(),
                method.as_ptr(),
                method.len() as u32,
                args.as_ptr(),
                args.len() as u32
            )
        }
    }
}

// Math utilities
fn sqrt(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    
    x as u64
}

// Helper functions
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

fn addr_to_bytes(addr: &[u8; 20]) -> Vec<u8> {
    addr.to_vec()
}

fn bytes_to_addr(bytes: &[u8]) -> Option<[u8; 20]> {
    if bytes.len() >= 20 {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&bytes[..20]);
        Some(addr)
    } else {
        None
    }
}

// Storage getters/setters
fn get_reserves() -> (u64, u64) {
    let reserve0 = storage::get(RESERVE0_KEY)
        .map(|b| bytes_to_u64(&b))
        .unwrap_or(0);
    let reserve1 = storage::get(RESERVE1_KEY)
        .map(|b| bytes_to_u64(&b))
        .unwrap_or(0);
    (reserve0, reserve1)
}

fn set_reserves(reserve0: u64, reserve1: u64) {
    storage::put(RESERVE0_KEY, &u64_to_bytes(reserve0));
    storage::put(RESERVE1_KEY, &u64_to_bytes(reserve1));
}

fn get_total_supply() -> u64 {
    storage::get(TOTAL_SUPPLY_KEY)
        .map(|b| bytes_to_u64(&b))
        .unwrap_or(0)
}

fn set_total_supply(supply: u64) {
    storage::put(TOTAL_SUPPLY_KEY, &u64_to_bytes(supply));
}

fn get_tokens() -> Option<([u8; 20], [u8; 20])> {
    let token0 = storage::get(TOKEN0_KEY).and_then(|b| bytes_to_addr(&b))?;
    let token1 = storage::get(TOKEN1_KEY).and_then(|b| bytes_to_addr(&b))?;
    Some((token0, token1))
}

fn set_tokens(token0: &[u8; 20], token1: &[u8; 20]) {
    storage::put(TOKEN0_KEY, &addr_to_bytes(token0));
    storage::put(TOKEN1_KEY, &addr_to_bytes(token1));
}

fn get_k_last() -> u128 {
    storage::get(K_LAST_KEY)
        .and_then(|b| {
            if b.len() >= 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&b[..16]);
                Some(u128::from_le_bytes(arr))
            } else {
                None
            }
        })
        .unwrap_or(0)
}

fn set_k_last(k: u128) {
    storage::put(K_LAST_KEY, &k.to_le_bytes());
}

fn make_liquidity_key(owner: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(LIQUIDITY_PREFIX.len() + 20);
    key.extend_from_slice(LIQUIDITY_PREFIX);
    key.extend_from_slice(owner);
    key
}

fn get_liquidity(owner: &[u8; 20]) -> u64 {
    let key = make_liquidity_key(owner);
    storage::get(&key)
        .map(|b| bytes_to_u64(&b))
        .unwrap_or(0)
}

fn set_liquidity(owner: &[u8; 20], amount: u64) {
    let key = make_liquidity_key(owner);
    if amount == 0 {
        storage::delete(&key);
    } else {
        storage::put(&key, &u64_to_bytes(amount));
    }
}

// Transfer tokens from user to contract
fn transfer_token_from(token: &[u8; 20], from: &[u8; 20], amount: u64) -> bool {
    // Create transfer call arguments
    let mut args = Vec::with_capacity(20 + 20 + 8 + 1);
    args.extend_from_slice(from);
    args.extend_from_slice(&[0u8; 20]); // To this contract
    args.extend_from_slice(&u64_to_bytes(amount));
    args.push(0); // No data
    
    runtime::call_contract(token, b"transfer", &args)
}

// Transfer tokens from contract to user
fn transfer_token_to(token: &[u8; 20], to: &[u8; 20], amount: u64) -> bool {
    let mut args = Vec::with_capacity(20 + 20 + 8 + 1);
    args.extend_from_slice(&[0u8; 20]); // From this contract
    args.extend_from_slice(to);
    args.extend_from_slice(&u64_to_bytes(amount));
    args.push(0);
    
    runtime::call_contract(token, b"transfer", &args)
}

// Emit events
fn emit_mint_event(provider: &[u8; 20], amount0: u64, amount1: u64, liquidity: u64) {
    let mut event = Vec::with_capacity(4 + 20 + 8 + 8 + 8);
    event.extend_from_slice(b"Mint");
    event.extend_from_slice(provider);
    event.extend_from_slice(&u64_to_bytes(amount0));
    event.extend_from_slice(&u64_to_bytes(amount1));
    event.extend_from_slice(&u64_to_bytes(liquidity));
    runtime::notify(&event);
}

fn emit_burn_event(provider: &[u8; 20], amount0: u64, amount1: u64, liquidity: u64) {
    let mut event = Vec::with_capacity(4 + 20 + 8 + 8 + 8);
    event.extend_from_slice(b"Burn");
    event.extend_from_slice(provider);
    event.extend_from_slice(&u64_to_bytes(amount0));
    event.extend_from_slice(&u64_to_bytes(amount1));
    event.extend_from_slice(&u64_to_bytes(liquidity));
    runtime::notify(&event);
}

fn emit_swap_event(user: &[u8; 20], amount0_in: u64, amount1_in: u64, 
                   amount0_out: u64, amount1_out: u64) {
    let mut event = Vec::with_capacity(4 + 20 + 8 + 8 + 8 + 8);
    event.extend_from_slice(b"Swap");
    event.extend_from_slice(user);
    event.extend_from_slice(&u64_to_bytes(amount0_in));
    event.extend_from_slice(&u64_to_bytes(amount1_in));
    event.extend_from_slice(&u64_to_bytes(amount0_out));
    event.extend_from_slice(&u64_to_bytes(amount1_out));
    runtime::notify(&event);
}

fn emit_sync_event(reserve0: u64, reserve1: u64) {
    let mut event = Vec::with_capacity(4 + 8 + 8);
    event.extend_from_slice(b"Sync");
    event.extend_from_slice(&u64_to_bytes(reserve0));
    event.extend_from_slice(&u64_to_bytes(reserve1));
    runtime::notify(&event);
}

// Core AMM functions

#[no_mangle]
pub extern "C" fn initialize(token0: *const u8, token1: *const u8) -> bool {
    // Check if already initialized
    if get_tokens().is_some() {
        return false;
    }
    
    let mut token0_addr = [0u8; 20];
    let mut token1_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(token0, token0_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(token1, token1_addr.as_mut_ptr(), 20);
    }
    
    // Tokens must be different
    if token0_addr == token1_addr {
        return false;
    }
    
    // Order tokens
    let (t0, t1) = if token0_addr < token1_addr {
        (token0_addr, token1_addr)
    } else {
        (token1_addr, token0_addr)
    };
    
    set_tokens(&t0, &t1);
    set_reserves(0, 0);
    set_total_supply(0);
    
    true
}

#[no_mangle]
pub extern "C" fn add_liquidity(amount0_desired: u64, amount1_desired: u64, 
                                amount0_min: u64, amount1_min: u64) -> u64 {
    let caller = runtime::get_caller();
    
    // Check witness
    if !runtime::check_witness(&caller) {
        return 0;
    }
    
    let (token0, token1) = match get_tokens() {
        Some(tokens) => tokens,
        None => return 0,
    };
    
    let (reserve0, reserve1) = get_reserves();
    let total_supply = get_total_supply();
    
    let (amount0, amount1) = if reserve0 == 0 && reserve1 == 0 {
        // First liquidity provision
        (amount0_desired, amount1_desired)
    } else {
        // Calculate optimal amounts based on current ratio
        let amount1_optimal = (amount0_desired as u128 * reserve1 as u128 / reserve0 as u128) as u64;
        
        let (a0, a1) = if amount1_optimal <= amount1_desired {
            if amount1_optimal < amount1_min {
                return 0;
            }
            (amount0_desired, amount1_optimal)
        } else {
            let amount0_optimal = (amount1_desired as u128 * reserve0 as u128 / reserve1 as u128) as u64;
            if amount0_optimal > amount0_desired || amount0_optimal < amount0_min {
                return 0;
            }
            (amount0_optimal, amount1_desired)
        };
        (a0, a1)
    };
    
    // Transfer tokens from user
    if !transfer_token_from(&token0, &caller, amount0) {
        return 0;
    }
    if !transfer_token_from(&token1, &caller, amount1) {
        return 0;
    }
    
    // Calculate liquidity to mint
    let liquidity = if total_supply == 0 {
        // Initial liquidity
        let liq = sqrt((amount0 as u128) * (amount1 as u128));
        // Lock minimum liquidity
        if liq <= MINIMUM_LIQUIDITY {
            return 0;
        }
        set_liquidity(&[0u8; 20], MINIMUM_LIQUIDITY); // Burn to zero address
        liq - MINIMUM_LIQUIDITY
    } else {
        // Proportional liquidity
        let liq0 = (amount0 as u128 * total_supply as u128 / reserve0 as u128) as u64;
        let liq1 = (amount1 as u128 * total_supply as u128 / reserve1 as u128) as u64;
        min(liq0, liq1)
    };
    
    if liquidity == 0 {
        return 0;
    }
    
    // Update state
    let user_liquidity = get_liquidity(&caller);
    set_liquidity(&caller, user_liquidity + liquidity);
    set_total_supply(total_supply + liquidity);
    set_reserves(reserve0 + amount0, reserve1 + amount1);
    
    // Update k
    let k = (reserve0 + amount0) as u128 * (reserve1 + amount1) as u128;
    set_k_last(k);
    
    // Emit events
    emit_mint_event(&caller, amount0, amount1, liquidity);
    emit_sync_event(reserve0 + amount0, reserve1 + amount1);
    
    liquidity
}

#[no_mangle]
pub extern "C" fn remove_liquidity(liquidity: u64, amount0_min: u64, amount1_min: u64) -> bool {
    let caller = runtime::get_caller();
    
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let user_liquidity = get_liquidity(&caller);
    if user_liquidity < liquidity {
        return false;
    }
    
    let (token0, token1) = match get_tokens() {
        Some(tokens) => tokens,
        None => return false,
    };
    
    let (reserve0, reserve1) = get_reserves();
    let total_supply = get_total_supply();
    
    if total_supply == 0 {
        return false;
    }
    
    // Calculate amounts to return
    let amount0 = (liquidity as u128 * reserve0 as u128 / total_supply as u128) as u64;
    let amount1 = (liquidity as u128 * reserve1 as u128 / total_supply as u128) as u64;
    
    if amount0 < amount0_min || amount1 < amount1_min {
        return false;
    }
    
    // Update liquidity
    set_liquidity(&caller, user_liquidity - liquidity);
    set_total_supply(total_supply - liquidity);
    
    // Update reserves
    set_reserves(reserve0 - amount0, reserve1 - amount1);
    
    // Transfer tokens back
    if !transfer_token_to(&token0, &caller, amount0) {
        return false;
    }
    if !transfer_token_to(&token1, &caller, amount1) {
        return false;
    }
    
    // Update k
    let k = (reserve0 - amount0) as u128 * (reserve1 - amount1) as u128;
    set_k_last(k);
    
    // Emit events
    emit_burn_event(&caller, amount0, amount1, liquidity);
    emit_sync_event(reserve0 - amount0, reserve1 - amount1);
    
    true
}

#[no_mangle]
pub extern "C" fn swap(amount0_in: u64, amount1_in: u64, amount0_out: u64, amount1_out: u64) -> bool {
    let caller = runtime::get_caller();
    
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    // Validate swap
    if (amount0_in == 0 && amount1_in == 0) || (amount0_out == 0 && amount1_out == 0) {
        return false;
    }
    
    if (amount0_in > 0 && amount0_out > 0) || (amount1_in > 0 && amount1_out > 0) {
        return false; // Can't swap same token
    }
    
    let (token0, token1) = match get_tokens() {
        Some(tokens) => tokens,
        None => return false,
    };
    
    let (reserve0, reserve1) = get_reserves();
    
    if amount0_out >= reserve0 || amount1_out >= reserve1 {
        return false;
    }
    
    // Transfer input tokens
    if amount0_in > 0 {
        if !transfer_token_from(&token0, &caller, amount0_in) {
            return false;
        }
    }
    if amount1_in > 0 {
        if !transfer_token_from(&token1, &caller, amount1_in) {
            return false;
        }
    }
    
    // Transfer output tokens
    if amount0_out > 0 {
        if !transfer_token_to(&token0, &caller, amount0_out) {
            return false;
        }
    }
    if amount1_out > 0 {
        if !transfer_token_to(&token1, &caller, amount1_out) {
            return false;
        }
    }
    
    // Update reserves
    let new_reserve0 = reserve0 + amount0_in - amount0_out;
    let new_reserve1 = reserve1 + amount1_in - amount1_out;
    
    // Check constant product formula with fees
    // (x + 0.997 * dx) * (y - dy) >= x * y
    let balance0_adjusted = (new_reserve0 as u128) * 1000 - (amount0_in as u128) * (FEE_RATE as u128);
    let balance1_adjusted = (new_reserve1 as u128) * 1000 - (amount1_in as u128) * (FEE_RATE as u128);
    
    let k_before = (reserve0 as u128) * (reserve1 as u128) * 1000000;
    let k_after = balance0_adjusted * balance1_adjusted;
    
    if k_after < k_before {
        return false;
    }
    
    set_reserves(new_reserve0, new_reserve1);
    
    // Emit events
    emit_swap_event(&caller, amount0_in, amount1_in, amount0_out, amount1_out);
    emit_sync_event(new_reserve0, new_reserve1);
    
    true
}

#[no_mangle]
pub extern "C" fn get_amount_out(amount_in: u64, reserve_in: u64, reserve_out: u64) -> u64 {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
        return 0;
    }
    
    let amount_in_with_fee = (amount_in as u128) * (1000 - FEE_RATE) as u128;
    let numerator = amount_in_with_fee * (reserve_out as u128);
    let denominator = (reserve_in as u128) * 1000 + amount_in_with_fee;
    
    (numerator / denominator) as u64
}

#[no_mangle]
pub extern "C" fn get_amount_in(amount_out: u64, reserve_in: u64, reserve_out: u64) -> u64 {
    if amount_out == 0 || reserve_in == 0 || reserve_out == 0 {
        return 0;
    }
    
    if amount_out >= reserve_out {
        return u64::MAX; // Invalid
    }
    
    let numerator = (reserve_in as u128) * (amount_out as u128) * 1000;
    let denominator = ((reserve_out - amount_out) as u128) * ((1000 - FEE_RATE) as u128);
    
    (numerator / denominator + 1) as u64
}

#[no_mangle]
pub extern "C" fn quote(amount_a: u64, reserve_a: u64, reserve_b: u64) -> u64 {
    if amount_a == 0 || reserve_a == 0 || reserve_b == 0 {
        return 0;
    }
    
    ((amount_a as u128) * (reserve_b as u128) / (reserve_a as u128)) as u64
}

#[no_mangle]
pub extern "C" fn get_reserves_view() -> u128 {
    let (r0, r1) = get_reserves();
    ((r0 as u128) << 64) | (r1 as u128)
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() {}