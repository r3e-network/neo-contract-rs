// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[repr(C)]
pub struct Placeholder(i32);

impl Placeholder {
    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn new<T: Into<i32>>(value: T) -> Self {
        Self(value.into())
    }

    #[cfg(target_family = "wasm")]
    pub fn new<T: Into<i32>>(value: T) -> Self {
        Self(value.into())
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        // In Neo VM, a null value is represented by a placeholder with value 0
        self.0 == 0
    }
}

impl Clone for Placeholder {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for Placeholder {}

impl PartialEq for Placeholder {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Placeholder {}

impl PartialOrd for Placeholder {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Placeholder {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

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

// Implementation for bool
impl IntoPlaceholder for bool {
    #[inline(always)]
    fn into_placeholder(self) -> Placeholder {
        Placeholder::new(if self { 1 } else { 0 })
    }
}

impl FromPlaceholder for bool {
    #[inline(always)]
    fn from_placeholder(placeholder: Placeholder) -> Self {
        placeholder.0 != 0
    }
}
