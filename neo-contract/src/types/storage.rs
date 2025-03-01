// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::*;

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct StorageItem<T> {
    value: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct StorageItem<T> {
    value: T,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(not(target_family = "wasm"))]
impl<T> StorageItem<T> {
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn new(value: T) -> Self {
        Self { value, _marker: core::marker::PhantomData }
    }
}
