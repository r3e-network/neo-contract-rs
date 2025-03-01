// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::types::*;

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
            
            Some(H160::from_bytes(bytes))
        } else if literal.starts_with("N") && literal.len() == 34 {
            // Address format: N...
            crate::utils::address::address_to_script_hash(ByteString::new(literal.to_string()))
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
                return Some(H256::hex_decode(&format!("0x{}", hex)));
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
                return Some(PublicKey(ByteString::new(literal.to_string())));
            }
            
            #[cfg(target_family = "wasm")]
            {
                let mut bytes = [0u8; 33];
                for i in 0..33 {
                    let byte = u8::from_str_radix(&hex[i*2..(i+1)*2], 16).ok()?;
                    bytes[i] = byte;
                }
                
                return Some(PublicKey::from_bytes(bytes));
            }
        }
        
        None
    }
    
    fn validate_literal(literal: &str) -> bool {
        // Check if the literal is a valid hex string of length 66
        literal.len() == 66 && literal.chars().all(|c| c.is_ascii_hexdigit())
    }
}
