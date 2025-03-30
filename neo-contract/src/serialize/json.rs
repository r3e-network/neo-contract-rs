// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

pub trait JsonSerialize {
    fn to_json(self) -> ByteString;
}

pub trait JsonDeserialize {
    fn from_json(data: ByteString) -> Self;
}

impl<T: IntoPlaceholder> JsonSerialize for T {
    #[cfg(target_family = "wasm")]
    #[inline(always)]
    fn to_json(self) -> ByteString {
        unsafe { env::stdlib::json_serialize(self.into_placeholder()) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn to_json(self) -> ByteString {
        todo!()
    }
}


impl<T: FromPlaceholder> JsonDeserialize for T {
    #[cfg(target_family = "wasm")]
    #[inline(always)]
    fn from_json(data: ByteString) -> Self {
        Self::from_placeholder(unsafe { env::stdlib::json_deserialize(data) })
    }

    #[cfg(not(target_family = "wasm"))]
    fn from_json(_data: ByteString) -> Self {
        todo!()
    }
}