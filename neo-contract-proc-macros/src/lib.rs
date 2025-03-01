// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

extern crate proc_macro;

use proc_macro::TokenStream;
mod contract;
mod manifest_extra;
mod contract_permission;
mod contract_trust;
mod supported_standards;

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

/// Attribute for adding extra information to the contract manifest
#[proc_macro_attribute]
pub fn manifest_extra(attr: TokenStream, item: TokenStream) -> TokenStream {
    manifest_extra::generate(attr, item)
}

/// Attribute for specifying the contract author
#[proc_macro_attribute]
pub fn contract_author(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = format!("\"Author\", {}", attr.to_string());
    let new_attr = attr_str.parse().unwrap();
    manifest_extra::generate(new_attr, item)
}

/// Attribute for specifying the contract email
#[proc_macro_attribute]
pub fn contract_email(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = format!("\"E-mail\", {}", attr.to_string());
    let new_attr = attr_str.parse().unwrap();
    manifest_extra::generate(new_attr, item)
}

/// Attribute for specifying the contract description
#[proc_macro_attribute]
pub fn contract_description(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = format!("\"Description\", {}", attr.to_string());
    let new_attr = attr_str.parse().unwrap();
    manifest_extra::generate(new_attr, item)
}

/// Attribute for specifying the contract version
#[proc_macro_attribute]
pub fn contract_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = format!("\"Version\", {}", attr.to_string());
    let new_attr = attr_str.parse().unwrap();
    manifest_extra::generate(new_attr, item)
}

/// Attribute for specifying the contract source code URL
#[proc_macro_attribute]
pub fn contract_source_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = format!("\"Sourcecode\", {}", attr.to_string());
    let new_attr = attr_str.parse().unwrap();
    manifest_extra::generate(new_attr, item)
}

/// Attribute for specifying the contract permissions
#[proc_macro_attribute]
pub fn contract_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_permission::generate(attr, item)
}

/// Attribute for specifying the contract trust
#[proc_macro_attribute]
pub fn contract_trust(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_trust::generate(attr, item)
}

/// Attribute for specifying the supported standards
#[proc_macro_attribute]
pub fn supported_standards(attr: TokenStream, item: TokenStream) -> TokenStream {
    supported_standards::generate(attr, item)
}
