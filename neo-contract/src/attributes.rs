// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Attributes for Neo N3 smart contracts
//!
//! This module provides attributes for Neo N3 smart contracts, including:
//! - Contract metadata attributes
//! - Contract permission attributes
//! - Contract trust attributes
//! - Supported standards attributes

/// Contract metadata attributes
pub mod metadata {
    /// Manifest extra attribute
    ///
    /// This attribute adds extra information to the contract manifest.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[manifest_extra("Author", "Neo Team")]
    /// struct MyContract;
    /// ```
    pub use neo_contract_proc_macros::manifest_extra;

    /// Contract permission attribute
    ///
    /// This attribute specifies which contracts and methods this contract is allowed to call.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[contract_permission("0102030405060708090a0b0c0d0e0f1011121314", "transfer", "balanceOf")]
    /// struct MyContract;
    /// ```
    pub use neo_contract_proc_macros::contract_permission;

    /// Contract trust attribute
    ///
    /// This attribute specifies which contracts or groups this contract trusts.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[contract_trust("0102030405060708090a0b0c0d0e0f1011121314")]
    /// struct MyContract;
    /// ```
    pub use neo_contract_proc_macros::contract_trust;

    /// Supported standards attribute
    ///
    /// This attribute specifies which standards this contract supports.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[supported_standards("NEP-17", "NEP-11")]
    /// struct MyContract;
    /// ```
    pub use neo_contract_proc_macros::supported_standards;
}

/// Static field initialization attributes
pub mod static_field {
    /// Byte array attribute
    ///
    /// This attribute initializes a static byte array field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[byte_array("0102030405")]
    /// static BYTE_ARRAY: [u8; 5] = [0; 5];
    /// ```
    pub use neo_contract_proc_macros::byte_array;

    /// Hash160 attribute
    ///
    /// This attribute initializes a static Hash160 field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[hash160("0102030405060708090a0b0c0d0e0f1011121314")]
    /// static HASH160: [u8; 20] = [0; 20];
    /// ```
    pub use neo_contract_proc_macros::hash160;

    /// Integer attribute
    ///
    /// This attribute initializes a static integer field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[integer("42")]
    /// static INTEGER: i64 = 0;
    /// ```
    pub use neo_contract_proc_macros::integer;

    /// Public key attribute
    ///
    /// This attribute initializes a static public key field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[public_key("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
    /// static PUBLIC_KEY: [u8; 33] = [0; 33];
    /// ```
    pub use neo_contract_proc_macros::public_key;

    /// String attribute
    ///
    /// This attribute initializes a static string field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[string("Hello, Neo!")]
    /// static STRING: &str = "";
    /// ```
    pub use neo_contract_proc_macros::string;

    /// Contract hash attribute
    ///
    /// This attribute initializes a static contract hash field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[contract_hash("0102030405060708090a0b0c0d0e0f1011121314")]
    /// static CONTRACT_HASH: [u8; 20] = [0; 20];
    /// ```
    pub use neo_contract_proc_macros::contract_hash;
}

/// Security attributes
pub mod security {
    /// Safe attribute
    ///
    /// This attribute marks a function as safe, meaning it doesn't modify the contract state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[safe]
    /// fn get_balance(account: &str) -> u64 {
    ///     42 // Return a value to satisfy the return type
    /// }
    /// ```
    pub use neo_contract_proc_macros::safe;

    /// No reentrant attribute
    ///
    /// This attribute prevents reentrancy attacks by adding a lock to the function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[no_reentrant]
    /// fn transfer(from: &str, to: &str, amount: u64) -> bool {
    ///     true // Return a value to satisfy the return type
    /// }
    /// ```
    pub use neo_contract_proc_macros::no_reentrant;

    /// No reentrant method attribute
    ///
    /// This attribute is used internally by the no_reentrant attribute.
    pub use neo_contract_proc_macros::no_reentrant_method;
}

/// Structure attributes
pub mod structure {
    /// Stored attribute
    ///
    /// This attribute adds storage functionality to a struct.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[stored]
    /// struct StoredData {
    ///     value: u64,
    /// }
    /// ```
    pub use neo_contract_proc_macros::stored;

    /// Modifier attribute
    ///
    /// This attribute creates a function modifier.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[modifier]
    /// fn only_owner() {
    ///     // Check if caller is owner
    /// }
    /// ```
    pub use neo_contract_proc_macros::modifier;

    /// Calling convention attribute
    ///
    /// This attribute specifies the calling convention for a function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[calling_convention(Cdecl)]
    /// fn external_function() {
    ///     // External function implementation
    /// }
    /// ```
    pub use neo_contract_proc_macros::calling_convention;

    /// Op code attribute
    ///
    /// This attribute specifies the op code for a function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[op_code(SYSCALL, "System.Runtime.GetTime")]
    /// fn get_time() -> u64 {
    ///     42 // Return a value to satisfy the return type
    /// }
    /// ```
    pub use neo_contract_proc_macros::op_code;

    /// Syscall attribute
    ///
    /// This attribute specifies the syscall for a function.
    ///
    /// # Example
    ///
    /// ```ignore
    /// #[syscall("System.Runtime.GetTime")]
    /// fn get_time() -> u64 {
    ///     42 // Return a value to satisfy the return type
    /// }
    /// ```
    pub use neo_contract_proc_macros::syscall;
}
