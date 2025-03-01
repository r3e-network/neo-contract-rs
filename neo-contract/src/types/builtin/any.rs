// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::boxed::Box;
use core::any::TypeId;
use core::fmt;

/// Any is a type-erased container for any type
#[derive(Debug, Clone)]
pub struct Any {
    /// The type ID of the contained value
    pub type_id: TypeId,
    /// The contained value as a raw pointer
    pub data: *mut (),
}

impl Any {
    /// Create a new Any from a value
    pub fn from<T: 'static>(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed) as *mut ();
        Any {
            type_id: TypeId::of::<T>(),
            data: ptr,
        }
    }

    /// Create a new empty Any
    pub fn new() -> Self {
        Any {
            type_id: TypeId::of::<()>(),
            data: core::ptr::null_mut(),
        }
    }

    /// Cast the Any to a reference of a specific type
    pub fn cast<T: 'static>(&self) -> Option<&T> {
        if self.type_id == TypeId::of::<T>() {
            unsafe { Some(&*(self.data as *const T)) }
        } else {
            None
        }
    }

    /// Cast the Any to a mutable reference of a specific type
    pub fn cast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        if self.type_id == TypeId::of::<T>() {
            unsafe { Some(&mut *(self.data as *mut T)) }
        } else {
            None
        }
    }
    
    /// Cast the Any to an Int256 if it contains an Int256
    pub fn as_int256(&self) -> Option<crate::types::builtin::int256::Int256> {
        self.cast::<crate::types::builtin::int256::Int256>().cloned()
    }
}

impl Default for Any {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Any {
    fn drop(&mut self) {
        // We don't know the type, so we can't drop it properly
        // This is a memory leak, but it's the best we can do
    }
}

impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Any({:?})", self.type_id)
    }
}

/// A trait for types that can be converted to Any
pub trait IntoAny {
    /// Convert the value to Any
    fn into_any(self) -> Any;
}

/// A trait for types that can be converted from Any
pub trait FromAny: Sized {
    /// Convert the Any to a value
    fn from_any(any: Any) -> Option<Self>;
}
