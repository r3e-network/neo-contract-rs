// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Security module for Neo smart contracts
//! This module provides security-related utilities like reentrancy protection

// use crate::storage::{StorageMap, Storable, StorageKey};
// use crate::builtin::ByteString;
use crate::prelude::{StorageMap, ByteString};
use crate::policy::voting::Storable;
// use crate::error::{Error, ErrorCode, Result};
use core::marker::PhantomData;

pub mod reentrancy;

/// Export the reentrancy guard
pub use reentrancy::ReentrancyGuard;
