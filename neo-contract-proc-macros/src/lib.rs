// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

extern crate proc_macro;

use proc_macro::TokenStream;
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
