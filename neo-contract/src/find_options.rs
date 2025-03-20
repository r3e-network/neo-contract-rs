//! Find Options for Neo Contract RS
//!
//! This module defines the options used when finding items in storage.

use alloc::vec::Vec;
use core::fmt;

/// Find options for storage operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindOptions(pub u8);

/// Constants for the FindOptions flags
pub mod flags {
    /// No special options
    pub const NONE: u8 = 0;

    /// Remove the prefix from the keys in the results
    pub const REMOVE_PREFIX: u8 = 1;

    /// Only return keys, not values
    pub const KEYS_ONLY: u8 = 2;

    /// Return values as key-value pairs (default)
    pub const VALUES: u8 = 4;

    /// Deserialize keys after retrieval
    pub const DESERIALIZE_VALUES: u8 = 8;

    /// Pick the first element without creating an iterator
    pub const PICK_FIELD_0: u8 = 16;
}

impl FindOptions {
    /// None - no special options
    pub const NONE: FindOptions = FindOptions(0);

    /// Key only - returns only the keys, not the values
    pub const KEYS_ONLY: FindOptions = FindOptions(0b00000001);

    /// Remove prefix - removes the prefix from the returned keys
    pub const REMOVE_PREFIX: FindOptions = FindOptions(0b00000010);

    /// Values only - returns only the values, not the keys
    pub const VALUES_ONLY: FindOptions = FindOptions(0b00000100);

    /// Deserialize values - deserialize the values into the specified type
    pub const DESERIALIZE_VALUES: FindOptions = FindOptions(0b00001000);

    /// Backward - search in backwards order
    pub const BACKWARD: FindOptions = FindOptions(0b00010000);

    /// Case insensitive - case insensitive search (for string keys)
    pub const CASE_INSENSITIVE: FindOptions = FindOptions(0b00100000);

    /// Creates a new FindOptions from a raw value
    pub fn new(value: u8) -> Self { FindOptions(value) }

    /// Gets the raw value of the options
    pub fn bits(&self) -> u8 { self.0 }

    /// Checks if the options contain the given option
    pub fn contains(&self, option: FindOptions) -> bool { (self.0 & option.0) == option.0 }

    /// Adds an option
    pub fn add(&mut self, option: FindOptions) { self.0 |= option.0; }

    /// Removes an option
    pub fn remove(&mut self, option: FindOptions) { self.0 &= !option.0; }

    /// Creates a new FindOptions with the given option added
    pub fn with(self, option: FindOptions) -> Self { FindOptions(self.0 | option.0) }

    /// Creates a new FindOptions with the given option removed
    pub fn without(self, option: FindOptions) -> Self { FindOptions(self.0 & !option.0) }

    /// Create a new FindOptions with default settings
    pub fn default_options() -> Self { Self(flags::NONE) }

    /// Set the option to remove prefix from keys in the results
    pub fn set_remove_prefix(mut self, value: bool) -> Self {
        if value {
            self.0 |= flags::REMOVE_PREFIX;
        } else {
            self.0 &= !flags::REMOVE_PREFIX;
        }
        self
    }

    /// Set the option to only return keys, not values
    pub fn set_keys_only(mut self, value: bool) -> Self {
        if value {
            self.0 |= flags::KEYS_ONLY;
        } else {
            self.0 &= !flags::KEYS_ONLY;
        }
        self
    }

    /// Set the option to return values as key-value pairs
    pub fn set_values(mut self, value: bool) -> Self {
        if value {
            self.0 |= flags::VALUES;
        } else {
            self.0 &= !flags::VALUES;
        }
        self
    }

    /// Check if the remove prefix option is set
    pub fn has_remove_prefix(&self) -> bool { (self.0 & flags::REMOVE_PREFIX) != 0 }

    /// Check if the keys only option is set
    pub fn has_keys_only(&self) -> bool { (self.0 & flags::KEYS_ONLY) != 0 }

    /// Check if the values option is set
    pub fn has_values(&self) -> bool { (self.0 & flags::VALUES) != 0 }
}

impl fmt::Display for FindOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut options = Vec::new();

        if self.contains(Self::KEYS_ONLY) {
            options.push("KEYS_ONLY");
        }
        if self.contains(Self::REMOVE_PREFIX) {
            options.push("REMOVE_PREFIX");
        }
        if self.contains(Self::VALUES_ONLY) {
            options.push("VALUES_ONLY");
        }
        if self.contains(Self::DESERIALIZE_VALUES) {
            options.push("DESERIALIZE_VALUES");
        }
        if self.contains(Self::BACKWARD) {
            options.push("BACKWARD");
        }
        if self.contains(Self::CASE_INSENSITIVE) {
            options.push("CASE_INSENSITIVE");
        }

        if options.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "{}", options.join(" | "))
        }
    }
}

impl Default for FindOptions {
    fn default() -> Self { Self::NONE }
}

impl From<u8> for FindOptions {
    fn from(value: u8) -> Self { FindOptions(value) }
}

impl From<FindOptions> for u8 {
    fn from(options: FindOptions) -> Self { options.0 }
}

impl core::ops::BitOr for FindOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output { FindOptions(self.0 | rhs.0) }
}

impl core::ops::BitAnd for FindOptions {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output { FindOptions(self.0 & rhs.0) }
}

impl core::ops::BitXor for FindOptions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output { FindOptions(self.0 ^ rhs.0) }
}

impl core::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXorAssign for FindOptions {
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}
