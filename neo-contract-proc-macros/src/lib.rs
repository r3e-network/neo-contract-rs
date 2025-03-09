extern crate proc_macro;

mod contract;
mod contract_permission;
mod contract_trust;
mod manifest_extra;
mod security;
mod static_field;
mod structure;
mod supported_standards;

use proc_macro::TokenStream;

/// Marks a module as a smart contract.
///
/// This is the main entry point for defining a Neo smart contract in Rust.
/// It processes the module to:
///
/// - Extract method definitions
/// - Process storage struct
/// - Register events
/// - Generate manifest information
/// - Set up entry points for calling the contract
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// mod example {
///     use neo_contract::prelude::*;
///     
///     #[storage]
///     struct ExampleContract {
///         counter: Item<u32>,
///     }
///     
///     impl ExampleContract {
///         #[constructor]
///         fn new() -> Self {
///             Self {
///                 counter: Item::new(0),
///             }
///         }
///         
///         #[method]
///         fn increment(&mut self) {
///             let counter = self.counter.get();
///             self.counter.set(counter + 1);
///         }
///         
///         #[safe]
///         fn get_counter(&self) -> u32 {
///             *self.counter.get()
///         }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn contract(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract::contract(attr, item)
}

/// Marks a function as a method of the smart contract.
///
/// Methods marked with this attribute will be exposed in the contract's
/// manifest and can be called from external contracts or applications.
/// These methods can modify contract state.
///
/// # Example
///
/// ```rust
/// #[method]
/// fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
///     // Implementation that modifies state
/// }
/// ```
#[proc_macro_attribute]
pub fn method(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::method(attr, item)
}

/// Marks a function as a safe method of the smart contract.
///
/// Safe methods do not modify contract state and are marked as "safe" in the
/// contract manifest. They can be called without requiring a full transaction.
///
/// This attribute automatically implies the method is exposed in the contract's
/// manifest, similar to #[method], but with safe execution guarantees.
///
/// # Example
///
/// ```rust
/// #[safe]
/// fn balance_of(&self, account: Address) -> u64 {
///     self.balances.get(&account).unwrap_or_default()
/// }
/// ```
#[proc_macro_attribute]
pub fn safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    security::safe::safe(attr, item)
}

/// Marks a function as a constructor for the smart contract.
///
/// Constructors are called during contract deployment and initialize the
/// contract's storage state. Only one constructor can be defined per contract.
///
/// # Example
///
/// ```rust
/// #[constructor]
/// fn new(owner: Address, name: String) -> Self {
///     Self {
///         owner: Item::new(owner),
///         name: Item::new(name),
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn constructor(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::constructor(attr, item)
}

/// Marks a struct as the contract's storage definition.
///
/// The storage struct defines all persistent state for the contract.
/// Only one storage struct can be defined per contract.
///
/// # Example
///
/// ```rust
/// #[storage]
/// struct TokenContract {
///     owner: Item<Address>,
///     total_supply: Item<u64>,
///     balances: Map<Address, u64>,
/// }
/// ```
#[proc_macro_attribute]
pub fn storage(attr: TokenStream, item: TokenStream) -> TokenStream {
    structure::storage(attr, item)
}

/// Defines a smart contract event.
///
/// Events are recorded on the blockchain and can be subscribed to by external services.
///
/// # Example
///
/// ```rust
/// #[event]
/// struct Transfer {
///     #[index]
///     from: Option<Address>,
///     #[index]
///     to: Option<Address>,
///     amount: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    events::event(attr, item)
}

/// Marks a field in an event struct as indexed.
///
/// Indexed fields can be used to filter event queries.
///
/// # Example
///
/// ```rust
/// #[event]
/// struct Transfer {
///     #[index]
///     from: Address,
///     #[index]
///     to: Address,
///     amount: u64,
/// }
/// ```
#[proc_macro_attribute]
pub fn index(attr: TokenStream, item: TokenStream) -> TokenStream {
    events::index(attr, item)
}

/// Marks a contract method as non-reentrant.
///
/// Non-reentrant methods cannot be called again while they are still running,
/// which prevents reentrancy attacks.
///
/// # Example
///
/// ```rust
/// #[method]
/// #[no_reentrant]
/// fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
///     // Implementation that's protected from reentrancy
/// }
/// ```
#[proc_macro_attribute]
pub fn no_reentrant(attr: TokenStream, item: TokenStream) -> TokenStream {
    security::no_reentrant::no_reentrant(attr, item)
}

/// Adds extra information to the contract manifest.
///
/// This allows defining additional metadata for the contract.
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[manifest_extra(
///     author = "Neo Project",
///     email = "contact@neo.org",
///     description = "An example NEP-17 token",
///     version = "1.0.0"
/// )]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn manifest_extra(attr: TokenStream, item: TokenStream) -> TokenStream {
    manifest_extra::manifest_extra(attr, item)
}

/// Specifies the NEP standards supported by the contract.
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[supported_standards("NEP-17")]
/// mod token {
///     // NEP-17 token implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn supported_standards(attr: TokenStream, item: TokenStream) -> TokenStream {
    supported_standards::supported_standards(attr, item)
}

/// Defines contract permissions.
///
/// This specifies which contracts and methods the contract is allowed to call.
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[contract_permission(
///     contract = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
///     methods = ["transfer", "balanceOf"]
/// )]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_permission::contract_permission(attr, item)
}

/// Defines contract trust relationships.
///
/// This specifies which contracts are trusted by this contract.
///
/// # Example
///
/// ```rust
/// #[neo_contract::contract]
/// #[contract_trust("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
/// mod token {
///     // Contract implementation
/// }
/// ```
#[proc_macro_attribute]
pub fn contract_trust(attr: TokenStream, item: TokenStream) -> TokenStream {
    contract_trust::contract_trust(attr, item)
}

// Static field macros
/// Defines a fixed byte array constant.
#[proc_macro_attribute]
pub fn byte_array_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::byte_array_fixed::byte_array_fixed(attr, item)
}

/// Defines a byte array constant.
#[proc_macro_attribute]
pub fn byte_array(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::byte_array::byte_array(attr, item)
}

/// Defines a fixed contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::contract_hash_fixed::contract_hash_fixed(attr, item)
}

/// Defines a contract hash constant.
#[proc_macro_attribute]
pub fn contract_hash(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::contract_hash::contract_hash(attr, item)
}

/// Defines a fixed Hash160 constant.
#[proc_macro_attribute]
pub fn hash160_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::hash160_fixed::hash160_fixed(attr, item)
}

/// Defines a Hash160 constant.
#[proc_macro_attribute]
pub fn hash160(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::hash160::hash160(attr, item)
}

/// Defines a fixed integer constant.
#[proc_macro_attribute]
pub fn integer_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::integer_fixed::integer_fixed(attr, item)
}

/// Defines an integer constant.
#[proc_macro_attribute]
pub fn integer(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::integer::integer(attr, item)
}

/// Defines a fixed public key constant.
#[proc_macro_attribute]
pub fn public_key_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::public_key_fixed::public_key_fixed(attr, item)
}

/// Defines a public key constant.
#[proc_macro_attribute]
pub fn public_key(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::public_key::public_key(attr, item)
}

/// Defines a fixed string constant.
#[proc_macro_attribute]
pub fn string_fixed(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::string_fixed::string_fixed(attr, item)
}

/// Defines a string constant.
#[proc_macro_attribute]
pub fn string(attr: TokenStream, item: TokenStream) -> TokenStream {
    static_field::string::string(attr, item)
}

// Internal modules
mod events;
