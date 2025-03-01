// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

pub mod any;
pub mod array;
pub mod h160;
pub mod h256;
pub mod int256;
pub mod map;
pub mod string;

// Re-exports
pub use any::Any;
pub use array::Array;
pub use h160::H160;
pub use h256::H256;
pub use int256::Int256;
pub use map::Map;
pub use string::ByteString;

/// Primitive trait for types that can be used as keys in maps
pub trait Primitive: Clone {}

// Implement Primitive for basic types
impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for bool {}
impl Primitive for char {}
impl Primitive for H160 {}
impl Primitive for H256 {}
impl Primitive for Int256 {}
impl Primitive for ByteString {}

// Implement Primitive for Array
impl<T: Primitive> Primitive for Array<T> {}

// Implement Primitive for Map
impl<K: Primitive + core::hash::Hash + Ord, V: Clone> Primitive for Map<K, V> {}
