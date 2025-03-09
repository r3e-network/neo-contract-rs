// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta,
};

/// Public interface for the supported_standards attribute macro
pub fn supported_standards(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the supported_standards attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the standards from the attribute arguments
    if args.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected at least one argument for supported_standards attribute"
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
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literals for standards"
            )
            .to_compile_error()
            .into();
        }
    }
    
    let _standards_str = standards.join(", ");
    
    // Return the original item
    item
}
