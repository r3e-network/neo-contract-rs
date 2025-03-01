// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

pub mod contract;
pub mod crypto;
pub mod env;
pub mod runtime;
pub mod serialize;
pub mod storage;
pub mod types;
pub mod utils;
pub mod macros;
pub mod static_values;
pub mod attributes;

// Re-export proc macros
pub use neo_contract_proc_macros::{
    contract,
    manifest_extra,
    contract_author,
    contract_email,
    contract_description,
    contract_version,
    contract_source_code,
    contract_permission,
    contract_trust,
    supported_standards,
};
