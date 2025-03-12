use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, ItemFn, FnArg, Pat,
};

/// Implementation of the `safe` attribute macro
///
/// This marks a function as a safe contract method that does not modify state.
/// The method will be callable externally without requiring a transaction, and 
/// will be included in the contract manifest as a safe method.
///
/// In Neo N3, safe methods are represented in the contract manifest with "safe": true
/// which is critical for optimizing contract execution and ensuring proper access control.
pub fn safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let _args = parse_macro_input!(attr as AttributeArgs);
    
    // Get function name and signature details
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;
    let fn_vis = &input.vis;
    
    // Extract parameter information for Neo N3 manifest generation
    let param_names = fn_args.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(&pat_ident.ident);
            }
        }
        None
    });
    
    // Extract parameter types for Neo N3 manifest
    let param_types = fn_args.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            let ty = &*pat_type.ty;
            return Some(ty);
        }
        None
    });
    
    // Generate the augmented method with safe=true for Neo N3
    let output = quote! {
        // Mark as safe=true in Neo N3 manifest (as required by Neo N3 standard)
        #[neo_method(safe = true)]
        #fn_vis fn #fn_name(#fn_args) #fn_return {
            // Log safe method invocation for Neo N3 debugging
            ::neo_contract::prelude::Runtime::log(&format!("Invoking Neo N3 safe method: {}", stringify!(#fn_name)));
            
            // Execute the method body (no state changes allowed in safe methods)
            #fn_body
        }
        
        // Register this as a safe method in the Neo N3 manifest with detailed type information
        #[cfg(feature = "manifest-generation")]
        const _: () = {
            ::neo_contract::manifest::register_method(
                stringify!(#fn_name), // method name
                true, // safe = true for Neo N3 read-only methods
                &[#(stringify!(#param_names)),*], // parameter names
                &[#(stringify!(#param_types)),*], // parameter types
                stringify!(#fn_return) // return type
            );
        };
    };
    
    output.into()
}
