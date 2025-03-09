// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta, ItemStatic,
};

/// Public interface for the integer attribute macro
pub fn integer(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the integer attribute macro
pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the static item
    let item_static = parse_macro_input!(item as ItemStatic);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the integer value from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            &args[0],
            "Expected exactly one argument for integer attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let int_str = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                &args[0],
                "Expected string literal for integer"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Parse the integer value
    let int_value = match int_str.parse::<i64>() {
        Ok(value) => value,
        Err(err) => {
            return syn::Error::new_spanned(
                &args[0],
                format!("Invalid integer value: {}", err)
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Generate the output with the integer initialization
    let ident = &item_static.ident;
    let vis = &item_static.vis;
    let attrs = &item_static.attrs;
    let ty = &item_static.ty;
    
    let output = quote! {
        #[allow(non_upper_case_globals)]
        #(#attrs)*
        #vis static #ident: #ty = #int_value;
    };
    
    output.into()
}
