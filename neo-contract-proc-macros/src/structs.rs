// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use proc_macro2::{Span, TokenStream};
use quote::format_ident;

pub(crate) fn expand_structs_impl(is_inner: bool, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse::<syn::Item>(input);
    match input {
        Ok(syn::Item::Struct(item)) => expand_struct_item(is_inner, item).into(),
        Ok(_) => {
            return syn::Error::new(Span::call_site(), "`structs` attribute can only be applied to `struct`")
                .to_compile_error()
                .into()
        }
        Err(err) => return err.to_compile_error().into(),
    }
}

fn expand_struct_item(is_inner: bool, item: syn::ItemStruct) -> TokenStream {
    let attrs = &item.attrs;
    let vis = &item.vis;
    let ident = &item.ident;
    let generics = &item.generics;
    let semi_token = &item.semi_token;
    let (impls, types, wheres) = &item.generics.split_for_impl();

    let _crate = if is_inner {
        syn::Ident::new("crate", Span::call_site())
    } else {
        syn::Ident::new("neo_contract", Span::call_site())
    };

    let mut expanded = match &item.fields {
        syn::Fields::Named(_fields) => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #(#attrs)*
                #vis struct #ident #generics {
                    placeholder: #_crate::types::placeholder::Placeholder,
                } #semi_token

                #[cfg(target_family = "wasm")]
                impl #impls #_crate::types::placeholder::FromPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn from_placeholder(placeholder: #_crate::types::placeholder::Placeholder) -> Self {
                        Self { placeholder }
                    }
                }

                #[cfg(target_family = "wasm")]
                impl #impls #_crate::types::placeholder::IntoPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn into_placeholder(self) -> #_crate::types::placeholder::Placeholder {
                        self.placeholder
                    }
                }
            }
        }
        syn::Fields::Unnamed(_fields) => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #(#attrs)*
                #vis struct #ident #generics (#_crate::types::placeholder::Placeholder) #semi_token

                #[cfg(target_family = "wasm")]
                impl #impls #_crate::types::placeholder::FromPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn from_placeholder(placeholder: #_crate::types::placeholder::Placeholder) -> Self {
                        Self(placeholder)
                    }
                }

                #[cfg(target_family = "wasm")]
                impl #impls #_crate::types::placeholder::IntoPlaceholder for #ident #types #wheres {
                    #[inline(always)]
                    fn into_placeholder(self) -> #_crate::types::placeholder::Placeholder {
                        self.0
                    }
                }
            }
        }
        syn::Fields::Unit => {
            quote::quote! {
                #[cfg(target_family = "wasm")]
                #item
            }
        }
    };

    let mut fields = item.fields.clone();
    remove_attr(&mut fields, "get");
    remove_attr(&mut fields, "set");
    expanded.extend(quote::quote! {
        #[cfg(not(target_family = "wasm"))]
        #(#attrs)*
        #vis struct #ident #generics #fields #semi_token
    });

    // impl getters for fields
    let getsets = expand_fields_getset(&_crate, &item.fields);
    if !getsets.is_empty() {
        expanded.extend(quote::quote! {
            impl #impls #ident #types #wheres {
                #getsets
            }
        });
    }

    expanded
}

fn expand_fields_getset(_crate: &syn::Ident, fields: &syn::Fields) -> TokenStream {
    let mut expanded = quote::quote! {};
    match fields {
        syn::Fields::Named(fields) => {
            expanded.extend(expand_named_field(_crate, fields));
        }
        syn::Fields::Unnamed(fields) => {
            expanded.extend(expand_unamed_field(_crate, fields));
        }
        syn::Fields::Unit => {}
    };

    expanded
}

fn expand_named_field(_crate: &syn::Ident, fields: &syn::FieldsNamed) -> TokenStream {
    let mut expanded = quote::quote! {};
    for (index, field) in fields.named.iter().enumerate() {
        let Some(name) = &field.ident else {
            continue;
        };

        let ty = &field.ty;
        let vis = find_attr(field, "get").map(|attr| get_vis(attr));
        expanded.extend(quote::quote! {
            #[cfg(target_family = "wasm")]
            #vis fn #name(&self) -> #ty {
                #_crate::types::structs::internal_struct_get::<#index, #ty>(self.placeholder)
            }

            #[cfg(not(target_family = "wasm"))]
            #vis fn #name(&self) -> &#ty {
                &self.#name
            }
        }); // private getter in default

        let name = &format_ident!("set_{}", name);
        if let Some(set) = find_attr(field, "set") {
            let vis = get_vis(set);
            expanded.extend(quote::quote! {
                #[cfg(target_family = "wasm")]
                #vis fn #name(&mut self, value: #ty) {
                    #_crate::types::structs::internal_struct_set::<#index, #ty>(self.placeholder, value);
                }

                #[cfg(not(target_family = "wasm"))]
                #vis fn #name(&mut self, value: #ty) {
                    self.#name = value;
                }
            });
        } // no set in default
    }

    expanded
}

fn expand_unamed_field(_crate: &syn::Ident, fields: &syn::FieldsUnnamed) -> TokenStream {
    let mut expanded = quote::quote! {};
    for (index, field) in fields.unnamed.iter().enumerate() {
        let ty = &field.ty;
        let name = &format_ident!("get_{}", index);
        let vis = find_attr(field, "get").map(|attr| get_vis(attr));
        expanded.extend(quote::quote! {
            #vis fn #name(&self) -> #ty {
                #_crate::types::structs::internal_struct_get::<#index, #ty>(self.0)
            }
        }); // private getter in default

        let name = &format_ident!("set_{}", index);
        if let Some(set) = find_attr(field, "set") {
            let vis = get_vis(set);
            expanded.extend(quote::quote! {
                #vis fn #name(&mut self, value: #ty) {
                    #_crate::types::structs::internal_struct_set::<#index, #ty>(self.0, value);
                }
            });
        } // no set in default
    }

    expanded
}

fn remove_attr(fields: &mut syn::Fields, name: &str) {
    match fields {
        syn::Fields::Named(fields) => {
            fields.named.iter_mut().for_each(|field| {
                field.attrs = field
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| !attr.path().is_ident(name))
                    .collect();
            });
        }
        syn::Fields::Unnamed(fields) => {
            fields.unnamed.iter_mut().for_each(|field| {
                field.attrs = field
                    .attrs
                    .clone()
                    .into_iter()
                    .filter(|attr| !attr.path().is_ident(name))
                    .collect();
            });
        }
        syn::Fields::Unit => {}
    }
}

fn find_attr<'a>(field: &'a syn::Field, name: &str) -> Option<&'a syn::Attribute> {
    field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(name) && matches!(attr.style, syn::AttrStyle::Outer))
}

fn get_vis(attr: &syn::Attribute) -> Option<TokenStream> {
    if let syn::Meta::List(vis) = &attr.meta {
        Some(vis.tokens.clone())
    } else {
        None
    }
}
