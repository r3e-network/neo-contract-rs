// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::{storage::{StorageContext, Iter}, types::*};

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
    pub fn get<T>(&self, _key: &[u8]) -> Option<T> {
        unimplemented!()
    }

    #[inline(always)]
    pub fn put<T>(&self, _key: &[u8], _value: &T) {
        unimplemented!()
    }
    
    #[inline(always)]
    pub fn delete(&self, _key: &[u8]) {
        unimplemented!()
    }
    
    #[inline(always)]
    pub fn find<T: Clone>(&self, _prefix: &[u8]) -> Iter<T> {
        #[cfg(target_family = "wasm")]
        unimplemented!();
        
        #[cfg(not(target_family = "wasm"))]
        Iter::new(Vec::new())
    }
}
