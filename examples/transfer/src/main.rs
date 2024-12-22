// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

use neo::contract::native::{Gas, Neo};
use neo::{runtime, storage::StorageContext, types::*};

// const OWNER: H160 = "NUuJw4C4XJFzxAvSZnFTfsNoWZytmQKXQP";

#[inline(always)]
fn assert(condition: bool) {
    runtime::assert(condition);
}

#[no_mangle]
pub fn transfer(from: H160, to: H160, amount: Int256) {
    assert(runtime::check_witness_with_account(from));

    let executing = runtime::get_executing_script_hash();
    assert(Neo::transfer(executing, to, amount));

    let balance = Gas::balance_of(executing);
    assert(Gas::transfer(executing, to, balance));

    let _ = StorageContext::new();
}
