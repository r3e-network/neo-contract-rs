// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::types::*;

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Any(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Any(Box<dyn std::any::Any + 'static>);

#[cfg(not(target_family = "wasm"))]
impl Clone for Any {
    fn clone(&self) -> Self {
        // For non-WASM, we can't actually clone the inner value
        // This is a placeholder implementation
        Any(Box::new(()))
    }
}

#[cfg(not(target_family = "wasm"))]
impl Default for Any {
    fn default() -> Self {
        Any(Box::new(()))
    }
}

#[cfg(not(target_family = "wasm"))]
impl Any {
    pub fn new<T: 'static>(value: T) -> Self {
        Any(Box::new(value))
    }
}

impl Any {
    #[inline(always)]
    pub fn is<T: 'static>(&self) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    pub fn downcast_into<T: 'static>(self) -> T {
        unimplemented!()
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Any);

pub trait IntoAny {
    fn into_any(self) -> Any;
}

#[cfg(not(target_family = "wasm"))]
macro_rules! impl_into_any {
    ($($type:ty),*) => {
        $(impl IntoAny for $type {
            #[inline(always)]
            fn into_any(self) -> Any {
                Any(Box::new(self))
            }
        })*
    };
}

#[cfg(target_family = "wasm")]
macro_rules! impl_into_any {
    ($($type:ty),*) => {
        $(impl IntoAny for $type {
            #[inline(always)]
            fn into_any(self) -> Any {
                Any(self.into_placeholder())
            }
        })*
    };
}

impl_into_any!(Buffer, H160, H256, Int256, Interop);

#[cfg(target_family = "wasm")]
impl IntoAny for ByteString {
    #[inline(always)]
    fn into_any(self) -> Any {
        unsafe { crate::env::stdlib::string_into_any(self) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl IntoAny for ByteString {
    #[inline(always)]
    fn into_any(self) -> Any {
        Any(Box::new(self))
    }
}

impl<T: 'static + Clone> IntoAny for Array<T> {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_any(self) -> Any {
        Any(self.into_placeholder())
    }

    #[cfg(not(target_family = "wasm"))]
    fn into_any(self) -> Any {
        Any(Box::new(self))
    }
}

impl<K: Primitive + 'static + std::hash::Hash, V: 'static> IntoAny for Map<K, V> {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_any(self) -> Any {
        Any(self.into_placeholder())
    }

    #[inline(always)]
    #[cfg(not(target_family = "wasm"))]
    fn into_any(self) -> Any {
        Any(Box::new(self))
    }
}
