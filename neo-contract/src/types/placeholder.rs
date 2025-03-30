// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[cfg(target_family = "wasm")]
use crate::env::extension::*;

#[repr(C)]
pub struct Placeholder(i32);

impl Placeholder {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn new(value: i32) -> Self {
        Self(value)
    }
}

impl Clone for Placeholder {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Placeholder {}

pub trait FromPlaceholder {
    fn from_placeholder(placeholder: Placeholder) -> Self;
}

pub trait IntoPlaceholder {
    fn into_placeholder(self) -> Placeholder;
}

// It for internal use, don't use it directly
#[macro_export]
macro_rules! impl_placeholder {
    ($type:ty) => {
        #[cfg(target_family = "wasm")]
        impl IntoPlaceholder for $type {
            #[inline(always)]
            fn into_placeholder(self) -> Placeholder {
                self.0
            }
        }

        #[cfg(target_family = "wasm")]
        impl FromPlaceholder for $type {
            #[inline(always)]
            fn from_placeholder(placeholder: Placeholder) -> Self {
                Self(placeholder)
            }
        }
    };
}

macro_rules! impl_placeholder_with {
    ($type:ty, $from:expr, $into:expr) => {
        #[cfg(target_family = "wasm")]
        impl IntoPlaceholder for $type {
            #[inline(always)]
            fn into_placeholder(self) -> Placeholder {
                unsafe { $from(self) }
            }
        }

        #[cfg(target_family = "wasm")]
        impl FromPlaceholder for $type {
            #[inline(always)]
            fn from_placeholder(placeholder: Placeholder) -> Self {
                unsafe { $into(placeholder) }
            }
        }
    };
}

impl_placeholder_with!(bool, placeholder_from_bool, placeholder_to_bool);
impl_placeholder_with!(u8, placeholder_from_u8, placeholder_to_u8);
impl_placeholder_with!(u16, placeholder_from_u16, placeholder_to_u16);
impl_placeholder_with!(u32, placeholder_from_u32, placeholder_to_u32);
impl_placeholder_with!(u64, placeholder_from_u64, placeholder_to_u64);
impl_placeholder_with!(i8, placeholder_from_i8, placeholder_to_i8);
impl_placeholder_with!(i16, placeholder_from_i16, placeholder_to_i16);
impl_placeholder_with!(i32, placeholder_from_i32, placeholder_to_i32);
impl_placeholder_with!(i64, placeholder_from_i64, placeholder_to_i64);
impl_placeholder_with!(usize, placeholder_from_usize, placeholder_to_usize);
impl_placeholder_with!(isize, placeholder_from_isize, placeholder_to_isize);
