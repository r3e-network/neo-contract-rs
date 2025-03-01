// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, Meta, NestedMeta, Item,
};

/// Process the supported_standards attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the module containing the contract
    let item = parse_macro_input!(item as Item);
    
    // Parse attribute arguments (if any)
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the supportedStandards from the attribute arguments
    if args.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "Expected at least one argument for supported standards attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let standards: Vec<String> = args.iter().filter_map(|arg| {
        match arg {
            NestedMeta::Lit(Lit::Str(lit)) => Some(lit.value()),
            _ => None,
        }
    }).collect();
    
    if standards.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "Expected at least one valid string argument for supported standards attribute"
        )
        .to_compile_error()
        .into();
    }
    
    // Generate the output - for now, we'll just pass through the item
    // In a real implementation, we would store the supported standards information
    // to be used when generating the contract manifest
    let standards_str = standards.join(", ");
    let output = quote! {
        // Store supported standards information: standards = [#standards_str]
        #item
    };
    
    output.into()
}
