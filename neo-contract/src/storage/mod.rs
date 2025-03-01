// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod map;
mod item;
pub mod iter;
mod context;

pub use {map::*, context::*};

use crate::types::*;

#[repr(C)]
pub struct StorageContext(Placeholder);

impl StorageContext {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn new() -> Self {
        #[cfg(target_family = "wasm")]
        unsafe { env::syscall::system_storage_get_context() }

        #[cfg(not(target_family = "wasm"))]
        StorageContext(Placeholder::new(0))
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn as_readonly(self) -> ReadOnlyStorageContext {
        #[cfg(target_family = "wasm")]
        unsafe { env::syscall::system_storage_as_readonly(self) }

        #[cfg(not(target_family = "wasm"))]
        ReadOnlyStorageContext(self.0)
    }
}

#[repr(C)]
pub struct ReadOnlyStorageContext(Placeholder);

impl ReadOnlyStorageContext {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn new() -> Self {
        #[cfg(target_family = "wasm")]
        unsafe { env::syscall::system_storage_get_readonly_context() }

        #[cfg(not(target_family = "wasm"))]
        ReadOnlyStorageContext(Placeholder::new(0))
    }
}

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Iter<T> {
    iter: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Iter<T> {
    values: Vec<T>,
    current_index: usize,
}

#[allow(private_bounds)]
#[cfg(target_family = "wasm")]
impl<T: FromPlaceholder> Iter<T> {
    #[inline(always)]
    pub fn next(&mut self) -> bool {
        unsafe { env::syscall::system_iterator_next(self.iter) }
    }

    #[inline(always)]
    pub fn value(&self) -> T {
        T::from_placeholder(unsafe { env::syscall::system_iterator_value(self.iter) })
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T: Clone> Iter<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            current_index: 0,
        }
    }
    
    pub fn next(&mut self) -> bool {
        if self.current_index < self.values.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }
    
    pub fn value(&self) -> T {
        self.values[self.current_index - 1].clone()
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(StorageContext);

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(ReadOnlyStorageContext);
