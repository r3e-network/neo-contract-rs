// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

/// Simple asset transfer contract that demonstrates native token transfers
/// using the Neo Contract annotation syntax

/// Event emitted when a transfer is completed successfully
#[neo_contract::event(
    from: H160,
    to: H160,
    neo_amount: Int256,
    gas_amount: Int256
)]
struct TransferCompleted {}

/// Simple transfer contract that handles Neo and Gas transfers
#[neo_contract::contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("Simple asset transfer contract")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
pub struct Transfer {
    // Empty storage as this is a simple transfer contract
}

impl Transfer {
    /// Contract constructor - initializes an empty contract
    #[constructor]
    pub fn new() -> Self {
        Self {}
    }

    /// Transfers Neo and Gas tokens from the sender to the specified recipient
    /// 
    /// # Arguments
    /// * `from` - The sender address (must be the caller)
    /// * `to` - The recipient address
    /// * `amount` - The amount of Neo tokens to transfer
    ///
    /// # Returns
    /// `true` if the transfer was successful, `false` otherwise
    #[method]
    #[no_reentry]
    pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
        // Verify that the sender is authorized
        assert!(Runtime::check_witness(&from), "No authorization");
        
        // Get the executing contract's script hash
        let executing = Runtime::executing_script_hash();
        
        // Transfer Neo tokens
        assert!(Neo::transfer(executing, to, amount), "Neo transfer failed");
        
        // Transfer all Gas tokens
        let gas_balance = Gas::balance_of(executing);
        assert!(Gas::transfer(executing, to, gas_balance), "Gas transfer failed");
        
        // Create a storage context (not used here, but shown for demonstration)
        let _ = StorageContext::new();
        
        // Emit the transfer event
        TransferCompleted {}.notify(&from, &to, &amount, &gas_balance);
        
        true
    }
    
    /// Gets the available Neo balance of the contract
    ///
    /// # Returns
    /// The Neo balance of the contract
    #[method]
    #[safe]
    pub fn get_neo_balance(&self) -> Int256 {
        let executing = Runtime::executing_script_hash();
        Neo::balance_of(executing)
    }

    /// Gets the available Gas balance of the contract
    ///
    /// # Returns
    /// The Gas balance of the contract
    #[method]
    #[safe]
    pub fn get_gas_balance(&self) -> Int256 {
        let executing = Runtime::executing_script_hash();
        Gas::balance_of(executing)
    }
}

/// Helper function for asserting conditions
#[inline(always)]
fn assert(condition: bool, message: &str) {
    if !condition {
        panic!("{}", message);
    }
}
