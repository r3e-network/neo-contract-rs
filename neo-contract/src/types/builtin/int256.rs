// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};

#[cfg(not(target_family = "wasm"))]
#[repr(C)]
pub struct Int256(num256::Int256);

#[cfg(target_family = "wasm")]
#[repr(C)]
pub struct Int256(Placeholder);

#[cfg(target_family = "wasm")]
impl Int256 {
    #[inline(always)]
    pub fn zero() -> Self {
        unsafe { env::numeric::int256_zero() }
    }

    #[inline(always)]
    pub fn one() -> Self {
        unsafe { env::numeric::int256_one() }
    }

    #[inline(always)]
    pub fn minus_one() -> Self {
        unsafe { env::numeric::int256_minus_one() }
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        unsafe { env::numeric::int256_is_zero(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_one(&self) -> bool {
        unsafe { env::numeric::int256_is_one(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        unsafe { env::numeric::int256_is_positive(Self(self.0)) }
    }

    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        unsafe { env::numeric::int256_is_negative(Self(self.0)) }
    }
}

#[cfg(not(target_family = "wasm"))]
impl Int256 {
    pub(crate) fn new(n: i64) -> Self {
        Int256(num256::Int256::from(n))
    }

    pub fn zero() -> Self {
        Int256(num256::Int256::from(0))
    }

    pub fn one() -> Self {
        Int256(num256::Int256::from(1))
    }

    pub fn minus_one() -> Self {
        Int256(num256::Int256::from(-1))
    }

    pub fn is_zero(&self) -> bool {
        self.0 == num256::Int256::from(0)
    }

    pub fn is_positive(&self) -> bool {
        self.0 > num256::Int256::from(0)
    }

    pub fn is_negative(&self) -> bool {
        self.0 < num256::Int256::from(0)
    }
}

impl PartialEq for Int256 {
    #[inline(always)]
    #[cfg(target_family = "wasm")]
    fn eq(&self, other: &Self) -> bool {
        unsafe { env::numeric::int256_eq(Self(self.0), Self(other.0)) }
    }

    #[cfg(not(target_family = "wasm"))]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Clone for Int256 {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Eq for Int256 {}
impl Copy for Int256 {}

#[cfg(target_family = "wasm")]
crate::impl_placeholder!(Int256);
