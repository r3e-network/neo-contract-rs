// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;

/// NEP-11 represents a non-fungible token in Neo N3
pub trait NEP11 {
    /// Get the symbol of the token
    fn symbol(&self) -> ByteString;

    /// Get the decimals of the token
    fn decimals(&self) -> u8;

    /// Get the total supply of the token
    fn total_supply(&self) -> Int256;

    /// Get the balance of an account
    fn balance_of(&self, owner: H160) -> Int256;

    /// Get the owner of a token
    fn owner_of(&self, token_id: ByteString) -> H160;

    /// Get the properties of a token
    fn properties(&self, token_id: ByteString) -> ByteString;

    /// Get the tokens owned by an account
    fn tokens_of(&self, owner: H160) -> Vec<ByteString>;

    /// Transfer a token
    fn transfer(&mut self, to: H160, token_id: ByteString, data: ByteString) -> bool;
}

/// NEP-11 Divisible represents a divisible non-fungible token in Neo N3
pub trait NEP11Divisible {
    /// Get the symbol of the token
    fn symbol(&self) -> ByteString;

    /// Get the decimals of the token
    fn decimals(&self) -> u8;

    /// Get the total supply of the token
    fn total_supply(&self) -> Int256;

    /// Get the balance of an account
    fn balance_of(&self, owner: H160) -> Int256;

    /// Get the balance of a token owned by an account
    fn balance_of_token(&self, owner: H160, token_id: ByteString) -> Int256;

    /// Get the owner of a token
    fn owner_of(&self, token_id: ByteString) -> H160;

    /// Get the properties of a token
    fn properties(&self, token_id: ByteString) -> ByteString;

    /// Get the tokens owned by an account
    fn tokens_of(&self, owner: H160) -> Vec<ByteString>;

    /// Transfer a token
    fn transfer(&mut self, to: H160, token_id: ByteString, amount: Int256, data: ByteString) -> bool;
}
