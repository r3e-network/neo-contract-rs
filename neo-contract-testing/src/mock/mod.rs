//! Mock implementations of Neo blockchain components
//!
//! This module provides mock implementations of various Neo
//! blockchain components for testing.

use alloc::vec::Vec;
use alloc::string::String;

// Mock implementations
pub mod storage;
pub mod runtime;
pub mod events;
pub mod transaction;

// Re-exports for convenience
pub use storage::MockStorage;
pub use runtime::MockRuntime;
pub use events::MockEvents;
pub use transaction::MockTransaction;

/// Global test state management
pub struct TestState;

impl TestState {
    /// Resets all mock components
    pub fn reset_all() {
        MockStorage::reset();
        MockRuntime::reset();
        MockEvents::reset();
        MockTransaction::reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reset_all() {
        // Set up some state
        MockStorage::put(b"test_key", b"test_value");
        MockRuntime::set_block_height(1000);
        MockRuntime::notify("TestEvent", vec![vec![1, 2, 3]]);
        
        // Reset all
        TestState::reset_all();
        
        // Check that state was reset
        assert_eq!(MockStorage::get(b"test_key"), None);
        assert_eq!(MockRuntime::get_block_height(), 0);
        assert_eq!(MockEvents::get_all().len(), 0);
    }
}