//! Type conversion between WebAssembly and Neo VM.
//!
//! This module provides utilities for converting between WebAssembly types
//! and Neo VM types.

use crate::error::Error;
use wasmparser::ValType;

/// Neo VM type system representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeoVmType {
    /// Any type (dynamic typing)
    Any,
    /// Boolean type
    Boolean,
    /// Integer type
    Integer,
    /// Byte array type
    ByteArray,
    /// String type
    String,
    /// Array type
    Array,
    /// Map type
    Map,
    /// InteropInterface type
    InteropInterface,
    /// Void type (for function returns)
    Void,
}

impl NeoVmType {
    /// Returns the string representation of the Neo VM type.
    pub fn to_string(&self) -> &'static str {
        match self {
            NeoVmType::Any => "Any",
            NeoVmType::Boolean => "Boolean",
            NeoVmType::Integer => "Integer",
            NeoVmType::ByteArray => "ByteArray",
            NeoVmType::String => "String",
            NeoVmType::Array => "Array",
            NeoVmType::Map => "Map",
            NeoVmType::InteropInterface => "InteropInterface",
            NeoVmType::Void => "Void",
        }
    }

    /// Creates a Neo VM type from a string representation.
    pub fn from_string(s: &str) -> Result<Self, Error> {
        match s {
            "Any" => Ok(NeoVmType::Any),
            "Boolean" => Ok(NeoVmType::Boolean),
            "Integer" => Ok(NeoVmType::Integer),
            "ByteArray" => Ok(NeoVmType::ByteArray),
            "String" => Ok(NeoVmType::String),
            "Array" => Ok(NeoVmType::Array),
            "Map" => Ok(NeoVmType::Map),
            "InteropInterface" => Ok(NeoVmType::InteropInterface),
            "Void" => Ok(NeoVmType::Void),
            _ => Err(Error::conversion(format!("Unknown Neo VM type: {}", s))),
        }
    }
}

/// Converts a WebAssembly value type to a Neo VM type.
pub fn wasm_to_neo_type(ty: ValType) -> NeoVmType {
    match ty {
        ValType::I32 => NeoVmType::Integer,
        ValType::I64 => NeoVmType::Integer,
        ValType::F32 => NeoVmType::Integer,    // Map to integer, limited support
        ValType::F64 => NeoVmType::Integer,    // Map to integer, limited support
        ValType::V128 => NeoVmType::ByteArray, // Map vector to byte array
        ValType::Ref(_) => NeoVmType::Any,     // References map to Any
    }
}

/// Gets the Neo VM type for a WebAssembly function return type.
pub fn get_return_type(types: &[ValType]) -> NeoVmType {
    if types.is_empty() {
        return NeoVmType::Void;
    }

    if types.len() == 1 {
        return wasm_to_neo_type(types[0]);
    }

    // Multiple return values not directly supported in Neo VM
    // We'll return them as an array
    NeoVmType::Array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_to_neo_type_conversion() {
        assert_eq!(wasm_to_neo_type(ValType::I32), NeoVmType::Integer);
        assert_eq!(wasm_to_neo_type(ValType::I64), NeoVmType::Integer);
        assert_eq!(get_return_type(&[]), NeoVmType::Void);
        assert_eq!(get_return_type(&[ValType::I32]), NeoVmType::Integer);
        assert_eq!(get_return_type(&[ValType::I32, ValType::I64]), NeoVmType::Array);
    }

    #[test]
    fn test_neo_type_string_conversion() {
        assert_eq!(NeoVmType::Integer.to_string(), "Integer");
        assert_eq!(NeoVmType::ByteArray.to_string(), "ByteArray");

        assert_eq!(NeoVmType::from_string("Integer").unwrap(), NeoVmType::Integer);
        assert_eq!(NeoVmType::from_string("ByteArray").unwrap(), NeoVmType::ByteArray);
        assert!(NeoVmType::from_string("Unknown").is_err());
    }
}
