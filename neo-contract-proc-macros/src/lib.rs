// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

extern crate proc_macro;

use proc_macro::TokenStream;
mod contract;

/// Attribute macro for defining Neo N3 smart contracts
///
/// This macro provides an ink!-style unified approach to defining Neo N3 smart contracts.
/// It allows you to use attribute macros for contract components instead of separate macros.
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
///         #[neo(constructor)]
///         pub fn new(initial_value: bool) -> Self {
///             Self { value: initial_value }
///         }
///
///         #[neo(message)]
///         pub fn get(&self) -> bool {
///             self.value
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr, item)
}
