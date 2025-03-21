// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod contract;
pub mod crypto;
pub mod env;
pub mod event;
pub mod macros;
pub mod runtime;
pub mod serialize;
pub mod storage;
pub mod types;

pub use neo_contract_proc_macros::{contract, structs};

// Export contract annotations
pub use neo_contract_proc_macros::{
    method, 
    safe as safe_attr,
    contract_author, 
    contract_permission, 
    contract_standards, 
    contract_meta
};

// Modules re-exported for convenience
pub mod prelude {
    pub use crate::contract::*;
    pub use crate::crypto::*;
    pub use crate::event::*;
    pub use crate::macros::*;
    pub use crate::runtime::*;
    pub use crate::storage::*;
    pub use crate::types::*;
    pub use neo_contract_proc_macros::*;
}
