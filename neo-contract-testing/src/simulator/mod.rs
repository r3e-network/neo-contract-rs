//! Contract simulator for Neo contracts
//!
//! This module provides a simulator for running Neo contracts
//! in a controlled environment.

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use alloc::boxed::Box;

use crate::mock::{MockStorage, MockRuntime, MockEvents, MockTransaction, TestState};
use crate::debug::{DebugCapture, TracingMode, DebugEventType};
use crate::fixture::TestContext;
use crate::{TestResult, TestError};

/// Result of a contract invocation
#[derive(Debug, Clone)]
pub struct InvocationResult {
    /// Whether the invocation was successful
    pub success: bool,
    /// Return value of the invocation, if any
    pub result: Option<Vec<u8>>,
    /// Events emitted during the invocation
    pub events: Vec<crate::mock::events::Event>,
    /// Gas consumed during the invocation
    pub gas_consumed: i64,
    /// Storage changes made during the invocation
    pub storage_changes: Vec<StorageChange>,
    /// Error message, if the invocation failed
    pub error: Option<String>,
    /// Debug events captured during the invocation
    pub debug_events: Vec<crate::debug::DebugEvent>,
}

/// Storage change made during a contract invocation
#[derive(Debug, Clone)]
pub enum StorageChange {
    /// A value was added or updated
    Put {
        /// The key
        key: Vec<u8>,
        /// The new value
        value: Vec<u8>,
        /// The previous value, if any
        old_value: Option<Vec<u8>>,
    },
    /// A value was deleted
    Delete {
        /// The key
        key: Vec<u8>,
        /// The previous value, if any
        old_value: Option<Vec<u8>>,
    },
}

/// Contract simulator for Neo contracts
///
/// This structure provides a simulator for running Neo contracts
/// in a controlled environment.
pub struct ContractSimulator {
    /// The test context
    pub context: TestContext,
    /// Whether to capture storage changes
    pub capture_storage_changes: bool,
    /// Whether to capture debug events
    pub capture_debug: bool,
    /// Tracing mode for debug capture
    pub tracing_mode: TracingMode,
}

impl ContractSimulator {
    /// Creates a new contract simulator with the default context
    pub fn new() -> Self {
        // Reset all mock components
        TestState::reset_all();
        DebugCapture::reset();
        
        let context = TestContext::default();
        context.apply();
        
        Self {
            context,
            capture_storage_changes: true,
            capture_debug: false,
            tracing_mode: TracingMode::None,
        }
    }
    
    /// Creates a new contract simulator with the specified context
    pub fn with_context(context: TestContext) -> Self {
        // Reset all mock components
        TestState::reset_all();
        DebugCapture::reset();
        
        // Apply context
        context.apply();
        
        Self {
            context,
            capture_storage_changes: true,
            capture_debug: false,
            tracing_mode: TracingMode::None,
        }
    }
    
    /// Sets whether to capture storage changes
    pub fn with_capture_storage_changes(mut self, capture: bool) -> Self {
        self.capture_storage_changes = capture;
        self
    }
    
    /// Sets whether to capture debug events
    pub fn with_capture_debug(mut self, capture: bool, mode: TracingMode) -> Self {
        self.capture_debug = capture;
        self.tracing_mode = mode;
        self
    }
    
    /// Set up initial storage state
    pub fn with_storage(self, pairs: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        MockStorage::setup(pairs);
        self
    }
    
    /// Set up initial block height
    pub fn with_block_height(self, height: u32) -> Self {
        MockRuntime::set_block_height(height);
        self
    }
    
    /// Set up initial timestamp
    pub fn with_timestamp(self, timestamp: u64) -> Self {
        MockRuntime::set_time(timestamp);
        self
    }
    
    /// Set up a witness
    pub fn with_witness(self, witness: Vec<u8>) -> Self {
        MockRuntime::add_witness(&witness);
        self
    }
    
    /// Invokes a contract with the given parameters
    ///
    /// # Arguments
    /// * `contract_function` - The contract function to invoke
    /// * `args` - The arguments to pass to the function
    ///
    /// # Returns
    /// * `InvocationResult` - The result of the invocation
    pub fn invoke<F, R, E>(&self, contract_function: F, args: Vec<Vec<u8>>) -> InvocationResult
    where
        F: FnOnce(Vec<Vec<u8>>) -> Result<R, E>,
        R: Into<Vec<u8>>,
        E: Into<String>,
    {
        // Apply context
        self.context.apply();
        
        // Enable debug capture if needed
        if self.capture_debug {
            DebugCapture::enable(self.tracing_mode);
        } else {
            DebugCapture::disable();
        }
        
        // Take snapshot of initial state
        let initial_gas = MockRuntime::get_gas_left();
        let initial_storage = if self.capture_storage_changes {
            MockStorage::get_all()
        } else {
            Vec::new()
        };
        
        // Clear notifications
        MockRuntime::clear_notifications();
        
        // Capture start event
        if self.capture_debug {
            DebugCapture::capture(
                DebugEventType::Other,
                "ContractSimulator:InvokeStart",
                Vec::new(),
                "Contract invocation started",
            );
        }
        
        // Invoke contract function
        let result = match contract_function(args) {
            Ok(value) => {
                // Success
                let bytes = value.into();
                if self.capture_debug {
                    DebugCapture::capture(
                        DebugEventType::Other,
                        "ContractSimulator:InvokeSuccess",
                        bytes.clone(),
                        "Contract invocation succeeded",
                    );
                }
                (true, Some(bytes), None)
            }
            Err(err) => {
                // Failure
                let error_msg = err.into();
                if self.capture_debug {
                    DebugCapture::capture(
                        DebugEventType::Error,
                        "ContractSimulator:InvokeFailure",
                        error_msg.clone().into_bytes(),
                        "Contract invocation failed",
                    );
                }
                (false, None, Some(error_msg))
            }
        };
        
        // Take snapshot of final state
        let final_gas = MockRuntime::get_gas_left();
        let gas_consumed = initial_gas - final_gas;
        
        // Collect events
        let events = MockEvents::get_all();
        
        // Capture end event
        if self.capture_debug {
            DebugCapture::capture(
                DebugEventType::Other,
                "ContractSimulator:InvokeEnd",
                Vec::new(),
                "Contract invocation ended",
            );
        }
        
        // Calculate storage changes
        let storage_changes = if self.capture_storage_changes {
            let final_storage = MockStorage::get_all();
            self.calculate_storage_changes(&initial_storage, &final_storage)
        } else {
            Vec::new()
        };
        
        // Get debug events
        let debug_events = if self.capture_debug {
            DebugCapture::events()
        } else {
            Vec::new()
        };
        
        // Create invocation result
        InvocationResult {
            success: result.0,
            result: result.1,
            events,
            gas_consumed,
            storage_changes,
            error: result.2,
            debug_events,
        }
    }
    
    /// Calculates storage changes between two storage snapshots
    fn calculate_storage_changes(
        &self,
        initial: &[(Vec<u8>, Vec<u8>)],
        final_state: &[(Vec<u8>, Vec<u8>)]
    ) -> Vec<StorageChange> {
        let mut changes = Vec::new();
        
        // Convert initial to map for easier lookup
        let mut initial_map = std::collections::HashMap::new();
        for (key, value) in initial {
            initial_map.insert(key, value);
        }
        
        // Find additions and modifications
        for (key, value) in final_state {
            if let Some(old_value) = initial_map.get(key) {
                // Key exists in both - check if value changed
                if *old_value != value {
                    changes.push(StorageChange::Put {
                        key: key.clone(),
                        value: value.clone(),
                        old_value: Some((*old_value).clone()),
                    });
                }
            } else {
                // Key only exists in final - addition
                changes.push(StorageChange::Put {
                    key: key.clone(),
                    value: value.clone(),
                    old_value: None,
                });
            }
            
            // Remove from map to track what's left
            initial_map.remove(key);
        }
        
        // Remaining keys in initial_map were deleted
        for (key, value) in initial_map {
            changes.push(StorageChange::Delete {
                key: (*key).clone(),
                old_value: Some((*value).clone()),
            });
        }
        
        changes
    }
    
    /// Gets debug report
    pub fn debug_report(&self) -> String {
        DebugCapture::generate_report()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Mock contract function for testing
    fn mock_contract_success(args: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        // Store each argument
        for (i, arg) in args.iter().enumerate() {
            let key = format!("arg:{}", i).into_bytes();
            MockStorage::put(&key, arg);
        }
        
        // Emit an event
        MockRuntime::notify("MockFunction", vec![vec![1, 2, 3]]);
        
        // Return success
        Ok(vec![42])
    }
    
    fn mock_contract_failure(_args: Vec<Vec<u8>>) -> Result<Vec<u8>, String> {
        // Emit an event before failing
        MockRuntime::notify("FailureEvent", vec![vec![9, 9, 9]]);
        
        // Return error
        Err("Mock failure".into())
    }
    
    #[test]
    fn test_simulator_basic() {
        // Create a simulator
        let simulator = ContractSimulator::new();
        
        // Invoke contract function
        let args = vec![
            b"arg1".to_vec(),
            b"arg2".to_vec(),
        ];
        
        let result = simulator.invoke(mock_contract_success, args);
        
        // Check result
        assert!(result.success);
        assert_eq!(result.result, Some(vec![42]));
        assert_eq!(result.events.len(), 1);
        assert!(result.gas_consumed > 0);
        
        // Check storage changes
        assert_eq!(result.storage_changes.len(), 2);
        
        // Check for stored arguments
        assert_eq!(MockStorage::get(b"arg:0"), Some(b"arg1".to_vec()));
        assert_eq!(MockStorage::get(b"arg:1"), Some(b"arg2".to_vec()));
    }
    
    #[test]
    fn test_simulator_failure() {
        // Create a simulator
        let simulator = ContractSimulator::new();
        
        // Invoke contract function that fails
        let result = simulator.invoke(mock_contract_failure, Vec::new());
        
        // Check result
        assert!(!result.success);
        assert_eq!(result.result, None);
        assert_eq!(result.error, Some("Mock failure".into()));
        
        // Check events (should still be captured even though the function failed)
        assert_eq!(result.events.len(), 1);
        assert_eq!(result.events[0].name, "FailureEvent");
    }
    
    #[test]
    fn test_simulator_with_debug() {
        // Create a simulator with debug enabled
        let simulator = ContractSimulator::new()
            .with_capture_debug(true, TracingMode::Full);
        
        // Invoke contract function
        let args = vec![b"debug_test".to_vec()];
        let result = simulator.invoke(mock_contract_success, args);
        
        // Check debug events were captured
        assert!(!result.debug_events.is_empty());
        
        // Generate debug report
        let report = simulator.debug_report();
        assert!(report.contains("InvokeStart"));
        assert!(report.contains("InvokeSuccess"));
        assert!(report.contains("InvokeEnd"));
    }
}