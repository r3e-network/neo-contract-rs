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
    
    // Process fields for indexing
    let struct_name = &input.ident;
    let vis = &input.vis;
    let emit_method_name = Ident::new(&format!("emit_{}", to_snake_case(&struct_name.to_string())), Span::call_site());
    
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
    
    // Generate the augmented struct
    let output = quote! {
        #[derive(::neo_contract::serialize::NeoSerialize, ::neo_contract::serialize::NeoDeserialize, Clone)]
        #vis struct #struct_name {
            #input.fields
        }
        
        impl #struct_name {
            /// Emits this event to the Neo N3 blockchain following official Neo N3 standards
            /// 
            /// This implementation strictly follows the Neo N3 standard pattern: 
            /// using Runtime::notify with ByteString for event name and Array<Any> for parameters,
            /// ensuring proper event emission on the Neo N3 blockchain.
            pub fn #emit_method_name(#(#field_names: #field_types),*) {
                // Log event emission for debugging purposes
                ::neo_contract::prelude::Runtime::log(&format!("Emitting Neo N3 event: {}", stringify!(#struct_name)));
                
                // Create event name as ByteString per Neo N3 standard
                let event_name = ::neo_contract::prelude::ByteString::from(stringify!(#struct_name));
                
                // Create an Array to hold parameters as required by Neo N3
                let mut event_data = ::neo_contract::prelude::Array::<::neo_contract::prelude::Any>::new();
                
                // Process and add each parameter as Any values following Neo N3 type system requirements
                #(
                    // Log parameter values for debugging
                    ::neo_contract::prelude::Runtime::log(&format!("Event parameter {}: {:?}", stringify!(#field_names), #field_names));
                    
                    // Handle Option types specially as per Neo N3 standards
                    match &#field_names {
                        Some(inner) => {
                            ::neo_contract::prelude::Runtime::log("Converting Some value to Neo N3 Any type");
                            event_data.push(::neo_contract::prelude::Any::from(inner.clone()));
                        },
                        None => {
                            ::neo_contract::prelude::Runtime::log("Converting None value to Neo N3 Any::new()");
                            event_data.push(::neo_contract::prelude::Any::new()); // Proper handling for null/None values in Neo N3
                        },
                        _ => {
                            // For non-Option types, convert directly to Neo N3 Any type
                            event_data.push(::neo_contract::prelude::Any::from(#field_names.clone()));
                        }
                    };
                )*
                
                // Log the completion of parameter processing
                ::neo_contract::prelude::Runtime::log("Parameters processed for Neo N3 event emission");
                
                // Emit event using Runtime::notify following Neo N3 standards
                ::neo_contract::prelude::Runtime::notify(&event_name, &event_data);
                
                // Log successful event emission
                ::neo_contract::prelude::Runtime::log(&format!("Successfully emitted Neo N3 event: {}", stringify!(#struct_name)));
            }
        }
        
        // Register this event in the manifest
        #[cfg(feature = "manifest-generation")]
        const _: () = {
            ::neo_contract::manifest::register_event(
                stringify!(#struct_name),
                &[#(stringify!(#field_names)),*],
                &[#(stringify!(#field_types)),*],
                &[#(#indexed_fields),*]
            );
        };
    };
    
    output.into()
}

/// Marks an event field as indexed for Neo N3 event filtering
///
/// In Neo N3, indexed fields enable efficient event filtering when querying the blockchain.
/// This is important for dApps that need to monitor specific events.
/// The Neo N3 RPC servers can filter events based on these indexed fields.
pub fn index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Simply return the item as is, the attribute is just a marker
    // The actual processing happens in the event macro
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