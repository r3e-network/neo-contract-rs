use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ItemFn,
};

/// Implementation of the `safe` attribute macro
///
/// This marks a function as a safe contract method that does not modify state.
/// The method will be callable externally without requiring a transaction, and 
/// will be included in the contract manifest as a safe method.
pub fn safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let _args = parse_macro_input!(attr as AttributeArgs);
    
    // Get function name and signature details
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;
    let fn_vis = &input.vis;
    
    // Generate the augmented method with safe=true
    // This also marks it as a method implicitly, so no need to add #[method] separately
    let output = quote! {
        #[neo_method(safe = true)]
        #fn_vis fn #fn_name(#fn_args) #fn_return {
            #fn_body
        }
    };
    
    output.into()
}
