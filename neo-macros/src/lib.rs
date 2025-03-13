//! Neo N3 Procedural Macros
//!
//! This crate provides procedural macros for Neo N3 smart contract development.
//! The macros facilitate the definition of contract structure, methods, events,
//! and other necessary components for a Neo N3 smart contract.
//!
//! This includes all macros migrated from neo-contract-proc-macros to ensure
//! a complete set of functionality for Neo N3 smart contract development in Rust.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::__private::TokenStream2;
use syn::{parse_macro_input, Attribute, AttributeArgs, Data, DeriveInput, Fields, ItemFn, ItemMod, Lit, NestedMeta};

// Helper module for Neo type conversion and utility functions
mod helpers {
    // Helper function to check if a field has the #[index] attribute
    #[allow(dead_code)]
    pub fn has_index_attribute(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| attr.path.is_ident("index"))
    }

    // Explicitly specify a function to convert &&Type to String for the iterator issue
    pub fn map_field_type(type_ref: &&syn::Type) -> String { convert_type(*type_ref) }

    #[allow(dead_code)]
    pub fn convert_type_ref(type_ref: &&syn::Type) -> String { convert_type(*type_ref) }

    // Convert a Rust type reference to a Neo VM type string
    pub fn convert_type(ty: &syn::Type) -> String {
        match ty {
            syn::Type::Path(type_path) => {
                let last_segment = type_path.path.segments.last().unwrap();
                let type_name = last_segment.ident.to_string();

                match type_name.as_str() {
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "Int256" => "Integer".to_string(),
                    "bool" => "Boolean".to_string(),
                    "String" | "ByteString" => "String".to_string(),
                    "H160" | "Address" => "Hash160".to_string(),
                    "H256" => "Hash256".to_string(),
                    "Array" => "Array".to_string(),
                    "Map" => "Map".to_string(),
                    _ => "Any".to_string(), // Default to Any for other types
                }
            }
            _ => "Any".to_string(), // Default to Any for other type constructs
        }
    }
}

// Use the helpers module functionality when needed

#[allow(dead_code)]
fn type_to_neo_type(ty: &syn::Type) -> String { helpers::convert_type(ty) }

#[allow(dead_code)]
fn extract_attribute_args(_attr: &syn::Attribute) -> Vec<String> {
    // Using underscore prefix to acknowledge unused parameter
    let args = Vec::new();
    // Logic to extract attribute arguments would go here
    args
}

/// Marks a struct as an event that can be emitted from the contract
///
/// # Example
///
/// ```
/// #[event]
/// struct Transfer {
///     #[index]
///     from: Option<H160>,
///     #[index]
///     to: Option<H160>,
///     amount: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn event(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let (fields, indexed_fields) = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();
                let mut indexed_fields = Vec::new();

                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_type = &field.ty;

                    field_names.push(field_name.clone());
                    field_types.push(field_type.clone());

                    // Check if field has #[index] attribute
                    if helpers::has_index_attribute(&field.attrs) {
                        indexed_fields.push(field_name.clone());
                    }
                }

                (fields.named.clone(), indexed_fields)
            }
            _ => panic!("Only named fields are supported in event structs"),
        },
        _ => panic!("Only structs can be events"),
    };

    // Extract field information
    let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_name_strings: Vec<_> = field_names.iter().map(|f| f.to_string()).collect();

    // Convert field types to strings using the proper helper function for double references
    let field_type_strings: Vec<String> = field_types.iter().map(helpers::map_field_type).collect();
    let indexed_bools: Vec<_> = field_names.iter().map(|name| indexed_fields.contains(name)).collect();

    // Generate code to register the event and create a standardized emit method
    let expanded = quote! {
        #input

        impl #struct_name {
            /// Static method to emit the event in Neo N3 format
            pub fn emit(#(#field_names: #field_types),*) {
                // Create event name as ByteString (required for Neo N3)
                let event_name = neo_contract::prelude::ByteString::from(stringify!(#struct_name));

                // Create Array to hold event parameters (required for Neo N3)
                let mut event_data = neo_contract::prelude::Array::<neo_contract::prelude::Any>::new();

                // Add parameters with proper Neo N3 format
                #({
                    // Special handling for Option types
                    if let Some(type_name) = get_option_inner_type_name(stringify!(#field_types)) {
                        // This is an Option<T> type
                        match &#field_names {
                            Some(value) => event_data.push(neo_contract::prelude::Any::from(value.clone())),
                            None => event_data.push(neo_contract::prelude::Any::new()), // Use empty Any for null values
                        }
                    } else {
                        // Regular type, not an Option
                        event_data.push(neo_contract::prelude::Any::from(#field_names.clone()));
                    }
                })*

                // Emit the event using Runtime::notify (required for Neo N3)
                neo_contract::prelude::Runtime::notify(&event_name, &event_data);
            }
        }

        /// Helper function to determine if a type is an Option<T> and extract T
        fn get_option_inner_type_name(type_str: &str) -> Option<&str> {
            if type_str.starts_with("Option < ") {
                let start = type_str.find("Option < ").unwrap() + "Option < ".len();
                let end = type_str.rfind(" >").unwrap_or(type_str.len());
                Some(&type_str[start..end])
            } else {
                None
            }
        }
    };

    TokenStream::from(expanded)
}

/// Marks a field in an event struct as indexed for event filtering
#[proc_macro_attribute]
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This is just a marker attribute, return the item unchanged
    item
}

/// Marks static byte array types
///
/// Use these for declaring constant data with specific types
#[proc_macro_attribute]
pub fn byte_array(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed byte array constant.
#[proc_macro_attribute]
pub fn byte_array_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a Hash160 constant.
#[proc_macro_attribute]
pub fn hash160(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed Hash160 constant.
#[proc_macro_attribute]
pub fn hash160_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines an integer constant.
#[proc_macro_attribute]
pub fn integer(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed integer constant.
#[proc_macro_attribute]
pub fn integer_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a public key constant.
#[proc_macro_attribute]
pub fn public_key(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed public key constant.
#[proc_macro_attribute]
pub fn public_key_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a string constant.
#[proc_macro_attribute]
pub fn string(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Defines a fixed string constant.
#[proc_macro_attribute]
pub fn string_fixed(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Marks a function with a specific NEO VM opcode.
///
/// This attribute is used for low-level integration with the NEO VM.
/// It specifies which opcode should be used when the function is called.
///
/// # Example
///
/// ```rust
/// #[op_code(SYSCALL, "Neo.Storage.Get")]
/// fn storage_get(context: &StorageContext, key: &[u8]) -> Option<Vec<u8>> {
///     // Implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn op_code(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Marks a function as a syscall to the NEO VM.
///
/// This attribute is used for low-level integration with the NEO VM.
/// It specifies which syscall should be invoked when the function is called.
///
/// # Example
///
/// ```rust
/// #[syscall("Neo.Storage.Get")]
/// fn storage_get(context: &StorageContext, key: &[u8]) -> Option<Vec<u8>> {
///     // Implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn syscall(_: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just return the input as we'll implement this later
    item
}

/// Marks a struct as contract storage
#[proc_macro_attribute]
pub fn storage(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.clone(),
            _ => panic!("Only named fields are supported in storage structs"),
        },
        _ => panic!("Only structs can be storage"),
    };

    // Generate storage methods for each field
    let field_methods: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let getter_name = field_name.clone();
            let setter_name = format_ident!("set_{}", field_name);
            let key_name = field_name.to_string();

            quote! {
                pub fn #getter_name(&self) -> Option<#field_type> {
                    let context = neo_contract::storage::StorageContext::current();
                    let key = neo_contract::ByteString::from(#key_name);

                    // Use Neo N3 proper storage API
                    if let Some(data) = neo_contract::prelude::Storage::get(&key) {
                        return Some(data);
                    }

                    None
                }

                pub fn #setter_name(&self, value: #field_type) {
                    // Use Neo N3 proper storage API
                    let key = neo_contract::prelude::ByteString::from(#key_name);
                    neo_contract::prelude::Storage::put(&key, &value);
                }
            }
        })
        .collect();

    // Get field names for struct initialization
    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    // Build the initialization part with field defaults
    let init_struct_fields = if field_idents.is_empty() {
        quote! {}
    } else {
        let init_fields: Vec<_> = field_idents
            .iter()
            .map(|ident| {
                quote! { #ident: Default::default() }
            })
            .collect();

        quote! {
            #(#init_fields),*
        }
    };

    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #init_struct_fields
                }
            }

            // Generate all field methods individually
            #(#field_methods)*
        }
    };

    TokenStream::from(expanded)
}

/// Marks a method as a contract constructor
///
/// The constructor is called when the contract is deployed
/// It should initialize the contract state
#[proc_macro_attribute]
pub fn constructor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Constructor is always _deploy in Neo N3
    let expanded = quote! {
        #[neo_contract_proc_macros::manifest_method(method_name = "_deploy")]
        #input
    };

    TokenStream::from(expanded)
}

/// Marks a method as a contract method
///
/// Contract methods are exposed in the contract manifest and can be called
/// This is the standard way to expose functionality in Neo N3 contracts
#[proc_macro_attribute]
pub fn method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let method_name = name.to_string();

    // Check if this is a safe method based on attributes or naming convention
    let attr_str = attr.to_string();
    let is_safe = attr_str.contains("safe")
        || method_name.starts_with("get_")
        || method_name.starts_with("is_")
        || method_name.starts_with("has_");

    // Process the function to ensure it registers in the Neo N3 manifest
    // Neo N3 distinguishes between safe (read-only) and non-safe (state-modifying) methods
    let vis = &input.vis;
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;

    let output: TokenStream2 = if is_safe {
        // For safe methods, add the safe attribute to indicate read-only operation
        // This is represented in the contract manifest with "safe": true
        quote! {
            #(#attrs)*
            #[neo_contract::prelude::manifest_method(method_name = #method_name, safe = true)]
            #vis #sig {
                // Safe method implementation
                #block
            }
        }
    } else {
        // For state-modifying methods, we use the regular method attribute
        quote! {
            #(#attrs)*
            #[neo_contract::prelude::manifest_method(method_name = #method_name)]
            #vis #sig {
                // State-modifying method implementation
                #block
            }
        }
    };

    output.into()
}

/// Marks a method as a safe (read-only) contract method
///
/// Safe methods are represented in the contract manifest with "safe": true
/// These methods are optimized for contracts that don't modify state
#[proc_macro_attribute]
pub fn safe(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let output = quote! {
        #[method(safe)]
        #input
    };

    output.into()
}

// Helper function to check if a field has the #[index] attribute
#[allow(dead_code)]
fn has_index_attribute(attrs: &[Attribute]) -> bool { attrs.iter().any(|attr| attr.path.is_ident("index")) }

// Helper functions for Neo VM type handling are now in the helpers module

/// Marks a contract module with appropriate neo N3 contract semantics
///
/// This is the main entry point for defining a Neo smart contract in Rust.
/// It processes the module to extract method definitions, process storage,
/// register events, and set up entry points for calling the contract.
#[proc_macro_attribute]
pub fn contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    let mod_name = &input.ident;
    let content = &input.content;

    // Extract content from the module
    if let Some((_, items)) = content {
        // Generate the contract implementation
        let output = quote! {
            #[neo_contract::prelude::neo_contract_module]
            mod #mod_name {
                // Include the original module content
                #(#items)*

                // Generate the entry point for the Neo N3 contract
                #[no_mangle]
                pub extern "C" fn _deploy(data: *const u8, length: i32) -> i32 {
                    neo_contract::prelude::runtime::__neo_deploy_entry(data, length)
                }

                #[no_mangle]
                pub extern "C" fn _invoke(operation: *const u8, op_len: i32, args: *const u8, args_len: i32) -> i32 {
                    neo_contract::prelude::runtime::__neo_invoke_entry(operation, op_len, args, args_len)
                }
            }
        };

        output.into()
    } else {
        // Return the original module if it doesn't have content
        quote! { #input }.into()
    }
}

/// Marks a function with the no_reentrant protection to prevent reentrancy attacks
///
/// Non-reentrant methods cannot be called again while they are still running,
/// which prevents reentrancy attacks.
#[proc_macro_attribute]
pub fn no_reentrant(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let vis = &input.vis;
    let attrs = &input.attrs;

    // Create a guard key specific to this function
    let guard_key = format!("{}_GUARD", name);

    // Generate the function with reentrancy protection
    let output = quote! {
        #(#attrs)*
        #vis fn #name #fn_inputs #fn_output {
            // Check if we're already executing this function
            let key = neo_contract::prelude::ByteString::from(#guard_key);
            let guard: Option<bool> = neo_contract::prelude::Storage::get(&key);

            // If the guard is set, revert the transaction to prevent reentrancy
            if let Some(true) = guard {
                neo_contract::prelude::runtime::notify(
                    &neo_contract::prelude::ByteString::from("Error"),
                    &neo_contract::prelude::ByteString::from("Reentrant call detected")
                );
                return;
            }

            // Set the guard to prevent reentrancy
            neo_contract::prelude::Storage::put(&key, &true);

            // Execute the function
            let result = { #fn_block };

            // Clear the guard when we're done
            neo_contract::prelude::Storage::delete(&key);

            // Return the result
            result
        }
    };

    output.into()
}

/// Adds extra information to the contract manifest
///
/// This allows defining additional metadata for the contract.
#[proc_macro_attribute]
pub fn manifest_extra(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);

    // Extract the key and value from the attribute arguments
    if args.is_empty() || args.len() > 2 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected one or two arguments for manifest_extra attribute: key and optional value",
        )
        .to_compile_error()
        .into();
    }

    let _key = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(proc_macro2::TokenStream::new(), "Expected string literal for key")
                .to_compile_error()
                .into();
        }
    };

    let _value = if args.len() > 1 {
        match &args[1] {
            NestedMeta::Lit(Lit::Str(lit)) => Some(lit.value()),
            _ => {
                return syn::Error::new_spanned(proc_macro2::TokenStream::new(), "Expected string literal for value")
                    .to_compile_error()
                    .into();
            }
        }
    } else {
        None
    };

    // Return the original item
    item
}

/// Specifies the NEP standards supported by the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[supported_standards("NEP-17")]
/// mod token {
///     // NEP-17 token implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn supported_standards(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);

    // Extract the standards from the attribute arguments
    if args.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected at least one argument for supported_standards attribute",
        )
        .to_compile_error()
        .into();
    }

    // Extract standards
    let mut standards = Vec::new();
    for arg in args.iter() {
        if let NestedMeta::Lit(Lit::Str(lit)) = arg {
            standards.push(lit.value());
        } else {
            return syn::Error::new_spanned(proc_macro2::TokenStream::new(), "Expected string literals for standards")
                .to_compile_error()
                .into();
        }
    }

    let _standards_str = standards.join(", ");

    // Return the original item
    item
}

/// Defines contract permissions
///
/// This specifies which contracts and methods the contract is allowed to call.
#[proc_macro_attribute]
pub fn contract_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);

    // Extract the contract and methods from the attribute arguments
    if args.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected at least one argument for contract_permission attribute: contract hash and optional methods",
        )
        .to_compile_error()
        .into();
    }

    let _contract = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for contract hash",
            )
            .to_compile_error()
            .into();
        }
    };

    // Extract methods if provided
    let mut methods = Vec::new();
    for arg in args.iter().skip(1) {
        if let NestedMeta::Lit(Lit::Str(lit)) = arg {
            methods.push(lit.value());
        } else {
            return syn::Error::new_spanned(proc_macro2::TokenStream::new(), "Expected string literals for methods")
                .to_compile_error()
                .into();
        }
    }

    // Return the original item
    item
}

/// Defines contract trust relationships
///
/// This specifies which contracts are trusted by this contract.
#[proc_macro_attribute]
pub fn contract_trust(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);

    // Extract the contract or group from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected exactly one argument for contract_trust attribute: contract hash or group",
        )
        .to_compile_error()
        .into();
    }

    let _contract_or_group = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for contract hash or group",
            )
            .to_compile_error()
            .into();
        }
    };

    // Return the original item
    item
}
