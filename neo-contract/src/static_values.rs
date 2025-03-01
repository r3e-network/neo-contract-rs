// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use alloc::string::ToString;

use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;
use crate::utils::hex;

/// Parse a static value from a literal string
pub fn parse_static_value(literal: &str, ty: &str) -> Option<String> {
    match ty {
        "H160" => {
            if literal.starts_with("0x") {
                // Hex string
                let hex = literal.trim_start_matches("0x");
                if hex.len() != 40 {
                    return None;
                }
                let bytes = hex::decode(hex)?;
                let mut result = [0u8; 20];
                result.copy_from_slice(&bytes);
                return Some(format!("{:?}", result));
            } else if literal.starts_with("N") {
                // Neo address
                return Some(format!("{:?}", crate::utils::address::address_to_script_hash(ByteString::from(literal))?));
            }
            None
        }
        "H256" => {
            if literal.starts_with("0x") {
                // Hex string
                let hex = literal.trim_start_matches("0x");
                if hex.len() != 64 {
                    return None;
                }
                return Some(format!("{:?}", H256::hex_decode(&format!("0x{}", hex))?));
            }
            None
        }
        "ByteArray" => {
            if literal.starts_with("0x") {
                // Hex string
                let hex = literal.trim_start_matches("0x");
                let bytes = hex::decode(hex)?;
                return Some(format!("{:?}", bytes));
            }
            None
        }
        "String" => {
            // String literal
            return Some(literal.to_string());
        }
        "Integer" => {
            // Integer literal
            let _ = literal.parse::<i64>().ok()?;
            return Some(literal.to_string());
        }
        "PublicKey" => {
            if literal.starts_with("0x") {
                // Hex string
                let hex = literal.trim_start_matches("0x");
                let bytes = hex::decode(hex)?;
                return Some(format!("{:?}", bytes));
            } else {
                // Public key string
                return Some(ByteString::from(literal).to_string());
            }
        }
        _ => None,
    }
}
