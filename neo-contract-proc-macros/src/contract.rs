// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, AttributeArgs, FnArg, ImplItem, ImplItemMethod, Item, ItemImpl,
    Meta, NestedMeta, Pat, PatIdent,
};

/// Public interface for the contract attribute macro
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the contract attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the module containing the contract
    let module = parse_macro_input!(item as Item);
    
    // Parse attribute arguments (if any)
    let _args = parse_macro_input!(attr as AttributeArgs);
    
    // Process the module and generate the contract code
    match process_contract_module(module, _args) {
        Ok(output) => output.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Process the contract module and generate the contract code
fn process_contract_module(module: Item, _args: AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    // Extract the module items
    let module_items = match &module {
        Item::Mod(mod_item) => {
            let content = mod_item.content.as_ref().ok_or_else(|| {
                syn::Error::new_spanned(mod_item, "Expected module with content")
            })?;
            &content.1
        },
        _ => return Err(syn::Error::new_spanned(
            module.to_token_stream(),
            "Expected module",
        )),
    };
    
    // Find the storage struct and implementation blocks
    let mut storage_struct = None;
    let mut impl_blocks = Vec::new();
    
    for item in module_items {
        match item {
            Item::Struct(s) => {
                // Check if this is the storage struct (has #[storage] attribute)
                if has_neo_attribute(&s.attrs, "storage") {
                    if storage_struct.is_some() {
                        return Err(syn::Error::new_spanned(
                            s.to_token_stream(),
                            "Multiple storage structs found",
                        ));
                    }
                    storage_struct = Some(s.clone());
                }
            }
            Item::Impl(i) => {
                impl_blocks.push(i.clone());
            }
            _ => {}
        }
    }
    
    // Ensure we found a storage struct
    let storage_struct = storage_struct.ok_or_else(|| {
        syn::Error::new_spanned(
            module.to_token_stream(),
            "No storage struct found. Add #[storage] to your contract's storage struct",
        )
    })?;
    
    // Process implementation blocks
    let _processed_impls = process_impl_blocks(impl_blocks)?;
    
    // Generate contract entry points for Neo N3
    let contract_name = &storage_struct.ident;
    
    // Generate proper Neo N3 contract dispatch mechanism with detailed logging
    let entry_points = quote! {
        #[no_mangle]
        pub extern "C" fn _deploy(data: bool) -> bool {
            // Setup contract initialization during deployment
            neo_contract::prelude::Runtime::log("Deploying Neo N3 contract");
            
            // Log deployment parameters
            neo_contract::prelude::Runtime::log(&format!("Deploy parameter: {}", data));
            
            // Initialize storage if needed during deployment
            if data {
                // This could be used to initialize contract storage during deployment
                neo_contract::prelude::Runtime::log("Initializing contract storage during deployment");
            }
            
            true
        }
        
        #[no_mangle]
        pub extern "C" fn _initialize() -> bool {
            // Contract initialization logic
            neo_contract::prelude::Runtime::log("Initializing Neo N3 contract");
            
            // Initialize contract state if needed
            #contract_name::init();
            
            true
        }
        
        // Function to verify Neo N3 contract ownership during invocation
        fn check_witness(required_witness: Option<&neo_contract::prelude::H160>) -> bool {
            if let Some(witness) = required_witness {
                // Verify that the transaction sender is authorized
                neo_contract::prelude::Runtime::check_witness(witness)
            } else {
                // No witness check required
                true
            }
        }
        
        // Neo N3 contract method dispatcher with enhanced error handling
        #[no_mangle]
        pub extern "C" fn invoke(operation: *const u8, args: *const u8) -> *const u8 {
            // Convert C-style parameters to Rust types following Neo N3 standards
            let operation = unsafe {
                neo_contract::prelude::NeoString::from_raw(operation)
            };
            let args = unsafe {
                neo_contract::prelude::NeoArray::from_raw(args)
            };
            
            // Detailed logging with parameter information for Neo N3 debugging
            neo_contract::prelude::Runtime::log(&format!("Neo N3 contract invocation: {} with {} arguments", operation, args.len()));
            
            // Log each argument for detailed debugging if needed
            for (i, arg) in args.iter().enumerate() {
                neo_contract::prelude::Runtime::log(&format!("Arg {}: {:?}", i, arg));
            }
            
            // Create contract instance
            let contract = #contract_name::new();
            
            // Dispatch to the appropriate method with proper error handling
            let result = neo_contract::prelude::dispatch(operation, args, contract);
            
            // Log the result of dispatch
            neo_contract::prelude::Runtime::log("Method execution completed");
            
            result
        }
        
        // Helper function to simplify contract initialization
        impl #contract_name {
            pub fn init() -> bool {
                neo_contract::prelude::Runtime::log("Contract initialization called");
                true
            }
        }
    };
    
    // Register contract in the Neo N3 manifest for proper compilation
    let manifest_registration = quote! {
        #[cfg(feature = "manifest-generation")]
        const _: () = {
            // Register the contract in the Neo N3 manifest generator
            ::neo_contract::manifest::register_contract(
                stringify!(#contract_name),
                "1.0.0", // Version
                "R3E Network", // Author
                "Neo N3 Smart Contract", // Description
                "Email", // Email
                true // Dynamic invoke allowed
            );
        };
    };
    
    // Generate the final output
    let output = quote! {
        // Re-export the original module for clean organization
        #module
        
        // Add contract entry points outside the module
        #entry_points
        
        // Add manifest registration for Neo N3
        #manifest_registration
    };
    
    Ok(output)
}

/// Process implementation blocks to extract and transform methods
fn process_impl_blocks(impl_blocks: Vec<ItemImpl>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut processed_impls = Vec::new();
    
    for impl_block in impl_blocks {
        // Process each implementation block
        let processed_impl = process_impl_block(impl_block)?;
        processed_impls.push(processed_impl);
    }
    
    Ok(processed_impls)
}

/// Process a single implementation block
fn process_impl_block(impl_block: ItemImpl) -> syn::Result<proc_macro2::TokenStream> {
    let self_ty = &impl_block.self_ty;
    let trait_path = impl_block.trait_.map(|(path, _, _)| path);
    
    // Process each method in the implementation
    let mut processed_methods = Vec::new();
    
    for item in &impl_block.items {
        if let ImplItem::Method(method) = item {
            // Check for neo attributes
            if has_neo_attribute(&method.attrs, "message") {
                let processed = process_message_method(method)?;
                processed_methods.push(processed);
            } else if has_neo_attribute(&method.attrs, "event") {
                let processed = process_event_method(method)?;
                processed_methods.push(processed);
            } else if has_neo_attribute(&method.attrs, "constructor") {
                // TODO: Implement constructor processing
                processed_methods.push(quote! { #method });
            } else {
                // Regular method, keep as is
                processed_methods.push(quote! { #method });
            }
        }
    }
    
    // Generate the processed implementation block
    let result = if let Some(trait_path) = trait_path {
        quote! {
            impl #trait_path for #self_ty {
                #(#processed_methods)*
            }
        }
    } else {
        quote! {
            impl #self_ty {
                #(#processed_methods)*
            }
        }
    };
    
    Ok(result)
}

/// Process a method with the #[neo(message)] attribute
fn process_message_method(method: &ImplItemMethod) -> syn::Result<proc_macro2::TokenStream> {
    let vis = &method.vis;
    let sig = &method.sig;
    let name = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;
    let block = &method.block;
    
    // Generate the contract method
    let result = quote! {
        #[no_mangle]
        #vis extern "C" fn #name(#inputs) #output #block
    };
    
    Ok(result)
}

/// Process a method with the #[neo(event)] attribute
fn process_event_method(method: &ImplItemMethod) -> syn::Result<proc_macro2::TokenStream> {
    let vis = &method.vis;
    let sig = &method.sig;
    let name = &sig.ident;
    let inputs = &sig.inputs;
    
    // Extract argument names and types
    let mut arg_names = Vec::new();
    
    for input in inputs {
        if let FnArg::Typed(pat_type) = input {
            if let Pat::Ident(PatIdent { ident, .. }) = &*pat_type.pat {
                arg_names.push(ident);
            }
        }
    }
    
    // Generate the event method
    let result = quote! {
        #vis fn #name(#inputs) {
            let args = neo_contract::types::builtin::Array::new();
            #(
                args.push(#arg_names.into());
            )*
            
            unsafe {
                neo_contract::env::syscall::system_runtime_notify(
                    neo_contract::types::builtin::ByteString::new(stringify!(#name)),
                    args
                );
            }
        }
    };
    
    Ok(result)
}

/// Check if an attribute list contains a specific neo attribute
fn has_neo_attribute(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| {
        // Check for #[neo(storage)] format
        if attr.path.is_ident("neo") {
            if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                return meta_list.nested.iter().any(|nested_meta| {
                    if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                        return path.is_ident(name);
                    }
                    false
                });
            }
        }
        // Check for #[neo_contract::storage] format
        else if attr.path.segments.len() > 1 && 
                attr.path.segments[0].ident == "neo_contract" {
            if let Ok(Meta::Path(path)) = attr.parse_meta() {
                return path.segments.last().map_or(false, |seg| seg.ident == name);
            }
        }
        // Check for ink! style #[storage] format
        else if attr.path.is_ident(name) {
            return true;
        }
        // Check for #[method] #[safe] #[constructor] etc.
        else if attr.path.is_ident(&format!("neo_{}", name)) {
            return true;
        }
        // For handling method->message and other potential renames
        else if (name == "message" && attr.path.is_ident("method")) || 
                (name == "method" && attr.path.is_ident("message")) {
            return true;
        }
        false
    })
}
