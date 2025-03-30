// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::{placeholder::*, *};

#[repr(C)]
pub struct PublicKey(ByteString);

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

#[cfg(target_family = "wasm")]
impl FromPlaceholder for PublicKey {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self(ByteString::from_placeholder(placeholder))
    }
}

#[cfg(target_family = "wasm")]
impl IntoPlaceholder for PublicKey {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.0.into_placeholder()
    }
}
