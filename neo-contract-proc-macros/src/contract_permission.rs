// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, Meta, NestedMeta, Item,
};

/// Process the contract_permission attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the module containing the contract
    let item = parse_macro_input!(item as Item);
    
    // Parse attribute arguments (if any)
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the contract and methods from the attribute arguments
    if args.len() < 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "Expected at least one argument for contract permission attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let contract = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::from(attr),
                "Expected string literal for contract"
            )
            .to_compile_error()
            .into();
        }
    };
    
    let methods: Vec<String> = args.iter().skip(1).filter_map(|arg| {
        match arg {
            NestedMeta::Lit(Lit::Str(lit)) => Some(lit.value()),
            _ => None,
        }
    }).collect();
    
    // Generate the output - for now, we'll just pass through the item
    // In a real implementation, we would store the contract permission information
    // to be used when generating the contract manifest
    let methods_str = methods.join(", ");
    let output = quote! {
        // Store contract permission information: contract = #contract, methods = [#methods_str]
        #item
    };
    
    output.into()
}
