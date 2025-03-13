use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, FieldsNamed, Fields};
use proc_macro2::Span;
use syn::Ident;

/// Marks a struct as a contract event for Neo N3
///
/// Events are emitted during contract execution and recorded on the Neo N3 blockchain.
/// They allow external observers to track contract activities and provide notifications 
/// for off-chain systems.
///
/// In Neo N3, events must be properly emitted using the Runtime::notify method
/// with a ByteString for the event name and an Array<Any> for parameters.
/// This implementation follows the official Neo N3 pattern.
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    
    // Extract struct details
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    let vis = &input.vis;
    
    // Extract field information
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    let mut indexed_fields = Vec::new();
    
    if let Fields::Named(FieldsNamed { named, .. }) = &input.fields {
        for field in named {
            let field_name = field.ident.as_ref().unwrap();
            field_names.push(field_name);
            field_types.push(&field.ty);
            
            // Check if this field is indexed
            let is_indexed = field.attrs.iter().any(|attr| attr.path.is_ident("index"));
            indexed_fields.push(is_indexed);
        }
    }
    
    // Generate parameter processing code
    let param_processing = field_names.iter().map(|field_name| {
        let field_name_str = field_name.to_string();
        quote! {
            // Handle Option types specially
            if let Some(inner_val) = #field_name.as_ref() {
                event_data.push(::neo_contract::prelude::Any::from(inner_val.clone()));
            } else if type_name::<#field_name>().contains("Option<") {
                // It's an Option type that's None
                event_data.push(::neo_contract::prelude::Any::new());
            } else {
                // Regular value
                event_data.push(::neo_contract::prelude::Any::from(#field_name.clone()));
            }
        }
    });
    
    // Generate the augmented struct
    let output = quote! {
        #[derive(::neo_contract::serialize::NeoSerialize, ::neo_contract::serialize::NeoDeserialize, Clone)]
        #vis struct #struct_name {
            #input.fields
        }
        
        impl #struct_name {
            /// Static method to emit the #struct_name event in Neo N3 format
            pub fn emit(#(#field_names: #field_types),*) {
                use std::any::type_name;
                
                // Create event name as ByteString (required for Neo N3)
                let event_name = ::neo_contract::prelude::ByteString::from(#struct_name_str);
                
                // Create Array to hold event parameters (required for Neo N3)
                let mut event_data = ::neo_contract::prelude::Array::<::neo_contract::prelude::Any>::new();
                
                // Add parameters with proper Neo N3 format
                #(#param_processing)*
                
                // Emit the event using Runtime::notify (required for Neo N3)
                ::neo_contract::prelude::Runtime::notify(&event_name, &event_data);
            }
        }
        
        // Register this event in the manifest
        #[cfg(feature = "manifest-generation")]
        impl ::neo_contract::manifest::ManifestEvent for #struct_name {
            fn register_event() {
                let event_name = stringify!(#struct_name);
                ::neo_contract::manifest::register_event(
                    event_name,
                    &[#(
                        (stringify!(#field_names), #indexed_fields)
                    ),*]
                );
            }
        }
    };
    
    output.into()
}

/// Marks an event field as indexed for Neo N3 event filtering
///
/// In Neo N3, indexed fields enable efficient event filtering when querying the blockchain.
/// This is important for dApps that need to monitor specific events.
/// The Neo N3 RPC servers can filter events based on these indexed fields.
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // This attribute doesn't modify the item, it just marks it for the event macro
    item
}

/// Helper function to convert CamelCase to snake_case
fn to_snake_case(camel_case: &str) -> String {
    let mut snake_case = String::new();
    for (i, c) in camel_case.char_indices() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
    }
    snake_case
}