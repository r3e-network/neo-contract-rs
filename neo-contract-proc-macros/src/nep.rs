// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use proc_macro2::TokenStream;
use syn::PathArguments;

pub(crate) const NEP11_TOKEN: &str = "Nep11Token";
pub(crate) const NEP17_TOKEN: &str = "Nep17Token";

pub(crate) fn expand_nep17_methods(item: &syn::ItemImpl) -> TokenStream {
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

pub(crate) fn expand_nep11_methods(item: &syn::ItemImpl) -> TokenStream {
    let mut methods: TokenStream = quote::quote! {};
    let token_state_type = item
        .trait_
        .as_ref()
        .map(|(_, path, _)| path)
        .and_then(|path| path.segments.last())
        .and_then(|segment| match &segment.arguments {
            PathArguments::AngleBracketed(args) => Some(args),
            _ => None,
        })
        .and_then(|args| args.args.last())
        .expect("Nep11Token must have one type argument");

    let self_type = item.self_ty.as_ref();

    // `_initialize` has default implementation
    if !has_method(item, "_initialize") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn _initialize() {
                #self_type::_initialize()
            }
        });
    }

    // `total_supply` has default implementation
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

    // `owner_of` has default implementation
    if !has_method(item, "owner_of") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn owner_of(token_id: neo_contract::types::ByteString) -> neo_contract::types::H160 {
                #self_type::owner_of(token_id)
            }
        });
    }

    // `properties` has default implementation
    if !has_method(item, "properties") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn properties(
                token_id: neo_contract::types::ByteString,
            ) -> neo_contract::types::Map<neo_contract::types::ByteString, neo_contract::types::Any> {
                #self_type::properties(token_id)
            }
        });
    }

    // `tokens` has default implementation
    if !has_method(item, "tokens") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn tokens() -> neo_contract::storage::Iter<#token_state_type> {
                #self_type::tokens()
            }
        });
    }

    // `tokens_of` has default implementation
    if !has_method(item, "tokens_of") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn tokens_of(owner: neo_contract::types::H160) -> neo_contract::storage::Iter<#token_state_type> {
                #self_type::tokens_of(owner)
            }
        });
    }

    // `transfer` has default implementation
    if !has_method(item, "transfer") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn transfer(to: neo_contract::types::H160, token_id: neo_contract::types::ByteString) -> bool {
                #self_type::transfer(to, token_id)
            }
        });
    }

    // `mint` has default implementation
    if !has_method(item, "mint") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn mint(token_id: neo_contract::types::ByteString, token_state: #token_state_type) {
                #self_type::mint(token_id, token_state)
            }
        });
    }

    // `burn` has default implementation
    if !has_method(item, "burn") {
        methods.extend(quote::quote! {
            #[no_mangle]
            pub fn burn(token_id: neo_contract::types::ByteString) {
                #self_type::burn(token_id)
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
