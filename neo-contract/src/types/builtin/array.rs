// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{
    env,
    types::{placeholder::*, *},
};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Array<T> {
    value: Vec<T>,
    // _marker: core::marker::PhantomData<T>,
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
    pub fn push(&mut self, _value: T) {
        // This is a placeholder implementation
        // The actual implementation would use env::asm::array_push
        // but it requires IntoPlaceholder which not all types implement
    }

    #[inline(always)]
    pub fn pop(&mut self) -> T {
        unimplemented!("pop is not implemented for WASM target")
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> T {
        unimplemented!("get is not implemented for WASM target")
    }

    #[inline(always)]
    pub fn set(&mut self, _index: usize, _value: T) {
        // This is a placeholder implementation
        // The actual implementation would use env::asm::array_set
        // but it requires IntoPlaceholder which not all types implement
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Array<T> {
    pub fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.value.len()
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
