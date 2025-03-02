// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

mod contract;
mod structs;

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
