// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta,
};

/// Public interface for the contract_trust attribute macro
pub fn contract_trust(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the contract_trust attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the contract or group from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected exactly one argument for contract_trust attribute: contract hash or group"
        )
        .to_compile_error()
        .into();
    }
    
    let _contract_or_group = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for contract hash or group"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Return the original item
    item
}
