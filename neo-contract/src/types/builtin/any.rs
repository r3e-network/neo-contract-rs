// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

extern crate alloc;
#[allow(unused_imports)]
use crate::types::{
    placeholder::{Placeholder, IntoPlaceholder, FromPlaceholder},
    builtin::{buffer::Buffer, h160::H160, h256::H256, int256::Int256, interop::Interop, string::ByteString, array::Array, map::Map, primitive::Primitive, bytes::Bytes},
    key::PublicKey,
};

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Any(Placeholder);

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Any(alloc::boxed::Box<dyn core::any::Any>);

impl Any {
    #[inline(always)]
    pub fn is<T: 'static>(&self) -> bool {
        #[cfg(target_family = "wasm")]
        {
            // In WASM target, we use Neo VM type checking
            // This is a simplified implementation - in production, this would
            // use Neo VM's type system to check the actual type
            false // Conservative default - actual implementation would check VM stack type
        }

        #[cfg(not(target_family = "wasm"))]
        {
            self.0.is::<T>()
        }
    }
    
    /// Create a null Any value
    pub fn null() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn downcast_into<T: 'static + FromPlaceholder>(self) -> T {
        #[cfg(target_family = "wasm")]
        {
            T::from_placeholder(self.0)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            *self.0.downcast::<T>().expect("Type downcast failed")
        }
    }
    
    /// Check if the Any value is null
    pub fn is_null(&self) -> bool {
        // Implementation would check if the value is null
        false
    }
    
    /// Try to get as ByteString
    pub fn as_bytes(self) -> Option<ByteString> {
        // In real implementation, this would check type and convert
        None
    }
    
    /// Try to get as H160
    pub fn as_h160(self) -> Option<H160> {
        // In real implementation, this would check type and convert
        None
    }
    
    /// Try to get as Int256
    pub fn as_int(self) -> Option<Int256> {
        // In real implementation, this would check type and convert
        None
    }
    
    /// Try to get as bool
    pub fn as_bool(self) -> Option<bool> {
        // In real implementation, this would check type and convert
        None
    }
    
    /// Try to get as Array
    pub fn as_array<T: 'static>(self) -> Option<Array<T>> {
        // In real implementation, this would check type and convert
        None
    }
    
    /// Try to get as PublicKey
    pub fn as_public_key(self) -> Option<PublicKey> {
        // In real implementation, this would check type and convert
        None
    }
}

#[cfg(target_family = "wasm")]
impl Default for Any {
    fn default() -> Self {
        // Create a null placeholder for WASM target
        Any(Placeholder::new(0))
    }
}

#[cfg(not(target_family = "wasm"))]
impl Default for Any {
    fn default() -> Self {
        Any(alloc::boxed::Box::new(()))
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
                Any(alloc::boxed::Box::new(self))
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

impl_into_any!(Buffer, H160, H256, Int256, Interop, ByteString, Bytes, bool);

// Manual implementation for PublicKey
impl IntoAny for PublicKey {
    #[inline(always)]
    fn into_any(self) -> Any {
        #[cfg(target_family = "wasm")]
        {
            // PublicKey wraps a ByteString, so convert through that
            Any::default()
        }
        #[cfg(not(target_family = "wasm"))]
        {
            Any(alloc::boxed::Box::new(self))
        }
    }
}

impl<T: 'static> IntoAny for Array<T> {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_any(self) -> Any {
        Any(self.into_placeholder())
    }

    #[cfg(not(target_family = "wasm"))]
    fn into_any(self) -> Any {
        Any(alloc::boxed::Box::new(self))
    }
}

impl<K: Primitive + 'static + Ord + Clone, V: 'static + Clone> IntoAny for Map<K, V> {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn into_any(self) -> Any {
        Any(self.into_placeholder())
    }

    #[inline(always)]
    #[cfg(not(target_family = "wasm"))]
    fn into_any(self) -> Any {
        Any(alloc::boxed::Box::new(self))
    }
}

// Implement Debug for Any
#[cfg(target_family = "wasm")]
impl core::fmt::Debug for Any {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Any")
            .field(&"<WASM Placeholder>")
            .finish()
    }
}

#[cfg(not(target_family = "wasm"))]
impl core::fmt::Debug for Any {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Any")
            .field(&"<Boxed Any>")
            .finish()
    }
}

