// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

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
