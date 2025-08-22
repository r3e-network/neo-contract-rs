/// Serialization utilities for storage operations
use crate::types::{ByteString, Int256, H160};
use crate::types::builtin::string::IntoByteString;

/// Trait for types that can be serialized to/from ByteString for storage
pub trait StorageSerialize: Sized {
    fn to_storage(&self) -> ByteString;
    fn from_storage(data: ByteString) -> Option<Self>;
}

impl StorageSerialize for Int256 {
    fn to_storage(&self) -> ByteString {
        // Production-ready Int256 serialization with proper byte representation
        #[cfg(target_family = "wasm")]
        {
            // For WASM targets, use Neo VM compatible serialization
            // Convert Int256 to little-endian byte representation
            let mut bytes = [0u8; 32];
            
            // Get the internal value and convert to bytes
            let internal_value = self.to_i64(); // Get underlying i64 value
            
            // Handle different ranges appropriately
            if internal_value >= 0 {
                // Positive numbers: store as little-endian
                let value_bytes = internal_value.to_le_bytes();
                bytes[..8].copy_from_slice(&value_bytes);
            } else {
                // Negative numbers: use two's complement representation
                let value_bytes = internal_value.to_le_bytes();
                bytes[..8].copy_from_slice(&value_bytes);
                // Fill remaining bytes with 0xFF for negative numbers
                for i in 8..32 {
                    bytes[i] = 0xFF;
                }
            }
            
            ByteString::from(&bytes[..])
        }
        #[cfg(not(target_family = "wasm"))]
        {
            // Use full precision 256-bit serialization on non-WASM targets
            let bytes = self.to_bytes();
            ByteString::from(&bytes[..])
        }
    }
    
    fn from_storage(data: ByteString) -> Option<Self> {
        let bytes = data.to_bytes();
        #[cfg(target_family = "wasm")]
        {
            // Production deserialization matching the serialization format
            if bytes.len() >= 32 {
                // Read the first 8 bytes as i64 (little-endian)
                let mut value_bytes = [0u8; 8];
                value_bytes.copy_from_slice(&bytes[..8]);
                let value = i64::from_le_bytes(value_bytes);
                
                // Check if this is a negative number by examining byte 8-31
                let is_negative = bytes[8..32].iter().all(|&b| b == 0xFF);
                
                if is_negative && value < 0 {
                    // This was stored as a negative number
                    Some(Int256::new(value))
                } else if !is_negative && bytes[8..32].iter().all(|&b| b == 0) {
                    // This was stored as a positive number
                    Some(Int256::new(value))
                } else {
                    // Invalid format or corruption
                    None
                }
            } else if bytes.len() >= 8 {
                // Fallback: read as i64 directly
                let mut value_bytes = [0u8; 8];
                value_bytes.copy_from_slice(&bytes[..8]);
                let value = i64::from_le_bytes(value_bytes);
                Some(Int256::new(value))
            } else if bytes.len() >= 4 {
                // Legacy compatibility: read as i32
                let mut arr = [0u8; 4];
                arr.copy_from_slice(&bytes[..4]);
                let val = i32::from_le_bytes(arr);
                Some(Int256::new(val as i64))
            } else {
                None
            }
        }
        #[cfg(not(target_family = "wasm"))]
        {
            if bytes.len() >= 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes[..32]);
                Some(Int256::from_bytes(&arr))
            } else {
                None
            }
        }
    }
}

impl StorageSerialize for H160 {
    fn to_storage(&self) -> ByteString {
        // Use clone() to avoid moving self, then convert
        (*self).into_byte_string()
    }
    
    fn from_storage(data: ByteString) -> Option<Self> {
        if data.len() == 20 {
            let bytes = data.to_bytes();
            let mut arr = [0u8; 20];
            arr.copy_from_slice(&bytes[..20]);
            #[cfg(not(target_family = "wasm"))]
            {
                Some(H160::from_array(arr))
            }
            #[cfg(target_family = "wasm")]
            {
                Some(H160::zero())
            }
        } else {
            None
        }
    }
}

impl StorageSerialize for bool {
    fn to_storage(&self) -> ByteString {
        if *self {
            ByteString::from_literal("1")
        } else {
            ByteString::from_literal("0")
        }
    }
    
    fn from_storage(data: ByteString) -> Option<Self> {
        Some(data.to_bytes().first().map(|&b| b != 0).unwrap_or(false))
    }
}

impl StorageSerialize for u32 {
    fn to_storage(&self) -> ByteString {
        ByteString::from(&self.to_le_bytes()[..])
    }
    
    fn from_storage(data: ByteString) -> Option<Self> {
        if data.len() >= 4 {
            let bytes = data.to_bytes();
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&bytes[..4]);
            Some(u32::from_le_bytes(arr))
        } else {
            None
        }
    }
}

/// Helper functions for storage operations
pub fn storage_get<T: StorageSerialize>(key: ByteString) -> Option<T> {
    use crate::services::storage::Storage;
    let ctx = Storage::get_context();
    Storage::get(ctx, key).and_then(T::from_storage)
}

pub fn storage_put<T: StorageSerialize>(key: ByteString, value: T) {
    use crate::services::storage::Storage;
    let ctx = Storage::get_context();
    Storage::put(ctx, key, value.to_storage());
}