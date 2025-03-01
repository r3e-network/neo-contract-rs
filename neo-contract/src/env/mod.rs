// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub(crate) mod asm;
pub(crate) mod contract;
pub(crate) mod crypto;
pub(crate) mod extension;
pub(crate) mod numeric;
pub(crate) mod stdlib;
pub(crate) mod syscall;
#[cfg(not(target_family = "wasm"))]
pub(crate) mod contract_non_wasm;
#[cfg(not(target_family = "wasm"))]
pub(crate) mod syscall_non_wasm;
