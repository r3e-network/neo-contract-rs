// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::utils::hex;

/// Convert an address to a script hash
pub fn address_to_script_hash(address: ByteString) -> Option<H160> {
    let address_str = alloc::string::ToString::to_string(&address);
    if address_str.len() != 34 || !address_str.starts_with('A') {
        return None;
    }
    Some(H160::hex_decode(&address_str).unwrap_or_else(H160::zero))
}

/// Convert a script hash to an address
pub fn script_hash_to_address(script_hash: H160) -> ByteString {
    let script_hash_str = hex::encode(script_hash.as_bytes());
    ByteString::from(alloc::format!("A{}", script_hash_str))
}
