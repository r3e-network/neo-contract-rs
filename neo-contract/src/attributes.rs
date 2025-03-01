// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Attribute macros for Neo N3 smart contracts
//!
//! This module re-exports the attribute macros from the neo-contract-proc-macros crate
//! and provides documentation for their usage.

/// Attribute for defining a Neo N3 smart contract
///
/// This attribute is used to define a Neo N3 smart contract module. It processes
/// the module and generates the necessary code for the contract.
///
/// # Example
///
/// ```
/// #[neo_contract::contract]
/// mod my_contract {
///     #[neo(storage)]
///     pub struct MyContract {
///         value: bool,
///     }
///
///     impl MyContract {
///         // Contract methods and events
///     }
/// }
/// ```
pub use neo_contract_proc_macros::contract;
