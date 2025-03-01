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
