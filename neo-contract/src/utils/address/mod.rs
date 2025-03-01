// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use crate::types::builtin::h160::H160;
use crate::utils::hex;

/// Convert a script hash to an address
pub fn script_hash_to_address(script_hash: &H160) -> String {
    // In a real implementation, this would convert a script hash to an address
    format!("N{}", hex::encode(script_hash.as_bytes()))
}

/// Convert an address to a script hash
pub fn address_to_script_hash(address: &str) -> Option<H160> {
    if !address.starts_with('N') {
        return None;
    }

    let hex = &address[1..];
    
    match hex::decode(hex) {
        Ok(bytes) => {
            if bytes.len() != 20 {
                return None;
            }
            
            let mut result = [0u8; 20];
            result.copy_from_slice(&bytes);
            Some(H160(result))
        },
        Err(_) => None
    }
}
