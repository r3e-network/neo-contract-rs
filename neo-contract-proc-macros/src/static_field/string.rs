// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta, ItemStatic,
};

/// Public interface for the string attribute macro
pub fn string(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the string attribute macro
pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the static item
    let item_static = parse_macro_input!(item as ItemStatic);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the string value from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            &args[0],
            "Expected exactly one argument for string attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let string_value = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                &args[0],
                "Expected string literal"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Generate the output with the string initialization
    let ident = &item_static.ident;
    let vis = &item_static.vis;
    let attrs = &item_static.attrs;
    let ty = &item_static.ty;
    
    let output = quote! {
        #[allow(non_upper_case_globals)]
        #(#attrs)*
        #vis static #ident: #ty = #string_value;
    };
    
    output.into()
}
