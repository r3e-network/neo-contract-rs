//! Test fixture module for Neo contracts
//!
//! This module provides utilities for setting up and tearing down
//! test environments for Neo contracts.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::mock::{MockEvents, MockRuntime, MockStorage, MockTransaction, TestState};
use crate::{TestError, TestResult};

/// Test context for contract invocations
#[derive(Debug, Clone)]
pub struct TestContext {
    /// Initial storage state
    pub initial_storage: Vec<(Vec<u8>, Vec<u8>)>,
    /// Block height
    pub block_height: u32,
    /// Timestamp
    pub timestamp: u64,
    /// Witnesses
    pub witnesses: Vec<Vec<u8>>,
    /// Gas limit
    pub gas_limit: i64,
    /// Calling script hash
    pub calling_script_hash: Option<Vec<u8>>,
    /// Transaction version
    pub tx_version: u8,
    /// Signers
    pub signers: Vec<Vec<u8>>,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            initial_storage: Vec::new(),
            block_height: 1,
            timestamp: 1_620_000_000_000, // May 3, 2021
            witnesses: Vec::new(),
            gas_limit: 10_000_000,
            calling_script_hash: None,
            tx_version: 0,
            signers: Vec::new(),
        }
    }
}

impl TestContext {
    /// Creates a new test context with default values
    pub fn new() -> Self { Self::default() }

    /// Sets initial storage state
    pub fn with_storage(mut self, pairs: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        self.initial_storage = pairs;
        self
    }

    /// Sets block height
    pub fn with_block_height(mut self, height: u32) -> Self {
        self.block_height = height;
        self
    }

    /// Sets timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Adds a witness
    pub fn with_witness(mut self, witness: Vec<u8>) -> Self {
        self.witnesses.push(witness);
        self
    }

    /// Sets gas limit
    pub fn with_gas_limit(mut self, gas_limit: i64) -> Self {
        self.gas_limit = gas_limit;
        self
    }

    /// Sets calling script hash
    pub fn with_calling_script_hash(mut self, script_hash: Vec<u8>) -> Self {
        self.calling_script_hash = Some(script_hash);
        self
    }

    /// Sets transaction version
    pub fn with_tx_version(mut self, version: u8) -> Self {
        self.tx_version = version;
        self
    }

    /// Adds a signer
    pub fn with_signer(mut self, signer: Vec<u8>) -> Self {
        self.signers.push(signer);
        self
    }

    /// Applies this context to the test environment
    pub fn apply(&self) {
        // Reset mock components
        TestState::reset_all();

        // Setup initial storage
        MockStorage::setup(self.initial_storage.clone());

        // Setup runtime state
        MockRuntime::set_block_height(self.block_height);
        MockRuntime::set_time(self.timestamp);
        MockRuntime::set_gas_left(self.gas_limit);

        // Setup witnesses
        for witness in &self.witnesses {
            MockRuntime::add_witness(witness);
        }

        // Setup calling script hash
        if let Some(ref script_hash) = self.calling_script_hash {
            MockRuntime::set_calling_script_hash(script_hash);
        }

        // Setup transaction state
        MockTransaction::set_version(self.tx_version);

        // Setup signers
        for signer in &self.signers {
            MockTransaction::add_signer(signer);
        }
    }
}

/// Test fixture for Neo contracts
pub struct TestFixture {
    /// The test context
    pub context: TestContext,
}

impl TestFixture {
    /// Creates a new test fixture with the default context
    pub fn new() -> Self {
        let context = TestContext::default();
        context.apply();

        Self { context }
    }

    /// Creates a new test fixture with the specified context
    pub fn with_context(context: TestContext) -> Self {
        context.apply();
        Self { context }
    }

    /// Sets initial storage state
    pub fn with_storage(self, pairs: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        let context = self.context.with_storage(pairs);
        context.apply();
        Self { context }
    }

    /// Sets block height
    pub fn with_block_height(self, height: u32) -> Self {
        let context = self.context.with_block_height(height);
        context.apply();
        Self { context }
    }

    /// Sets timestamp
    pub fn with_timestamp(self, timestamp: u64) -> Self {
        let context = self.context.with_timestamp(timestamp);
        context.apply();
        Self { context }
    }

    /// Adds a witness
    pub fn with_witness(self, witness: Vec<u8>) -> Self {
        let context = self.context.with_witness(witness);
        context.apply();
        Self { context }
    }

    /// Sets gas limit
    pub fn with_gas_limit(self, gas_limit: i64) -> Self {
        let context = self.context.with_gas_limit(gas_limit);
        context.apply();
        Self { context }
    }

    /// Sets calling script hash
    pub fn with_calling_script_hash(self, script_hash: Vec<u8>) -> Self {
        let context = self.context.with_calling_script_hash(script_hash);
        context.apply();
        Self { context }
    }

    /// Runs a test function with this fixture
    pub fn run<F, R, E>(&self, test_fn: F) -> TestResult<R, E>
    where F: FnOnce() -> Result<R, E> {
        // Apply context
        self.context.apply();

        // Run the test
        let result = test_fn();

        // Return the result
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_apply() {
        // Create a test context
        let context = TestContext::new()
            .with_block_height(1000)
            .with_timestamp(1_620_000_000_000)
            .with_witness(vec![1, 2, 3, 4, 5]);

        // Apply the context
        context.apply();

        // Check that the context was applied
        assert_eq!(MockRuntime::get_block_height(), 1000);
        assert_eq!(MockRuntime::get_time(), 1_620_000_000_000);
        assert!(MockRuntime::check_witness(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_fixture_run() {
        // Create a test fixture
        let fixture = TestFixture::new().with_block_height(1000).with_timestamp(1_620_000_000_000);

        // Run a test function
        let result = fixture.run(|| {
            // Check that the context was applied
            assert_eq!(MockRuntime::get_block_height(), 1000);
            assert_eq!(MockRuntime::get_time(), 1_620_000_000_000);

            Ok::<_, ()>(42)
        });

        // Check the result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }
}
