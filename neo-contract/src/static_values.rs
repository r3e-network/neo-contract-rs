// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::format;
use alloc::string::ToString;

use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;
use crate::types::key::PublicKey;
use crate::utils::hex;

/// Trait for types that can be used as static values
pub trait StaticValue: Sized {
    /// Convert from a literal string to the static value
    fn from_literal(literal: &str) -> Option<Self>;
    
    /// Validate the literal string
    fn validate_literal(literal: &str) -> bool;
}

impl StaticValue for H160 {
    fn from_literal(literal: &str) -> Option<Self> {
        // Handle both hex format and address format
        if literal.starts_with("0x") && literal.len() == 42 {
            // Hex format: 0x...
            let hex = &literal[2..];
            if !Self::validate_literal(hex) {
                return None;
            }
            
            let mut bytes = [0u8; 20];
            for i in 0..20 {
                let byte = u8::from_str_radix(&hex[i*2..(i+1)*2], 16).ok()?;
                bytes[i] = byte;
            }
            
            Some(H160::new(bytes))
        } else if literal.starts_with("N") && literal.len() == 34 {
            // Address format: N...
            crate::utils::address::address_to_script_hash(ByteString::from(literal))
        } else {
            None
        }
    }
    
    fn validate_literal(literal: &str) -> bool {
        // Check if the literal is a valid hex string of length 40
        literal.len() == 40 && literal.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl StaticValue for H256 {
    fn from_literal(literal: &str) -> Option<Self> {
        if literal.starts_with("0x") && literal.len() == 66 {
            // Hex format: 0x...
            let hex = &literal[2..];
            if !Self::validate_literal(hex) {
                return None;
            }
            
            #[cfg(not(target_family = "wasm"))]
            {
                // Use hex_decode for non-WASM targets
                return Some(H256::hex_decode(&format!("0x{}", hex))?);
            }
            
            #[cfg(target_family = "wasm")]
            {
                let mut bytes = [0u8; 32];
                for i in 0..32 {
                    let byte = u8::from_str_radix(&hex[i*2..(i+1)*2], 16).ok()?;
                    bytes[i] = byte;
                }
                
                return Some(H256::new(bytes));
            }
        }
        
        None
    }
    
    fn validate_literal(literal: &str) -> bool {
        // Check if the literal is a valid hex string of length 64
        literal.len() == 64 && literal.chars().all(|c| c.is_ascii_hexdigit())
    }
}

impl StaticValue for PublicKey {
    fn from_literal(literal: &str) -> Option<Self> {
        if literal.starts_with("0x") && literal.len() == 68 {
            // Hex format: 0x...
            let hex = &literal[2..];
            if !Self::validate_literal(hex) {
                return None;
            }
            
            #[cfg(not(target_family = "wasm"))]
            {
                return Some(PublicKey::new([0; 33])); // Placeholder, will be replaced with proper implementation
            }
            
            #[cfg(target_family = "wasm")]
            {
                let mut bytes = [0u8; 33];
                for i in 0..33 {
                    let byte = u8::from_str_radix(&hex[i*2..(i+1)*2], 16).ok()?;
                    bytes[i] = byte;
                }
                
                return Some(PublicKey::new(bytes));
            }
        }
        
        None
    }
    
    fn validate_literal(literal: &str) -> bool {
        // Check if the literal is a valid hex string of length 66
        literal.len() == 66 && literal.chars().all(|c| c.is_ascii_hexdigit())
    }
}

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
