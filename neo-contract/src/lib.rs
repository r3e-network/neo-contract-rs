// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![allow(unused_variables)]

extern crate alloc;

pub mod attributes;
mod call_flags;
mod contract;
mod crypto;
mod env;
mod macros;
mod runtime;
mod serialize;
mod static_values;
mod storage;
mod types;
mod utils;

// Re-exports
pub use call_flags::CallFlags;
pub use contract::{nep11, nep17, native};
pub use crypto::*;
pub use env::{contract as env_contract, contract_non_wasm, syscall, syscall_non_wasm};
pub use macros::*;
pub use runtime::*;
pub use static_values::*;
pub use storage::*;
pub use types::{block, builtin as types_builtin, bytes, context, key, notification, signer, storage as types_storage, tx};
pub use utils::*;

// Re-export proc macros
pub use neo_contract_proc_macros::{export_trait, smart_contract};

// Re-export builtin types for easier access
pub mod builtin {
    pub use crate::types::builtin::h160::H160;
    pub use crate::types::builtin::h256::H256;
    pub use crate::types::builtin::int256::Int256;
    pub use crate::types::builtin::string::ByteString;
    pub use crate::types::builtin::array::Array;
    pub use crate::types::builtin::any::Any;
    pub use crate::types::builtin::map::Map;
}
