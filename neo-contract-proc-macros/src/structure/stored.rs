// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemStruct,
};

/// Process the stored attribute macro
pub(crate) fn generate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the struct
    let item_struct = parse_macro_input!(item as ItemStruct);
    
    // Extract struct information
    let struct_name = &item_struct.ident;
    let struct_vis = &item_struct.vis;
    let struct_attrs = &item_struct.attrs;
    let _struct_fields = &item_struct.fields;
    
    // Generate the output with storage functionality
    let output = quote! {
        #[allow(non_snake_case)]
        #(#struct_attrs)*
        #struct_vis struct #struct_name {
            #item_struct
        }
        
        impl #struct_name {
            pub fn load(key: &str) -> Option<Self> {
                let storage_context = neo::runtime::get_storage_context();
                let data = storage_context.get(key)?;
                Some(neo::serialize::from_bytes(&data))
            }
            
            pub fn save(&self, key: &str) {
                let storage_context = neo::runtime::get_storage_context();
                let data = neo::serialize::to_bytes(self);
                storage_context.put(key, &data);
            }
            
            pub fn delete(key: &str) {
                let storage_context = neo::runtime::get_storage_context();
                storage_context.delete(key);
            }
        }
    };
    
    output.into()
}
