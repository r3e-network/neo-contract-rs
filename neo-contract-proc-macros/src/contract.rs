// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use crate::nep::*;

use proc_macro2::{Span, TokenStream};

pub(crate) fn expand_contract_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut src_impl: TokenStream = input.clone().into();
    let input = syn::parse::<syn::Item>(input);
    match input {
        Ok(syn::Item::Impl(item)) => src_impl.extend(expand_impl_item(&item)),
        Ok(_) => {
            return syn::Error::new(Span::call_site(), "`#[contract]` can only be applied to `impl` block")
                .to_compile_error()
                .into()
        }
        Err(err) => return err.to_compile_error().into(),
    }

    src_impl.into()
}

fn expand_impl_item(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = item
        .items
        .iter()
        .filter_map(|x| match x {
            syn::ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .map(|method| {
            let name = &method.sig.ident;
            let args = &method.sig.inputs;
            let returns = &method.sig.output;
            quote::quote! {
                #[no_mangle]
                pub fn #name() #returns {
                    #self_type::#name(#args)
                }
            }
        })
        .collect();

    if let Some((None, path, _for)) = &item.trait_ {
        // check path is Nep17Token or not
        if path.segments.last().map(|x| x.ident == NEP17_TOKEN).unwrap_or(false) {
            methods.extend(expand_nep17_methods(item));
        }

        if path.segments.last().map(|x| x.ident == NEP11_TOKEN).unwrap_or(false) {
            methods.extend(expand_nep11_methods(item));
        }
    }

    methods
}

