//! WebAssembly integration example
//!
//! This example demonstrates how to use the framework with wasm-bindgen
//! for browser-based testing.

use neo_contract_testing::mock::{MockRuntime, MockStorage, TestState};
use neo_contract_testing::prelude::*;
#[cfg(feature = "wasm-bindings")]
use wasm_bindgen::prelude::*;

/// Structure to hold the test results for JavaScript
#[cfg_attr(feature = "wasm-bindings", wasm_bindgen)]
#[derive(Debug)]
pub struct TestResult {
    success: bool,
    message: String,
    storage_changes: String,
    events: String,
    gas_used: i64,
}

#[cfg_attr(feature = "wasm-bindings", wasm_bindgen)]
impl TestResult {
    /// Creates a new test result
    pub fn new(success: bool, message: String, storage_changes: String, events: String, gas_used: i64) -> Self {
        Self { success, message, storage_changes, events, gas_used }
    }

    /// Get whether the test was successful
    pub fn success(&self) -> bool { self.success }

    /// Get the result message
    pub fn message(&self) -> String { self.message.clone() }

    /// Get the storage changes
    pub fn storage_changes(&self) -> String { self.storage_changes.clone() }

    /// Get the events
    pub fn events(&self) -> String { self.events.clone() }

    /// Get the gas used
    pub fn gas_used(&self) -> i64 { self.gas_used }
}

/// Mock NEP-17 token transfer function for WASM
///
/// In a real test, this would call your actual contract code.
fn mock_nep17_transfer_wasm(from: &[u8], to: &[u8], amount: u64) -> Result<Vec<u8>, String> {
    // Check witness
    if !MockRuntime::check_witness(from) {
        return Err("No authorization".into());
    }

    // Construct storage keys
    let from_key = [b"balance:".to_vec(), from.to_vec()].concat();
    let to_key = [b"balance:".to_vec(), to.to_vec()].concat();

    // Get current balances
    let from_balance =
        MockStorage::get(&from_key).map(|v| u64::from_le_bytes(v.try_into().unwrap_or([0; 8]))).unwrap_or(0);

    let to_balance = MockStorage::get(&to_key).map(|v| u64::from_le_bytes(v.try_into().unwrap_or([0; 8]))).unwrap_or(0);

    // Check sufficient balance
    if from_balance < amount {
        return Err("Insufficient balance".into());
    }

    // Update balances
    let new_from_balance = from_balance - amount;
    let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

    // Store new balances
    MockStorage::put(&from_key, &new_from_balance.to_le_bytes());
    MockStorage::put(&to_key, &new_to_balance.to_le_bytes());

    // Emit transfer event
    MockRuntime::notify("Transfer", vec![from.to_vec(), to.to_vec(), amount.to_le_bytes().to_vec()]);

    // Return success
    Ok(vec![1]) // Return true
}

/// Runs a NEP-17 transfer test and returns the result to JavaScript
#[cfg_attr(feature = "wasm-bindings", wasm_bindgen)]
pub fn test_nep17_transfer(from_hex: &str, to_hex: &str, amount: u64, authorize: bool) -> TestResult {
    // Convert hex strings to bytes
    let from = hex::decode(from_hex).unwrap_or_default();
    let to = hex::decode(to_hex).unwrap_or_default();

    // Create a simulator with debug enabled
    let simulator = ContractSimulator::new().with_capture_debug(true, TracingMode::Full);

    // Set up initial balances (both accounts start with 1000 tokens)
    let from_key = [b"balance:".to_vec(), from.clone()].concat();
    let to_key = [b"balance:".to_vec(), to.clone()].concat();
    MockStorage::put(&from_key, &1000u64.to_le_bytes());
    MockStorage::put(&to_key, &1000u64.to_le_bytes());

    // Authorize from account if needed
    if authorize {
        MockRuntime::add_witness(&from);
    }

    // Capture gas before invocation
    let gas_before = MockRuntime::get_gas_left();

    // Invoke contract
    let args = vec![from.clone(), to.clone(), amount.to_le_bytes().to_vec()];

    // Here we need a wrapper to adapt our function to the simulator's expected signature
    let wrapper = |args: Vec<Vec<u8>>| -> Result<Vec<u8>, String> {
        if args.len() != 3 {
            return Err("Expected 3 arguments".into());
        }
        mock_nep17_transfer_wasm(&args[0], &args[1], u64::from_le_bytes(args[2].clone().try_into().unwrap_or([0; 8])))
    };

    let result = simulator.invoke(wrapper, args);

    // Prepare result for JavaScript
    let message = if result.success {
        "Transaction successful".to_string()
    } else {
        format!("Transaction failed: {}", result.error.unwrap_or("Unknown error".into()))
    };

    // Format storage changes as JSON
    let storage_changes = serde_json::to_string(&result.storage_changes).unwrap_or_default();

    // Format events as JSON
    let events = serde_json::to_string(&result.events).unwrap_or_default();

    // Calculate gas used
    let gas_used = gas_before - MockRuntime::get_gas_left();

    // Return test result
    TestResult::new(result.success, message, storage_changes, events, gas_used)
}

/// Sets up the testing environment in WASM
#[cfg_attr(feature = "wasm-bindings", wasm_bindgen(start))]
pub fn wasm_main() {
    // Initialize console logging
    #[cfg(feature = "wasm-bindings")]
    {
        use web_sys::console;
        console::log_1(&"Neo Contract Testing Framework initialized".into());
    }

    // Reset test environment
    TestState::reset_all();
    DebugCapture::reset();
}

#[cfg(test)]
#[cfg(feature = "wasm-bindings")]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // Configure the test to run in a browser environment
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_transfer_success() {
        // Set up the test environment
        wasm_main();

        // Create test accounts
        let from_hex = "0101010101010101010101010101010101010101"; // 20 bytes
        let to_hex = "0202020202020202020202020202020202020202"; // 20 bytes

        // Run the test with authorization
        let result = test_nep17_transfer(from_hex, to_hex, 500, true);

        // Assert success
        assert!(result.success());
        assert_eq!(result.message(), "Transaction successful");
        assert!(result.gas_used() > 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_transfer_unauthorized() {
        // Set up the test environment
        wasm_main();

        // Create test accounts
        let from_hex = "0101010101010101010101010101010101010101"; // 20 bytes
        let to_hex = "0202020202020202020202020202020202020202"; // 20 bytes

        // Run the test without authorization
        let result = test_nep17_transfer(from_hex, to_hex, 500, false);

        // Assert failure
        assert!(!result.success());
        assert_eq!(result.message(), "Transaction failed: No authorization");
    }

    #[wasm_bindgen_test]
    fn test_wasm_transfer_insufficient_balance() {
        // Set up the test environment
        wasm_main();

        // Create test accounts
        let from_hex = "0101010101010101010101010101010101010101"; // 20 bytes
        let to_hex = "0202020202020202020202020202020202020202"; // 20 bytes

        // Run the test with authorization but exceeding balance
        let result = test_nep17_transfer(from_hex, to_hex, 2000, true); // Trying to transfer 2000 when balance is 1000

        // Assert failure
        assert!(!result.success());
        assert_eq!(result.message(), "Transaction failed: Insufficient balance");
    }
}

fn main() {}