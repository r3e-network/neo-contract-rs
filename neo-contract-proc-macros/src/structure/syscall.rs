// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta, ItemFn,
};

/// Process the syscall attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let mut item_fn = parse_macro_input!(item as ItemFn);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the syscall name from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected exactly one argument for syscall attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let syscall_name = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected string literal for syscall name"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Add a marker attribute to indicate the syscall
    let syscall_attr = syn::parse_quote!(#[syscall = #syscall_name]);
    item_fn.attrs.push(syscall_attr);
    
    // Generate the output
    let output = quote! {
        #item_fn
    };
    
    output.into()
}
