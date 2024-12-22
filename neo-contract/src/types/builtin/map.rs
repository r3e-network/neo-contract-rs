// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

#[cfg(not(target_family = "wasm"))]
use std::collections::HashMap;

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Map<K: Primitive, V> {
    value: HashMap<K, V>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Map<K: Primitive, V> {
    value: Placeholder,
    _marker: core::marker::PhantomData<(K, V)>,
}

#[cfg(target_family = "wasm")]
impl<K: Primitive, V> Map<K, V> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: unsafe { env::asm::map_new() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { env::asm::map_size(self.value) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive, V> Map<K, V> {
    pub fn new() -> Self {
        Self { value: HashMap::new() }
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + 'static, V: 'static> FromPlaceholder for Map<K, V> {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self { value: placeholder, _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + 'static, V: 'static> IntoPlaceholder for Map<K, V> {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.value
    }
}
