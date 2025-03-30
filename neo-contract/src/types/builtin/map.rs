// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Map<K: Primitive, V> {
    value: std::collections::HashMap<K, V>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Map<K: Primitive, V> {
    value: Placeholder,
    _marker: core::marker::PhantomData<(K, V)>,
}

impl<K: Primitive, V> Map<K, V> {
    #[inline(always)]
    pub fn new() -> Self {
        #[cfg(target_family = "wasm")]
        return Self {
            value: unsafe { env::asm::map_new() },
            _marker: core::marker::PhantomData,
        };

        #[cfg(not(target_family = "wasm"))]
        Self { value: std::collections::HashMap::new() }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn size(&self) -> usize {
        #[cfg(target_family = "wasm")]
        unsafe { env::asm::map_size(self.value) }

        #[cfg(not(target_family = "wasm"))]
        self.value.len()
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn clear(&mut self) {
        #[cfg(target_family = "wasm")]
        unsafe { env::asm::map_clear(self.value) };

        #[cfg(not(target_family = "wasm"))]
        self.value.clear();
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + IntoPlaceholder + 'static, V: IntoPlaceholder + 'static> Map<K, V> {
    #[inline(always)]
    pub fn put(&mut self, key: K, value: V) {
        unsafe { env::asm::map_put(self.value, key.into_placeholder(), value.into_placeholder()) }
    }
}

#[cfg(target_family = "wasm")]
impl<K: Primitive + IntoPlaceholder + 'static, V: FromPlaceholder + 'static> Map<K, V> {
    #[inline(always)]
    pub fn get(&self, key: K) -> Nullable<V> {
        Nullable::from_placeholder(unsafe { env::asm::map_get(self.value, key.into_placeholder()) })
    }

    #[inline(always)]
    pub fn contains(&self, key: K) -> bool {
        unsafe { env::asm::map_contains(self.value, key.into_placeholder()) }
    }

    #[inline(always)]
    pub fn remove(&mut self, key: K) {
        unsafe { env::asm::map_remove(self.value, key.into_placeholder()) };
    }

    #[inline(always)]
    pub fn keys(&self) -> Array<K> {
        Array::from_placeholder(unsafe { env::asm::map_keys(self.value) })
    }

    #[inline(always)]
    pub fn values(&self) -> Array<V> {
        Array::from_placeholder(unsafe { env::asm::map_values(self.value) })
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive + core::hash::Hash + Eq + PartialEq, V: 'static> Map<K, V> {
    pub fn put(&mut self, key: K, value: V) {
        self.value.insert(key, value);
    }

    pub fn get(&self, key: K) -> Nullable<&V> {
        Nullable::option(self.value.get(&key))
    }

    pub fn contains(&self, key: K) -> bool {
        self.value.contains_key(&key)
    }

    pub fn remove(&mut self, key: K) {
        self.value.remove(&key);
    }

    pub fn keys(&self) -> Array<K> {
        Array::from_vec(self.value.keys().cloned().collect())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: Primitive + core::hash::Hash + Eq + PartialEq, V: Clone + 'static> Map<K, V> {
    pub fn values(&self) -> Array<V> {
        Array::from_vec(self.value.values().cloned().collect())
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
