// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs, Lit, NestedMeta, ItemFn,
};

/// Process the op_code attribute macro
pub(crate) fn generate(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function
    let mut item_fn = parse_macro_input!(item as ItemFn);
    
    // Parse attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    
    // Extract the opcode and optional syscall from the attribute arguments
    if args.is_empty() || args.len() > 2 {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(),
            "Expected one or two arguments for op_code attribute: OpCode and optional syscall name"
        )
        .to_compile_error()
        .into();
    }
    
    let opcode = match &args[0] {
        NestedMeta::Meta(meta) => {
            if let Some(ident) = meta.path().get_ident() {
                Some(ident.to_string())
            } else {
                None
            }
        },
        _ => {
            return syn::Error::new_spanned(
                proc_macro2::TokenStream::new(),
                "Expected OpCode identifier (e.g., SYSCALL, CALL)"
            )
            .to_compile_error()
            .into();
        }
    };
    
    let syscall = if args.len() > 1 {
        match &args[1] {
            NestedMeta::Lit(Lit::Str(lit)) => Some(lit.value()),
            _ => {
                return syn::Error::new_spanned(
                    proc_macro2::TokenStream::new(),
                    "Expected string literal for syscall name"
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        None
    };
    
    // Add marker attributes to indicate the opcode and syscall
    if let Some(opcode) = opcode {
        let opcode_attr = syn::parse_quote!(#[opcode = #opcode]);
        item_fn.attrs.push(opcode_attr);
        
        if let Some(syscall) = syscall {
            let syscall_attr = syn::parse_quote!(#[syscall = #syscall]);
            item_fn.attrs.push(syscall_attr);
        }
    }
    
    // Generate the output
    let output = quote! {
        #item_fn
    };
    
    output.into()
}
