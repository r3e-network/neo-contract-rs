// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{storage::StorageContext, types::*};

#[repr(C)]
pub struct StorageMap {
    cx: StorageContext,
}

impl StorageMap {
    #[inline(always)]
    pub fn new() -> Self {
        Self { cx: StorageContext::new() }
    }

    #[inline(always)]
    pub fn get(&self, _key: ByteString) -> ByteString {
        unimplemented!()
    }

    #[inline(always)]
    pub fn put(&self, _key: ByteString, _value: ByteString) {
        unimplemented!()
    }
}
