// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// Options for storage find operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindOptions {
    /// Remove the prefix from the keys in the iterator
    RemovePrefix = 0,
    /// Keep the prefix in the keys in the iterator
    KeepPrefix = 1,
    /// Keys only
    KeysOnly = 2,
    /// Values only
    ValuesOnly = 3,
    /// Remove prefix and return keys only
    RemovePrefixKeysOnly = 4,
    /// Remove prefix and return values only
    RemovePrefixValuesOnly = 5,
    /// Keep prefix and return keys only
    KeepPrefixKeysOnly = 6,
    /// Keep prefix and return values only
    KeepPrefixValuesOnly = 7,
    /// Deserialize values
    DeserializeValues = 8,
    /// Pick fields
    PickFields = 9,
}
