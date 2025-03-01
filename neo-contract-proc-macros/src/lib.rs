// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

extern crate proc_macro;

use proc_macro::TokenStream;

mod contract;
mod contract_permission;
mod contract_trust;
mod manifest_extra;
mod security;
mod static_field;
mod structure;
mod supported_standards;

// Import necessary items for export_trait and smart_contract macros
use quote::quote;
use syn::{parse_macro_input, ItemImpl, ItemTrait, TraitItem};

/// Macro to export trait functions
/// 
/// This macro generates wrapper functions for trait methods that can be exported
/// to the Neo VM. It creates a new implementation block with static functions
/// that call the trait methods on a singleton instance.
#[proc_macro_attribute]
pub fn export_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    
    // Generate the original trait
    let expanded = quote! {
        #input
        
        // Generated trait for exported functions
        pub trait ExportedTrait: #trait_name {
            fn instance() -> Self;
        }
    };
    
    expanded.into()
}

/// Macro to implement a smart contract
/// 
/// This macro generates the necessary boilerplate code for a Neo N3 smart contract.
#[proc_macro_attribute]
pub fn smart_contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = match &*input.self_ty {
        syn::Type::Path(type_path) => &type_path.path.segments.last().unwrap().ident,
        _ => panic!("Expected a struct name"),
    };
    
    // Generate the expanded implementation with Neo N3 specific code
    let expanded = quote! {
        #input
        
        #[no_mangle]
        pub extern "C" fn _deploy(data: bool) -> bool {
            #struct_name::deploy(data)
        }
        
        #[no_mangle]
        pub extern "C" fn _initialize() -> bool {
            #struct_name::initialize()
        }
    };
    
    expanded.into()
}

/// Attribute macro for adding extra information to the contract manifest
#[proc_macro_attribute]
pub fn manifest_extra(attr: TokenStream, item: TokenStream) -> TokenStream {
    manifest_extra::generate(attr, item)
}

/// Attribute macro for specifying contract permissions
#[proc_macro_attribute]
pub fn contract_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_permission::generate(attr, item)
}

/// Attribute macro for specifying contract trust
#[proc_macro_attribute]
pub fn contract_trust(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_trust::generate(attr, item)
}

/// Attribute macro for specifying supported standards
#[proc_macro_attribute]
pub fn supported_standards(attr: TokenStream, item: TokenStream) -> TokenStream {
    supported_standards::generate(attr, item)
}

/// Attribute macro for initializing a static byte array field
#[proc_macro_attribute]
pub fn byte_array(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::byte_array_generate(attr, item)
}

/// Attribute macro for initializing a static hash160 field
#[proc_macro_attribute]
pub fn hash160(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::hash160_generate(attr, item)
}

/// Attribute macro for initializing a static integer field
#[proc_macro_attribute]
pub fn integer(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::integer_generate(attr, item)
}

/// Attribute macro for initializing a static public key field
#[proc_macro_attribute]
pub fn public_key(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::public_key_generate(attr, item)
}

/// Attribute macro for initializing a static string field
#[proc_macro_attribute]
pub fn string(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::string_generate(attr, item)
}

/// Attribute macro for initializing a static contract hash field
#[proc_macro_attribute]
pub fn contract_hash(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::contract_hash_generate(attr, item)
}

/// Attribute macro for marking a function as safe
#[proc_macro_attribute]
pub fn safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    security::safe::generate(attr, item)
}

/// Attribute macro for preventing reentrancy attacks
#[proc_macro_attribute]
pub fn no_reentrant(attr: TokenStream, item: TokenStream) -> TokenStream {
    security::no_reentrant::generate(attr, item)
}

/// Attribute macro for preventing reentrancy attacks on a method
#[proc_macro_attribute]
pub fn no_reentrant_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    security::no_reentrant_method::generate(attr, item)
}

/// Attribute macro for adding storage functionality to a struct
#[proc_macro_attribute]
pub fn stored(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::stored::generate(attr, item)
}

/// Attribute macro for creating a function modifier
#[proc_macro_attribute]
pub fn modifier(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::modifier::generate(attr, item)
}

/// Attribute macro for specifying the calling convention for a function
#[proc_macro_attribute]
pub fn calling_convention(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::calling_convention::generate(attr, item)
}

/// Attribute macro for specifying the op code for a function
#[proc_macro_attribute]
pub fn op_code(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::op_code::generate(attr, item)
}

/// Attribute macro for specifying the syscall for a function
#[proc_macro_attribute]
pub fn syscall(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::syscall::generate(attr, item)
}

/// Attribute macro for defining a Neo N3 smart contract
#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::generate(attr, item)
}
