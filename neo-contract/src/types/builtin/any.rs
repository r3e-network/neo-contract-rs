// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::types::{placeholder::*, *};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Any(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Any(Option<Box<dyn std::any::Any>>);

impl Any {
    #[inline(always)]
    #[cfg(not(target_family = "wasm"))]
    pub fn null() -> Self {
        Any(None)
    }

    #[inline(always)]
    #[cfg(target_family = "wasm")]
    pub fn null() -> Self {
        Any(unsafe { crate::env::extension::nullable_null() })
    }

    #[inline(always)]
    pub fn is_array(&self) -> bool {
        unimplemented!()
    }

    pub fn is_integer(&self) -> bool {
        unimplemented!()
    }

    pub fn is_bytes_string(&self) -> bool {
        unimplemented!()
    }

    pub fn is_buffer(&self) -> bool {
        unimplemented!()
    }

    pub fn is_map(&self) -> bool {
        unimplemented!()
    }

    pub fn is_interop(&self) -> bool {
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

#[cfg(target_family = "wasm")]
impl<T: IntoPlaceholder> IntoAny for T {
    #[inline(always)]
    fn into_any(self) -> Any {
        Any(self.into_placeholder())
    }
}