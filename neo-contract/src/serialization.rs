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
        // For Int256, we'll use a simplified serialization
        // Convert to ByteString using available methods
        #[cfg(target_family = "wasm")]
        {
            // For WASM, use a placeholder approach
            // We'll just store "0" for now as a simple implementation
            ByteString::from_literal("0")
        }
        #[cfg(not(target_family = "wasm"))]
        {
            // Use full precision on non-WASM targets
            let bytes = self.to_bytes();
            ByteString::from(&bytes[..])
        }
    }
    
    fn from_storage(data: ByteString) -> Option<Self> {
        let bytes = data.to_bytes();
        #[cfg(target_family = "wasm")]
        {
            if bytes.len() >= 4 {
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