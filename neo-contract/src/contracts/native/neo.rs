// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// NEO native contract for Neo N3
/// 
/// NEO is the governance token of the Neo N3 blockchain that represents ownership
/// of the Neo blockchain and gives its holders the right to vote.
/// 
/// Contract Hash: 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
#[allow(non_snake_case)]
pub struct NEO;

impl NEO {
    /// Returns the contract hash for the NEO native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::neo_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_neo_contract_hash()
        }
    }

    /// Gets the symbol of the NEO token
    /// 
    /// # Returns
    /// 
    /// The symbol as a ByteString, which is "NEO"
    #[safe]
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from("NEO"))
    }

    /// Gets the decimals of the NEO token
    /// 
    /// # Returns
    /// 
    /// The number of decimals, which is 0 (NEO is indivisible)
    #[safe]
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets the total supply of NEO tokens
    /// 
    /// # Returns
    /// 
    /// The total supply as an integer
    #[safe]
    pub fn total_supply() -> u64 {
        let method = ByteString::from("totalSupply");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets the balance of NEO for an account
    /// 
    /// # Arguments
    /// 
    /// * `account` - The script hash of the account to check
    /// 
    /// # Returns
    /// 
    /// The account balance as an integer
    #[safe]
    pub fn balance_of(account: &H160) -> u64 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Transfers NEO from the calling contract to another account
    /// 
    /// # Arguments
    /// 
    /// * `from` - The script hash of the sending account
    /// * `to` - The script hash of the receiving account
    /// * `amount` - The amount of NEO to transfer
    /// * `data` - Optional data to include with the transfer
    /// 
    /// # Returns
    /// 
    /// True if the transfer was successful, false otherwise
    pub fn transfer(from: &H160, to: &H160, amount: u64, data: Option<Any>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(from.clone()));
        args.push(Any::from(to.clone()));
        args.push(Any::from(amount));
        
        if let Some(transfer_data) = data {
            args.push(transfer_data);
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Gets the number of votes allocated to a candidate
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - The public key of the candidate in ByteString format
    /// 
    /// # Returns
    /// 
    /// The number of votes
    #[safe]
    pub fn get_candidate_vote(public_key: &ByteString) -> u64 {
        let method = ByteString::from("getCandidateVote");
        let mut args = Array::<Any>::new();
        args.push(Any::from(public_key.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets all candidates with their vote counts
    /// 
    /// # Returns
    /// 
    /// An array of candidate information as Any
    #[safe]
    pub fn get_all_candidates() -> Array<Any> {
        let method = ByteString::from("getAllCandidates");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|| Array::<Any>::new())
    }

    /// Registers a candidate for consensus node
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - The public key of the candidate in ByteString format
    /// 
    /// # Returns
    /// 
    /// True if registration was successful, false otherwise
    pub fn register_candidate(public_key: &ByteString) -> bool {
        let method = ByteString::from("registerCandidate");
        let mut args = Array::<Any>::new();
        args.push(Any::from(public_key.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Unregisters a candidate from consensus
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - The public key of the candidate in ByteString format
    /// 
    /// # Returns
    /// 
    /// True if unregistration was successful, false otherwise
    pub fn unregister_candidate(public_key: &ByteString) -> bool {
        let method = ByteString::from("unregisterCandidate");
        let mut args = Array::<Any>::new();
        args.push(Any::from(public_key.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Votes for candidates using NEO from an account
    /// 
    /// # Arguments
    /// 
    /// * `account` - The script hash of the voting account
    /// * `candidates` - An array of candidate public keys to vote for
    /// 
    /// # Returns
    /// 
    /// True if voting was successful, false otherwise
    pub fn vote(account: &H160, candidates: &Array<ByteString>) -> bool {
        let method = ByteString::from("vote");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account.clone()));
        
        // Convert Array<ByteString> to Array<Any>
        let candidates_any: Array<Any> = candidates.iter().map(|c| Any::from(c.clone())).collect();
        args.push(Any::from(candidates_any));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }

    /// Gets NEO balance that can be claimed for GAS
    /// 
    /// # Arguments
    /// 
    /// * `account` - The script hash of the account to check
    /// 
    /// # Returns
    /// 
    /// The amount of unclaimed GAS
    #[safe]
    pub fn unclaimed_gas(account: &H160) -> u64 {
        let method = ByteString::from("unclaimedGas");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }
}
