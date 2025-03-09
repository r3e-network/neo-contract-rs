//! Mock implementation of Neo runtime
//!
//! This module provides a mock implementation of Neo runtime
//! for testing smart contracts.

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use core::cell::RefCell;
use alloc::collections::{BTreeMap, BTreeSet};

use crate::mock::events::MockEvents;

thread_local! {
    /// Block height
    static BLOCK_HEIGHT: RefCell<u32> = RefCell::new(0);
    
    /// Timestamp
    static TIMESTAMP: RefCell<u64> = RefCell::new(0);
    
    /// Gas left
    static GAS_LEFT: RefCell<i64> = RefCell::new(10_000_000);
    
    /// Witnesses
    static WITNESSES: RefCell<BTreeSet<Vec<u8>>> = RefCell::new(BTreeSet::new());
    
    /// Attributes
    static ATTRIBUTES: RefCell<BTreeMap<u8, Vec<u8>>> = RefCell::new(BTreeMap::new());
    
    /// Calling script hash
    static CALLING_SCRIPT_HASH: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    
    /// Executing script hash
    static EXECUTING_SCRIPT_HASH: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    
    /// Entry script hash
    static ENTRY_SCRIPT_HASH: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    
    /// Random seed
    static RANDOM_SEED: RefCell<u32> = RefCell::new(0);
}

/// Mock implementation of Neo runtime
pub struct MockRuntime;

impl MockRuntime {
    /// Reset runtime to default state
    pub fn reset() {
        BLOCK_HEIGHT.with(|h| *h.borrow_mut() = 0);
        TIMESTAMP.with(|t| *t.borrow_mut() = 0);
        GAS_LEFT.with(|g| *g.borrow_mut() = 10_000_000);
        WITNESSES.with(|w| w.borrow_mut().clear());
        ATTRIBUTES.with(|a| a.borrow_mut().clear());
        CALLING_SCRIPT_HASH.with(|h| h.borrow_mut().clear());
        EXECUTING_SCRIPT_HASH.with(|h| h.borrow_mut().clear());
        ENTRY_SCRIPT_HASH.with(|h| h.borrow_mut().clear());
        RANDOM_SEED.with(|s| *s.borrow_mut() = 0);
        
        // Also reset events (notifications)
        MockEvents::reset();
    }
    
    /// Get current block height
    pub fn get_block_height() -> u32 {
        BLOCK_HEIGHT.with(|h| *h.borrow())
    }
    
    /// Set block height
    pub fn set_block_height(height: u32) {
        BLOCK_HEIGHT.with(|h| *h.borrow_mut() = height);
    }
    
    /// Get current timestamp
    pub fn get_time() -> u64 {
        TIMESTAMP.with(|t| *t.borrow())
    }
    
    /// Set timestamp
    pub fn set_time(timestamp: u64) {
        TIMESTAMP.with(|t| *t.borrow_mut() = timestamp);
    }
    
    /// Get gas left
    pub fn get_gas_left() -> i64 {
        GAS_LEFT.with(|g| *g.borrow())
    }
    
    /// Set gas left
    pub fn set_gas_left(gas: i64) {
        GAS_LEFT.with(|g| *g.borrow_mut() = gas);
    }
    
    /// Consume gas
    pub fn consume_gas(gas: i64) {
        GAS_LEFT.with(|g| *g.borrow_mut() -= gas);
    }
    
    /// Check if a witness is valid
    pub fn check_witness(witness: &[u8]) -> bool {
        WITNESSES.with(|w| w.borrow().contains(&witness.to_vec()))
    }
    
    /// Add a witness
    pub fn add_witness(witness: &[u8]) {
        WITNESSES.with(|w| w.borrow_mut().insert(witness.to_vec()));
    }
    
    /// Remove a witness
    pub fn remove_witness(witness: &[u8]) {
        WITNESSES.with(|w| w.borrow_mut().remove(&witness.to_vec()));
    }
    
    /// Clear all witnesses
    pub fn clear_witnesses() {
        WITNESSES.with(|w| w.borrow_mut().clear());
    }
    
    /// Get all witnesses
    pub fn get_all_witnesses() -> Vec<Vec<u8>> {
        WITNESSES.with(|w| w.borrow().iter().cloned().collect())
    }
    
    /// Get attribute
    pub fn get_attribute(type_id: u8) -> Option<Vec<u8>> {
        ATTRIBUTES.with(|a| a.borrow().get(&type_id).cloned())
    }
    
    /// Set attribute
    pub fn set_attribute(type_id: u8, value: &[u8]) {
        ATTRIBUTES.with(|a| a.borrow_mut().insert(type_id, value.to_vec()));
    }
    
    /// Clear attribute
    pub fn clear_attribute(type_id: u8) {
        ATTRIBUTES.with(|a| a.borrow_mut().remove(&type_id));
    }
    
    /// Get calling script hash
    pub fn get_calling_script_hash() -> Vec<u8> {
        CALLING_SCRIPT_HASH.with(|h| h.borrow().clone())
    }
    
    /// Set calling script hash
    pub fn set_calling_script_hash(hash: &[u8]) {
        CALLING_SCRIPT_HASH.with(|h| *h.borrow_mut() = hash.to_vec());
    }
    
    /// Get executing script hash
    pub fn get_executing_script_hash() -> Vec<u8> {
        EXECUTING_SCRIPT_HASH.with(|h| h.borrow().clone())
    }
    
    /// Set executing script hash
    pub fn set_executing_script_hash(hash: &[u8]) {
        EXECUTING_SCRIPT_HASH.with(|h| *h.borrow_mut() = hash.to_vec());
    }
    
    /// Get entry script hash
    pub fn get_entry_script_hash() -> Vec<u8> {
        ENTRY_SCRIPT_HASH.with(|h| h.borrow().clone())
    }
    
    /// Set entry script hash
    pub fn set_entry_script_hash(hash: &[u8]) {
        ENTRY_SCRIPT_HASH.with(|h| *h.borrow_mut() = hash.to_vec());
    }
    
    /// Get a random number
    pub fn get_random() -> u32 {
        RANDOM_SEED.with(|s| {
            // Simple linear congruential generator
            let mut seed = *s.borrow();
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            *s.borrow_mut() = seed;
            seed
        })
    }
    
    /// Set random seed
    pub fn set_random_seed(seed: u32) {
        RANDOM_SEED.with(|s| *s.borrow_mut() = seed);
    }
    
    /// Log a message
    pub fn log(message: &str) {
        // In a real implementation, this would print to a test log
        #[cfg(feature = "std")]
        {
            println!("[LOG] {}", message);
        }
    }
    
    /// Notify (emit event)
    pub fn notify<T: AsRef<[u8]>>(name: &str, args: Vec<T>) {
        let args_vec: Vec<Vec<u8>> = args.into_iter()
            .map(|arg| arg.as_ref().to_vec())
            .collect();
        
        // Get current timestamp
        let timestamp = Self::get_time();
        
        MockEvents::emit(name, args_vec, timestamp);
    }
    
    /// Verify if a notification has been emitted
    pub fn verify_notification(name: &str) -> bool {
        MockEvents::has_event(name)
    }
    
    /// Clear all notifications
    pub fn clear_notifications() {
        MockEvents::reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_height() {
        // Reset runtime
        MockRuntime::reset();
        
        // Check default block height
        assert_eq!(MockRuntime::get_block_height(), 0);
        
        // Set block height
        MockRuntime::set_block_height(1000);
        
        // Check block height
        assert_eq!(MockRuntime::get_block_height(), 1000);
    }
    
    #[test]
    fn test_timestamp() {
        // Reset runtime
        MockRuntime::reset();
        
        // Set timestamp
        let timestamp = 1620000000000;
        MockRuntime::set_time(timestamp);
        
        // Check timestamp
        assert_eq!(MockRuntime::get_time(), timestamp);
    }
    
    #[test]
    fn test_gas() {
        // Reset runtime
        MockRuntime::reset();
        
        // Check default gas
        assert_eq!(MockRuntime::get_gas_left(), 10_000_000);
        
        // Consume gas
        MockRuntime::consume_gas(1000);
        
        // Check gas left
        assert_eq!(MockRuntime::get_gas_left(), 10_000_000 - 1000);
        
        // Set gas left
        MockRuntime::set_gas_left(5000);
        
        // Check gas left
        assert_eq!(MockRuntime::get_gas_left(), 5000);
    }
    
    #[test]
    fn test_witness() {
        // Reset runtime
        MockRuntime::reset();
        
        // Add a witness
        let witness = [1, 2, 3, 4, 5];
        MockRuntime::add_witness(&witness);
        
        // Check witness
        assert!(MockRuntime::check_witness(&witness));
        assert!(!MockRuntime::check_witness(&[6, 7, 8, 9, 10]));
        
        // Remove witness
        MockRuntime::remove_witness(&witness);
        
        // Check witness
        assert!(!MockRuntime::check_witness(&witness));
    }
    
    #[test]
    fn test_notify() {
        // Reset runtime
        MockRuntime::reset();
        
        // Set timestamp
        let timestamp = 1620000000000;
        MockRuntime::set_time(timestamp);
        
        // Emit notification
        MockRuntime::notify("TestEvent", vec![vec![1, 2, 3]]);
        
        // Check notification
        assert!(MockRuntime::verify_notification("TestEvent"));
        assert!(!MockRuntime::verify_notification("OtherEvent"));
        
        // Check events
        let events = MockEvents::get_all();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "TestEvent");
        assert_eq!(events[0].args.len(), 1);
        assert_eq!(events[0].args[0], vec![1, 2, 3]);
        assert_eq!(events[0].timestamp, timestamp);
        
        // Clear notifications
        MockRuntime::clear_notifications();
        
        // Check notifications cleared
        assert!(!MockRuntime::verify_notification("TestEvent"));
        assert_eq!(MockEvents::get_all().len(), 0);
    }
    
    #[test]
    fn test_random() {
        // Reset runtime
        MockRuntime::reset();
        
        // Set random seed
        MockRuntime::set_random_seed(12345);
        
        // Get a random number
        let random1 = MockRuntime::get_random();
        
        // Get another random number
        let random2 = MockRuntime::get_random();
        
        // They should be different
        assert_ne!(random1, random2);
        
        // Reset and set the same seed
        MockRuntime::reset();
        MockRuntime::set_random_seed(12345);
        
        // Get a random number
        let random3 = MockRuntime::get_random();
        
        // It should be the same as before
        assert_eq!(random1, random3);
    }
}