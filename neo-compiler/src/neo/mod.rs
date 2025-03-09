//! Neo VM Types and Opcodes
//!
//! This module provides types and constants related to the Neo VM.
//! 
//! This is a compatibility layer that re-exports from the unified neo.rs file.
//! New code should directly use types from the root neo module.

// Re-export OpCode from the opcodes module
pub mod opcodes;
pub use self::opcodes::OpCode;