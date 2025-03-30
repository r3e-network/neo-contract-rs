// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::{placeholder::*, *}};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Nullable<T> {
    value: Placeholder,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Nullable<T> {
    value: Option<T>,
}

#[cfg(target_family = "wasm")]
impl<T: FromPlaceholder + IntoPlaceholder> Nullable<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self { 
            value: value.into_placeholder(),
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn null() -> Self {
        Self { 
            value: unsafe { env::extension::nullable_null() },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn is_null(&self) -> bool {
        unsafe { env::extension::nullable_is_null(self.value) }
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn unwrap(self) -> T {
        if self.is_null() {
            unsafe { env::asm::abort() }
        }
        T::from_placeholder(self.value)
    }

    #[inline(always)]
    pub unsafe fn unwrap_unchecked(self) -> T {
        T::from_placeholder(self.value)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Nullable<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }

    pub(crate) fn option(value: Option<T>) -> Self {
        Self { value }
    }

    pub fn null() -> Self {
        Self { value: None }
    }

    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }

    pub fn unwrap(self) -> T {
        self.value.unwrap()
    }

    pub unsafe fn unwrap_unchecked(self) -> T {
        self.value.unwrap_unchecked()
    }
}

#[cfg(target_family = "wasm")]
impl<T: FromPlaceholder + IntoPlaceholder> Default for Nullable<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> Default for Nullable<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::null()
    }
}

impl<T: Clone> Clone for Nullable<T> {
    #[inline(always)]
    #[cfg(not(target_family = "wasm"))]
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }

    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> FromPlaceholder for Nullable<T> {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        Self { value: placeholder, _marker: core::marker::PhantomData }
    }
}

#[cfg(target_family = "wasm")]
impl<T: 'static> IntoPlaceholder for Nullable<T> {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        self.value
    }
}
