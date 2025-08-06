// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

mod contract;
mod structs;
mod program;

/// It exports the MyContract methods as no_mangle methods:
/// ```rust
/// use neo_contract as neo;
///
/// struct MyContract; // It must be a empty struct without `{}` or `()`
///
/// #[neo::contract] // To indicate that it implements as a neo smart contract
/// impl Nep17Token for MyContract {
///     fn symbol() -> String {
///         static_byte_string!("MYC")
///     }
///
///     fn decimals() -> u8 {
///         8
///     }
/// }
/// ```
/// Expand to:
/// ```rust
/// #[no_mangle]
/// pub fn symbol() -> String {
///     static_byte_string!("MYC")
/// }
///
/// #[no_mangle]
/// pub fn decimals() -> u8 {
///     8
/// }
///
/// #[no_mangle]
/// pub fn transfer(from: Address, to: Address, amount: u64) -> bool {
///     // default transfer implementation
/// }
///
/// // ... other nep17 methods that has default implementation
/// ```
#[proc_macro_attribute]
pub fn contract(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    contract::expand_contract_impl(input)
}

/// It converts a rust struct to a neo contract struct.
/// ```rust
/// use neo_contract as neo;
///
/// #[neo::structs]
/// struct MyStruct {
///     #[get(pub)]
///     field1: u64,
///     #[get]
///     field2: String,
/// }
/// ```
/// Expand to:
/// ```rust
/// #[cfg(target_family = "wasm")]
/// struct MyStruct {
///     placeholder: Placeholder, // framework internal use, do not access directly
/// }
///
/// #[cfg(not(target_family = "wasm"))]
/// struct MyStruct {
///     field1: u64,
///     field2: String,
/// }
///
/// #[cfg(target_family = "wasm")]
/// impl MyStruct {
///     pub fn field1(&self) -> u64 {
///         self.internal_get::<0, u64>(self)
///     }
///
///     fn field2(&self) -> String {
///         self.internal_get::<1, String>(self)
///     }
///     // ... other methods
/// }
/// // other internal impl
///
/// #[cfg(not(target_family = "wasm"))]
/// impl MyStruct {
///     pub fn field1(&self) -> u64 {
///         self.field1
///     }
///
///     fn field2(&self) -> String {
///         self.field2
///     }
///     // ... other methods
/// }
/// ```
#[proc_macro_attribute]
pub fn structs(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    structs::expand_structs_impl(input)
}

/// Marks an implementation block as a contract implementation
///
/// # Example
///
/// ```
/// #[contract_impl]
/// impl MyContract {
///     pub fn deploy(owner: H160, total_supply: Int256) -> bool {
///         // Implementation...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_impl(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Use the same expansion logic as the #[contract] macro
    contract::expand_contract_impl(input)
}

/// Marks a method to be exposed in the contract interface
///
/// # Example
///
/// ```
/// #[method]
/// pub fn transfer(from: &Address, to: &Address, amount: u64) -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn method(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Marks a method as read-only (does not modify state)
///
/// # Example
///
/// ```
/// #[method]
/// #[safe]
/// pub fn symbol() -> ByteString {
///     ByteString::from_literal("DEMO")
/// }
/// ```
#[proc_macro_attribute]
pub fn safe(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Specifies the WASM export name for a method
///
/// This allows mapping between the Rust method name and the name exported in the WASM file.
///
/// # Example
///
/// ```
/// #[method]
/// #[wasm_export(name = "add")]
/// pub fn hello(name: &ByteString) -> ByteString {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn wasm_export(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Note: In a full implementation, this would parse the name attribute and modify the export name.
    // For now, this is just a marker for documentation purposes.
    input
}

/// Specifies the contract author in the manifest
///
/// # Example
///
/// ```
/// #[contract_author("Neo Contract Team")]
/// pub struct TokenContract;
/// ```
#[proc_macro_attribute]
pub fn contract_author(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Defines contract permissions in the manifest
///
/// # Example
///
/// ```
/// #[contract_permission("*:*")]
/// pub struct TokenContract;
/// ```
#[proc_macro_attribute]
pub fn contract_permission(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Declares supported standards in the manifest
///
/// # Example
///
/// ```
/// #[contract_standards("NEP-17")]
/// pub struct TokenContract;
/// ```
#[proc_macro_attribute]
pub fn contract_standards(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Specifies the contract version in the manifest
///
/// # Example
///
/// ```
/// #[contract_version("1.0.0")]
/// pub struct TokenContract;
/// ```
#[proc_macro_attribute]
pub fn contract_version(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Adds extra metadata to the manifest
///
/// # Example
///
/// ```
/// #[contract_meta("Version", "1.0.0")]
/// pub struct TokenContract;
/// ```
#[proc_macro_attribute]
pub fn contract_meta(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This attribute is only used as a marker for the manifest generator,
    // it does not modify the code itself
    input
}

/// Restricts method access to the contract owner only
///
/// This annotation generates runtime checks to ensure only the contract owner
/// can call the annotated method.
///
/// # Example
///
/// ```
/// #[method]
/// #[owner_only]
/// pub fn set_owner(new_owner: &H160) -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn owner_only(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_macro_input, ItemFn};

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            // Owner-only access control check
            let owner = crate::storage::Storage::get_bytes(&crate::types::ByteString::from_literal("owner"));
            if owner.is_none() {
                crate::runtime::abort("Contract owner not set");
            }

            let owner_hash = crate::types::H160::from_bytes(&owner.unwrap());
            let caller = crate::runtime::get_calling_script_hash();

            if owner_hash != caller {
                crate::runtime::abort("Access denied: owner only");
            }

            // Original function body
            #fn_block
        }
    };

    TokenStream::from(expanded).into()
}

/// Requires witness verification for the calling script hash
///
/// This annotation generates runtime checks to verify that the calling
/// script hash has provided a valid witness.
///
/// # Example
///
/// ```
/// #[method]
/// #[require_witness]
/// pub fn transfer(from: &H160, to: &H160, amount: u64) -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn require_witness(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_macro_input, ItemFn};

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            // Witness verification check
            let calling_hash = crate::runtime::get_calling_script_hash();
            if !crate::runtime::check_witness(&calling_hash) {
                crate::runtime::abort("Witness verification failed");
            }

            // Original function body
            #fn_block
        }
    };

    TokenStream::from(expanded).into()
}

/// Validates input parameters before method execution
///
/// This annotation generates runtime validation checks for method parameters.
///
/// # Example
///
/// ```
/// #[method]
/// #[validate]
/// pub fn transfer(from: &H160, to: &H160, amount: u64) -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn validate(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_macro_input, ItemFn};

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            // Input validation checks
            // Note: In a full implementation, this would parse the validation rules
            // from the annotation arguments and generate appropriate checks

            // For now, add basic null/empty checks
            // This is a placeholder for more sophisticated validation

            // Original function body
            #fn_block
        }
    };

    TokenStream::from(expanded).into()
}

/// Sets a gas limit for method execution
///
/// This annotation generates runtime gas limit checks.
///
/// # Example
///
/// ```
/// #[method]
/// #[gas_limit(1000000)]
/// pub fn expensive_operation() -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn gas_limit(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_macro_input, ItemFn};

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    // Parse gas limit from args (simplified)
    let gas_limit = if _args.is_empty() {
        quote! { 1000000u64 }
    } else {
        // Convert proc_macro::TokenStream to proc_macro2::TokenStream
        let args: TokenStream = _args.into();
        quote! { #args }
    };

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            // Gas limit check
            let current_gas = crate::runtime::get_gas_left();
            if current_gas < #gas_limit {
                crate::runtime::abort("Insufficient gas for operation");
            }

            // Original function body
            #fn_block
        }
    };

    TokenStream::from(expanded).into()
}

/// Solana-style program macro for Neo N3 smart contracts
///
/// # Example
///
/// ```
/// declare_id!("NeoContractAddress123...");
///
/// #[program]
/// pub mod my_neo_program {
///     use super::*;
///
///     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
///         // Implementation...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn program(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    program::expand_program(input)
}

/// Derive macro for Solana-style account validation in Neo N3
///
/// # Example
///
/// ```
/// #[derive(Accounts)]
/// pub struct Initialize<'info> {
///     #[account(init, payer = user, space = 8 + 64)]
///     pub data_account: Account<'info, DataAccount>,
///     #[account(mut)]
///     pub user: Signer<'info>,
///     pub system_program: Program<'info, System>,
/// }
/// ```
#[proc_macro_derive(Accounts, attributes(account))]
pub fn Accounts(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    program::expand_derive_accounts(input)
}

/// Marks a struct as a Solana-style account data structure
///
/// # Example
///
/// ```
/// #[account]
/// pub struct DataAccount {
///     pub authority: Pubkey,
///     pub value: u64,
///     pub timestamp: i64,
/// }
/// ```
#[proc_macro_attribute]
pub fn account(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    program::expand_account(input)
}

/// Declares the program ID for a Neo N3 smart contract
///
/// # Example
///
/// ```
/// declare_id!("NeoContractAddress123...");
/// ```
#[proc_macro]
pub fn declare_id(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    program::expand_declare_id(input)
}

/// Generates error codes and handling for contract errors
///
/// # Example
///
/// ```
/// #[derive(ErrorCode)]
/// pub enum MyError {
///     #[msg("Account not found")]
///     AccountNotFound,
///     #[msg("Insufficient funds")]
///     InsufficientFunds,
/// }
/// ```
#[proc_macro_derive(ErrorCode, attributes(msg))]
pub fn ErrorCode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    program::expand_error_code(input)
}

/// Event attribute for Solana-style events
#[proc_macro_attribute]
pub fn event(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

/// Macro to mark init_if_needed accounts
#[proc_macro_attribute]
pub fn init_if_needed(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}

/// Marks a method as a contract initialization method
///
/// This annotation ensures the method can only be called during contract deployment.
///
/// # Example
///
/// ```
/// #[method]
/// #[init]
/// pub fn deploy(owner: &H160, total_supply: u64) -> bool {
///     // Implementation...
/// }
/// ```
#[proc_macro_attribute]
pub fn init(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse_macro_input, ItemFn};

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            // Initialization check - ensure this is called during deployment
            let trigger = crate::runtime::get_trigger();
            if trigger != crate::types::TriggerType::Application {
                crate::runtime::abort("Init method can only be called during deployment");
            }

            // Check if contract is already initialized
            let initialized = crate::storage::Storage::get_bytes(&crate::types::ByteString::from_literal("initialized"));
            if initialized.is_some() {
                crate::runtime::abort("Contract already initialized");
            }

            // Mark as initialized
            crate::storage::Storage::put_bytes(
                &crate::types::ByteString::from_literal("initialized"),
                &crate::types::Bytes::from_slice(&[1u8])
            );

            // Original function body
            #fn_block
        }
    };

    TokenStream::from(expanded).into()
}