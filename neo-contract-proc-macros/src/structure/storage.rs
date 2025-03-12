use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemStruct, Fields};

/// Implementation of the `storage` attribute macro
///
/// This marks a struct as the contract's storage definition, which defines
/// all persistent state variables for the contract.
pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemStruct);
    
    // Get struct name and fields
    let struct_name = &input.ident;
    let struct_vis = &input.vis;
    
    // Only named fields are allowed in a storage struct
    let fields = match &input.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Storage struct must have named fields")
    };
    
    // Generate the augmented storage struct
    let output = quote! {
        #[neo_storage]
        #struct_vis struct #struct_name {
            #fields
        }
        
        impl ::neo_contract::storage::NeoStorage for #struct_name {}
    };
    
    output.into()
}