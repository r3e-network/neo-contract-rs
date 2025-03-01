// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

/// CallFlags represents the flags for a contract call
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallFlags {
    /// None
    None = 0,
    /// ReadOnly
    ReadOnly = 1,
    /// ReadStates
    ReadStates = 2,
    /// WriteStates
    WriteStates = 4,
    /// AllowCall
    AllowCall = 8,
    /// AllowNotify
    AllowNotify = 16,
    /// States
    States = 6,
    /// ReadOnlyStates
    ReadOnlyStates = 3,
    /// All
    All = 31,
}

impl CallFlags {
    /// Create a new CallFlags
    pub fn new(value: u8) -> Self {
        match value {
            0 => CallFlags::None,
            1 => CallFlags::ReadOnly,
            2 => CallFlags::ReadStates,
            4 => CallFlags::WriteStates,
            8 => CallFlags::AllowCall,
            16 => CallFlags::AllowNotify,
            6 => CallFlags::States,
            3 => CallFlags::ReadOnlyStates,
            31 => CallFlags::All,
            _ => CallFlags::None,
        }
    }

    /// Get the value
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl From<u8> for CallFlags {
    fn from(value: u8) -> Self {
        CallFlags::new(value)
    }
}

impl From<CallFlags> for u8 {
    fn from(value: CallFlags) -> Self {
        value.value()
    }
}
