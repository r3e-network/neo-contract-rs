// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#![no_std]
#![no_main]

use neo_contract as neo;

use neo::{contract::*, types::*};

pub struct DemoNep17Token;

#[neo::contract]
impl Nep17Token for DemoNep17Token {
    fn symbol() -> ByteString {
        ByteString::empty()
    }

    fn decimals() -> u32 {
        8
    }
}
