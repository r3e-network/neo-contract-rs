// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemFn,
};

/// Process the safe attribute macro
pub(crate) fn generate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let mut item_fn = parse_macro_input!(item as ItemFn);
    
    // Add a marker attribute to indicate this is a safe method
    let safe_attr = syn::parse_quote!(#[safe_method]);
    item_fn.attrs.push(safe_attr);
    
    // Generate the output
    let output = quote! {
        #item_fn
    };
    
    output.into()
}
