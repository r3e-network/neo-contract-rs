// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta,
};

/// Public interface for the manifest_extra attribute macro
pub fn manifest_extra(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the manifest_extra attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the key and value from the attribute arguments
    if args.is_empty() || args.len() > 2 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected one or two arguments for manifest_extra attribute: key and optional value"
        )
        .to_compile_error()
        .into();
    }
    
    let _key = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for key"
            )
            .to_compile_error()
            .into();
        }
    };
    
    let _value = if args.len() > 1 {
        match &args[1] {
            NestedMeta::Lit(Lit::Str(lit)) => Some(lit.value()),
            _ => {
                return syn::Error::new_spanned(
                    proc_macro2::TokenStream::new(),
                    "Expected string literal for value"
                )
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
