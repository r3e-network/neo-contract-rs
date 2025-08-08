//! Neo Contract Framework Test Suite
//! 
//! Comprehensive tests for the Neo N3 Rust smart contract framework
//! with Solana-style syntax support.

#[cfg(test)]
mod types_test;

#[cfg(test)]
mod native_contracts_test;

#[cfg(test)]
mod nep_standards_test;

#[cfg(test)]
mod storage_runtime_test;

#[cfg(test)]
mod end_to_end_test;

#[cfg(test)]
mod test_utils;

/// Re-export test utilities for use in other test modules
#[cfg(test)]
pub use test_utils::test_utils::*;