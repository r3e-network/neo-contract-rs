// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, Meta, NestedMeta, Item,
};

/// Process the manifest_extra attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the module containing the contract
    let item = parse_macro_input!(item as Item);
    
    // Parse attribute arguments (if any)
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the key and value from the attribute arguments
    if args.len() < 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "Expected at least one argument for manifest extra attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let key = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::from(attr),
                "Expected string literal for key"
            )
            .to_compile_error()
            .into();
        }
    };
    
    let value = if args.len() > 1 {
        match &args[1] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => {
                return syn::Error::new_spanned(
                    proc_macro2::TokenStream::from(attr),
                    "Expected string literal for value"
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        String::new()
    };
    
    // Generate the output - for now, we'll just pass through the item
    // In a real implementation, we would store the manifest extra information
    // to be used when generating the contract manifest
    let output = quote! {
        // Store manifest extra information: key = #key, value = #value
        #item
    };
    
    output.into()
}
