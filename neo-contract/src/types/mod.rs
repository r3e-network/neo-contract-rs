//! Types module for Neo Contract RS
//!
//! This module defines the common types used throughout the framework.

// Re-export core types
pub use self::builtin::h160::H160;
pub use self::builtin::h256::H256;
pub use self::builtin::string::ByteString;
pub use self::builtin::int256::Int256;
pub use self::builtin::array::Array;
pub use self::builtin::map::Map;
pub use self::builtin::any::Any;

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
