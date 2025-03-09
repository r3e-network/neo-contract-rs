use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct};

/// Marks a struct as a contract event
///
/// Events are emitted during contract execution and recorded on the blockchain.
/// They allow external observers to track contract activities.
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    
    // Process fields for indexing
    let struct_name = &input.ident;
    let vis = &input.vis;
    
    // Generate the augmented struct
    let output = quote! {
        #[derive(::neo_contract::serialize::NeoSerialize, ::neo_contract::serialize::NeoDeserialize, Clone)]
        #vis struct #struct_name {
            #input.fields
        }
        
        impl ::neo_contract::events::NeoEvent for #struct_name {}
    };
    
    output.into()
}

/// Marks an event field as indexed
///
/// Indexed fields can be used for filtering events when querying the blockchain.
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Simply return the item as is, the attribute is just a marker
    // The actual processing happens in the event macro
    item
}