// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Codec implementations for various types

// Make the module public so its contents can be exported
pub mod impl_builtin;

// Re-export the Codec trait implementation
pub use crate::storage::item::Codec;