// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta,
};

/// Process the contract_permission attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the contract and methods from the attribute arguments
    if args.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected at least one argument for contract_permission attribute: contract hash and optional methods"
        )
        .to_compile_error()
        .into();
    }
    
    let _contract = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for contract hash"
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
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literals for methods"
            )
            .to_compile_error()
            .into();
        }
    }
    
    let _methods_str = methods.join(", ");
    
    // Return the original item
    item
}
