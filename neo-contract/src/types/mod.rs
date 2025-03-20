//! Types module for Neo Contract RS
//!
//! This module defines the common types used throughout the framework.

// Import Vec from alloc
use alloc::vec::Vec;

// Re-export core types
pub use crate::types::builtin::any::Any;
pub use crate::types::builtin::array::Array;
pub use crate::types::builtin::h160::H160;
pub use crate::types::builtin::h256::H256;
pub use crate::types::builtin::int256::Int256;
pub use crate::types::builtin::map::Map;
pub use crate::types::builtin::string::ByteString;

pub mod block;
pub mod bytes;
pub mod consts;
pub mod context;
pub mod contract;
pub mod key;
pub mod notification;
pub mod placeholder;
pub mod signer;
pub mod storage;
pub mod tx;

// Builtin types for Neo N3
pub mod builtin {
    pub mod any;
    pub mod array;
    pub mod h160;
    pub mod h256;
    pub mod int256;
    pub mod map;
    pub mod string;
}

/// Trait for converting Rust types to Neo N3 parameter types
/// 
/// This trait is used to convert Rust types to Neo N3 parameter types
/// for the contract manifest generation.
pub trait ToNeoParameter {
    /// Converts the type to a Neo N3 parameter type string
    fn to_neo_parameter_type() -> &'static str;
}

// Implement ToNeoParameter for common types
impl ToNeoParameter for bool {
    fn to_neo_parameter_type() -> &'static str { "Boolean" }
}

impl ToNeoParameter for i8 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for i16 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for i32 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for i64 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for u8 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for u16 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for u32 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for u64 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for Int256 {
    fn to_neo_parameter_type() -> &'static str { "Integer" }
}

impl ToNeoParameter for ByteString {
    fn to_neo_parameter_type() -> &'static str { "String" }
}

impl ToNeoParameter for Vec<u8> {
    fn to_neo_parameter_type() -> &'static str { "ByteArray" }
}

impl ToNeoParameter for H160 {
    fn to_neo_parameter_type() -> &'static str { "Hash160" }
}

impl ToNeoParameter for H256 {
    fn to_neo_parameter_type() -> &'static str { "Hash256" }
}

impl<T> ToNeoParameter for Array<T> {
    fn to_neo_parameter_type() -> &'static str { "Array" }
}

impl<K, V> ToNeoParameter for Map<K, V> {
    fn to_neo_parameter_type() -> &'static str { "Map" }
}

impl<T> ToNeoParameter for Option<T> where T: ToNeoParameter {
    fn to_neo_parameter_type() -> &'static str { 
        // In Neo N3, Option<T> is represented as the same type as T
        // with null values allowed
        T::to_neo_parameter_type() 
    }
}

/// Helper function to get Neo N3 type string from a Rust type string
/// 
/// This is used in the manifest generation process to convert Rust type
/// descriptions to Neo N3 type descriptions.
pub fn rust_type_to_neo_type(rust_type: &str) -> &'static str {
    match rust_type.trim() {
        "bool" => "Boolean",
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => "Integer",
        "Int256" => "Integer",
        "String" | "str" | "&str" | "ByteString" => "String",
        "Vec<u8>" | "&[u8]" => "ByteArray",
        "H160" => "Hash160",
        "H256" => "Hash256",
        t if t.starts_with("Vec<") || t.starts_with("Array<") => "Array",
        t if t.starts_with("Map<") || t.starts_with("HashMap<") => "Map",
        "Any" => "Any",
        "()" | "Void" => "Void",
        _ => "Any",
    }
}
