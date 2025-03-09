//! Call Flags for Neo Contract RS
//!
//! This module defines the flags for contract calls in Neo.

use core::fmt;
use alloc::vec::Vec;
use alloc::string::String;

/// Call flags for contract calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallFlags(pub u8);

impl CallFlags {
    /// None - no special flags
    pub const NONE: CallFlags = CallFlags(0);
    
    /// Read only - doesn't allow state changes
    pub const READ_ONLY: CallFlags = CallFlags(0b00000001);
    
    /// Allow notify - allows sending notifications
    pub const ALLOW_NOTIFY: CallFlags = CallFlags(0b00000010);
    
    /// Allow states - allows state changes
    pub const ALLOW_STATES: CallFlags = CallFlags(0b00000100);
    
    /// Allow write - allows contract call to write to storage
    pub const ALLOW_WRITE: CallFlags = CallFlags(0b00001000);
    
    /// Allow call - allows contract call to call another contract
    pub const ALLOW_CALL: CallFlags = CallFlags(0b00010000);
    
    /// Allow all - allows all operations
    pub const ALLOW_ALL: CallFlags = CallFlags(0b00011110);
    
    /// States and notify - combination of ALLOW_STATES and ALLOW_NOTIFY
    pub const STATES_AND_NOTIFY: CallFlags = CallFlags(0b00000110);
    
    /// States and read only - combination of ALLOW_STATES and READ_ONLY
    pub const STATES_AND_READ_ONLY: CallFlags = CallFlags(0b00000101);
    
    /// Creates a new CallFlags from a raw value
    pub fn new(value: u8) -> Self {
        CallFlags(value)
    }
    
    /// Gets the raw value of the flags
    pub fn bits(&self) -> u8 {
        self.0
    }
    
    /// Checks if the flags contain the given flag
    pub fn contains(&self, flag: CallFlags) -> bool {
        (self.0 & flag.0) == flag.0
    }
    
    /// Adds a flag
    pub fn add(&mut self, flag: CallFlags) {
        self.0 |= flag.0;
    }
    
    /// Removes a flag
    pub fn remove(&mut self, flag: CallFlags) {
        self.0 &= !flag.0;
    }
    
    /// Creates a new CallFlags with the given flag added
    pub fn with(self, flag: CallFlags) -> Self {
        CallFlags(self.0 | flag.0)
    }
    
    /// Creates a new CallFlags with the given flag removed
    pub fn without(self, flag: CallFlags) -> Self {
        CallFlags(self.0 & !flag.0)
    }
    
    /// Checks if the call is read-only
    pub fn is_read_only(&self) -> bool {
        self.contains(Self::READ_ONLY)
    }
    
    /// Checks if the call allows notifications
    pub fn allows_notify(&self) -> bool {
        self.contains(Self::ALLOW_NOTIFY)
    }
    
    /// Checks if the call allows state changes
    pub fn allows_states(&self) -> bool {
        self.contains(Self::ALLOW_STATES)
    }
    
    /// Checks if the call allows writing to storage
    pub fn allows_write(&self) -> bool {
        self.contains(Self::ALLOW_WRITE)
    }
    
    /// Checks if the call allows calling other contracts
    pub fn allows_call(&self) -> bool {
        self.contains(Self::ALLOW_CALL)
    }
}

impl fmt::Display for CallFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags = Vec::new();
        
        if self.contains(Self::READ_ONLY) {
            flags.push("READ_ONLY");
        }
        if self.contains(Self::ALLOW_NOTIFY) {
            flags.push("ALLOW_NOTIFY");
        }
        if self.contains(Self::ALLOW_STATES) {
            flags.push("ALLOW_STATES");
        }
        if self.contains(Self::ALLOW_WRITE) {
            flags.push("ALLOW_WRITE");
        }
        if self.contains(Self::ALLOW_CALL) {
            flags.push("ALLOW_CALL");
        }
        
        if flags.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "{}", flags.join(" | "))
        }
    }
}

impl Default for CallFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<u8> for CallFlags {
    fn from(value: u8) -> Self {
        CallFlags(value)
    }
}

impl From<CallFlags> for u8 {
    fn from(flags: CallFlags) -> Self {
        flags.0
    }
}

impl core::ops::BitOr for CallFlags {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        CallFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for CallFlags {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        CallFlags(self.0 & rhs.0)
    }
}

impl core::ops::BitXor for CallFlags {
    type Output = Self;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        CallFlags(self.0 ^ rhs.0)
    }
}

impl core::ops::BitOrAssign for CallFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAndAssign for CallFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl core::ops::BitXorAssign for CallFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
