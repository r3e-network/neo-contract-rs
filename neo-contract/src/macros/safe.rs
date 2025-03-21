// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

/// Attribute to mark a contract method as read-only (safe)
///
/// Methods marked with this attribute will be included in the manifest with `"safe": true`
/// meaning they do not modify contract state and can be safely called without incurring
/// additional fees.
///
/// # Example
///
/// ```rust
/// #[neo::contract]
/// impl ExampleContract {
///     /// Returns the total supply without modifying state
///     #[safe]
///     pub fn total_supply() -> Int256 {
///         // Read-only operation
///         let storage = StorageMap::new();
///         storage.get("total_supply").unwrap_or_default()
///     }
///
///     /// Modifies contract state - not marked as safe
///     pub fn mint(to: H160, amount: Int256) {
///         // This changes contract state
///         let mut storage = StorageMap::new();
///         // ... state modification code ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! safe {
    () => {};
}

/// Attribute to mark a contract method as read-only (safe)
///
/// Use this attribute on methods that only read contract state and don't modify it.
/// These methods will be marked with `"safe": true` in the manifest.
pub use safe as safe;
