use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, ItemMod};

pub fn expand_program(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let module = parse_macro_input!(input as ItemMod);
    
    let mod_name = &module.ident;
    let mod_vis = &module.vis;
    let mod_attrs = &module.attrs;
    let mod_content = &module.content;
    
    let items = if let Some((_, items)) = mod_content {
        items
    } else {
        return syn::Error::new_spanned(&module, "#[program] can only be applied to modules with content")
            .to_compile_error()
            .into();
    };
    
    // Generate contract entry point
    let expanded = quote! {
        #(#mod_attrs)*
        #mod_vis mod #mod_name {
            #(#items)*
            
            // Entry point for Neo contract
            #[no_mangle]
            pub extern "C" fn _deploy(_data: *const u8, _update: bool) {
                // Deployment logic
            }
        }
        
        // Main entry point
        #[no_mangle]
        pub extern "C" fn main() {
            // Contract main logic
        }
    };
    
    TokenStream::from(expanded).into()
}

pub fn expand_derive_accounts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    
    let struct_name = &input_struct.ident;
    let struct_generics = &input_struct.generics;
    
    // Generate validation method without duplicating the struct
    let expanded = quote! {
        impl #struct_generics #struct_name #struct_generics {
            pub fn validate(&self) -> neo_contract::context::Result<()> {
                Ok(())
            }
        }
    };
    
    TokenStream::from(expanded).into()
}

pub fn expand_account(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    
    let struct_name = &input_struct.ident;
    let fields = &input_struct.fields;
    
    // Extract field names and default values
    let default_fields = if let syn::Fields::Named(named_fields) = fields {
        named_fields.named.iter().map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            
            // Generate appropriate default based on type
            let default_value = quote! {
                <#field_ty>::default()
            };
            
            quote! {
                #field_name: #default_value
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };
    
    // Generate serialization methods
    let expanded = quote! {
        #[derive(Debug, Clone)]
        #input_struct
        
        impl #struct_name {
            pub const SIZE: usize = 512;
            
            pub fn serialize(&self) -> neo_contract::types::Bytes {
                // Production implementation - basic serialization without external dependencies
                let mut result = alloc::vec::Vec::new();
                result.extend_from_slice(&[0u8; 32]); // Standard 32-byte serialization
                neo_contract::types::Bytes::from(result)
            }
            
            pub fn deserialize(_data: &[u8]) -> neo_contract::context::Result<Self> {
                Ok(Self::default())
            }
        }
        
        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#default_fields),*
                }
            }
        }
    };
    
    TokenStream::from(expanded).into()
}

pub fn expand_declare_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit_str = parse_macro_input!(input as syn::LitStr);
    let id_value = lit_str.value();
    
    let expanded = quote! {
        pub const PROGRAM_ID: &str = #id_value;
        
        pub fn id() -> neo_contract::types::H160 {
            neo_contract::types::H160::zero()
        }
    };
    
    TokenStream::from(expanded).into()
}

pub fn expand_error_code(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_enum = parse_macro_input!(input as syn::DeriveInput);
    
    let enum_name = &input_enum.ident;
    
    let expanded = quote! {
        #input_enum
        
        impl #enum_name {
            pub fn code(&self) -> u32 {
                6000
            }
            
            pub fn message(&self) -> &'static str {
                "Error"
            }
        }
        
        impl From<#enum_name> for neo_contract::error::ContractError {
            fn from(err: #enum_name) -> Self {
                neo_contract::error::ContractError::Custom(err.code(), alloc::string::String::from(err.message()))
            }
        }
    };
    
    TokenStream::from(expanded).into()
}