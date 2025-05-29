// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::{
    placeholder::{Placeholder, IntoPlaceholder, FromPlaceholder},
    builtin::{buffer::Buffer, h160::H160, h256::H256, int256::Int256, interop::Interop, string::ByteString, array::Array, map::Map, primitive::Primitive},
};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Any(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Any(Box<dyn std::any::Any>);

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
impl Default for Any {
    fn default() -> Self {
        unimplemented!()
    }
}

#[cfg(not(target_family = "wasm"))]
impl Default for Any {
    fn default() -> Self {
        Any(Box::new(()))
    }
}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Any);

#[cfg(target_family = "wasm")]
impl Clone for Any {
    fn clone(&self) -> Self {
        Any(self.0.clone())
    }
}

#[cfg(not(target_family = "wasm"))]
impl Clone for Any {
    fn clone(&self) -> Self {
        // For non-WASM, we can't clone Box<dyn Any>, so create a default
        Any::default()
    }
}

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

impl_into_any!(Buffer, H160, H256, Int256, Interop, ByteString);

impl<T: 'static> IntoAny for Array<T> {
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

impl<K: Primitive + 'static + std::hash::Hash + Eq, V: 'static> IntoAny for Map<K, V> {
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
