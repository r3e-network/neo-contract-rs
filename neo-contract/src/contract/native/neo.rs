// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::int256::Int256;
use crate::types::builtin::string::ByteString;

/// NEO represents the NEO native contract
pub struct NEO;

impl NEO {
    /// Get the contract hash
    pub fn hash() -> H160 {
        H160::hex_decode("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5").expect("Invalid hash")
    }

    /// Get the symbol
    pub fn symbol() -> ByteString {
        ByteString::from("NEO")
    }

    /// Get the decimals
    pub fn decimals() -> u8 {
        0
    }

    /// Get the total supply
    pub fn total_supply() -> Int256 {
        Int256::from(100_000_000)
    }
}
