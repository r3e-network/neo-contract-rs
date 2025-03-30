// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Array<T> {
    value: Vec<T>,
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Array<T> {
    value: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(target_family = "wasm")]
impl<T> Array<T> {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: unsafe { env::asm::array_new() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        unsafe { env::asm::array_size(self.value) }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        unsafe { env::asm::array_clear(self.value) }
    }

    #[inline(always)]
    pub fn reverse(&mut self) {
        unsafe { env::asm::array_reverse(self.value) }
    }

    #[inline(always)]
    pub fn remove(&mut self, index: usize) {
        unsafe { env::asm::array_remove(self.value, index) }
    }
}

#[cfg(target_family = "wasm")]
impl<T: IntoPlaceholder> Array<T> {
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        unsafe { env::asm::array_push(self.value, value.into_placeholder()) }
    }

    #[inline(always)]
    pub fn set(&mut self, index: usize, value: T) {
        unsafe { env::asm::array_set(self.value, index, value.into_placeholder()) }
    }
}

#[cfg(target_family = "wasm")]
impl<T: FromPlaceholder> Array<T> {
    #[inline(always)]
    pub fn pop(&mut self) -> T {
        T::from_placeholder(unsafe { env::asm::array_pop(self.value) })
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        T::from_placeholder(unsafe { env::asm::array_get(self.value, index) })
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Array<T> {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub(crate) fn from_vec(value: Vec<T>) -> Self {
        Self { value }
    }

    pub fn size(&self) -> usize {
        self.value.len()
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn reverse(&mut self) {
        self.value.reverse();
    }

    pub fn remove(&mut self, index: usize) {
        self.value.remove(index);
    }

    pub fn push(&mut self, value: T) {
        self.value.push(value);
    }

    pub fn pop(&mut self) -> T {
        self.value.pop().unwrap()
    }

    pub fn get(&self, index: usize) -> &T {
        &self.value[index] // TODO: return a clone
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.value[index] = value;
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> FromPlaceholder for Array<T> {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self { value: placeholder, _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> IntoPlaceholder for Array<T> {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.value
    }
}
