// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta, ItemStatic,
};

/// Public interface for the contract_hash_fixed attribute macro
pub fn contract_hash_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate(attr, item)
}

/// Process the contract_hash_fixed attribute macro
pub fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the static item
    let item_static = parse_macro_input!(item as ItemStatic);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the contract hash value from the attribute arguments
    if args.len() != 1 {
        return syn::Error::new_spanned(
            &args[0],
            "Expected exactly one argument for contract_hash attribute"
        )
        .to_compile_error()
        .into();
    }
    
    let hex_string = match &args[0] {
        NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
        _ => {
            return syn::Error::new_spanned(
                &args[0],
                "Expected string literal for contract hash"
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Convert hex string to bytes
    let bytes = match hex_to_bytes(&hex_string) {
        Ok(bytes) => bytes,
        Err(err) => {
            return syn::Error::new_spanned(
                &args[0],
                format!("Invalid hex string: {}", err)
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Generate the output with the contract hash initialization
    let ident = &item_static.ident;
    let vis = &item_static.vis;
    let attrs = &item_static.attrs;
    let ty = &item_static.ty;
    
    let output = quote! {
        #[allow(non_upper_case_globals)]
        #(#attrs)*
        #vis static #ident: #ty = [#(#bytes),*];
    };
    
    output.into()
}

/// Convert a hex string to a vector of bytes
fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    let hex = hex.trim().trim_start_matches("0x");
    
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }
    
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    
    for i in (0..hex.len()).step_by(2) {
        let byte_str = &hex[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|_| format!("Invalid hex character in '{}'", byte_str))?;
        bytes.push(byte);
    }
    
    Ok(bytes)
}
