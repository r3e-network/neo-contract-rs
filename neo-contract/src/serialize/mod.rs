// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod json;

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

pub use json::*;

pub trait Serialize {
    fn serialize(self) -> ByteString;
}

pub trait Deserialize {
    fn deserialize(data: ByteString) -> Self;
}

impl<T: IntoPlaceholder> Serialize for T {
    #[cfg(target_family = "wasm")]
    #[inline(always)]
    fn serialize(self) -> ByteString {
        unsafe { env::stdlib::serialize(self.into_placeholder()) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn serialize(self) -> ByteString {
        todo!()
    }
}

impl<T: FromPlaceholder> Deserialize for T {
    #[cfg(target_family = "wasm")]
    #[inline(always)]
    fn deserialize(data: ByteString) -> Self {
        Self::from_placeholder(unsafe { env::stdlib::deserialize(data) })
    }

    #[cfg(not(target_family = "wasm"))]
    fn deserialize(_data: ByteString) -> Self {
        todo!()
    }
}
