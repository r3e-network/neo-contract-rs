// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

use neo::{contract::*, types::*};

pub struct DemoNep17Token;

impl SmartContract for DemoNep17Token {}

impl Nep17Token for DemoNep17Token {}

impl TokenContract for DemoNep17Token {
    fn symbol() -> ByteString {
        ByteString::empty()
    }

    fn decimals() -> u32 {
        8
    }
}

#[no_mangle]
pub fn transfer(from: H160, to: H160, amount: Int256) {
    DemoNep17Token::transfer(from, to, amount);
}
