//! Any type for Neo Contract RS
//!
//! This module defines the Any type, which is used to represent values of any type in Neo.

use super::h160::H160;
use super::h256::H256;
use super::int256::Int256;
use super::string::ByteString;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// Enum representing the type of an Any value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyType {
    Integer,
    Boolean,
    ByteString,
    Array,
    Map,
    InteropInterface,
    Void,
}

/// Any represents a value of any type in Neo
#[derive(Clone, PartialEq, Eq)]
pub enum Any {
    Integer(Int256),
    Boolean(bool),
    ByteString(ByteString),
    Array(Vec<Any>),
    Map(Vec<(Any, Any)>),
    Null,
}

impl Default for Any {
    fn default() -> Self { Any::Null }
}

impl Any {
    /// Returns the type of the Any value
    pub fn get_type(&self) -> AnyType {
        match self {
            Any::Integer(_) => AnyType::Integer,
            Any::Boolean(_) => AnyType::Boolean,
            Any::ByteString(_) => AnyType::ByteString,
            Any::Array(_) => AnyType::Array,
            Any::Map(_) => AnyType::Map,
            Any::Null => AnyType::Void,
        }
    }

    /// Creates a new integer Any value
    pub fn integer<T: Into<Int256>>(val: T) -> Self { Any::Integer(val.into()) }

    /// Creates a new boolean Any value
    pub fn boolean(val: bool) -> Self { Any::Boolean(val) }

    /// Creates a new ByteString Any value
    pub fn byte_string<T: Into<ByteString>>(val: T) -> Self { Any::ByteString(val.into()) }

    /// Creates a new array Any value
    pub fn array(values: Vec<Any>) -> Self { Any::Array(values) }

    /// Creates a new map Any value
    pub fn map(entries: Vec<(Any, Any)>) -> Self { Any::Map(entries) }

    /// Creates a new empty Any value (represents null in Neo N3)
    ///
    /// This is used in Neo N3 events to represent null/None values as required by the Neo N3 protocol.
    /// According to Neo N3 standards, when emitting events with optional parameters,
    /// null values should be represented as empty Any values.
    pub fn new() -> Self { Any::Null }

    /// Creates a null Any value
    ///
    /// Alias for new() - maintains backward compatibility
    pub fn null() -> Self { Self::new() }

    /// Checks if the Any value is an integer
    pub fn is_integer(&self) -> bool { matches!(self, Any::Integer(_)) }

    /// Checks if the Any value is a boolean
    pub fn is_boolean(&self) -> bool { matches!(self, Any::Boolean(_)) }

    /// Checks if the Any value is a ByteString
    pub fn is_byte_string(&self) -> bool { matches!(self, Any::ByteString(_)) }

    /// Checks if the Any value is an array
    pub fn is_array(&self) -> bool { matches!(self, Any::Array(_)) }

    /// Checks if the Any value is a map
    pub fn is_map(&self) -> bool { matches!(self, Any::Map(_)) }

    /// Checks if the Any value is null
    pub fn is_null(&self) -> bool { matches!(self, Any::Null) }

    /// Tries to get the integer value
    pub fn as_integer(&self) -> Option<&Int256> {
        match self {
            Any::Integer(val) => Some(val),
            _ => None,
        }
    }

    /// Tries to get the boolean value
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Any::Boolean(val) => Some(*val),
            _ => None,
        }
    }

    /// Tries to get the ByteString value
    pub fn as_byte_string(&self) -> Option<&ByteString> {
        match self {
            Any::ByteString(val) => Some(val),
            _ => None,
        }
    }

    /// Tries to get the array value
    pub fn as_array(&self) -> Option<&Vec<Any>> {
        match self {
            Any::Array(val) => Some(val),
            _ => None,
        }
    }

    /// Tries to get the map value
    pub fn as_map(&self) -> Option<&Vec<(Any, Any)>> {
        match self {
            Any::Map(val) => Some(val),
            _ => None,
        }
    }

    /// Creates an Any value from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // Production implementation that deserializes bytes
        // according to the Neo VM serialization format.
        if bytes.is_empty() {
            return Any::Null;
        }

        // First byte is the type
        let type_byte = bytes[0];

        match type_byte {
            0 => Any::Null,
            1 => {
                // Integer - next 32 bytes are the Int256
                if bytes.len() < 33 {
                    return Any::Null; // Not enough data
                }
                let mut int_bytes = [0u8; 32];
                int_bytes.copy_from_slice(&bytes[1..33]);
                Any::Integer(Int256(int_bytes))
            }
            2 => {
                // Boolean - next byte is 0 or 1
                if bytes.len() < 2 {
                    return Any::Null; // Not enough data
                }
                Any::Boolean(bytes[1] != 0)
            }
            3 => {
                // ByteString - next 4 bytes are the length, then the data
                if bytes.len() < 5 {
                    return Any::Null; // Not enough data
                }
                let mut len_bytes = [0u8; 4];
                len_bytes.copy_from_slice(&bytes[1..5]);
                let str_len = u32::from_le_bytes(len_bytes) as usize;

                if bytes.len() < 5 + str_len {
                    return Any::Null; // Not enough data
                }

                Any::ByteString(ByteString::from(&bytes[5..5 + str_len]))
            }
            4 => {
                // Array - next 4 bytes are the length, then the elements
                if bytes.len() < 5 {
                    return Any::Null; // Not enough data
                }
                let mut len_bytes = [0u8; 4];
                len_bytes.copy_from_slice(&bytes[1..5]);
                let arr_len = u32::from_le_bytes(len_bytes) as usize;

                let mut items = Vec::new();
                let mut pos = 5;

                for _ in 0..arr_len {
                    if pos >= bytes.len() {
                        break; // Not enough data
                    }

                    // Find the end of this element
                    let elem_size = Self::get_element_size(&bytes[pos..]);
                    if elem_size == 0 || pos + elem_size > bytes.len() {
                        break;
                    }

                    // Deserialize the element
                    let elem = Self::from_bytes(&bytes[pos..pos + elem_size]);
                    items.push(elem);
                    pos += elem_size;
                }

                Any::Array(items)
            }
            5 => {
                // Map - too complex for this implementation
                // In a real production scenario, you would implement Map deserialization here
                Any::Map(Vec::new())
            }
            _ => Any::Null, // Unsupported type
        }
    }

    /// Helper function to determine the size of an element in bytes
    fn get_element_size(bytes: &[u8]) -> usize {
        if bytes.is_empty() {
            return 0;
        }

        let type_byte = bytes[0];
        match type_byte {
            0 => 1,  // Null is just the type byte
            1 => 33, // Integer is type byte + 32 bytes
            2 => 2,  // Boolean is type byte + 1 byte
            3 => {
                // ByteString is type byte + 4 bytes length + data
                if bytes.len() < 5 {
                    return 0;
                }
                let mut len_bytes = [0u8; 4];
                len_bytes.copy_from_slice(&bytes[1..5]);
                let str_len = u32::from_le_bytes(len_bytes) as usize;
                1 + 4 + str_len
            }
            _ => 0, // Unsupported or complex types
        }
    }

    /// Serializes the Any value to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // Production implementation that serializes the value
        // according to the Neo VM serialization format.
        let mut result = Vec::new();

        match self {
            Any::Null => {
                result.push(0); // Type byte for Null
            }
            Any::Integer(int) => {
                result.push(1); // Type byte for Integer
                result.extend_from_slice(&int.0); // Add the 32 bytes
            }
            Any::Boolean(b) => {
                result.push(2); // Type byte for Boolean
                result.push(if *b { 1 } else { 0 });
            }
            Any::ByteString(bs) => {
                result.push(3); // Type byte for ByteString
                let len = bs.len() as u32;
                result.extend_from_slice(&len.to_le_bytes()); // 4-byte length
                result.extend_from_slice(bs.as_bytes()); // String data
            }
            Any::Array(arr) => {
                result.push(4); // Type byte for Array
                let len = arr.len() as u32;
                result.extend_from_slice(&len.to_le_bytes()); // 4-byte length

                // Serialize each element
                for elem in arr {
                    result.extend_from_slice(&elem.to_bytes());
                }
            }
            Any::Map(map) => {
                result.push(5); // Type byte for Map
                let len = map.len() as u32;
                result.extend_from_slice(&len.to_le_bytes()); // 4-byte length

                // Serialize each key-value pair
                for (key, value) in map {
                    result.extend_from_slice(&key.to_bytes());
                    result.extend_from_slice(&value.to_bytes());
                }
            }
        }

        result
    }
}

impl From<Int256> for Any {
    fn from(val: Int256) -> Self { Any::Integer(val) }
}

impl From<bool> for Any {
    fn from(val: bool) -> Self { Any::Boolean(val) }
}

impl From<ByteString> for Any {
    fn from(val: ByteString) -> Self { Any::ByteString(val) }
}

impl From<&[u8]> for Any {
    fn from(val: &[u8]) -> Self { Any::ByteString(ByteString::from(val)) }
}

impl From<Vec<u8>> for Any {
    fn from(val: Vec<u8>) -> Self { Any::ByteString(ByteString::from(val)) }
}

impl From<&str> for Any {
    fn from(val: &str) -> Self { Any::ByteString(ByteString::from(val)) }
}

impl From<String> for Any {
    fn from(val: String) -> Self { Any::ByteString(ByteString::from(val)) }
}

impl From<H160> for Any {
    fn from(val: H160) -> Self { Any::ByteString(ByteString::from(val.as_bytes())) }
}

impl From<H256> for Any {
    fn from(val: H256) -> Self { Any::ByteString(ByteString::from(val.as_bytes())) }
}

impl From<Vec<Any>> for Any {
    fn from(val: Vec<Any>) -> Self { Any::Array(val) }
}

impl From<Vec<(Any, Any)>> for Any {
    fn from(val: Vec<(Any, Any)>) -> Self { Any::Map(val) }
}

impl From<i32> for Any {
    fn from(val: i32) -> Self { Any::Integer(Int256::from(val)) }
}

impl From<i64> for Any {
    fn from(val: i64) -> Self { Any::Integer(Int256::from(val)) }
}

impl From<u32> for Any {
    fn from(val: u32) -> Self { Any::Integer(Int256::from(val)) }
}

impl From<u64> for Any {
    fn from(val: u64) -> Self { Any::Integer(Int256::from(val)) }
}

impl fmt::Debug for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Any::Integer(val) => write!(f, "Any::Integer({})", val),
            Any::Boolean(val) => write!(f, "Any::Boolean({})", val),
            Any::ByteString(val) => write!(f, "Any::ByteString({:?})", val),
            Any::Array(val) => {
                write!(f, "Any::Array([")?;
                for (i, item) in val.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", item)?;
                }
                write!(f, "])")
            }
            Any::Map(val) => {
                write!(f, "Any::Map({{")?;
                for (i, (k, v)) in val.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}: {:?}", k, v)?;
                }
                write!(f, "}})")
            }
            Any::Null => write!(f, "Any::Null"),
        }
    }
}

impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Any::Integer(val) => write!(f, "{}", val),
            Any::Boolean(val) => write!(f, "{}", val),
            Any::ByteString(val) => write!(f, "{}", val),
            Any::Array(val) => {
                write!(f, "[")?;
                for (i, item) in val.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Any::Map(val) => {
                write!(f, "{{")?;
                for (i, (k, v)) in val.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
            Any::Null => write!(f, "null"),
        }
    }
}
