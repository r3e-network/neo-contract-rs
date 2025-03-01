// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

pub mod contract;
pub mod contract_non_wasm;
pub mod syscall;
pub mod syscall_non_wasm;

// Re-exports
#[cfg(target_family = "wasm")]
pub use contract::*;
#[cfg(not(target_family = "wasm"))]
pub use contract_non_wasm::*;
#[cfg(target_family = "wasm")]
pub use syscall::*;
#[cfg(not(target_family = "wasm"))]
pub use syscall_non_wasm::*;
