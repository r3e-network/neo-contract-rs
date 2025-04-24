// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(not(target_family = "wasm"))]
use std::collections::HashMap;

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Map<K: Primitive + std::hash::Hash + Eq, V> {
    value: HashMap<K, V>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Map<K: Primitive, V> {
    value: Placeholder,
    _marker: core::marker::PhantomData<(K, V)>,
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + IntoPlaceholder, V: IntoPlaceholder + FromPlaceholder> Map<K, V> {
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

    #[inline(always)]
    pub fn put(&mut self, key: K, value: V) {
        // Use map_set instead of map_put since that's what's available in the API
        unsafe { env::asm::map_set(self.value, key.into_placeholder(), value.into_placeholder()) }
    }

    #[inline(always)]
    pub fn get(&self, key: &K) -> Option<V> {
        let placeholder = unsafe { env::asm::map_get(self.value, key.into_placeholder()) };
        if placeholder.is_null() {
            None
        } else {
            Some(V::from_placeholder(placeholder))
        }
    }

    #[inline(always)]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        // This is a placeholder implementation
        // The actual implementation would use env::asm::map_remove
        // but it's not available in the current API
        None
    }

    #[inline(always)]
    pub fn contains_key(&self, key: &K) -> bool {
        // This is a placeholder implementation
        // The actual implementation would use env::asm::map_contains_key
        // but it's not available in the current API
        false
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive + std::hash::Hash + Eq, V> Map<K, V> {
    pub fn new() -> Self {
        Self { value: HashMap::new() }
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }

    pub fn put(&mut self, key: K, value: V) {
        self.value.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.value.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.value.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.value.contains_key(key)
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
