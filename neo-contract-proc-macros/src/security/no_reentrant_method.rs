// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemFn,
};

/// Process the no_reentrant_method attribute macro
pub(crate) fn generate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let item_fn = parse_macro_input!(item as ItemFn);
    
    // Generate the output with reentrancy protection
    let fn_name = &item_fn.sig.ident;
    let fn_args = &item_fn.sig.inputs;
    let fn_output = &item_fn.sig.output;
    let fn_body = &item_fn.block;
    let fn_vis = &item_fn.vis;
    let fn_attrs = &item_fn.attrs;
    
    let output = quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name(#fn_args) #fn_output {
            // Reentrancy protection
            use neo::runtime;
            let executing_script_hash = runtime::get_executing_script_hash();
            let key = format!("__no_reentrant_{}", stringify!(#fn_name));
            let storage_context = runtime::get_storage_context();
            
            // Check if we're already in this function
            if storage_context.get(&key).is_some() {
                runtime::log("Reentrancy attack detected");
                panic!("Reentrancy attack detected");
            }
            
            // Set the flag
            storage_context.put(&key, &[1]);
            
            // Execute the function
            let result = {
                #fn_body
            };
            
            // Clear the flag
            storage_context.delete(&key);
            
            result
        }
    };
    
    output.into()
}
