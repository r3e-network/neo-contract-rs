// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::prelude::{H160, Int256, ByteString};

/// NEP-17 represents a fungible token in Neo N3
pub trait NEP17 {
    /// Get the symbol of the token
    fn symbol(&self) -> ByteString;

    /// Get the decimals of the token
    fn decimals(&self) -> u8;

    /// Get the total supply of the token
    fn total_supply(&self) -> Int256;

    /// Get the balance of an account
    fn balance_of(&self, account: H160) -> Int256;

    /// Transfer tokens
    fn transfer(&mut self, from: H160, to: H160, amount: Int256, data: ByteString) -> bool;
}

/// NEP-17 Token implementation
pub struct NEP17Token {
    /// The name of the token
    name: ByteString,
    /// The symbol of the token
    symbol: ByteString,
    /// The decimals of the token
    decimals: u8,
    /// The total supply of the token
    total_supply: Int256,
}

impl NEP17Token {
    /// Create a new NEP-17 token
    pub fn new(name: ByteString, symbol: ByteString, decimals: u8, total_supply: Int256) -> Self {
        Self {
            name,
            symbol,
            decimals,
            total_supply,
        }
    }

    /// Get the name of the token
    pub fn name(&self) -> ByteString {
        self.name.clone()
    }
}

impl NEP17 for NEP17Token {
    fn symbol(&self) -> ByteString {
        self.symbol.clone()
    }

    fn decimals(&self) -> u8 {
        self.decimals
    }

    fn total_supply(&self) -> Int256 {
        self.total_supply
    }

    fn balance_of(&self, _account: H160) -> Int256 {
        Int256::zero()
    }

    fn transfer(&mut self, _from: H160, _to: H160, _amount: Int256, _data: ByteString) -> bool {
        false
    }
}
