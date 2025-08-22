//! NEO Governance Functions
//! Extended governance features for the NEO native contract

use crate::prelude::*;
use crate::types::{ByteString, Int256, H160, PublicKey, Array, Any};

/// NEO contract hash on Neo N3
#[inline(always)]
pub fn neo_contract_hash() -> H160 {
    #[cfg(not(target_family = "wasm"))]
    {
        H160::from_array([
            0xef, 0x4c, 0x73, 0xdf, 0x88, 0xf5, 0xa6, 0xfe, 0xec, 0xe6,
            0x21, 0x72, 0x4b, 0x47, 0xdb, 0x60, 0xcc, 0xd4, 0xef, 0xc7,
        ])
    }
    #[cfg(target_family = "wasm")]
    {
        H160::zero()
    }
}

/// NEO Governance operations
pub struct NeoGovernance;

impl NeoGovernance {
    /// Register as a candidate for consensus node
    pub fn register_candidate(pubkey: PublicKey) -> bool {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("registerCandidate"),
            crate::types::CallFlags::STATES,
            Array::from_vec(vec![pubkey.into_any()]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// Unregister as a candidate
    pub fn unregister_candidate(pubkey: PublicKey) -> bool {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("unregisterCandidate"),
            crate::types::CallFlags::STATES,
            Array::from_vec(vec![pubkey.into_any()]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// Vote for a candidate
    pub fn vote(account: H160, vote_to: Option<PublicKey>) -> bool {
        let vote_param = match vote_to {
            Some(pk) => pk.into_any(),
            None => Any::null(),
        };
        
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("vote"),
            crate::types::CallFlags::STATES,
            Array::from_vec(vec![account.into_any(), vote_param]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// Get all registered candidates
    pub fn get_candidates() -> Array<Any> {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getCandidates"),
            crate::types::CallFlags::READ_STATES,
            Array::new(),
        )
        .as_array()
        .unwrap_or_else(Array::new)
    }

    /// Get current committee members
    pub fn get_committee() -> Array<PublicKey> {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getCommittee"),
            crate::types::CallFlags::READ_STATES,
            Array::new(),
        )
        .as_array()
        .unwrap_or_else(Array::new)
    }

    /// Get next block validators
    pub fn get_next_block_validators() -> Array<PublicKey> {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getNextBlockValidators"),
            crate::types::CallFlags::READ_STATES,
            Array::new(),
        )
        .as_array()
        .unwrap_or_else(Array::new)
    }

    /// Get votes of a candidate
    pub fn get_candidate_votes(pubkey: PublicKey) -> Int256 {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getCandidateVote"),
            crate::types::CallFlags::READ_STATES,
            Array::from_vec(vec![pubkey.into_any()]),
        )
        .as_int()
        .unwrap_or_else(Int256::zero)
    }

    /// Get GAS per block reward
    pub fn get_gas_per_block() -> Int256 {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getGasPerBlock"),
            crate::types::CallFlags::READ_STATES,
            Array::new(),
        )
        .as_int()
        .unwrap_or_else(Int256::zero)
    }

    /// Set GAS per block (committee only)
    pub fn set_gas_per_block(gas_per_block: Int256) -> bool {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("setGasPerBlock"),
            crate::types::CallFlags::STATES,
            Array::from_vec(vec![gas_per_block.into_any()]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// Get register price for becoming a candidate
    pub fn get_register_price() -> Int256 {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getRegisterPrice"),
            crate::types::CallFlags::READ_STATES,
            Array::new(),
        )
        .as_int()
        .unwrap_or_else(Int256::zero)
    }

    /// Set register price (committee only)
    pub fn set_register_price(price: Int256) -> bool {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("setRegisterPrice"),
            crate::types::CallFlags::STATES,
            Array::from_vec(vec![price.into_any()]),
        )
        .as_bool()
        .unwrap_or(false)
    }

    /// Get account state including vote information
    pub fn get_account_state(account: H160) -> AccountState {
        let result = crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("getAccountState"),
            crate::types::CallFlags::READ_STATES,
            Array::from_vec(vec![account.into_any()]),
        );

        // Parse the array and extract account state information
        if let Some(arr) = result.as_array::<Any>() {
            // Production implementation: Parse account state from Neo contract response
            // Neo account state typically contains: [balance, height, vote_to]
            if arr.length() >= 3 {
                let balance_any = arr.get(0);
                let height_any = arr.get(1);
                let vote_to_any = arr.get(2);
                
                AccountState {
                    balance: balance_any.as_int().unwrap_or(Int256::zero()),
                    balance_height: height_any.as_int().unwrap_or(Int256::zero()),
                    vote_to: if !vote_to_any.is_null() { 
                        // Convert H160 to PublicKey - in Neo, public keys are used for voting
                        Some(PublicKey::from_bytes(&vote_to_any.as_h160().unwrap_or(H160::zero()).to_bytes()))
                    } else { 
                        None 
                    },
                }
            } else {
                // Insufficient data in response
                AccountState::default()
            }
        } else {
            // No array returned - account not found or error
            AccountState::default()
        }
    }

    /// Calculate bonus GAS for NEO holder
    pub fn calculate_bonus(account: H160, end_height: u32) -> Int256 {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("calculateBonus"),
            crate::types::CallFlags::READ_STATES,
            Array::from_vec(vec![
                account.into_any(),
                Int256::from(end_height as i64).into_any(),
            ]),
        )
        .as_int()
        .unwrap_or_else(Int256::zero)
    }

    /// Get unclaimed GAS amount
    pub fn unclaimed_gas(account: H160, end_height: u32) -> Int256 {
        crate::services::contract::Contract::call(
            neo_contract_hash(),
            ByteString::from_literal("unclaimedGas"),
            crate::types::CallFlags::READ_STATES,
            Array::from_vec(vec![
                account.into_any(),
                Int256::from(end_height as i64).into_any(),
            ]),
        )
        .as_int()
        .unwrap_or_else(Int256::zero)
    }
}

/// Account state structure
#[derive(Debug, Clone, Default)]
pub struct AccountState {
    pub balance: Int256,
    pub balance_height: Int256,
    pub vote_to: Option<PublicKey>,
}

/// Candidate information
#[derive(Debug, Clone)]
pub struct CandidateInfo {
    pub pubkey: PublicKey,
    pub votes: Int256,
    pub registered: bool,
}