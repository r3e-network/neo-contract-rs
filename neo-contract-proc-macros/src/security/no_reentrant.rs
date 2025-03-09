use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ItemFn,
};

/// Implementation of the `no_reentrant` attribute macro
///
/// This attribute prevents a method from being called again while it's already executing.
/// It protects against reentrancy attacks, a common vulnerability in smart contracts.
pub fn no_reentrant(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    // Get function name and signature details
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;
    let fn_vis = &input.vis;
    
    // Generate a unique lock key based on the function name
    let lock_key = format!("reentrancy_lock:{}", fn_name);
    
    // Generate the guarded function with reentrancy protection
    let output = quote! {
        #fn_vis fn #fn_name(#fn_args) #fn_return {
            // Use the reentrancy guard
            let _guard = ::neo_contract::security::reentrancy::ReentrancyGuard::new(#lock_key);
            
            // Execute the original function body
            #fn_body
        }
    };
    
    output.into()
}
