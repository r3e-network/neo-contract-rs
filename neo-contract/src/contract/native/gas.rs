// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;

/// GAS represents the GAS native contract
pub struct GAS;

impl GAS {
    /// Get the contract hash
    pub fn hash() -> H160 {
        H160::hex_decode("0xd2a4cff31913016155e38e474a2c06d08be276cf").expect("Invalid hash")
    }

    /// Get the symbol
    pub fn symbol() -> ByteString {
        ByteString::from("GAS")
    }

    /// Get the decimals
    pub fn decimals() -> u8 {
        8
    }

    /// Get the total supply
    pub fn total_supply() -> Int256 {
        Int256::from(100_000_000)
    }
}
