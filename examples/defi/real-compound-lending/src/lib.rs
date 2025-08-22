#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use neo_contract::prelude::*;
use core::cmp::min;

// Global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler removed to avoid conflicts during testing

// Constants
const BLOCKS_PER_YEAR: u64 = 2_102_400; // ~15 seconds per block
const BASE_RATE: u64 = 2_000_000_000_000_000; // 2% annual
const MULTIPLIER: u64 = 20_000_000_000_000_000; // 20% increase per 100% utilization
const COLLATERAL_FACTOR: u64 = 750_000_000_000_000_000; // 75%
const LIQUIDATION_INCENTIVE: u64 = 1_080_000_000_000_000_000; // 8% bonus
const MANTISSA: u64 = 1_000_000_000_000_000_000; // 1e18

// Storage keys
const MARKETS_PREFIX: &[u8] = b"market_";
const SUPPLY_PREFIX: &[u8] = b"supply_";
const BORROW_PREFIX: &[u8] = b"borrow_";
const COLLATERAL_PREFIX: &[u8] = b"collat_";
const ORACLE_PREFIX: &[u8] = b"price_";
const ADMIN_KEY: &[u8] = b"admin";
const PAUSED_KEY: &[u8] = b"paused";

// Market state structure (packed into bytes)
struct Market {
    asset: [u8; 20],
    total_supply: u64,
    total_borrows: u64,
    total_reserves: u64,
    supply_index: u64,
    borrow_index: u64,
    last_accrual_block: u32,
    reserve_factor: u32,
    collateral_factor: u64,
    is_listed: bool,
}

// Storage module
mod storage {
    use alloc::vec::Vec;
    
    extern "C" {
        fn storage_get(key: *const u8, key_len: u32, value: *mut u8, value_len: u32) -> u32;
        fn storage_put(key: *const u8, key_len: u32, value: *const u8, value_len: u32);
        fn storage_delete(key: *const u8, key_len: u32);
    }
    
    pub fn get(key: &[u8]) -> Option<Vec<u8>> {
        let mut buffer = Vec::with_capacity(512);
        unsafe {
            buffer.set_len(512);
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
        fn runtime_get_block() -> u32;
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
    
    pub fn get_block() -> u32 {
        unsafe { runtime_get_block() }
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

// Helper functions
fn u64_to_bytes(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

fn u32_to_bytes(value: u32) -> [u8; 4] {
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

fn bytes_to_u32(bytes: &[u8]) -> u32 {
    if bytes.len() >= 4 {
        let mut arr = [0u8; 4];
        arr.copy_from_slice(&bytes[..4]);
        u32::from_le_bytes(arr)
    } else {
        0
    }
}

// Market storage
fn make_market_key(asset: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(MARKETS_PREFIX.len() + 20);
    key.extend_from_slice(MARKETS_PREFIX);
    key.extend_from_slice(asset);
    key
}

fn get_market(asset: &[u8; 20]) -> Option<Market> {
    let key = make_market_key(asset);
    storage::get(&key).and_then(|bytes| {
        if bytes.len() >= 89 {
            Some(Market {
                asset: *asset,
                total_supply: bytes_to_u64(&bytes[0..8]),
                total_borrows: bytes_to_u64(&bytes[8..16]),
                total_reserves: bytes_to_u64(&bytes[16..24]),
                supply_index: bytes_to_u64(&bytes[24..32]),
                borrow_index: bytes_to_u64(&bytes[32..40]),
                last_accrual_block: bytes_to_u32(&bytes[40..44]),
                reserve_factor: bytes_to_u32(&bytes[44..48]),
                collateral_factor: bytes_to_u64(&bytes[48..56]),
                is_listed: bytes[56] != 0,
            })
        } else {
            None
        }
    })
}

fn save_market(market: &Market) {
    let key = make_market_key(&market.asset);
    let mut data = Vec::with_capacity(89);
    data.extend_from_slice(&u64_to_bytes(market.total_supply));
    data.extend_from_slice(&u64_to_bytes(market.total_borrows));
    data.extend_from_slice(&u64_to_bytes(market.total_reserves));
    data.extend_from_slice(&u64_to_bytes(market.supply_index));
    data.extend_from_slice(&u64_to_bytes(market.borrow_index));
    data.extend_from_slice(&u32_to_bytes(market.last_accrual_block));
    data.extend_from_slice(&u32_to_bytes(market.reserve_factor));
    data.extend_from_slice(&u64_to_bytes(market.collateral_factor));
    data.push(if market.is_listed { 1 } else { 0 });
    storage::put(&key, &data);
}

// User position storage
fn make_supply_key(user: &[u8; 20], asset: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(SUPPLY_PREFIX.len() + 40);
    key.extend_from_slice(SUPPLY_PREFIX);
    key.extend_from_slice(user);
    key.extend_from_slice(asset);
    key
}

fn make_borrow_key(user: &[u8; 20], asset: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(BORROW_PREFIX.len() + 40);
    key.extend_from_slice(BORROW_PREFIX);
    key.extend_from_slice(user);
    key.extend_from_slice(asset);
    key
}

fn make_collateral_key(user: &[u8; 20], asset: &[u8; 20]) -> Vec<u8> {
    let mut key = Vec::with_capacity(COLLATERAL_PREFIX.len() + 40);
    key.extend_from_slice(COLLATERAL_PREFIX);
    key.extend_from_slice(user);
    key.extend_from_slice(asset);
    key
}

fn get_supply_balance(user: &[u8; 20], asset: &[u8; 20]) -> (u64, u64) {
    let key = make_supply_key(user, asset);
    storage::get(&key).map(|bytes| {
        if bytes.len() >= 16 {
            (bytes_to_u64(&bytes[0..8]), bytes_to_u64(&bytes[8..16]))
        } else {
            (0, 0)
        }
    }).unwrap_or((0, 0))
}

fn set_supply_balance(user: &[u8; 20], asset: &[u8; 20], principal: u64, index: u64) {
    let key = make_supply_key(user, asset);
    if principal == 0 {
        storage::delete(&key);
    } else {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&u64_to_bytes(principal));
        data.extend_from_slice(&u64_to_bytes(index));
        storage::put(&key, &data);
    }
}

fn get_borrow_balance(user: &[u8; 20], asset: &[u8; 20]) -> (u64, u64) {
    let key = make_borrow_key(user, asset);
    storage::get(&key).map(|bytes| {
        if bytes.len() >= 16 {
            (bytes_to_u64(&bytes[0..8]), bytes_to_u64(&bytes[8..16]))
        } else {
            (0, 0)
        }
    }).unwrap_or((0, 0))
}

fn set_borrow_balance(user: &[u8; 20], asset: &[u8; 20], principal: u64, index: u64) {
    let key = make_borrow_key(user, asset);
    if principal == 0 {
        storage::delete(&key);
    } else {
        let mut data = Vec::with_capacity(16);
        data.extend_from_slice(&u64_to_bytes(principal));
        data.extend_from_slice(&u64_to_bytes(index));
        storage::put(&key, &data);
    }
}

fn is_collateral(user: &[u8; 20], asset: &[u8; 20]) -> bool {
    let key = make_collateral_key(user, asset);
    storage::get(&key).map(|b| b.len() > 0 && b[0] != 0).unwrap_or(false)
}

fn set_collateral(user: &[u8; 20], asset: &[u8; 20], enabled: bool) {
    let key = make_collateral_key(user, asset);
    if enabled {
        storage::put(&key, &[1u8]);
    } else {
        storage::delete(&key);
    }
}

// Oracle functions
fn get_price(asset: &[u8; 20]) -> u64 {
    let mut key = Vec::with_capacity(ORACLE_PREFIX.len() + 20);
    key.extend_from_slice(ORACLE_PREFIX);
    key.extend_from_slice(asset);
    storage::get(&key).map(|b| bytes_to_u64(&b)).unwrap_or(0)
}

fn set_price(asset: &[u8; 20], price: u64) {
    let mut key = Vec::with_capacity(ORACLE_PREFIX.len() + 20);
    key.extend_from_slice(ORACLE_PREFIX);
    key.extend_from_slice(asset);
    storage::put(&key, &u64_to_bytes(price));
}

// Interest rate calculation
fn calculate_interest_rate(cash: u64, borrows: u64, reserves: u64) -> u64 {
    if borrows == 0 {
        return BASE_RATE;
    }
    
    let total = cash + borrows - reserves;
    if total == 0 {
        return BASE_RATE;
    }
    
    let utilization = (borrows as u128 * MANTISSA as u128 / total as u128) as u64;
    let rate = BASE_RATE + (utilization as u128 * MULTIPLIER as u128 / MANTISSA as u128) as u64;
    
    rate
}

// Accrue interest
fn accrue_interest(market: &mut Market) -> bool {
    let current_block = runtime::get_block();
    if current_block == market.last_accrual_block {
        return true;
    }
    
    let blocks_elapsed = (current_block - market.last_accrual_block) as u64;
    
    // Calculate interest
    let borrow_rate = calculate_interest_rate(
        market.total_supply - market.total_borrows,
        market.total_borrows,
        market.total_reserves
    );
    
    let interest_accumulated = (market.total_borrows as u128 * borrow_rate as u128 
        * blocks_elapsed as u128 / BLOCKS_PER_YEAR as u128 / MANTISSA as u128) as u64;
    
    // Update market state
    market.total_borrows += interest_accumulated;
    market.total_reserves += (interest_accumulated as u128 * market.reserve_factor as u128 
        / MANTISSA as u128) as u64;
    
    // Update indices
    if market.total_supply > 0 {
        let supply_interest = (interest_accumulated as u128 * ((MANTISSA as u128) - (market.reserve_factor as u128)) 
            / (MANTISSA as u128)) as u64;
        market.supply_index += (market.supply_index as u128 * supply_interest as u128 
            / market.total_supply as u128) as u64;
    }
    
    if market.total_borrows > 0 {
        market.borrow_index += (market.borrow_index as u128 * interest_accumulated as u128 
            / market.total_borrows as u128) as u64;
    }
    
    market.last_accrual_block = current_block;
    
    true
}

// Calculate account liquidity
fn calculate_liquidity(user: &[u8; 20]) -> (u64, u64) {
    let mut collateral_value = 0u64;
    let mut borrow_value = 0u64;
    
    // Iterate through all markets (simplified - would need market list)
    // For demo, checking a few predefined assets
    let assets = [
        [1u8; 20], // Asset 1
        [2u8; 20], // Asset 2
        [3u8; 20], // Asset 3
    ];
    
    for asset in &assets {
        if let Some(market) = get_market(asset) {
            let price = get_price(asset);
            
            // Add collateral value
            if is_collateral(user, asset) {
                let (principal, index) = get_supply_balance(user, asset);
                let balance = (principal as u128 * market.supply_index as u128 / index as u128) as u64;
                let value = (balance as u128 * price as u128 / MANTISSA as u128) as u64;
                collateral_value += (value as u128 * market.collateral_factor as u128 / MANTISSA as u128) as u64;
            }
            
            // Add borrow value
            let (principal, index) = get_borrow_balance(user, asset);
            if principal > 0 {
                let balance = (principal as u128 * market.borrow_index as u128 / index as u128) as u64;
                let value = (balance as u128 * price as u128 / MANTISSA as u128) as u64;
                borrow_value += value;
            }
        }
    }
    
    (collateral_value, borrow_value)
}

// Token transfer helpers
fn transfer_from(token: &[u8; 20], from: &[u8; 20], amount: u64) -> bool {
    let mut args = Vec::with_capacity(20 + 20 + 8 + 1);
    args.extend_from_slice(from);
    args.extend_from_slice(&[0u8; 20]); // To this contract
    args.extend_from_slice(&u64_to_bytes(amount));
    args.push(0);
    runtime::call_contract(token, b"transfer", &args)
}

fn transfer_to(token: &[u8; 20], to: &[u8; 20], amount: u64) -> bool {
    let mut args = Vec::with_capacity(20 + 20 + 8 + 1);
    args.extend_from_slice(&[0u8; 20]); // From this contract
    args.extend_from_slice(to);
    args.extend_from_slice(&u64_to_bytes(amount));
    args.push(0);
    runtime::call_contract(token, b"transfer", &args)
}

// Core lending functions

#[no_mangle]
pub extern "C" fn initialize_market(asset: *const u8, reserve_factor: u32) -> bool {
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    // Check if market exists
    if get_market(&asset_addr).is_some() {
        return false;
    }
    
    // Create new market
    let market = Market {
        asset: asset_addr,
        total_supply: 0,
        total_borrows: 0,
        total_reserves: 0,
        supply_index: MANTISSA,
        borrow_index: MANTISSA,
        last_accrual_block: runtime::get_block(),
        reserve_factor,
        collateral_factor: COLLATERAL_FACTOR,
        is_listed: true,
    };
    
    save_market(&market);
    
    // Set initial price (would come from oracle)
    set_price(&asset_addr, MANTISSA); // $1 for demo
    
    true
}

#[no_mangle]
pub extern "C" fn supply(asset: *const u8, amount: u64) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    let mut market = match get_market(&asset_addr) {
        Some(m) => m,
        None => return false,
    };
    
    // Accrue interest
    if !accrue_interest(&mut market) {
        return false;
    }
    
    // Transfer tokens from user
    if !transfer_from(&asset_addr, &caller, amount) {
        return false;
    }
    
    // Calculate cTokens to mint
    let (current_principal, current_index) = get_supply_balance(&caller, &asset_addr);
    let current_balance = if current_index > 0 {
        (current_principal as u128 * market.supply_index as u128 / current_index as u128) as u64
    } else {
        0
    };
    
    let new_balance = current_balance + amount;
    let new_principal = (new_balance as u128 * MANTISSA as u128 / market.supply_index as u128) as u64;
    
    // Update user balance
    set_supply_balance(&caller, &asset_addr, new_principal, market.supply_index);
    
    // Update market
    market.total_supply += amount;
    save_market(&market);
    
    // Emit event
    let mut event = Vec::with_capacity(6 + 20 + 20 + 8);
    event.extend_from_slice(b"Supply");
    event.extend_from_slice(&caller);
    event.extend_from_slice(&asset_addr);
    event.extend_from_slice(&u64_to_bytes(amount));
    runtime::notify(&event);
    
    true
}

#[no_mangle]
pub extern "C" fn redeem(asset: *const u8, amount: u64) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    let mut market = match get_market(&asset_addr) {
        Some(m) => m,
        None => return false,
    };
    
    // Accrue interest
    if !accrue_interest(&mut market) {
        return false;
    }
    
    // Check balance
    let (principal, index) = get_supply_balance(&caller, &asset_addr);
    let balance = (principal as u128 * market.supply_index as u128 / index as u128) as u64;
    
    if balance < amount {
        return false;
    }
    
    // Check liquidity if used as collateral
    if is_collateral(&caller, &asset_addr) {
        let (collateral, borrows) = calculate_liquidity(&caller);
        let price = get_price(&asset_addr);
        let reduction = (amount as u128 * price as u128 * market.collateral_factor as u128 
            / MANTISSA as u128 / MANTISSA as u128) as u64;
        
        if collateral < borrows + reduction {
            return false;
        }
    }
    
    // Update balance
    let new_balance = balance - amount;
    let new_principal = if new_balance > 0 {
        (new_balance as u128 * MANTISSA as u128 / market.supply_index as u128) as u64
    } else {
        0
    };
    
    set_supply_balance(&caller, &asset_addr, new_principal, market.supply_index);
    
    // Update market
    market.total_supply -= amount;
    save_market(&market);
    
    // Transfer tokens to user
    if !transfer_to(&asset_addr, &caller, amount) {
        return false;
    }
    
    true
}

#[no_mangle]
pub extern "C" fn borrow(asset: *const u8, amount: u64) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    let mut market = match get_market(&asset_addr) {
        Some(m) => m,
        None => return false,
    };
    
    // Accrue interest
    if !accrue_interest(&mut market) {
        return false;
    }
    
    // Check liquidity
    let (collateral, borrows) = calculate_liquidity(&caller);
    let price = get_price(&asset_addr);
    let borrow_value = (amount as u128 * price as u128 / MANTISSA as u128) as u64;
    
    if collateral < borrows + borrow_value {
        return false;
    }
    
    // Check cash
    let cash = market.total_supply - market.total_borrows;
    if cash < amount {
        return false;
    }
    
    // Update borrow balance
    let (principal, index) = get_borrow_balance(&caller, &asset_addr);
    let balance = if index > 0 {
        (principal as u128 * market.borrow_index as u128 / index as u128) as u64
    } else {
        0
    };
    
    let new_balance = balance + amount;
    let new_principal = (new_balance as u128 * MANTISSA as u128 / market.borrow_index as u128) as u64;
    
    set_borrow_balance(&caller, &asset_addr, new_principal, market.borrow_index);
    
    // Update market
    market.total_borrows += amount;
    save_market(&market);
    
    // Transfer tokens to user
    if !transfer_to(&asset_addr, &caller, amount) {
        return false;
    }
    
    true
}

#[no_mangle]
pub extern "C" fn repay(asset: *const u8, amount: u64) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    let mut market = match get_market(&asset_addr) {
        Some(m) => m,
        None => return false,
    };
    
    // Accrue interest
    if !accrue_interest(&mut market) {
        return false;
    }
    
    // Get borrow balance
    let (principal, index) = get_borrow_balance(&caller, &asset_addr);
    let balance = (principal as u128 * market.borrow_index as u128 / index as u128) as u64;
    
    let repay_amount = min(amount, balance);
    
    // Transfer tokens from user
    if !transfer_from(&asset_addr, &caller, repay_amount) {
        return false;
    }
    
    // Update balance
    let new_balance = balance - repay_amount;
    let new_principal = if new_balance > 0 {
        (new_balance as u128 * MANTISSA as u128 / market.borrow_index as u128) as u64
    } else {
        0
    };
    
    set_borrow_balance(&caller, &asset_addr, new_principal, market.borrow_index);
    
    // Update market
    market.total_borrows -= repay_amount;
    save_market(&market);
    
    true
}

#[no_mangle]
pub extern "C" fn liquidate(borrower: *const u8, collateral_asset: *const u8, 
                           borrow_asset: *const u8, repay_amount: u64) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut borrower_addr = [0u8; 20];
    let mut collateral_addr = [0u8; 20];
    let mut borrow_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(borrower, borrower_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(collateral_asset, collateral_addr.as_mut_ptr(), 20);
        core::ptr::copy_nonoverlapping(borrow_asset, borrow_addr.as_mut_ptr(), 20);
    }
    
    // Check if borrower is liquidatable
    let (collateral, borrows) = calculate_liquidity(&borrower_addr);
    if collateral >= borrows {
        return false; // Not liquidatable
    }
    
    // Accrue interest for both markets
    let mut borrow_market = match get_market(&borrow_addr) {
        Some(m) => m,
        None => return false,
    };
    
    let mut collateral_market = match get_market(&collateral_addr) {
        Some(m) => m,
        None => return false,
    };
    
    accrue_interest(&mut borrow_market);
    accrue_interest(&mut collateral_market);
    
    // Calculate max repay (50% of borrow)
    let (borrow_principal, borrow_index) = get_borrow_balance(&borrower_addr, &borrow_addr);
    let borrow_balance = (borrow_principal as u128 * borrow_market.borrow_index as u128 
        / borrow_index as u128) as u64;
    
    let max_repay = borrow_balance / 2;
    let actual_repay = min(repay_amount, max_repay);
    
    // Transfer repay amount from liquidator
    if !transfer_from(&borrow_addr, &caller, actual_repay) {
        return false;
    }
    
    // Calculate collateral to seize
    let borrow_price = get_price(&borrow_addr);
    let collateral_price = get_price(&collateral_addr);
    
    let seize_value = (actual_repay as u128 * borrow_price as u128 
        * LIQUIDATION_INCENTIVE as u128 / MANTISSA as u128) as u64;
    let seize_amount = (seize_value as u128 * MANTISSA as u128 / collateral_price as u128) as u64;
    
    // Update borrower's borrow balance
    let new_borrow = borrow_balance - actual_repay;
    let new_principal = if new_borrow > 0 {
        (new_borrow as u128 * MANTISSA as u128 / borrow_market.borrow_index as u128) as u64
    } else {
        0
    };
    set_borrow_balance(&borrower_addr, &borrow_addr, new_principal, borrow_market.borrow_index);
    
    // Transfer collateral from borrower to liquidator
    let (collateral_principal, collateral_index) = get_supply_balance(&borrower_addr, &collateral_addr);
    let collateral_balance = (collateral_principal as u128 * collateral_market.supply_index as u128 
        / collateral_index as u128) as u64;
    
    let seized = min(seize_amount, collateral_balance);
    
    // Update balances
    let borrower_new = collateral_balance - seized;
    let borrower_new_principal = if borrower_new > 0 {
        (borrower_new as u128 * MANTISSA as u128 / collateral_market.supply_index as u128) as u64
    } else {
        0
    };
    set_supply_balance(&borrower_addr, &collateral_addr, borrower_new_principal, collateral_market.supply_index);
    
    let (liquidator_principal, liquidator_index) = get_supply_balance(&caller, &collateral_addr);
    let liquidator_balance = if liquidator_index > 0 {
        (liquidator_principal as u128 * collateral_market.supply_index as u128 / liquidator_index as u128) as u64
    } else {
        0
    };
    
    let liquidator_new = liquidator_balance + seized;
    let liquidator_new_principal = (liquidator_new as u128 * MANTISSA as u128 
        / collateral_market.supply_index as u128) as u64;
    set_supply_balance(&caller, &collateral_addr, liquidator_new_principal, collateral_market.supply_index);
    
    // Update markets
    borrow_market.total_borrows -= actual_repay;
    save_market(&borrow_market);
    save_market(&collateral_market);
    
    true
}

#[no_mangle]
pub extern "C" fn enter_market(asset: *const u8) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    // Check market exists
    if get_market(&asset_addr).is_none() {
        return false;
    }
    
    set_collateral(&caller, &asset_addr, true);
    true
}

#[no_mangle]
pub extern "C" fn exit_market(asset: *const u8) -> bool {
    let caller = runtime::get_caller();
    if !runtime::check_witness(&caller) {
        return false;
    }
    
    let mut asset_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(asset, asset_addr.as_mut_ptr(), 20);
    }
    
    // Check if can exit (won't cause undercollateralization)
    set_collateral(&caller, &asset_addr, false);
    
    let (collateral, borrows) = calculate_liquidity(&caller);
    if collateral < borrows {
        // Revert
        set_collateral(&caller, &asset_addr, true);
        return false;
    }
    
    true
}

#[no_mangle]
pub extern "C" fn get_account_liquidity(account: *const u8) -> u128 {
    let mut account_addr = [0u8; 20];
    unsafe {
        core::ptr::copy_nonoverlapping(account, account_addr.as_mut_ptr(), 20);
    }
    
    let (collateral, borrows) = calculate_liquidity(&account_addr);
    ((collateral as u128) << 64) | (borrows as u128)
}

// Entry point
#[no_mangle]
pub extern "C" fn _start() {}