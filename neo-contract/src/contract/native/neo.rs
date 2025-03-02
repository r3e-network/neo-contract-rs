// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use alloc::string::String;
use alloc::vec::Vec;
use crate::builtin::{H160, ByteString, Int256, Array, Any};
use crate::Runtime;

/// NEO native contract
pub struct Neo;

/// NEO script hash
pub const SCRIPT_HASH: H160 = H160([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

impl Neo {
    /// Get the NEO contract hash
    pub fn hash() -> H160 {
        SCRIPT_HASH
    }
    
    /// Transfer NEO from one account to another
    pub fn transfer(from: H160, to: H160, amount: Int256, data: Option<ByteString>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        
        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        if let Some(data_bs) = data {
            args.push(Any::from(data_bs));
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Register a candidate for consensus
    pub fn register_candidate(candidate_key: ByteString) -> bool {
        let method = ByteString::from("registerCandidate");
        let mut args = Array::<Any>::new();
        args.push(Any::from(candidate_key));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Unregister a candidate
    pub fn unregister_candidate(candidate_key: ByteString) -> bool {
        let method = ByteString::from("unregisterCandidate");
        let mut args = Array::<Any>::new();
        args.push(Any::from(candidate_key));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Vote for a candidate
    pub fn vote(account: H160, candidate_key: Option<ByteString>) -> bool {
        let method = ByteString::from("vote");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account));
        
        if let Some(candidate) = candidate_key {
            args.push(Any::from(candidate));
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get all candidates
    pub fn get_all_candidates() -> Array<Any> {
        let method = ByteString::from("getAllCandidates");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Array::<Any>::try_from(result) {
            Ok(candidates) => candidates,
            Err(_) => Array::<Any>::new(),
        }
    }
    
    /// Get candidates
    pub fn get_candidates() -> Array<Any> {
        let method = ByteString::from("getCandidates");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Array::<Any>::try_from(result) {
            Ok(candidates) => candidates,
            Err(_) => Array::<Any>::new(),
        }
    }
    
    /// Get committee
    pub fn get_committee() -> Array<ByteString> {
        let method = ByteString::from("getCommittee");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Array::<ByteString>::try_from(result) {
            Ok(committee) => committee,
            Err(_) => Array::<ByteString>::new(),
        }
    }
    
    /// Get next validators
    pub fn get_next_block_validators() -> Array<ByteString> {
        let method = ByteString::from("getNextBlockValidators");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Array::<ByteString>::try_from(result) {
            Ok(validators) => validators,
            Err(_) => Array::<ByteString>::new(),
        }
    }
    
    /// Get candidate voter count
    pub fn get_candidate_vote(candidate_key: ByteString) -> Int256 {
        let method = ByteString::from("getCandidateVote");
        let mut args = Array::<Any>::new();
        args.push(Any::from(candidate_key));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(vote_count) => vote_count,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Get account that a voter has voted for
    pub fn get_candidate_by_voter(voter: H160) -> Option<ByteString> {
        let method = ByteString::from("getVoterCandidates");
        let mut args = Array::<Any>::new();
        args.push(Any::from(voter));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match ByteString::try_from(result) {
            Ok(candidate) => Some(candidate),
            Err(_) => None,
        }
    }
    
    /// Get unclaimed GAS for an account
    pub fn get_unclaimed_gas(account: H160) -> Int256 {
        let method = ByteString::from("unclaimedGas");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account));
        args.push(Any::from(Runtime::get_time()));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(gas) => gas,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Get symbol of the NEO token
    #[safe]
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match ByteString::try_from(result) {
            Ok(symbol) => symbol,
            Err(_) => ByteString::from("NEO"),
        }
    }
    
    /// Get decimals of the NEO token
    #[safe]
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match u8::try_from(result) {
            Ok(decimals) => decimals,
            Err(_) => 0, // NEO has 0 decimals (integer)
        }
    }
    
    /// Get total supply of the NEO token
    #[safe]
    pub fn total_supply() -> Int256 {
        let method = ByteString::from("totalSupply");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(supply) => supply,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Get balance of NEO for an account
    #[safe]
    pub fn balance_of(account: H160) -> Int256 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account));
        
        let result = Runtime::call_contract(
            Neo::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(balance) => balance,
            Err(_) => Int256::zero(),
        }
    }
}
