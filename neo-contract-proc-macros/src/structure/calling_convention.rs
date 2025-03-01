// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, NestedMeta, ItemFn,
};

/// Process the calling_convention attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let mut item_fn = parse_macro_input!(item as ItemFn);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the calling convention from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected exactly one argument for calling_convention attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let convention = match &args[0] {
        NestedMeta::Meta(meta) => {
            if let Some(ident) = meta.path().get_ident() {
                Some(ident.to_string())
            } else {
                None
            }
        },
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected calling convention identifier (e.g., Cdecl, StdCall)"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Add a marker attribute to indicate the calling convention
    if let Some(convention) = convention {
        let calling_convention_attr = syn::parse_quote!(#[calling_convention = #convention]);
        item_fn.attrs.push(calling_convention_attr);
    }
    
    // Generate the output
    let output = quote! {
        #item_fn
    };
    
    output.into()
}
