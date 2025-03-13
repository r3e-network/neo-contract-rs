//! Neo Contract Testing Framework
//!
//! This crate provides utilities for testing Neo smart contracts
//! without a real blockchain environment.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::boxed::Box;
use alloc::string::String;

/// Mock implementations of Neo blockchain components
pub mod mock;

/// Contract simulator for controlled testing
pub mod simulator;

/// Test fixtures for setup and teardown
pub mod fixture;

/// Debugging utilities
pub mod debug;

/// Assertion utilities for testing
pub mod assertions;

/// Common imports for testing
pub mod prelude {
    pub use crate::assertions::*;
    pub use crate::debug::{DebugCapture, DebugEvent, DebugEventType, TracingMode};
    pub use crate::fixture::{TestContext, TestFixture};
    pub use crate::simulator::{ContractSimulator, InvocationResult, StorageChange};
    pub use crate::{TestError, TestResult};

    // Re-export mock modules for easy access
    pub use crate::mock;

    #[cfg(test)]
    pub use crate::testing;
}

/// Result type for test operations
pub type TestResult<T, E = TestError> = Result<T, E>;

/// Error type for test operations
#[derive(Debug)]
pub struct TestError {
    /// Error message
    pub message: String,
    /// Optional error cause
    pub cause: Option<Box<dyn core::fmt::Debug>>,
}

impl TestError {
    /// Creates a new test error with the given message
    pub fn new(message: impl Into<String>) -> Self { Self { message: message.into(), cause: None } }

    /// Creates a new test error with the given message and cause
    pub fn with_cause<C: core::fmt::Debug + 'static>(message: impl Into<String>, cause: C) -> Self {
        Self { message: message.into(), cause: Some(Box::new(cause)) }
    }
}

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref cause) = self.cause {
            write!(f, ": {:?}", cause)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TestError {}

/// Testing utilities for internal use
#[cfg(test)]
pub(crate) mod testing {
    /// Mock implementations for testing the testing framework itself
    pub mod mock_runtime {
        use crate::mock::MockRuntime;

        /// Adds a witness for testing
        pub fn add_witness(hash: &[u8]) { MockRuntime::add_witness(hash); }

        /// Sets the block height for testing
        pub fn set_block_height(height: u32) { MockRuntime::set_block_height(height); }

        /// Sets the timestamp for testing
        pub fn set_time(timestamp: u64) { MockRuntime::set_time(timestamp); }
    }

    /// Mock storage for testing
    pub mod mock_storage {
        use crate::mock::MockStorage;

        /// Sets up storage for testing
        pub fn setup(pairs: Vec<(Vec<u8>, Vec<u8>)>) { MockStorage::setup(pairs); }

        /// Gets a value from storage
        pub fn get(key: &[u8]) -> Option<Vec<u8>> { MockStorage::get(key) }

        /// Puts a value into storage
        pub fn put(key: &[u8], value: &[u8]) { MockStorage::put(key, value); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    // Test function to check the simulator works
    fn test_fn(args: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        if args.is_empty() {
            return Err("No arguments provided".into());
        }

        // Store the first argument in storage
        mock::MockStorage::put(b"test_key", &args[0]);

        // Emit an event
        mock::MockRuntime::notify("TestEvent", vec![args[0].clone()]);

        // Return success
        Ok(vec![42])
    }

    #[test]
    fn test_simulator_basics() {
        // Create a simulator
        let simulator = ContractSimulator::new();

        // Invoke a test function
        let args = vec![vec![1, 2, 3]];
        let result = simulator.invoke(test_fn, args);

        // Check the result
        assert!(result.success);
        assert_eq!(result.result, Some(vec![42]));
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].name, "TestEvent");

        // Check storage was updated
        assert_eq!(mock::MockStorage::get(b"test_key"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_fixture() {
        // Create a test fixture
        let fixture = TestFixture::new().with_block_height(1000).with_timestamp(1620000000000);

        // Run a test function
        let result = fixture.run(|| {
            assert_eq!(mock::MockRuntime::get_block_height(), 1000);
            assert_eq!(mock::MockRuntime::get_time(), 1620000000000);
            Ok::<_, String>("success")
        });

        // Check the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}
