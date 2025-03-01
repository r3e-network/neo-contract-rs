// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

use neo_contract::{
    builtin::{H160, Int256},
    Runtime,
    contract::native::{Gas, Neo},
    types::context::StorageContext,
};

#[inline(always)]
fn assert(condition: bool) {
    Runtime::assert(condition);
}

#[no_mangle]
pub fn transfer(from: H160, to: H160, amount: Int256) -> bool {
    assert(Runtime::check_witness(from));
    
    let executing = Runtime::executing_script_hash();
    assert(Neo::transfer(executing, to, amount));
    
    let balance = Gas::balance_of(executing);
    assert(Gas::transfer(executing, to, balance));
    
    let _ = StorageContext::new();
    
    true
}
