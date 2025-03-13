// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Safe method attribute for Neo N3 smart contracts
//!
//! This module provides the safe attribute implementation, which marks methods
//! as read-only (non-state-modifying) in Neo N3 smart contracts.
//!
//! Methods marked with `#[safe]` will be represented in the contract manifest
//! with `"safe": true`, which allows Neo N3 nodes to optimize their execution.

use alloc::string::String;

/// Safe method attribute for Neo N3 smart contracts
///
/// This attribute marks methods that do not modify contract state.
/// In the Neo N3 contract manifest, these methods will have `"safe": true`.
///
/// # Example
///
/// ```rust
/// #[safe]
/// pub fn balance_of(&self, owner: H160) -> Int256 {
///     self.balances.get(&owner).unwrap_or_default()
/// }
/// ```
///
/// # Neo N3 Specifics
///
/// In Neo N3, safe methods:
/// - Can be executed without requiring a full transaction
/// - Do not modify contract state
/// - Can be called for free in read-only mode
/// - Provide better performance for dApp integrations
///
/// When generating the contract manifest, this attribute ensures the method
/// is marked with `"safe": true` in its ABI definition.
#[derive(Debug, Clone)]
pub struct SafeAttribute;

impl SafeAttribute {
    /// Creates a new safe attribute
    pub fn new() -> Self { Self }

    /// Returns the attribute name used in the manifest
    pub fn name(&self) -> String { "safe".into() }

    /// Returns the attribute value used in the manifest (always true)
    pub fn value(&self) -> bool { true }
}
