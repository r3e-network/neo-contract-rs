use alloc::string::String;
// Copyright @ 2024 - present, R3E Network
use alloc::string::String;
// All Rights Reserved.
use alloc::string::String;

use alloc::string::String;
use crate::types::ByteString;
use alloc::string::String;

use alloc::string::String;
#[inline(always)]
use alloc::string::String;
pub fn assert(condition: bool) {
use alloc::string::String;
    #[cfg(target_family = "wasm")]
use alloc::string::String;
    unsafe { crate::env::asm::assert(condition) };
use alloc::string::String;

    #[cfg(not(target_family = "wasm"))]
    assert!(condition);
}

#[inline(always)]
pub fn assert_with_message(condition: bool, message: ByteString) {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::assert_with_message(condition, message) };

    #[cfg(not(target_family = "wasm"))]
    assert!(condition, "{}", message.to_string());
}

#[inline(always)]
pub fn abort() {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::abort()  };

    #[cfg(not(target_family = "wasm"))]
    panic!();
}

#[inline(always)]
pub fn abort_with_message(message: ByteString) {
    #[cfg(target_family = "wasm")]
    unsafe { crate::env::asm::abort_with_message(message) };

    #[cfg(not(target_family = "wasm"))]
    panic!("{}", message.to_string());
}
