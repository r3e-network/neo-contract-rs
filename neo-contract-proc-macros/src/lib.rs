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

/// Attribute macro for marking a struct as storage
#[proc_macro_attribute]
pub fn stored(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    
    // Get the struct fields
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return TokenStream::from(
                quote! {
                    compile_error!("stored attribute can only be applied to structs");
                    #input
                }
            );
        }
    };
    
    // If there are no fields, simply return the original input
    if fields.is_empty() {
        return TokenStream::from(
            quote! {
                #input
            }
        );
    }
    
    // Generate the getter and setter methods for each field
    let field_methods = generate_field_methods(fields);
    
    // Create a new implementation block with the getter and setter methods
    let struct_name = &input.ident;
    let expanded = quote! {
        #input
        
        impl #struct_name {
            #field_methods
        }
    };
    
    TokenStream::from(expanded)
}

/// Generate the getter and setter methods for struct fields
fn generate_field_methods(fields: &syn::Fields) -> proc_macro2::TokenStream {
    let mut methods = quote! {};
    
    for field in fields.iter() {
        if let Some(ident) = &field.ident {
            let field_type = &field.ty;
            let getter_name = ident;
            let setter_name = quote::format_ident!("set_{}", ident);
            
            // Generate getter method
            let getter = quote! {
                pub fn #getter_name(&self) -> &#field_type {
                    &self.#ident
                }
            };
            
            // Generate setter method
            let setter = quote! {
                pub fn #setter_name(&mut self, value: #field_type) {
                    self.#ident = value;
                }
            };
            
            methods = quote! {
                #methods
                #getter
                #setter
            };
        }
    }
    
    methods
}

/// Attribute macro for marking a struct field as storage
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Just pass the item through, the storage implementation is in the stored attribute
    stored(attr, item)
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

/// Attribute macro for marking a function as a constructor
#[proc_macro_attribute]
pub fn constructor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    
    // For constructors, we leave them as-is but might need special handling for initialization
    let expanded = quote! {
        #[no_mangle]
        pub fn #fn_name(#fn_inputs) #fn_output #fn_body
    };
    
    expanded.into()
}

/// Attribute macro for marking a function as a message
#[proc_macro_attribute]
pub fn message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    
    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn #fn_name(#fn_inputs) #fn_output #fn_body
    };
    
    expanded.into()
}

/// Attribute macro for marking a function as an event
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    
    // Extract argument names
    let mut arg_names = Vec::new();
    for input in fn_inputs.iter() {
        if let syn::FnArg::Typed(pat_type) = input {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                arg_names.push(&pat_ident.ident);
            }
        }
    }
    
    // Create a new identifier by concatenating "__notify_" with the function name
    let notify_fn_name = syn::Ident::new(
        &format!("__notify_{}", fn_name),
        fn_name.span()
    );
    
    // Generate a notification function with the same parameters but with a notification body
    let output = quote! {
        #input
        
        // Define a separate notification function to avoid duplicate definition errors
        // The original #[no_mangle] extern "C" function is what causes the duplication
        // so we'll use a private function that won't be exported
        fn #notify_fn_name(#fn_inputs) {
            use alloc::format;
            use neo_contract::types::builtin::array::Array;
            use neo_contract::types::builtin::string::ByteString;
            use neo_contract::types::builtin::any::Any;
            use neo_contract::env::syscall;
            
            let mut args = Array::<Any>::new();
            #(
                let arg_str = format!("{:?}", #arg_names);
                args.push(Any::from(ByteString::from(arg_str)));
            )*
            
            let event_name = ByteString::from(stringify!(#fn_name));
            unsafe {
                syscall::system_runtime_notify(event_name, args);
            }
        }
        
        // This is the function that will actually be called
        #[no_mangle]
        pub extern "C" fn #fn_name(#fn_inputs) {
            #notify_fn_name(#(#arg_names),*);
        }
    };
    
    output.into()
}

/// Attribute macro for specifying the contract author
#[proc_macro_attribute]
pub fn contract_author(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Store the author info for manifest generation
    let _author = parse_macro_input!(_attr as syn::LitStr);
    
    // For now, pass through the item without modification
    // In a more complete implementation, this would modify the contract manifest
    item
}

/// Attribute macro for specifying the contract email
#[proc_macro_attribute]
pub fn contract_email(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Store the email info for manifest generation
    let _email = parse_macro_input!(_attr as syn::LitStr);
    
    // For now, pass through the item without modification
    item
}

/// Attribute macro for specifying the contract description
#[proc_macro_attribute]
pub fn contract_description(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Store the description info for manifest generation
    let _description = parse_macro_input!(_attr as syn::LitStr);
    
    // For now, pass through the item without modification
    item
}

/// Attribute macro for specifying the contract version
#[proc_macro_attribute]
pub fn contract_version(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Store the version info for manifest generation
    let _version = parse_macro_input!(_attr as syn::LitStr);
    
    // For now, pass through the item without modification
    item
}
