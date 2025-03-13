// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// Macro to define a contract method that can be called from other contracts
#[macro_export]
macro_rules! contract_method {
    ($(#[$attr:meta])* $vis:vis fn $name:ident($($arg:ident : $arg_ty:ty),*) -> $ret:ty $body:block) => {
        $(#[$attr])*
        #[no_mangle]
        $vis extern "C" fn $name($($arg : $arg_ty),*) -> $ret $body
    };
}

/// Macro to define a contract event
#[macro_export]
macro_rules! contract_event {
    ($(#[$attr:meta])* $vis:vis fn $name:ident($($arg:ident : $arg_ty:ty),*)) => {
        $(#[$attr])*
        $vis fn $name($($arg : $arg_ty),*) {
            let args = neo_contract::types::builtin::Array::new();
            $(
                args.push($arg.into());
            )*

            unsafe {
                neo_contract::env::syscall::system_runtime_notify(
                    neo_contract::types::builtin::ByteString::new(stringify!($name)),
                    args
                );
            }
        }
    };
}

/// Macro to define a contract storage map
#[macro_export]
macro_rules! storage_map {
    // Single type version (for simple key types like H160)
    ($key_ty:ty) => {
        neo_contract::storage::StorageMap<$key_ty, neo_contract::builtin::Int256>
    };

    // Two type version (for key and value types)
    ($key_ty:ty, $value_ty:ty) => {
        neo_contract::storage::StorageMap<$key_ty, $value_ty>
    };

    // Original version with struct definition
    ($(#[$attr:meta])* $vis:vis $name:ident : $key_ty:ty => $value_ty:ty) => {
        $(#[$attr])*
        $vis struct $name;

        impl $name {
            pub fn new() -> Self {
                Self
            }

            pub fn get(&self, key: &$key_ty) -> Option<$value_ty> {
                let storage = neo_contract::storage::StorageMap::new();
                storage.get(key)
            }

            pub fn put(&self, key: &$key_ty, value: &$value_ty) {
                let storage = neo_contract::storage::StorageMap::new();
                storage.put(key, value);
            }

            pub fn delete(&self, key: &$key_ty) {
                let storage = neo_contract::storage::StorageMap::new();
                storage.delete(key);
            }
        }
    };
}

/// Macro to define a static value with a literal
#[macro_export]
macro_rules! static_value {
    ($(#[$attr:meta])* $vis:vis static $name:ident : $ty:ty = $literal:expr;) => {
        $(#[$attr])*
        $vis static $name: $ty = {
            let literal = $literal;
            match <$ty as neo_contract::static_values::StaticValue>::from_literal(literal) {
                Some(value) => value,
                None => panic!("Invalid literal for static value: {}", literal),
            }
        };
    };
}
