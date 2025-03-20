//! Neo N3 Procedural Macros
//!
//! This crate provides procedural macros for Neo N3 smart contract development.
//! The macros facilitate the definition of contract structure, methods, events,
//! and other necessary components for a Neo N3 smart contract.
//!
//! This includes all macros migrated from neo-contract-proc-macros to ensure
//! a complete set of functionality for Neo N3 smart contract development in Rust.
//!
//! ## Note on Declarative Macros
//! 
//! Declarative macros (`macro_rules!`) cannot be defined in a proc-macro crate as per
//! Rust language limitations. For declarative macros like `emit_event!`, `profile!`, etc.,
//! a separate non-proc-macro crate should be created (e.g., `neo-macros-core`).
//!
//! The recommended architecture is:
//! 1. `neo-macros` (this crate): Contains procedural macros only
//! 2. `neo-macros-core`: A new crate for declarative macros
//! 3. `neo-contract`: Re-exports macros from both above crates
//!
//! For now, the declarative macros remain defined in the `neo-contract` crate directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parser,
    parse_macro_input,
    AttributeArgs, Data, DeriveInput, 
    Fields, ItemFn, ItemStruct, Lit, Meta, 
    NestedMeta, FnArg, Pat, PatType, PatIdent
};

mod helpers {
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
/// This macro processes a struct to create a proper Neo N3 event definition.
/// Fields marked with #[index] will be indexed in the blockchain for better filtering.
///
/// # Example
///
/// ```rust
/// #[event]
/// struct Transfer {
///     #[index]
///     from: H160,
///     #[index]
///     to: H160,
///     amount: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Clone the item before parsing to avoid move issues
    let item_clone = item.clone();
    let input = parse_macro_input!(item_clone as ItemStruct);
    
    // Generate the event emitter
    if let ItemStruct { ref ident, ref fields, .. } = input {
        // Make sure we only support named fields
        if let Fields::Named(ref named_fields) = fields {
            let event_name = ident.to_string();
            let struct_vis = &input.vis;
            
            // Extract field information for notification
            let field_names = named_fields.named.iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect::<Vec<_>>();
            
            let field_types = named_fields.named.iter()
                .map(|field| &field.ty)
                .collect::<Vec<_>>();
            
            // Check for indexed fields
            let indexed_fields = named_fields.named.iter()
                .map(|field| {
                    field.attrs.iter().any(|attr| {
                        attr.path.is_ident("index")
                    })
                })
                .collect::<Vec<_>>();
            
            // Generate notification emitter
            let item_proc_macro2 = proc_macro2::TokenStream::from(item.clone());
            let output = quote! {
                // Return the original struct
                #item_proc_macro2
                
                // Generate notification emitter extension
                #[doc(hidden)]
                impl #ident {
                    #struct_vis fn emit(&self) {
                        use neo_contract::runtime;
                        let mut notify_args = Vec::new();
                        
                        // Convert each field to a notify arg
                        #(
                            let field_value = &self.#field_names;
                            notify_args.push(runtime::to_stackitem(field_value));
                        )*
                        
                        // Emit the notification
                        runtime::notify(#event_name, notify_args);
                    }
                }
            };
            
            return TokenStream::from(output);
        } else {
            // Return error: only named fields are supported
            let error = syn::Error::new_spanned(
                proc_macro2::TokenStream::from(item),
                "event attribute can only be applied to struct with named fields"
            );
            return error.to_compile_error().into();
        }
    } else {
        // This will not happen as we are parsing as ItemStruct, but kept for clarity
        let error = syn::Error::new_spanned(
            proc_macro2::TokenStream::from(item),
            "event attribute can only be applied to struct"
        );
        return error.to_compile_error().into();
    }
}

/// Marks a field in an event struct as indexed for event filtering
///
/// This attribute can only be used on fields within a struct marked with the `#[event]` attribute.
/// Indexed fields can be used to filter events when querying the blockchain.
///
/// # Example
///
/// ```rust
/// #[event]
/// struct Transfer {
///     #[index]
///     from: H160,
///     #[index]
///     to: H160,
///     amount: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Simply return the input token stream unmodified
    // This is a marker attribute that will be processed by the event macro
    item
}

/// Defines a fixed byte array constant.
#[proc_macro_attribute]
pub fn byte_array(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the length from the attribute arguments
    let length = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Int(lit)) => lit.base10_parse::<usize>().unwrap_or(32),
            _ => 32, // Default to 32 bytes if not specified correctly
        }
    } else {
        32 // Default length is 32 bytes
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new byte array initialized with zeros
            pub fn new() -> Self {
                Self::default()
            }
            
            /// Creates a byte array from a slice
            pub fn from_slice(slice: &[u8]) -> Self {
                let mut result = Self::new();
                let copy_len = core::cmp::min(slice.len(), #length);
                result.0[..copy_len].copy_from_slice(&slice[..copy_len]);
                result
            }
            
            /// Returns the length of the byte array
            pub fn len(&self) -> usize {
                #length
            }
            
            /// Returns whether the byte array is empty (always false for fixed arrays)
            pub fn is_empty(&self) -> bool {
                false
            }
            
            /// Returns a slice of the byte array
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
        
        impl From<[u8; #length]> for #struct_name {
            fn from(bytes: [u8; #length]) -> Self {
                Self(bytes)
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl AsMut<[u8]> for #struct_name {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Defines a contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the contract script hash from the attribute arguments
    let script_hash = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 contract hash
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new contract hash from a hex string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                use core::str::FromStr;
                match neo_contract::prelude::H160::from_str(hex) {
                    Ok(hash) => Ok(Self(hash.0)),
                    Err(_) => Err("Invalid contract hash format"),
                }
            }
            
            /// Returns the contract hash as a H160 type
            pub fn as_h160(&self) -> neo_contract::prelude::H160 {
                neo_contract::prelude::H160(self.0)
            }
            
            /// Get the default contract hash for this contract
            pub fn script_hash() -> Self {
                #[allow(unused_mut)]
                let mut result = Self::default();
                
                #[cfg(not(feature = "mock"))]
                {
                    if !#script_hash.is_empty() {
                        if let Ok(hash) = neo_contract::prelude::H160::from_str(&#script_hash) {
                            result = Self(hash.0);
                        }
                    }
                }
                
                result
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<neo_contract::prelude::H160> for #struct_name {
            fn from(hash: neo_contract::prelude::H160) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::H160 {
            fn from(hash: #struct_name) -> Self {
                Self(hash.0)
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the contract script hash from the attribute arguments
    let script_hash = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed contract hash
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new contract hash from a hex string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                use core::str::FromStr;
                if hex.len() != 40 {
                    return Err("Contract hash must be exactly 40 hex characters (20 bytes)");
                }
                
                let hash = neo_contract::prelude::H160::from_str(hex)
                    .map_err(|_| "Invalid contract hash format")?;
                Ok(Self(hash.0))
            }
            
            /// Returns the contract hash as a H160 type
            pub fn as_h160(&self) -> neo_contract::prelude::H160 {
                neo_contract::prelude::H160(self.0)
            }
            
            /// Returns the fixed constant contract hash value
            pub fn constant() -> Self {
                let hex = #script_hash;
                if hex.is_empty() {
                    return Self([0; 20]);
                }
                
                let mut bytes = [0u8; 20];
                
                // Handle fixed hash from hex string
                if hex.len() == 40 {
                    for i in 0..20 {
                        let byte_str = &hex[i*2..i*2+2];
                        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                            bytes[i] = byte;
                        }
                    }
                }
                
                Self(bytes)
            }
            
            /// Call a method on the contract
            pub fn call<T: neo_contract::prelude::FromNeoValue>(&self, method: &str, args: &[neo_contract::prelude::Any]) -> Option<T> {
                neo_contract::prelude::ContractExtension::call_contract(&neo_contract::prelude::H160(self.0), method, args)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 20])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<neo_contract::prelude::H160> for #struct_name {
            fn from(hash: neo_contract::prelude::H160) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::H160 {
            fn from(hash: #struct_name) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<[u8; 20]> for #struct_name {
            fn from(bytes: [u8; 20]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Registers a method in the Neo N3 contract manifest
/// 
/// This is used to expose methods to be callable from outside the contract
/// and to specify their properties in the contract manifest.
///
/// # Example
///
/// ```
/// #[manifest_method(method_name = "transfer", safe = true)]
/// fn transfer_tokens(from: H160, to: H160, amount: u64) -> bool {
///     // Implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn manifest_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    // Extract method parameters
    let method_name = extract_method_name(&args).unwrap_or_else(|| input.sig.ident.to_string());
    let is_safe = extract_safe_parameter(&args).unwrap_or(false);
    
    // Get function signature details
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_attrs = &input.attrs;
    
    // Process parameters for Neo N3 manifest
    let parameters: Vec<_> = fn_sig.inputs.iter().collect();
    
    // Generate parameter type information for manifest
    let param_types: Vec<_> = parameters.iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                let ty = &pat_type.ty;
                Some(helpers::convert_type(ty))
            } else {
                None
            }
        })
        .collect();
    
    // Generate return type information for manifest
    let return_type = match &fn_sig.output {
        syn::ReturnType::Default => "Void".to_string(),
        syn::ReturnType::Type(_, ty) => helpers::convert_type(ty),
    };
    
    // Generate the implementation
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Register the method in the manifest at compile time
            #[cfg(feature = "manifest-validation")]
            {
                extern "C" {
                    // This function is provided by the Neo N3 VM during manifest generation
                    fn _neo_register_method(
                        name: *const u8, name_len: i32,
                        params: *const u8, params_len: i32,
                        return_type: *const u8, return_type_len: i32,
                        safe: i32
                    ) -> i32;
                }
                
                // Method name from attribute or function name
                let name = #method_name;
                
                // Convert parameter types to JSON format
                let params = format!(
                    "[{}]",
                    vec![#(#param_types),*].iter()
                        .map(|ty| format!(r#"{{"type":"{}"}}"#, ty))
                        .collect::<Vec<_>>()
                        .join(",")
                );
                
                // Return type in Neo N3 format
                let return_type = #return_type;
                
                // Set safe flag based on attribute
                let safe_flag = if #is_safe { 1 } else { 0 };
                
                unsafe {
                    _neo_register_method(
                        name.as_ptr(), name.len() as i32,
                        params.as_ptr(), params.len() as i32,
                        return_type.as_ptr(), return_type.len() as i32,
                        safe_flag
                    );
                }
            }
            
            // Execute the original function body
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

// Helper function to extract method name from attributes
fn extract_method_name(args: &[NestedMeta]) -> Option<String> {
    for arg in args {
        if let NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("method_name") {
                if let Lit::Str(lit_str) = &name_value.lit {
                    return Some(lit_str.value());
                }
            }
        }
    }
    None
}

// Helper function to extract safe parameter from attributes
fn extract_safe_parameter(args: &[NestedMeta]) -> Option<bool> {
    for arg in args {
        if let NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("safe") {
                if let Lit::Bool(lit_bool) = &name_value.lit {
                    return Some(lit_bool.value);
                }
            }
        }
    }
    
    for arg in args {
        if let NestedMeta::Meta(syn::Meta::Path(path)) = arg {
            if path.is_ident("safe") {
                return Some(true);
            }
        }
    }
    
    None
}

/// Defines a Hash160 constant.
#[proc_macro_attribute]
pub fn hash160(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the hash value from the attribute arguments (if any)
    let hash_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 Hash160 type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new Hash160 from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                use core::str::FromStr;
                let hash = neo_contract::prelude::H160::from_str(hex)
                    .map_err(|_| "Invalid Hash160 format")?;
                Ok(Self(hash.0))
            }
            
            /// Creates a Hash160 from a NEO address string
            pub fn from_address(address: &str) -> Result<Self, &'static str> {
                use neo_contract::prelude::AddressExtension;
                let hash = neo_contract::prelude::H160::from_address(address)
                    .map_err(|_| "Invalid NEO address format")?;
                Ok(Self(hash.0))
            }
            
            /// Converts the Hash160 to a NEO address string
            pub fn to_address(&self) -> String {
                use neo_contract::prelude::AddressExtension;
                let h160 = neo_contract::prelude::H160(self.0);
                h160.to_address()
            }
            
            /// Returns the Hash160
            pub fn as_h160(&self) -> neo_contract::prelude::H160 {
                neo_contract::prelude::H160(self.0)
            }
            
            /// Returns whether this is the zero hash
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 20])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<neo_contract::prelude::H160> for #struct_name {
            fn from(hash: neo_contract::prelude::H160) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::H160 {
            fn from(hash: #struct_name) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<[u8; 20]> for #struct_name {
            fn from(bytes: [u8; 20]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed Hash160 constant.
#[proc_macro_attribute]
pub fn hash160_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the hash160 value from the attribute arguments
    let hash_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed Hash160 type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new Hash160 from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 40 {
                    return Err("Hash160 must be exactly 40 hex characters");
                }
                
                let mut bytes = [0u8; 20];
                for i in 0..20 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                Ok(Self(bytes))
            }
            
            /// Creates a Hash160 from a NEO address string
            pub fn from_address(address: &str) -> Result<Self, &'static str> {
                use neo_contract::prelude::AddressExtension;
                let hash = neo_contract::prelude::H160::from_address(address)
                    .map_err(|_| "Invalid NEO address format")?;
                Ok(Self(hash.0))
            }
            
            /// Converts the Hash160 to a NEO address string
            pub fn to_address(&self) -> String {
                use neo_contract::prelude::AddressExtension;
                let h160 = neo_contract::prelude::H160(self.0);
                h160.to_address()
            }
            
            /// Returns the Hash160
            pub fn as_h160(&self) -> neo_contract::prelude::H160 {
                neo_contract::prelude::H160(self.0)
            }
            
            /// Returns the fixed constant value
            pub fn constant() -> Self {
                let hex = #hash_string;
                if hex.is_empty() {
                    return Self([0; 20]);
                }
                
                let mut bytes = [0u8; 20];
                
                // Handle fixed hash from hex string
                if hex.len() == 40 {
                    for i in 0..20 {
                        let byte_str = &hex[i*2..i*2+2];
                        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                            bytes[i] = byte;
                        }
                    }
                }
                
                Self(bytes)
            }
            
            /// Returns whether this is the zero hash
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 20])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<neo_contract::prelude::H160> for #struct_name {
            fn from(hash: neo_contract::prelude::H160) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<[u8; 20]> for #struct_name {
            fn from(bytes: [u8; 20]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines an integer constant.
#[proc_macro_attribute]
pub fn integer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the integer value from the attribute arguments (if any)
    let value_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            NestedMeta::Lit(Lit::Int(lit)) => lit.base10_digits().to_string(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 Integer type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new Integer from a string value
            pub fn from_str(s: &str) -> Result<Self, &'static str> {
                use core::str::FromStr;
                match neo_contract::prelude::Int256::from_str(s) {
                    Ok(value) => Ok(Self(value.0)),
                    Err(_) => Err("Invalid integer format"),
                }
            }
            
            /// Converts the integer to a Neo N3 Int256 type
            pub fn as_int256(&self) -> neo_contract::prelude::Int256 {
                neo_contract::prelude::Int256(self.0)
            }
            
            /// Converts the integer to a u64 if possible
            pub fn to_u64(&self) -> Option<u64> {
                let int256 = neo_contract::prelude::Int256(self.0);
                if int256.is_negative() {
                    None
                } else {
                    int256.to_u64()
                }
            }
            
            /// Converts the integer to an i64 if possible
            pub fn to_i64(&self) -> Option<i64> {
                let int256 = neo_contract::prelude::Int256(self.0);
                int256.to_i64()
            }
            
            /// Returns whether this is zero
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
            
            /// Returns whether this integer is negative
            pub fn is_negative(&self) -> bool {
                neo_contract::prelude::Int256(self.0).is_negative()
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 32])
            }
        }
        
        impl From<neo_contract::prelude::Int256> for #struct_name {
            fn from(value: neo_contract::prelude::Int256) -> Self {
                Self(value.0)
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::Int256 {
            fn from(value: #struct_name) -> Self {
                Self(value.0)
            }
        }
        
        impl From<u64> for #struct_name {
            fn from(value: u64) -> Self {
                let int256 = neo_contract::prelude::Int256::from(value);
                Self(int256.0)
            }
        }
        
        impl From<i64> for #struct_name {
            fn from(value: i64) -> Self {
                let int256 = neo_contract::prelude::Int256::from(value);
                Self(int256.0)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed integer constant.
#[proc_macro_attribute]
pub fn integer_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the integer value from the attribute arguments
    let value_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            NestedMeta::Lit(Lit::Int(lit)) => lit.base10_digits().to_string(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed Integer type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new Integer from a string value
            pub fn from_str(s: &str) -> Result<Self, &'static str> {
                use core::str::FromStr;
                match neo_contract::prelude::Int256::from_str(s) {
                    Ok(value) => Ok(Self(value.0)),
                    Err(_) => Err("Invalid integer format"),
                }
            }
            
            /// Converts the integer to a Neo N3 Int256 type
            pub fn as_int256(&self) -> neo_contract::prelude::Int256 {
                neo_contract::prelude::Int256(self.0)
            }
            
            /// Converts the integer to a u64 if possible
            pub fn to_u64(&self) -> Option<u64> {
                let int256 = neo_contract::prelude::Int256(self.0);
                if int256.is_negative() {
                    None
                } else {
                    int256.to_u64()
                }
            }
            
            /// Converts the integer to an i64 if possible
            pub fn to_i64(&self) -> Option<i64> {
                let int256 = neo_contract::prelude::Int256(self.0);
                int256.to_i64()
            }
            
            /// Returns the fixed constant value
            pub fn constant() -> Self {
                use core::str::FromStr;
                let value_str = #value_string;
                if value_str.is_empty() {
                    return Self([0; 32]);
                }
                
                if let Ok(int256) = neo_contract::prelude::Int256::from_str(value_str) {
                    Self(int256.0)
                } else {
                    Self([0; 32])
                }
            }
            
            /// Returns whether this is zero
            pub fn is_zero(&self) -> bool {
                self.0.iter().all(|&b| b == 0)
            }
            
            /// Returns whether this integer is negative
            pub fn is_negative(&self) -> bool {
                neo_contract::prelude::Int256(self.0).is_negative()
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 32])
            }
        }
        
        impl From<neo_contract::prelude::Int256> for #struct_name {
            fn from(value: neo_contract::prelude::Int256) -> Self {
                Self(value.0)
            }
        }
        
        impl From<u64> for #struct_name {
            fn from(value: u64) -> Self {
                let int256 = neo_contract::prelude::Int256::from(value);
                Self(int256.0)
            }
        }
        
        impl From<i64> for #struct_name {
            fn from(value: i64) -> Self {
                let int256 = neo_contract::prelude::Int256::from(value);
                Self(int256.0)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a public key constant.
#[proc_macro_attribute]
pub fn public_key(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract public key from attributes (if any)
    let key_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 public key type (33-byte secp256r1 compressed point)
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new public key from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 66 {
                    return Err("Public key must be exactly 66 hex characters (33 bytes)");
                }
                
                let mut bytes = [0u8; 33];
                for i in 0..33 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                // Check if it's a valid compressed public key (starts with 02 or 03)
                if bytes[0] != 0x02 && bytes[0] != 0x03 {
                    return Err("Invalid public key format - must be compressed (start with 02 or 03)");
                }
                
                Ok(Self(bytes))
            }
            
            /// Verifies a signature against a message using this public key
            /// This is a wrapper around the Neo N3 Crypto.VerifyWithECDsaSecp256r1 syscall
            pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool {
                use neo_contract::prelude::CryptoExtension;
                neo_contract::prelude::Crypto::verify_with_ecdsa_secp256r1(&self.0, message, signature)
            }
            
            /// Returns the public key as a byte array
            pub fn as_bytes(&self) -> &[u8; 33] {
                &self.0
            }
            
            /// Returns the public key as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 33])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<[u8; 33]> for #struct_name {
            fn from(bytes: [u8; 33]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed public key constant.
#[proc_macro_attribute]
pub fn public_key_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the public key from the attribute arguments
    let key_string = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed public key type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new public key from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 66 {
                    return Err("Public key must be exactly 66 hex characters (33 bytes)");
                }
                
                let mut bytes = [0u8; 33];
                for i in 0..33 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                // Check if it's a valid compressed public key (starts with 02 or 03)
                if bytes[0] != 0x02 && bytes[0] != 0x03 {
                    return Err("Invalid public key format - must be compressed (start with 02 or 03)");
                }
                
                Ok(Self(bytes))
            }
            
            /// Verifies a signature against a message using this public key
            /// This is a wrapper around the Neo N3 Crypto.VerifyWithECDsaSecp256r1 syscall
            pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> bool {
                use neo_contract::prelude::CryptoExtension;
                neo_contract::prelude::Crypto::verify_with_ecdsa_secp256r1(&self.0, message, signature)
            }
            
            /// Returns the fixed constant public key value
            pub fn constant() -> Self {
                let hex = #key_string;
                if hex.is_empty() || hex.len() != 66 {
                    return Self([0; 33]);
                }
                
                let mut bytes = [0u8; 33];
                for i in 0..33 {
                    let byte_str = &hex[i*2..i*2+2];
                    if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                        bytes[i] = byte;
                    }
                }
                
                Self(bytes)
            }
            
            /// Returns the public key as a byte array
            pub fn as_bytes(&self) -> &[u8; 33] {
                &self.0
            }
            
            /// Returns the public key as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 33])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<[u8; 33]> for #struct_name {
            fn from(bytes: [u8; 33]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a string constant.
#[proc_macro_attribute]
pub fn string(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the string value from the attribute arguments (if any)
    let string_value = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 string type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new string value
            pub fn new() -> Self {
                Self::default()
            }
            
            /// Creates a string from a Rust string
            pub fn from_str(s: &str) -> Self {
                Self(s.as_bytes().to_vec())
            }
            
            /// Returns the string as a Neo ByteString
            pub fn as_byte_string(&self) -> neo_contract::prelude::ByteString {
                neo_contract::prelude::ByteString::from(&self.0[..])
            }
            
            /// Attempts to convert the bytes to a UTF-8 string
            pub fn to_string(&self) -> Result<String, core::str::Utf8Error> {
                let s = core::str::from_utf8(&self.0)?;
                Ok(s.to_string())
            }
            
            /// Returns the length of the string in bytes
            pub fn len(&self) -> usize {
                self.0.len()
            }
            
            /// Returns whether the string is empty
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            
            /// Returns the string data as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self(Vec::new())
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<&str> for #struct_name {
            fn from(s: &str) -> Self {
                Self(s.as_bytes().to_vec())
            }
        }
        
        impl From<String> for #struct_name {
            fn from(s: String) -> Self {
                Self(s.into_bytes())
            }
        }
        
        impl From<neo_contract::prelude::ByteString> for #struct_name {
            fn from(bs: neo_contract::prelude::ByteString) -> Self {
                Self(bs.into_iter().collect())
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::ByteString {
            fn from(s: #struct_name) -> Self {
                Self::from(&s.0[..])
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed string constant.
#[proc_macro_attribute]
pub fn string_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the string value from the attribute arguments
    let string_value = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed string type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a string from a Rust string
            pub fn from_str(s: &str) -> Self {
                Self(s.as_bytes().to_vec())
            }
            
            /// Returns the string as a Neo ByteString
            pub fn as_byte_string(&self) -> neo_contract::prelude::ByteString {
                neo_contract::prelude::ByteString::from(&self.0[..])
            }
            
            /// Attempts to convert the bytes to a UTF-8 string
            pub fn to_string(&self) -> Result<String, core::str::Utf8Error> {
                let s = core::str::from_utf8(&self.0)?;
                Ok(s.to_string())
            }
            
            /// Returns the length of the string in bytes
            pub fn len(&self) -> usize {
                self.0.len()
            }
            
            /// Returns whether the string is empty
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            
            /// Returns the string data as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0
            }
            
            /// Returns the fixed constant string value
            pub fn constant() -> Self {
                Self(#string_value.as_bytes().to_vec())
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self(Vec::new())
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<&str> for #struct_name {
            fn from(s: &str) -> Self {
                Self(s.as_bytes().to_vec())
            }
        }
        
        impl From<String> for #struct_name {
            fn from(s: String) -> Self {
                Self(s.into_bytes())
            }
        }
        
        impl From<neo_contract::prelude::ByteString> for #struct_name {
            fn from(bs: neo_contract::prelude::ByteString) -> Self {
                Self(bs.into_iter().collect())
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::ByteString {
            fn from(s: #struct_name) -> Self {
                Self::from(&s.0[..])
            }
        }
    };
    
    TokenStream::from(expanded)
}

/// Marks a function with a specific NEO VM opcode.
///
/// This attribute is used to indicate that the function should be compiled
/// to the specified VM opcode in the Neo N3 bytecode.
///
/// # Example
/// ```rust
/// #[vm_opcode(0xA0)] // SHA1
/// fn sha1_hash(data: &[u8]) -> [u8; 20] {
///     // Implementation will be replaced with the SHA1 opcode in Neo VM
///     unimplemented!()
/// }
/// ```
#[proc_macro_attribute]
pub fn vm_opcode(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    // Extract the opcode value from the attribute
    let opcode = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Int(lit)) => {
                lit.base10_parse::<u8>().unwrap_or(0)
            },
            NestedMeta::Lit(Lit::Str(lit)) => {
                if let Ok(val) = u8::from_str_radix(&lit.value().trim_start_matches("0x"), 16) {
                    val
                } else {
                    0
                }
            },
            _ => 0,
        }
    } else {
        0
    };
    
    // Get the function name
    let fn_name = &input.sig.ident;
    
    // Generate the output
    let expanded = quote! {
        #[cfg_attr(target_arch = "wasm32", link_section = "neo.vm.opcode")]
        #input
        
        inventory::submit! {
            neo_contract::manifest::OpcodeDescriptor::new(
                stringify!(#fn_name).to_string(),
                #opcode
            )
        }
    };
    
    TokenStream::from(expanded)
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

/// Registers an event in the Neo N3 contract manifest
/// and generates code for emitting the event.
/// 
/// The event name will be converted to a Neo VM ByteString.
/// The parameter names will be included in the contract manifest.
///
/// # Example
///
/// ```rust
/// #[register_event]
/// struct Transfer {
///     from: Option<H160>,
///     to: Option<H160>,
///     amount: Int256,
/// }
/// ```
#[proc_macro_attribute]
pub fn register_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input_struct = parse_macro_input!(item as ItemStruct);
    
    // Extract the event name from the attribute arguments or struct name
    let event_name = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => input_struct.ident.to_string(),
        }
    } else {
        input_struct.ident.to_string()
    };
    
    // Get the struct name
    let struct_name = &input_struct.ident;
    
    // Extract fields and their attributes to determine parameter names, types, and indexed status
    let fields = match &input_struct.fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("Only named fields are supported in event structs"),
    };
    
    // Generate parameter names, types, and indexed status
    let mut param_names = Vec::new();
    let mut param_types = Vec::new();
    let mut indexed_params = Vec::new();
    let mut field_names = Vec::new();
    
    for field in fields.iter() {
        // Get field name
        let field_name = field.ident.as_ref().unwrap();
        field_names.push(field_name.clone());
        
        // Get field type
        let field_type = &field.ty;
        let type_str = quote! { #field_type }.to_string();
        
        // Determine if indexed
        let mut indexed = false;
        for attr in &field.attrs {
            if attr.path.is_ident("event_param") {
                let meta = attr.parse_meta().unwrap();
                if let Meta::List(list) = meta {
                    for nested in list.nested.iter() {
                        if let NestedMeta::Meta(Meta::NameValue(name_value)) = nested {
                            if name_value.path.is_ident("indexed") {
                                if let Lit::Bool(lit_bool) = &name_value.lit {
                                    indexed = lit_bool.value;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        param_names.push(field_name.to_string());
        param_types.push(type_str);
        indexed_params.push(indexed);
    }
    
    // Generate code for event registration
    let register_code = quote! {
        inventory::submit! {
            neo_contract::manifest::EventDescriptor::new(
                #event_name.to_string(),
                vec![#(#param_names.to_string()),*],
                vec![#(#param_types.to_string()),*],
                vec![#(#indexed_params),*]
            )
        }
    };
    
    // Generate emit_event method for Neo N3 style event emission
    let emit_fn_name = format_ident!("emit_{}", event_name.to_lowercase());
    let param_conversions = field_names.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            // Convert parameter to Any type for Neo N3 Runtime::notify
            match &self.#name {
                Some(value) => event_data.push(neo_contract::prelude::Any::from(value.clone())),
                None => event_data.push(neo_contract::prelude::Any::new()),
            }
        }
    });
    
    // Generate implementation with event emission code following Neo N3 pattern
    let implementation = quote! {
        #input_struct
        
        impl #struct_name {
            /// Emits this event following Neo N3 pattern
            pub fn emit(&self) {
                use neo_contract::prelude::{ByteString, Runtime, Array, Any};
                
                // Create event name as ByteString (Neo N3 pattern)
                let event_name = ByteString::from(#event_name);
                
                // Create an Array to hold event parameters
                let mut event_data = Array::<Any>::new();
                
                // Add parameters as Any values
                #(#param_conversions)*
                
                // Emit the event using Neo N3 Runtime::notify
                Runtime::notify(&event_name, &event_data);
            }
        }
        
        // Register the event in the contract manifest
        #register_code
    };
    
    TokenStream::from(implementation)
}

/// Defines a 32-byte hash (Hash256) value.
#[proc_macro_attribute]
pub fn hash256(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 Hash256 type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new empty hash256 value
            pub fn new() -> Self {
                Self([0; 32])
            }
            
            /// Creates a hash256 from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 64 {
                    return Err("Hash256 must be exactly 64 hex characters (32 bytes)");
                }
                
                let mut bytes = [0u8; 32];
                for i in 0..32 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                Ok(Self(bytes))
            }
            
            /// Returns the hash as a byte array
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
            
            /// Returns the hash as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
            
            /// Converts the hash to a hexadecimal string
            pub fn to_hex(&self) -> String {
                let mut hex = String::with_capacity(64);
                for byte in &self.0 {
                    hex.push_str(&format!("{:02x}", byte));
                }
                hex
            }
            
            /// Returns the hash as a Neo H256 type
            pub fn as_h256(&self) -> neo_contract::prelude::H256 {
                neo_contract::prelude::H256(self.0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 32])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<neo_contract::prelude::H256> for #struct_name {
            fn from(hash: neo_contract::prelude::H256) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<#struct_name> for neo_contract::prelude::H256 {
            fn from(hash: #struct_name) -> Self {
                Self(hash.0)
            }
        }
        
        impl From<[u8; 32]> for #struct_name {
            fn from(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed 32-byte hash (Hash256) constant.
#[proc_macro_attribute]
pub fn hash256_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the hash value from the attribute arguments
    let hash_value = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed Hash256 type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a hash256 from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 64 {
                    return Err("Hash256 must be exactly 64 hex characters (32 bytes)");
                }
                
                let mut bytes = [0u8; 32];
                for i in 0..32 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                Ok(Self(bytes))
            }
            
            /// Returns the fixed constant hash value
            pub fn constant() -> Self {
                let hex = #hash_value;
                if hex.is_empty() || hex.len() != 64 {
                    return Self([0; 32]);
                }
                
                let mut bytes = [0u8; 32];
                for i in 0..32 {
                    let byte_str = &hex[i*2..i*2+2];
                    if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                        bytes[i] = byte;
                    }
                }
                
                Self(bytes)
            }
            
            /// Returns the hash as a byte array
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
            
            /// Returns the hash as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
            
            /// Converts the hash to a hexadecimal string
            pub fn to_hex(&self) -> String {
                let mut hex = String::with_capacity(64);
                for byte in &self.0 {
                    hex.push_str(&format!("{:02x}", byte));
                }
                hex
            }
            
            /// Returns the hash as a Neo H256 type
            pub fn as_h256(&self) -> neo_contract::prelude::H256 {
                neo_contract::prelude::H256(self.0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 32])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<[u8; 32]> for #struct_name {
            fn from(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a signature constant.
#[proc_macro_attribute]
pub fn signature(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 signature type (64-byte ECDSA secp256r1 signature)
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a new empty signature value
            pub fn new() -> Self {
                Self([0; 64])
            }
            
            /// Creates a signature from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 128 {
                    return Err("Signature must be exactly 128 hex characters (64 bytes)");
                }
                
                let mut bytes = [0u8; 64];
                for i in 0..64 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                Ok(Self(bytes))
            }
            
            /// Returns the signature as a byte array
            pub fn as_bytes(&self) -> &[u8; 64] {
                &self.0
            }
            
            /// Returns the signature as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
            
            /// Converts the signature to a hexadecimal string
            pub fn to_hex(&self) -> String {
                let mut hex = String::with_capacity(128);
                for byte in &self.0 {
                    hex.push_str(&format!("{:02x}", byte));
                }
                hex
            }
            
            /// Verifies the signature against a message using a specified public key
            pub fn verify(&self, message: &[u8], public_key: &[u8; 33]) -> bool {
                use neo_contract::prelude::CryptoExtension;
                neo_contract::prelude::Crypto::verify_with_ecdsa_secp256r1(public_key, message, &self.0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 64])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<[u8; 64]> for #struct_name {
            fn from(bytes: [u8; 64]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
}

/// Defines a fixed signature constant.
#[proc_macro_attribute]
pub fn signature_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    
    // Extract the signature value from the attribute arguments
    let signature_value = if !args.is_empty() {
        match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => String::new(),
        }
    } else {
        String::new()
    };
    
    // Get the struct name
    let struct_name = &input.ident;
    
    // Generate implementation for Neo N3 fixed signature type
    let expanded = quote! {
        #input
        
        impl #struct_name {
            /// Creates a signature from a hexadecimal string
            pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
                if hex.len() != 128 {
                    return Err("Signature must be exactly 128 hex characters (64 bytes)");
                }
                
                let mut bytes = [0u8; 64];
                for i in 0..64 {
                    let byte_str = &hex[i*2..i*2+2];
                    bytes[i] = u8::from_str_radix(byte_str, 16).map_err(|_| "Invalid hex character")?;
                }
                
                Ok(Self(bytes))
            }
            
            /// Returns the fixed constant signature value
            pub fn constant() -> Self {
                let hex = #signature_value;
                if hex.is_empty() || hex.len() != 128 {
                    return Self([0; 64]);
                }
                
                let mut bytes = [0u8; 64];
                for i in 0..64 {
                    let byte_str = &hex[i*2..i*2+2];
                    if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                        bytes[i] = byte;
                    }
                }
                
                Self(bytes)
            }
            
            /// Returns the signature as a byte array
            pub fn as_bytes(&self) -> &[u8; 64] {
                &self.0
            }
            
            /// Returns the signature as a byte slice
            pub fn as_slice(&self) -> &[u8] {
                &self.0[..]
            }
            
            /// Converts the signature to a hexadecimal string
            pub fn to_hex(&self) -> String {
                let mut hex = String::with_capacity(128);
                for byte in &self.0 {
                    hex.push_str(&format!("{:02x}", byte));
                }
                hex
            }
            
            /// Verifies the signature against a message using a specified public key
            pub fn verify(&self, message: &[u8], public_key: &[u8; 33]) -> bool {
                use neo_contract::prelude::CryptoExtension;
                neo_contract::prelude::Crypto::verify_with_ecdsa_secp256r1(public_key, message, &self.0)
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self([0; 64])
            }
        }
        
        impl AsRef<[u8]> for #struct_name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
        
        impl From<[u8; 64]> for #struct_name {
            fn from(bytes: [u8; 64]) -> Self {
                Self(bytes)
            }
        }
        
        impl PartialEq for #struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        
        impl Eq for #struct_name {}
    };
    
    TokenStream::from(expanded)
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

/// Marks a function as the constructor for the contract
///
/// This macro identifies the function that should be called when the contract is deployed.
/// Only one constructor is allowed per contract.
///
/// # Example
///
/// ```rust
/// #[constructor]
/// pub fn new() -> Self {
///     // Initialize contract state
///     Self {
///         // ...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn constructor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    
    // Generate a more robust constructor wrapper that integrates with Neo VM
    let output = quote! {
        // Return the original function
        #input_fn
        
        // Register with inventory system for contract manifest generation
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const _: () = {
            extern crate neo_contract;
            use neo_contract::exports::inventory;
            
            static _CONSTRUCTOR_REG: inventory::submit<&'static str> = 
                inventory::submit(stringify!(#fn_name));
        };
    };
    
    TokenStream::from(output)
}

/// Marks a function as a contract method that can be invoked
///
/// This macro identifies functions that can be called externally on the contract.
///
/// # Example
///
/// ```rust
/// #[method]
/// pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
///     // Transfer tokens
///     true
/// }
/// ```
#[proc_macro_attribute]
pub fn method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let return_type = &input_fn.sig.output;
    
    // Get function parameters
    let params = input_fn.sig.inputs.iter().collect::<Vec<_>>();
    let param_names = params.iter().filter_map(|arg| {
        if let FnArg::Typed(PatType { pat, .. }) = arg {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                return Some(ident);
            }
        }
        None
    }).collect::<Vec<_>>();
    
    // Generate method with proper export for NEO VM
    let output = quote! {
        // Original function remains
        #input_fn
        
        // Register with inventory system for manifest generation
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const _: () = {
            extern crate neo_contract;
            use neo_contract::exports::inventory;
            
            static _METHOD_REG: inventory::submit<&'static str> = 
                inventory::submit(stringify!(#fn_name));
        };
    };
    
    TokenStream::from(output)
}

/// Marks a function as a read-only contract method
///
/// This macro identifies functions that can be called externally but don't modify state.
///
/// # Example
///
/// ```rust
/// #[safe]
/// pub fn balance_of(&self, address: H160) -> u64 {
///     // Get token balance
///     0
/// }
/// ```
#[proc_macro_attribute]
pub fn safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    
    // Return the original function with inventory registration
    let output = quote! {
        // Return the original function
        #input_fn
        
        // Register with inventory system for contract manifest generation
        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        const _: () = {
            use neo_contract::exports::inventory;
            static _SAFE_METHOD_REG: inventory::submit<&'static str> = 
                inventory::submit::new(stringify!(#fn_name));
        };
    };
    
    TokenStream::from(output)
}

/// Marks a module as a smart contract.
///
/// This is the main entry point for defining a Neo smart contract in Rust.
/// It processes the module to:
///
/// - Extract method definitions
/// - Process storage struct
/// - Register events
/// - Generate manifest information
/// - Set up entry points for calling the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// mod example {
///     use neo_contract::prelude::*;
///     
///     #[storage]
///     struct ExampleContract {
///         counter: Item<u32>,
///     }
///     
///     impl ExampleContract {
///         #[constructor]
///         fn new() -> Self {
///             Self {
///                 counter: Item::new(0),
///             }
///         }
///         
///         #[method]
///         fn increment(&mut self) {
///             let counter = self.counter.get();
///             self.counter.set(counter + 1);
///         }
///         
///         #[safe]
///         fn get_counter(&self) -> u32 {
///             *self.counter.get()
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Keep a clone of the original item for parsing
    let item_for_parsing = item.clone();
    
    // Keep a separate clone for error reporting
    let item_for_error = item.clone();
    
    // Parse the module containing the contract
    let input = parse_macro_input!(item_for_parsing as syn::Item);
    
    // Convert proc_macro::TokenStream to proc_macro2::TokenStream for use with quote!
    let item_proc_macro2 = proc_macro2::TokenStream::from(item);
    
    // Generate contract entry points for Neo N3
    let output = match input {
        syn::Item::Mod(ref module) => {
            let module_ident = &module.ident;
            
            let output = quote! {
                // Return the original module
                #item_proc_macro2
                
                // Generate the NEO entry points
                #[no_mangle]
                pub extern "C" fn _deploy() -> bool {
                    // Setup code that runs at deployment time
                    #module_ident::deploying()
                }
                
                #[no_mangle]
                pub extern "C" fn main() {
                    // Main entry point that delegates to the contract implementation
                    #module_ident::main();
                }
                
                // Registration with inventory system for manifest generation
                #[doc(hidden)]
                #[allow(non_upper_case_globals)]
                const _: () = {
                    extern crate neo_contract;
                    use neo_contract::exports::inventory;
                    
                    static _CONTRACT_REG: inventory::submit<&'static str> = 
                        inventory::submit(stringify!(#module_ident));
                };
            };
            
            output
        }
        syn::Item::Struct(ref struct_item) => {
            let struct_ident = &struct_item.ident;
            quote! {
                // Return the original struct
                #item_proc_macro2
                
                // Generate the NEO entry points
                #[no_mangle]
                pub extern "C" fn _deploy() -> bool {
                    // Setup code that runs at deployment time
                    true
                }
                
                #[no_mangle]
                pub extern "C" fn main() {
                    // Main entry point that delegates to the contract implementation
                    // Default implementation uses the invoke pattern
                    let operation = neo_contract::runtime::get_invocation_operation();
                    let args = neo_contract::runtime::get_invocation_args();
                    let result = #struct_ident::invoke(operation, args);
                    neo_contract::runtime::set_invocation_result(result);
                }
                
                // Registration with inventory system for manifest generation
                #[doc(hidden)]
                #[allow(non_upper_case_globals)]
                const _: () = {
                    extern crate neo_contract;
                    use neo_contract::exports::inventory;
                    
                    static _CONTRACT_REG: inventory::submit<&'static str> = 
                        inventory::submit(stringify!(#struct_ident));
                };
            }
        }
        _ => {
            // Not a module or struct, return error
            let error = syn::Error::new_spanned(
                proc_macro2::TokenStream::from(item_for_error),
                "contract attribute can only be applied to a module or struct"
            );
            return error.to_compile_error().into();
        }
    };
    
    TokenStream::from(output)
}

// Add contract attribute macros (contract_author, contract_description, contract_version)
/// Sets the author of the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[contract_author("Neo Project")]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_author(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Return the original item
    item
}

/// Sets the description of the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[contract_description("A token contract for the Neo blockchain")]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_description(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Return the original item
    item
}

/// Sets the version of the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[contract_version("1.0.0")]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_version(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Return the original item
    item
}
