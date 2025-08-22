//! NEP-26: NFT Transfer Callback
//! NEP-27: Token Transfer Callback
//! Provides callback mechanisms for token transfers

extern crate alloc;
use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, Array, Any};
use crate::services::contract::Contract;
use crate::types::CallFlags;

type Result<T> = core::result::Result<T, ByteString>;

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
        _contract: H160,
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
            CallFlags::ALL,
            Array::from_vec(vec![
                from.into_any(),
                amount.into_any(),
                token_id.into_any(),
                data,
            ]),
        );

        // Check if callback was successful
        result.as_bool().unwrap_or(false)
    }

    /// Invoke NEP-27 callback for token transfer
    pub fn invoke_nep27_callback(
        _contract: H160,
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
            CallFlags::ALL,
            Array::from_vec(vec![
                from.into_any(),
                amount.into_any(),
                data,
            ]),
        );

        // Check if callback was successful
        result.as_bool().unwrap_or(false)
    }

    /// Check if an address is a contract
    fn is_contract(address: H160) -> bool {
        // Production implementation: Check if address contains deployed contract
        #[cfg(target_family = "wasm")]
        {
            use crate::native::contract_management::ContractManagement;
            // Query ContractManagement native contract to verify if address has deployed contract
            match ContractManagement::get_contract_by_hash(address) {
                Some(_contract) => true,  // Contract exists at this address
                None => false,           // No contract deployed at this address
            }
        }
        
        #[cfg(not(target_family = "wasm"))]
        {
            // Testing implementation: Check for non-zero address
            !address.is_zero()
        }
    }

    /// Safe transfer with NEP-26 callback for NFTs
    pub fn safe_nft_transfer(
        from: H160,
        to: H160,
        token_id: ByteString,
        data: Any,
    ) -> Result<()> {
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
        use crate::services::runtime::Runtime;
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
    ) -> Result<()> {
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
        use crate::services::runtime::Runtime;
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
    accepted_tokens: alloc::vec::Vec<H160>,
    accepted_nfts: alloc::vec::Vec<H160>,
}

impl ReceiverContract {
    pub fn new() -> Self {
        Self {
            accepted_tokens: alloc::vec::Vec::new(),
            accepted_nfts: alloc::vec::Vec::new(),
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
    fn is_token_accepted(&self, _token: H160) -> bool {
        use crate::services::runtime::Runtime;
        let caller = Runtime::get_calling_script_hash();
        self.accepted_tokens.iter().any(|&t| t == caller)
    }

    /// Check if NFT is accepted
    fn is_nft_accepted(&self, _nft: H160) -> bool {
        use crate::services::runtime::Runtime;
        let caller = Runtime::get_calling_script_hash();
        self.accepted_nfts.iter().any(|&n| n == caller)
    }
}

impl NEP26Receiver for ReceiverContract {
    fn on_nep11_payment(
        &mut self,
        _from: H160,
        _amount: Int256,
        _token_id: ByteString,
        _data: Any,
    ) -> bool {
        use crate::services::runtime::Runtime;
        
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
        _from: H160,
        _amount: Int256,
        _data: Any,
    ) -> bool {
        use crate::services::runtime::Runtime;
        
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