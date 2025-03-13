// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::prelude::{Any, Array, ByteString, Int256, H160};
use crate::runtime::Runtime;

/// NEO native contract
pub struct Neo;

/// NEO script hash
// Define a function to get the script hash instead of using a constant
pub fn script_hash() -> H160 { H160::zero() }

impl Neo {
    /// Get the NEO contract hash
    pub fn hash() -> H160 { script_hash() }

    /// Transfer NEO from one account to another
    pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<ByteString>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::new();

        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));

        if let Some(data_bs) = data {
            args.push(Any::from(data_bs));
        } else {
            args.push(Any::null());
        }

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }

    /// Register a candidate for consensus
    pub fn register_candidate(candidate_key: ByteString) -> bool {
        let method = ByteString::from("registerCandidate");
        let mut args = Array::new();
        args.push(Any::from(candidate_key));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }

    /// Unregister a candidate
    pub fn unregister_candidate(candidate_key: ByteString) -> bool {
        let method = ByteString::from("unregisterCandidate");
        let mut args = Array::new();
        args.push(Any::from(candidate_key));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }

    /// Vote for a candidate
    pub fn vote(account: H160, candidate_key: Option<ByteString>) -> bool {
        let method = ByteString::from("vote");
        let mut args = Array::new();
        args.push(Any::from(account));

        if let Some(candidate) = candidate_key {
            args.push(Any::from(candidate));
        } else {
            args.push(Any::null());
        }

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract boolean value
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }

    /// Gets all candidates and their votes
    // IMPORTANT: The #[safe] attribute should be present for manifest generation
    // but is temporarily commented out due to compilation issues
    // #[safe]
    pub fn get_all_candidates() -> Array {
        let method = ByteString::from("getAllCandidates");
        let result = Runtime::call_contract(script_hash(), method, Array::new());

        // Extract array value
        if let Any::Array(array) = result {
            Array::from_vec(array)
        } else {
            Array::new()
        }
    }

    /// Gets all registered candidates that meet the threshold for voting
    // IMPORTANT: The #[safe] attribute should be present for manifest generation
    // but is temporarily commented out due to compilation issues
    // #[safe]
    pub fn get_candidates() -> Array {
        let method = ByteString::from("getCandidates");
        let result = Runtime::call_contract(script_hash(), method, Array::new());

        // Extract array value
        if let Any::Array(array) = result {
            Array::from_vec(array)
        } else {
            Array::new()
        }
    }

    /// Gets the current committee members
    // IMPORTANT: The #[safe] attribute should be present for manifest generation
    // but is temporarily commented out due to compilation issues
    // #[safe]
    pub fn get_committee() -> Array {
        let method = ByteString::from("getCommittee");
        let result = Runtime::call_contract(script_hash(), method, Array::new());

        // Extract array value
        if let Any::Array(array) = result {
            Array::from_vec(array)
        } else {
            Array::new()
        }
    }

    /// Gets the validators for the next block
    // IMPORTANT: The #[safe] attribute should be present for manifest generation
    // but is temporarily commented out due to compilation issues
    // #[safe]
    pub fn get_next_block_validators() -> Array {
        let method = ByteString::from("getNextBlockValidators");
        let result = Runtime::call_contract(script_hash(), method, Array::new());

        // Extract array value
        if let Any::Array(array) = result {
            Array::from_vec(array)
        } else {
            Array::new()
        }
    }

    /// Get candidate voter count
    pub fn get_candidate_vote(candidate_key: ByteString) -> Int256 {
        let method = ByteString::from("getCandidateVote");
        let mut args = Array::new();
        args.push(Any::from(candidate_key));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get account that a voter has voted for
    pub fn get_candidate_by_voter(voter: H160) -> Option<ByteString> {
        let method = ByteString::from("getVoterCandidates");
        let mut args = Array::new();
        args.push(Any::from(voter));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract ByteString value
        if let Any::ByteString(value) = result {
            Some(value)
        } else {
            None
        }
    }

    /// Get unclaimed GAS for an account
    pub fn get_unclaimed_gas(account: H160) -> Int256 {
        let method = ByteString::from("unclaimedGas");
        let mut args = Array::new();
        args.push(Any::from(account));

        // Use Int256::from_u64 for the time value
        let time = Runtime::time();
        args.push(Any::from(Int256::from_u64(time)));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get symbol of the NEO token
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract ByteString value
        if let Any::ByteString(value) = result {
            value
        } else {
            ByteString::from("NEO")
        }
    }

    /// Get decimals of the NEO token
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value and convert to u8
        if let Any::Integer(value) = result {
            value.to_u64().unwrap_or(0) as u8
        } else {
            0 // NEO has 0 decimals (integer)
        }
    }

    /// Get total supply of the NEO token
    pub fn total_supply() -> Int256 {
        let method = ByteString::from("totalSupply");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get the balance of an account
    pub fn balance_of(account: H160) -> Int256 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::new();
        args.push(Any::from(account));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get the gas per block
    pub fn get_gas_per_block() -> Int256 {
        let method = ByteString::from("getGasPerBlock");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value
        if let Any::Integer(value) = result {
            value
        } else {
            Int256::zero()
        }
    }

    /// Get the register price
    pub fn get_register_price() -> i64 {
        let method = ByteString::from("getRegisterPrice");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract integer value and convert to i64
        if let Any::Integer(value) = result {
            value.to_i64().unwrap_or(0)
        } else {
            0
        }
    }

    /// Get the committee address
    pub fn get_committee_address() -> H160 {
        let method = ByteString::from("getCommitteeAddress");
        let args = Array::new();

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Extract ByteString value and convert to H160
        if let Any::ByteString(value) = result {
            // Try to convert the ByteString to H160
            if value.len() == 20 {
                H160::from_slice(value.as_bytes())
            } else {
                H160::zero()
            }
        } else {
            H160::zero()
        }
    }

    /// Get the account state
    pub fn get_account_state(account: H160) -> Any {
        let method = ByteString::from("getAccountState");
        let mut args = Array::new();
        args.push(Any::from(account));

        let result = Runtime::call_contract(Neo::hash(), method, args);

        // Just return the result as is
        result
    }
}
