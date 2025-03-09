//! Mock implementation of Neo transaction
//!
//! This module provides a mock implementation of Neo transaction
//! for testing smart contracts.

use alloc::vec::Vec;
use core::cell::RefCell;
use alloc::collections::BTreeMap;

thread_local! {
    /// Transaction version
    static VERSION: RefCell<u8> = RefCell::new(0);
    
    /// Transaction nonce
    static NONCE: RefCell<u32> = RefCell::new(0);
    
    /// Transaction sender script hash
    static SENDER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    
    /// Transaction system fee
    static SYSTEM_FEE: RefCell<i64> = RefCell::new(0);
    
    /// Transaction network fee
    static NETWORK_FEE: RefCell<i64> = RefCell::new(0);
    
    /// Transaction valid until block
    static VALID_UNTIL_BLOCK: RefCell<u32> = RefCell::new(0);
    
    /// Transaction signers
    static SIGNERS: RefCell<Vec<Vec<u8>>> = RefCell::new(Vec::new());
    
    /// Transaction attributes
    static TX_ATTRIBUTES: RefCell<BTreeMap<u8, Vec<u8>>> = RefCell::new(BTreeMap::new());
}

/// Mock implementation of Neo transaction
pub struct MockTransaction;

impl MockTransaction {
    /// Reset transaction to default state
    pub fn reset() {
        VERSION.with(|v| *v.borrow_mut() = 0);
        NONCE.with(|n| *n.borrow_mut() = 0);
        SENDER.with(|s| s.borrow_mut().clear());
        SYSTEM_FEE.with(|f| *f.borrow_mut() = 0);
        NETWORK_FEE.with(|f| *f.borrow_mut() = 0);
        VALID_UNTIL_BLOCK.with(|b| *b.borrow_mut() = 0);
        SIGNERS.with(|s| s.borrow_mut().clear());
        TX_ATTRIBUTES.with(|a| a.borrow_mut().clear());
    }
    
    /// Get transaction version
    pub fn get_version() -> u8 {
        VERSION.with(|v| *v.borrow())
    }
    
    /// Set transaction version
    pub fn set_version(version: u8) {
        VERSION.with(|v| *v.borrow_mut() = version);
    }
    
    /// Get transaction nonce
    pub fn get_nonce() -> u32 {
        NONCE.with(|n| *n.borrow())
    }
    
    /// Set transaction nonce
    pub fn set_nonce(nonce: u32) {
        NONCE.with(|n| *n.borrow_mut() = nonce);
    }
    
    /// Get transaction sender
    pub fn get_sender() -> Vec<u8> {
        SENDER.with(|s| s.borrow().clone())
    }
    
    /// Set transaction sender
    pub fn set_sender(sender: &[u8]) {
        SENDER.with(|s| *s.borrow_mut() = sender.to_vec());
    }
    
    /// Get transaction system fee
    pub fn get_system_fee() -> i64 {
        SYSTEM_FEE.with(|f| *f.borrow())
    }
    
    /// Set transaction system fee
    pub fn set_system_fee(fee: i64) {
        SYSTEM_FEE.with(|f| *f.borrow_mut() = fee);
    }
    
    /// Get transaction network fee
    pub fn get_network_fee() -> i64 {
        NETWORK_FEE.with(|f| *f.borrow())
    }
    
    /// Set transaction network fee
    pub fn set_network_fee(fee: i64) {
        NETWORK_FEE.with(|f| *f.borrow_mut() = fee);
    }
    
    /// Get transaction valid until block
    pub fn get_valid_until_block() -> u32 {
        VALID_UNTIL_BLOCK.with(|b| *b.borrow())
    }
    
    /// Set transaction valid until block
    pub fn set_valid_until_block(block: u32) {
        VALID_UNTIL_BLOCK.with(|b| *b.borrow_mut() = block);
    }
    
    /// Get transaction signers
    pub fn get_signers() -> Vec<Vec<u8>> {
        SIGNERS.with(|s| s.borrow().clone())
    }
    
    /// Add a signer
    pub fn add_signer(signer: &[u8]) {
        SIGNERS.with(|s| s.borrow_mut().push(signer.to_vec()));
    }
    
    /// Remove a signer
    pub fn remove_signer(signer: &[u8]) {
        SIGNERS.with(|s| {
            let mut signers = s.borrow_mut();
            let signer_vec = signer.to_vec();
            if let Some(idx) = signers.iter().position(|s| *s == signer_vec) {
                signers.remove(idx);
            }
        });
    }
    
    /// Check if a signer exists
    pub fn has_signer(signer: &[u8]) -> bool {
        SIGNERS.with(|s| {
            let signers = s.borrow();
            let signer_vec = signer.to_vec();
            signers.iter().any(|s| *s == signer_vec)
        })
    }
    
    /// Clear all signers
    pub fn clear_signers() {
        SIGNERS.with(|s| s.borrow_mut().clear());
    }
    
    /// Get transaction attribute
    pub fn get_attribute(type_id: u8) -> Option<Vec<u8>> {
        TX_ATTRIBUTES.with(|a| a.borrow().get(&type_id).cloned())
    }
    
    /// Set transaction attribute
    pub fn set_attribute(type_id: u8, value: &[u8]) {
        TX_ATTRIBUTES.with(|a| a.borrow_mut().insert(type_id, value.to_vec()));
    }
    
    /// Clear transaction attribute
    pub fn clear_attribute(type_id: u8) {
        TX_ATTRIBUTES.with(|a| a.borrow_mut().remove(&type_id));
    }
    
    /// Get all transaction attributes
    pub fn get_all_attributes() -> BTreeMap<u8, Vec<u8>> {
        TX_ATTRIBUTES.with(|a| a.borrow().clone())
    }
    
    /// Clear all transaction attributes
    pub fn clear_all_attributes() {
        TX_ATTRIBUTES.with(|a| a.borrow_mut().clear());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        // Reset transaction
        MockTransaction::reset();
        
        // Check default version
        assert_eq!(MockTransaction::get_version(), 0);
        
        // Set version
        MockTransaction::set_version(1);
        
        // Check version
        assert_eq!(MockTransaction::get_version(), 1);
    }
    
    #[test]
    fn test_sender() {
        // Reset transaction
        MockTransaction::reset();
        
        // Set sender
        let sender = [1, 2, 3, 4, 5];
        MockTransaction::set_sender(&sender);
        
        // Check sender
        assert_eq!(MockTransaction::get_sender(), sender);
    }
    
    #[test]
    fn test_fees() {
        // Reset transaction
        MockTransaction::reset();
        
        // Check default fees
        assert_eq!(MockTransaction::get_system_fee(), 0);
        assert_eq!(MockTransaction::get_network_fee(), 0);
        
        // Set fees
        MockTransaction::set_system_fee(100);
        MockTransaction::set_network_fee(200);
        
        // Check fees
        assert_eq!(MockTransaction::get_system_fee(), 100);
        assert_eq!(MockTransaction::get_network_fee(), 200);
    }
    
    #[test]
    fn test_signers() {
        // Reset transaction
        MockTransaction::reset();
        
        // Add signers
        let signer1 = [1, 2, 3, 4, 5];
        let signer2 = [6, 7, 8, 9, 10];
        MockTransaction::add_signer(&signer1);
        MockTransaction::add_signer(&signer2);
        
        // Check signers
        let signers = MockTransaction::get_signers();
        assert_eq!(signers.len(), 2);
        assert!(signers.contains(&signer1.to_vec()));
        assert!(signers.contains(&signer2.to_vec()));
        
        // Check if signers exist
        assert!(MockTransaction::has_signer(&signer1));
        assert!(MockTransaction::has_signer(&signer2));
        assert!(!MockTransaction::has_signer(&[11, 12, 13, 14, 15]));
        
        // Remove signer
        MockTransaction::remove_signer(&signer1);
        
        // Check signers again
        let signers = MockTransaction::get_signers();
        assert_eq!(signers.len(), 1);
        assert!(!signers.contains(&signer1.to_vec()));
        assert!(signers.contains(&signer2.to_vec()));
        
        // Clear signers
        MockTransaction::clear_signers();
        
        // Check signers again
        let signers = MockTransaction::get_signers();
        assert_eq!(signers.len(), 0);
    }
    
    #[test]
    fn test_attributes() {
        // Reset transaction
        MockTransaction::reset();
        
        // Set attributes
        MockTransaction::set_attribute(1, &[1, 2, 3]);
        MockTransaction::set_attribute(2, &[4, 5, 6]);
        
        // Check attributes
        assert_eq!(MockTransaction::get_attribute(1), Some(vec![1, 2, 3]));
        assert_eq!(MockTransaction::get_attribute(2), Some(vec![4, 5, 6]));
        assert_eq!(MockTransaction::get_attribute(3), None);
        
        // Get all attributes
        let attrs = MockTransaction::get_all_attributes();
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs.get(&1), Some(&vec![1, 2, 3]));
        assert_eq!(attrs.get(&2), Some(&vec![4, 5, 6]));
        
        // Clear attribute
        MockTransaction::clear_attribute(1);
        
        // Check attributes again
        assert_eq!(MockTransaction::get_attribute(1), None);
        assert_eq!(MockTransaction::get_attribute(2), Some(vec![4, 5, 6]));
        
        // Clear all attributes
        MockTransaction::clear_all_attributes();
        
        // Check attributes again
        assert_eq!(MockTransaction::get_all_attributes().len(), 0);
    }
    
    #[test]
    fn test_reset() {
        // Set transaction state
        MockTransaction::set_version(1);
        MockTransaction::set_nonce(123);
        MockTransaction::set_sender(&[1, 2, 3]);
        MockTransaction::set_system_fee(100);
        MockTransaction::set_network_fee(200);
        MockTransaction::set_valid_until_block(1000);
        MockTransaction::add_signer(&[4, 5, 6]);
        MockTransaction::set_attribute(1, &[7, 8, 9]);
        
        // Reset transaction
        MockTransaction::reset();
        
        // Check default values
        assert_eq!(MockTransaction::get_version(), 0);
        assert_eq!(MockTransaction::get_nonce(), 0);
        assert_eq!(MockTransaction::get_sender().len(), 0);
        assert_eq!(MockTransaction::get_system_fee(), 0);
        assert_eq!(MockTransaction::get_network_fee(), 0);
        assert_eq!(MockTransaction::get_valid_until_block(), 0);
        assert_eq!(MockTransaction::get_signers().len(), 0);
        assert_eq!(MockTransaction::get_all_attributes().len(), 0);
    }
}