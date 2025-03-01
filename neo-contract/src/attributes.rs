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

/// Attribute for adding extra information to the contract manifest
///
/// This attribute is used to add extra information to the contract manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::manifest_extra("key", "value")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::manifest_extra;

/// Attribute for specifying the contract author
///
/// This attribute is used to specify the author of the contract in the manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_author("John Doe")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_author;

/// Attribute for specifying the contract email
///
/// This attribute is used to specify the email of the contract author in the manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_email("john.doe@example.com")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_email;

/// Attribute for specifying the contract description
///
/// This attribute is used to specify the description of the contract in the manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_description("A simple contract")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_description;

/// Attribute for specifying the contract version
///
/// This attribute is used to specify the version of the contract in the manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_version("1.0.0")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_version;

/// Attribute for specifying the contract source code URL
///
/// This attribute is used to specify the URL of the contract source code in the manifest.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_source_code("https://github.com/example/contract")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_source_code;

/// Attribute for specifying the contract permissions
///
/// This attribute is used to specify which contracts and methods are allowed to call from this contract.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_permission("0x1234567890abcdef1234567890abcdef12345678", "method1", "method2")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_permission;

/// Attribute for specifying the contract trust
///
/// This attribute is used to specify which contracts are trusted by this contract.
///
/// # Example
///
/// ```
/// #[neo_contract::contract_trust("0x1234567890abcdef1234567890abcdef12345678")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::contract_trust;

/// Attribute for specifying the supported standards
///
/// This attribute is used to specify which standards the contract supports.
///
/// # Example
///
/// ```
/// #[neo_contract::supported_standards("NEP-17", "NEP-11")]
/// mod my_contract {
///     // Contract code
/// }
/// ```
pub use neo_contract_proc_macros::supported_standards;
