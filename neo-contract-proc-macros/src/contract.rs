// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use proc_macro2::{Span, TokenStream};

const NEP11_TOKEN: &str = "Nep11Token";
const NEP17_TOKEN: &str = "Nep17Token";

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

fn expand_nep17_methods(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = quote::quote! {};

    // `_initialize` has default implementation
    if !has_method(item, "_initialize") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn _initialize() {
                #self_type::_initialize()
            }
        });
    }

    // total_supply has default implementation
    if !has_method(item, "total_supply") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn total_supply() -> neo_contract::types::Int256 {
                #self_type::total_supply()
            }
        });
    }

    // `balance_of` has default implementation
    if !has_method(item, "balance_of") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn balance_of(owner: neo_contract::types::H160) -> neo_contract::types::Int256 {
                #self_type::balance_of(owner)
            }
        });
    }

    // `transfer` has default implementation
    if !has_method(item, "transfer") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn transfer(
                from: neo_contract::types::H160,
                to: neo_contract::types::H160,
                amount: neo_contract::types::Int256,
            ) -> bool {
                #self_type::transfer(from, to, amount)
            }
        });
    }

    // `mint` has default implementation
    if !has_method(item, "mint") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn mint(to: neo_contract::types::H160, amount: neo_contract::types::Int256) {
                #self_type::mint(to, amount)
            }
        });
    }

    // `burn` has default implementation
    if !has_method(item, "burn") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn burn(from: neo_contract::types::H160, amount: neo_contract::types::Int256) {
                #self_type::burn(from, amount)
            }
        });
    }

    methods
}

fn expand_nep11_methods(item: &syn::ItemImpl) -> TokenStream {
    let self_type = item.self_ty.as_ref();
    let mut methods: TokenStream = quote::quote! {};
    if !has_method(item, "_initialize") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn _initialize() {
                #self_type::_initialize()
            }
        });
    }

    methods
}

fn has_method(item: &syn::ItemImpl, name: &str) -> bool {
    item.items.iter().any(|item| {
        if let syn::ImplItem::Fn(method) = item {
            method.sig.ident == name
        } else {
            false
        }
    })
}
