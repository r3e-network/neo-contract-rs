// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Item};

/// Process the no_reentrant attribute macro
pub(crate) fn generate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the item (can be a function or a module)
    let item_clone = proc_macro2::TokenStream::from(item.clone());
    let input_result = parse::<Item>(TokenStream::from(item_clone));
    
    match input_result {
        Ok(Item::Fn(mut item_fn)) => {
            // Add a marker attribute to indicate this function has reentrancy protection
            let no_reentrant_attr = syn::parse_quote!(#[no_reentrant_method]);
            item_fn.attrs.push(no_reentrant_attr);
            
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
                        runtime::log("Reentrancy attack detected".into());
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
        },
        Ok(Item::Mod(item_mod)) => {
            // For modules, we need to add the attribute to all public functions
            let mod_name = &item_mod.ident;
            let mod_content = if let Some((brace, items)) = &item_mod.content {
                let mut new_items = Vec::new();
                
                for item in items {
                    if let Item::Fn(mut item_fn) = item.clone() {
                        if is_public(&item_fn.vis) {
                            // Add a marker attribute to indicate this function has reentrancy protection
                            let no_reentrant_attr = syn::parse_quote!(#[no_reentrant_method]);
                            item_fn.attrs.push(no_reentrant_attr);
                            new_items.push(Item::Fn(item_fn));
                        } else {
                            new_items.push(Item::Fn(item_fn));
                        }
                    } else {
                        new_items.push(item.clone());
                    }
                }
                
                Some((brace.clone(), new_items))
            } else {
                None
            };
            
            let vis = &item_mod.vis;
            let attrs = &item_mod.attrs;
            
            let output = if let Some((_brace, content)) = mod_content {
                quote! {
                    #[allow(non_snake_case)]
                    #(#attrs)*
                    #vis mod #mod_name {
                        #(#content)*
                    }
                }
            } else {
                quote! {
                    #[allow(non_snake_case)]
                    #(#attrs)*
                    #vis mod #mod_name;
                }
            };
            
            output.into()
        },
        _ => {
            // If it's not a function or a module, or if parsing failed, return the original item
            item
        }
    }
}

fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}
