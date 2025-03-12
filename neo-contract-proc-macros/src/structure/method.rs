use proc_macro::TokenStream;
use quote::quote;
// Suppressing unused import warning as Type may be needed for future enhancements
#[allow(unused_imports)]
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Pat, Meta, NestedMeta, Lit};

/// Implementation of the `method` attribute macro
///
/// This marks a function as a contract method that can modify state.
/// The method will be callable externally and included in the contract manifest.
/// 
/// In Neo N3, contract methods are properly registered in the manifest,
/// distinguishing between safe (read-only) and non-safe (state-modifying) methods.
pub fn method(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    
    // Parse the safe attribute if present
    let mut is_safe = false;
    for arg in &args {
        if let NestedMeta::Meta(Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("safe") {
                if let Lit::Bool(lit_bool) = &name_value.lit {
                    is_safe = lit_bool.value;
                }
            }
        }
    }
    
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
    
    // Extract parameter types for Neo N3 manifest generation
    let param_types = fn_args.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            let ty = &*pat_type.ty;
            return Some(ty);
        }
        None
    });
    
    // The param_types variable defined above is used directly in the manifest registration on line 112
    
    // Clone parameter names for generating parameter access code
    let param_names_for_code = fn_args.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(&pat_ident.ident);
            }
        }
        None
    });
    
    // Generate the safe or non-safe attribute for Neo N3
    let safe_attr = if is_safe {
        quote! { #[neo_method(safe = true)] }
    } else {
        quote! { #[neo_method(safe = false)] }
    };
    
    // Generate the augmented method with Neo N3 specific attributes
    let output = quote! {
        // Apply the appropriate safe attribute for Neo N3 manifest
        #safe_attr
        
        // Generate the modified function with metadata for Neo N3 manifest
        #fn_vis fn #fn_name(#fn_args) #fn_return {
            // Log method invocation for Neo N3 debugging with detailed safety information
            ::neo_contract::prelude::Runtime::log(&format!("Neo N3 {} method invocation: {}", 
                if #is_safe { "safe (read-only)" } else { "non-safe (state-modifying)" }, 
                stringify!(#fn_name)));
            
            // Log parameter values for debugging
            #(::neo_contract::prelude::Runtime::log(&format!("Parameter {}: {:?}", 
                stringify!(#param_names_for_code), #param_names_for_code));)*
            
            // For state-modifying methods, we might want to check witness to ensure proper authorization
            if !#is_safe {
                ::neo_contract::prelude::Runtime::log("Note: This is a state-modifying method that could require witness checks");
            }
            
            // Execute the original method body
            let result = #fn_body;
            
            // Log method completion
            ::neo_contract::prelude::Runtime::log(&format!("Method {} execution completed", stringify!(#fn_name)));
            
            result
        }
        
        // Register the method with the Neo N3 manifest using detailed type information
        #[cfg(feature = "manifest-generation")]
        const _: () = {
            // Register method in Neo N3 manifest with proper type handling
            ::neo_contract::manifest::register_method(
                stringify!(#fn_name),  // method name
                #is_safe,  // safe status for Neo N3 manifest (true = read-only, false = state-modifying)
                &[#(stringify!(#param_names)),*],  // parameter names for Neo N3 manifest
                &[#(stringify!(#param_types)),*],  // parameter types for Neo N3 manifest
                stringify!(#fn_return)  // return type for Neo N3 manifest
            );
            
            // Log the method registration at compile time
            ::neo_contract::prelude::Runtime::log(&format!("Registered Neo N3 method: {} (safe: {})", 
                stringify!(#fn_name), #is_safe));
        };
    };
    
    output.into()
}