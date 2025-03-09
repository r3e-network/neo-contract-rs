use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ItemFn,
};

/// Implementation of the `method` attribute macro
///
/// This marks a function as a contract method that can modify state.
/// The method will be callable externally and included in the contract manifest.
pub fn method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    // Get function name and signature details
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;
    let fn_vis = &input.vis;
    
    // Generate the augmented method
    let output = quote! {
        #[neo_method(safe = false)]
        #fn_vis fn #fn_name(#fn_args) #fn_return {
            #fn_body
        }
    };
    
    output.into()
}