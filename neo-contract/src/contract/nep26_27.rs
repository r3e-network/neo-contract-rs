//! NEP-26: NFT Transfer Callback
//! NEP-27: Token Transfer Callback
//! Provides callback mechanisms for token transfers

use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, Array, Any};
use crate::services::contract::{Contract, CallFlags};

/// NEP-26: NFT Transfer Callback Interface
pub trait NEP26Receiver {
    /// Called when receiving an NFT
    /// Returns true if the transfer is accepted
    fn on_nep11_payment(
        &mut self,
        from: H160,
        amount: Int256,
        token_id: ByteString,
        data: Any,
    ) -> bool;
}

/// NEP-27: Token Transfer Callback Interface
pub trait NEP27Receiver {
    /// Called when receiving tokens
    /// Returns true if the transfer is accepted
    fn on_nep17_payment(
        &mut self,
        from: H160,
        amount: Int256,
        data: Any,
    ) -> bool;
}

/// NEP-26/27 Implementation Helper
pub struct TransferCallback;

impl TransferCallback {
    /// Invoke NEP-26 callback for NFT transfer
    pub fn invoke_nep26_callback(
        contract: H160,
        from: H160,
        to: H160,
        amount: Int256,
        token_id: ByteString,
        data: Any,
    ) -> bool {
        // Check if the recipient is a contract
        if !Self::is_contract(to) {
            return true; // Not a contract, no callback needed
        }

        // Try to invoke the callback
        let result = Contract::call(
            to,
            ByteString::from_literal("onNEP11Payment"),
            Array::from_vec(vec![
                from.into_any(),
                amount.into_any(),
                token_id.into_any(),
                data,
            ]),
            CallFlags::ALL,
        );

        // Check if callback was successful
        match result {
            Some(val) => val.as_bool().unwrap_or(false),
            None => false, // Callback failed or doesn't exist
        }
    }

    /// Invoke NEP-27 callback for token transfer
    pub fn invoke_nep27_callback(
        contract: H160,
        from: H160,
        to: H160,
        amount: Int256,
        data: Any,
    ) -> bool {
        // Check if the recipient is a contract
        if !Self::is_contract(to) {
            return true; // Not a contract, no callback needed
        }

        // Try to invoke the callback
        let result = Contract::call(
            to,
            ByteString::from_literal("onNEP17Payment"),
            Array::from_vec(vec![
                from.into_any(),
                amount.into_any(),
                data,
            ]),
            CallFlags::ALL,
        );

        // Check if callback was successful
        match result {
            Some(val) => val.as_bool().unwrap_or(false),
            None => false, // Callback failed or doesn't exist
        }
    }

    /// Check if an address is a contract
    fn is_contract(address: H160) -> bool {
        use crate::contract::native::ContractManagement;
        
        ContractManagement::get_contract(address).is_some()
    }

    /// Safe transfer with NEP-26 callback for NFTs
    pub fn safe_nft_transfer(
        from: H160,
        to: H160,
        token_id: ByteString,
        data: Any,
    ) -> Result<(), ByteString> {
        // Perform the actual transfer (implementation specific)
        // This would be done by the NFT contract
        
        // Invoke callback if recipient is a contract
        if Self::is_contract(to) {
            let callback_result = Self::invoke_nep26_callback(
                H160::zero(), // Would be actual contract hash
                from,
                to,
                Int256::one(), // NFT amount is always 1
                token_id.clone(),
                data,
            );

            if !callback_result {
                return Err(ByteString::from_literal("Transfer rejected by recipient"));
            }
        }

        // Emit transfer event
        use crate::runtime::Runtime;
        Runtime::notify(
            ByteString::from_literal("Transfer"),
            Array::from_vec(vec![
                from.into_any(),
                to.into_any(),
                Int256::one().into_any(),
                token_id.into_any(),
            ]),
        );

        Ok(())
    }

    /// Safe transfer with NEP-27 callback for tokens
    pub fn safe_token_transfer(
        from: H160,
        to: H160,
        amount: Int256,
        data: Any,
    ) -> Result<(), ByteString> {
        // Perform the actual transfer (implementation specific)
        // This would be done by the token contract
        
        // Invoke callback if recipient is a contract
        if Self::is_contract(to) {
            let callback_result = Self::invoke_nep27_callback(
                H160::zero(), // Would be actual contract hash
                from,
                to,
                amount,
                data,
            );

            if !callback_result {
                return Err(ByteString::from_literal("Transfer rejected by recipient"));
            }
        }

        // Emit transfer event
        use crate::runtime::Runtime;
        Runtime::notify(
            ByteString::from_literal("Transfer"),
            Array::from_vec(vec![
                from.into_any(),
                to.into_any(),
                amount.into_any(),
            ]),
        );

        Ok(())
    }
}

/// Helper macro for implementing NEP-26 receiver
#[macro_export]
macro_rules! impl_nep26_receiver {
    ($contract:ty) => {
        impl NEP26Receiver for $contract {
            fn on_nep11_payment(
                &mut self,
                from: H160,
                amount: Int256,
                token_id: ByteString,
                data: Any,
            ) -> bool {
                // Default implementation: accept all transfers
                true
            }
        }
    };
}

/// Helper macro for implementing NEP-27 receiver
#[macro_export]
macro_rules! impl_nep27_receiver {
    ($contract:ty) => {
        impl NEP27Receiver for $contract {
            fn on_nep17_payment(
                &mut self,
                from: H160,
                amount: Int256,
                data: Any,
            ) -> bool {
                // Default implementation: accept all transfers
                true
            }
        }
    };
}

/// Example implementation of a contract that receives tokens/NFTs
pub struct ReceiverContract {
    accepted_tokens: Vec<H160>,
    accepted_nfts: Vec<H160>,
}

impl ReceiverContract {
    pub fn new() -> Self {
        Self {
            accepted_tokens: Vec::new(),
            accepted_nfts: Vec::new(),
        }
    }

    /// Add accepted token contract
    pub fn add_accepted_token(&mut self, token: H160) {
        self.accepted_tokens.push(token);
    }

    /// Add accepted NFT contract
    pub fn add_accepted_nft(&mut self, nft: H160) {
        self.accepted_nfts.push(nft);
    }

    /// Check if token is accepted
    fn is_token_accepted(&self, token: H160) -> bool {
        use crate::runtime::Runtime;
        let caller = Runtime::get_calling_script_hash();
        self.accepted_tokens.iter().any(|&t| t == caller)
    }

    /// Check if NFT is accepted
    fn is_nft_accepted(&self, nft: H160) -> bool {
        use crate::runtime::Runtime;
        let caller = Runtime::get_calling_script_hash();
        self.accepted_nfts.iter().any(|&n| n == caller)
    }
}

impl NEP26Receiver for ReceiverContract {
    fn on_nep11_payment(
        &mut self,
        from: H160,
        amount: Int256,
        token_id: ByteString,
        data: Any,
    ) -> bool {
        use crate::runtime::Runtime;
        
        // Check if the NFT contract is accepted
        let nft_contract = Runtime::get_calling_script_hash();
        if !self.is_nft_accepted(nft_contract) {
            Runtime::log(ByteString::from_literal("NFT not accepted"));
            return false;
        }

        // Process the NFT (store, register, etc.)
        Runtime::log(ByteString::from_literal("NFT received"));
        
        // Return true to accept the transfer
        true
    }
}

impl NEP27Receiver for ReceiverContract {
    fn on_nep17_payment(
        &mut self,
        from: H160,
        amount: Int256,
        data: Any,
    ) -> bool {
        use crate::runtime::Runtime;
        
        // Check if the token contract is accepted
        let token_contract = Runtime::get_calling_script_hash();
        if !self.is_token_accepted(token_contract) {
            Runtime::log(ByteString::from_literal("Token not accepted"));
            return false;
        }

        // Process the tokens (credit account, etc.)
        Runtime::log(ByteString::from_literal("Tokens received"));
        
        // Return true to accept the transfer
        true
    }
}