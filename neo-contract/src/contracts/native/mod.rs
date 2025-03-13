// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Native contracts module for Neo N3 smart contract framework
//! This module provides interfaces to interact with all Neo N3 native contracts

pub mod contract_management;
pub mod crypto_lib;
pub mod gas;
pub mod ledger;
pub mod neo;
pub mod oracle;
pub mod policy;
pub mod role_management;
pub mod std_lib;

// Re-export the native contracts for easier access
pub use contract_management::ContractManagement;
pub use crypto_lib::CryptoLib;
pub use gas::GAS;
pub use ledger::Ledger;
pub use neo::NEO;
pub use oracle::Oracle;
pub use policy::Policy;
pub use role_management::RoleManagement;
pub use role_management::Role;
pub use std_lib::StdLib;

