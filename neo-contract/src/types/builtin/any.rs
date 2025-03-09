//! Any type for Neo Contract RS
//!
//! This module defines the Any type, which is used to represent values of any type in Neo.

use core::fmt;
use alloc::vec::Vec;
use alloc::string::String;
use super::string::ByteString;
use super::h160::H160;
use super::h256::H256;
use super::int256::Int256;

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
#[derive(Clone)]
pub enum Any {
    Integer(Int256),
    Boolean(bool),
    ByteString(ByteString),
    Array(Vec<Any>),
    Map(Vec<(Any, Any)>),
    Null,
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
    pub fn integer<T: Into<Int256>>(val: T) -> Self {
        Any::Integer(val.into())
    }
    
    /// Creates a new boolean Any value
    pub fn boolean(val: bool) -> Self {
        Any::Boolean(val)
    }
    
    /// Creates a new ByteString Any value
    pub fn byte_string<T: Into<ByteString>>(val: T) -> Self {
        Any::ByteString(val.into())
    }
    
    /// Creates a new array Any value
    pub fn array(values: Vec<Any>) -> Self {
        Any::Array(values)
    }
    
    /// Creates a new map Any value
    pub fn map(entries: Vec<(Any, Any)>) -> Self {
        Any::Map(entries)
    }
    
    /// Creates a null Any value
    pub fn null() -> Self {
        Any::Null
    }
    
    /// Checks if the Any value is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, Any::Integer(_))
    }
    
    /// Checks if the Any value is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, Any::Boolean(_))
    }
    
    /// Checks if the Any value is a ByteString
    pub fn is_byte_string(&self) -> bool {
        matches!(self, Any::ByteString(_))
    }
    
    /// Checks if the Any value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Any::Array(_))
    }
    
    /// Checks if the Any value is a map
    pub fn is_map(&self) -> bool {
        matches!(self, Any::Map(_))
    }
    
    /// Checks if the Any value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Any::Null)
    }
    
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
        // In a real implementation, this would deserialize the bytes
        // according to the Neo VM serialization format.
        // For simplicity, we'll just return a ByteString wrapping the bytes.
        Any::ByteString(ByteString::from(bytes))
    }
    
    /// Serializes the Any value to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // In a real implementation, this would serialize the value
        // according to the Neo VM serialization format.
        // For simplicity, we'll just return the raw bytes for ByteString
        // and empty bytes for other types.
        match self {
            Any::ByteString(val) => val.as_bytes().to_vec(),
            _ => Vec::new(),
        }
    }
}

impl From<Int256> for Any {
    fn from(val: Int256) -> Self {
        Any::Integer(val)
    }
}

impl From<bool> for Any {
    fn from(val: bool) -> Self {
        Any::Boolean(val)
    }
}

impl From<ByteString> for Any {
    fn from(val: ByteString) -> Self {
        Any::ByteString(val)
    }
}

impl From<&[u8]> for Any {
    fn from(val: &[u8]) -> Self {
        Any::ByteString(ByteString::from(val))
    }
}

impl From<Vec<u8>> for Any {
    fn from(val: Vec<u8>) -> Self {
        Any::ByteString(ByteString::from(val))
    }
}

impl From<&str> for Any {
    fn from(val: &str) -> Self {
        Any::ByteString(ByteString::from(val))
    }
}

impl From<String> for Any {
    fn from(val: String) -> Self {
        Any::ByteString(ByteString::from(val))
    }
}

impl From<H160> for Any {
    fn from(val: H160) -> Self {
        Any::ByteString(ByteString::from(val.as_bytes()))
    }
}

impl From<H256> for Any {
    fn from(val: H256) -> Self {
        Any::ByteString(ByteString::from(val.as_bytes()))
    }
}

impl From<Vec<Any>> for Any {
    fn from(val: Vec<Any>) -> Self {
        Any::Array(val)
    }
}

impl From<Vec<(Any, Any)>> for Any {
    fn from(val: Vec<(Any, Any)>) -> Self {
        Any::Map(val)
    }
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
