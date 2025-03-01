// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[repr(C)]
pub struct PublicKey(pub ByteString);

impl PublicKey {
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        true // TODO: implement
    }
    
    #[cfg(not(target_family = "wasm"))]
    pub fn from_bytes(bytes: [u8; 33]) -> Self {
        let mut hex = String::with_capacity(2 + bytes.len() * 2);
        hex.push_str("0x");
        for byte in bytes.iter() {
            hex.push_str(&format!("{:02x}", byte));
        }
        Self(ByteString::new(hex))
    }
    
    #[cfg(target_family = "wasm")]
    pub fn from_bytes(bytes: [u8; 33]) -> Self {
        unsafe { crate::env::extension::public_key_from_bytes(bytes) }
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    
    pub fn to_string(&self) -> String {
        self.0.clone().to_string()
    }
}

impl Clone for PublicKey {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for PublicKey {}

impl PartialEq for PublicKey {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
