// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;

/// NEP-11 token standard
pub trait NEP11 {
    /// Get the symbol of the token
    fn symbol(&self) -> ByteString;

    /// Get the decimals of the token
    fn decimals(&self) -> u8;

    /// Get the total supply of the token
    fn total_supply(&self) -> Int256;

    /// Get the balance of the token for the given owner
    fn balance_of(&self, owner: H160) -> Int256;

    /// Get the owner of the token
    fn owner_of(&self, token_id: &[u8]) -> H160;

    /// Get the properties of the token
    fn properties(&self, token_id: &[u8]) -> ByteString;

    /// Get the tokens of the given owner
    fn tokens_of(&self, owner: H160) -> ByteString;

    /// Transfer the token
    fn transfer(&mut self, to: H160, token_id: &[u8], data: &[u8]) -> bool;
}

/// NEP-11 token standard for divisible tokens
pub trait NEP11Divisible: NEP11 {
    /// Get the balance of the token for the given owner and token id
    fn balance_of_token(&self, owner: H160, token_id: &[u8]) -> Int256;

    /// Transfer the token
    fn transfer_divisible(&mut self, to: H160, token_id: &[u8], amount: Int256, data: &[u8]) -> bool;
}

/// NEP-11 token standard for non-divisible tokens
pub trait NEP11NonDivisible: NEP11 {
    /// Get the tokens of the given owner
    fn tokens_of_owner(&self, owner: H160) -> Vec<ByteString>;
}

/// NEP-11 token standard for divisible tokens with metadata
pub trait NEP11DivisibleWithMetadata: NEP11Divisible {
    /// Get the token URI
    fn token_uri(&self, token_id: &[u8]) -> ByteString {
        ByteString::from_string(&String::from_utf8_lossy(token_id).to_string())
    }

    /// Get the token metadata
    fn token_metadata(&self, token_id: &[u8]) -> ByteString;
}

/// NEP-11 token standard for non-divisible tokens with metadata
pub trait NEP11NonDivisibleWithMetadata: NEP11NonDivisible {
    /// Get the token URI
    fn token_uri(&self, token_id: &[u8]) -> ByteString {
        ByteString::from_string(&String::from_utf8_lossy(token_id).to_string())
    }

    /// Get the token metadata
    fn token_metadata(&self, token_id: &[u8]) -> ByteString;
}

/// NEP-11 token standard for divisible tokens with royalties
pub trait NEP11DivisibleWithRoyalties: NEP11Divisible {
    /// Get the token royalties
    fn royalties(&self, token_id: &[u8]) -> ByteString;
}

/// NEP-11 token standard for non-divisible tokens with royalties
pub trait NEP11NonDivisibleWithRoyalties: NEP11NonDivisible {
    /// Get the token royalties
    fn royalties(&self, token_id: &[u8]) -> ByteString;
}

/// NEP-11 token standard for divisible tokens with metadata and royalties
pub trait NEP11DivisibleWithMetadataAndRoyalties: NEP11DivisibleWithMetadata + NEP11DivisibleWithRoyalties {
    /// Get the token URI
    fn token_uri(&self, token_id: &[u8]) -> ByteString {
        ByteString::from_string(&String::from_utf8_lossy(token_id).to_string())
    }
}

/// NEP-11 token standard for non-divisible tokens with metadata and royalties
pub trait NEP11NonDivisibleWithMetadataAndRoyalties: NEP11NonDivisibleWithMetadata + NEP11NonDivisibleWithRoyalties {
    /// Get the token URI
    fn token_uri(&self, token_id: &[u8]) -> ByteString {
        ByteString::from_string(&String::from_utf8_lossy(token_id).to_string())
    }
}

