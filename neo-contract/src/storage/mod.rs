// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod map;

pub use map::*;

#[allow(unused_imports)]
use crate::{env, types::*};

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

impl Clone for StorageContext {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
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

impl Clone for ReadOnlyStorageContext {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[repr(C)]
pub struct Iter<T> {
    iter: Placeholder,
    _marker: core::marker::PhantomData<T>,
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

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(StorageContext);

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(ReadOnlyStorageContext);
