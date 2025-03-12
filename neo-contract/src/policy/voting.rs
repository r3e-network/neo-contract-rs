// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Voting mechanism for decentralized governance
//! This module provides tools for creating and managing votes and proposals

use alloc::format;

use alloc::vec::Vec;
use alloc::vec;

// Fix imports to use prelude for all types

use crate::prelude::{H160, ByteString, Array, Any, StorageMap};
use crate::runtime::Runtime;
use crate::error::{Error, ErrorCode, Result};
use crate::storage::item::Codec;

// Define Storable trait here since it's not accessible from storage
pub trait Storable {
    fn to_storage(&self) -> ByteString;
    fn from_storage(data: &ByteString) -> Option<Self> where Self: Sized;
}

/// Represents a vote value
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoteValue {
    /// Vote in favor
    Yes,
    /// Vote against
    No,
    /// Abstain from voting
    Abstain,
}

impl Storable for VoteValue {
    fn to_storage(&self) -> ByteString {
        match self {
            VoteValue::Yes => ByteString::from("yes"),
            VoteValue::No => ByteString::from("no"),
            VoteValue::Abstain => ByteString::from("abstain"),
        }
    }
    
    fn from_storage(data: &ByteString) -> Option<Self> {
        let value = unsafe { core::str::from_utf8_unchecked(data.as_bytes()) };
        match value {
            "yes" => Some(VoteValue::Yes),
            "no" => Some(VoteValue::No),
            "abstain" => Some(VoteValue::Abstain),
            _ => None,
        }
    }
}

impl Codec for VoteValue {
    fn encode(&self) -> Vec<u8> {
        match self {
            VoteValue::Yes => vec![1],
            VoteValue::No => vec![2],
            VoteValue::Abstain => vec![3],
        }
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 1 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        match bytes[0] {
            1 => Ok(VoteValue::Yes),
            2 => Ok(VoteValue::No),
            3 => Ok(VoteValue::Abstain),
            _ => Err(Error::new(ErrorCode::InvalidFormat)),
        }
    }
}

/// Status of a proposal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProposalStatus {
    /// Proposal is pending and can be voted on
    Pending,
    /// Proposal has been accepted
    Accepted,
    /// Proposal has been rejected
    Rejected,
    /// Proposal has been canceled
    Canceled,
    /// Proposal is in queue for execution
    Queued,
    /// Proposal has been executed
    Executed,
    /// Proposal has expired
    Expired,
}

impl Storable for ProposalStatus {
    fn to_storage(&self) -> ByteString {
        match self {
            ProposalStatus::Pending => ByteString::from("pending"),
            ProposalStatus::Accepted => ByteString::from("accepted"),
            ProposalStatus::Rejected => ByteString::from("rejected"),
            ProposalStatus::Canceled => ByteString::from("canceled"),
            ProposalStatus::Queued => ByteString::from("queued"),
            ProposalStatus::Executed => ByteString::from("executed"),
            ProposalStatus::Expired => ByteString::from("expired"),
        }
    }
    
    fn from_storage(data: &ByteString) -> Option<Self> {
        let value = unsafe { core::str::from_utf8_unchecked(data.as_bytes()) };
        match value {
            "pending" => Some(ProposalStatus::Pending),
            "accepted" => Some(ProposalStatus::Accepted),
            "rejected" => Some(ProposalStatus::Rejected),
            "canceled" => Some(ProposalStatus::Canceled),
            "queued" => Some(ProposalStatus::Queued),
            "executed" => Some(ProposalStatus::Executed),
            "expired" => Some(ProposalStatus::Expired),
            _ => None,
        }
    }
}

impl Codec for ProposalStatus {
    fn encode(&self) -> Vec<u8> {
        match self {
            ProposalStatus::Pending => vec![1],
            ProposalStatus::Accepted => vec![2],
            ProposalStatus::Rejected => vec![3],
            ProposalStatus::Canceled => vec![4],
            ProposalStatus::Queued => vec![5],
            ProposalStatus::Executed => vec![6],
            ProposalStatus::Expired => vec![7],
        }
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 1 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        match bytes[0] {
            1 => Ok(ProposalStatus::Pending),
            2 => Ok(ProposalStatus::Accepted),
            3 => Ok(ProposalStatus::Rejected),
            4 => Ok(ProposalStatus::Canceled),
            5 => Ok(ProposalStatus::Queued),
            6 => Ok(ProposalStatus::Executed),
            7 => Ok(ProposalStatus::Expired),
            _ => Err(Error::new(ErrorCode::InvalidFormat)),
        }
    }
}

/// A proposal that can be voted on
pub struct Proposal {
    /// Unique identifier for the proposal
    id: ByteString,
    /// Description of the proposal
    description: ByteString,
    /// Script hash of the proposer
    proposer: H160,
    /// Start time for voting (in block height)
    start_block: u32,
    /// End time for voting (in block height)
    end_block: u32,
    /// Quorum required for the proposal to pass (percentage)
    quorum: u8,
    /// Contract calls to execute if the proposal passes
    actions: Vec<ProposalAction>,
    /// Status of the proposal
    status: ProposalStatus,
}

impl Codec for Proposal {
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Encode the ID
        let id_bytes = self.id.encode();
        let id_len = id_bytes.len() as u32;
        result.extend_from_slice(&id_len.to_le_bytes());
        result.extend_from_slice(&id_bytes);
        
        // Encode the description
        let desc_bytes = self.description.encode();
        let desc_len = desc_bytes.len() as u32;
        result.extend_from_slice(&desc_len.to_le_bytes());
        result.extend_from_slice(&desc_bytes);
        
        // Encode the proposer
        let proposer_bytes = self.proposer.encode();
        result.extend_from_slice(&proposer_bytes);
        
        // Encode blocks and quorum
        result.extend_from_slice(&self.start_block.to_le_bytes());
        result.extend_from_slice(&self.end_block.to_le_bytes());
        result.push(self.quorum);
        
        // Encode actions
        let actions_len = self.actions.len() as u32;
        result.extend_from_slice(&actions_len.to_le_bytes());
        
        for action in &self.actions {
            let action_bytes = action.encode();
            let action_len = action_bytes.len() as u32;
            result.extend_from_slice(&action_len.to_le_bytes());
            result.extend_from_slice(&action_bytes);
        }
        
        // Encode status
        let status_bytes = self.status.encode();
        result.extend_from_slice(&status_bytes);
        
        result
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let mut pos = 0;
        
        // Decode ID
        let mut id_len_bytes = [0u8; 4];
        id_len_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let id_len = u32::from_le_bytes(id_len_bytes) as usize;
        
        if pos + id_len > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let id = ByteString::decode(&bytes[pos..pos+id_len])?;
        pos += id_len;
        
        // Decode description
        if pos + 4 > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let mut desc_len_bytes = [0u8; 4];
        desc_len_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let desc_len = u32::from_le_bytes(desc_len_bytes) as usize;
        
        if pos + desc_len > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let description = ByteString::decode(&bytes[pos..pos+desc_len])?;
        pos += desc_len;
        
        // Decode proposer
        if pos + 20 > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let proposer = H160::decode(&bytes[pos..pos+20])?;
        pos += 20;
        
        // Decode block heights and quorum
        if pos + 9 > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let mut start_block_bytes = [0u8; 4];
        start_block_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let start_block = u32::from_le_bytes(start_block_bytes);
        
        let mut end_block_bytes = [0u8; 4];
        end_block_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let end_block = u32::from_le_bytes(end_block_bytes);
        
        let quorum = bytes[pos];
        pos += 1;
        
        // Decode actions
        if pos + 4 > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let mut actions_len_bytes = [0u8; 4];
        actions_len_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let actions_len = u32::from_le_bytes(actions_len_bytes) as usize;
        
        let mut actions = Vec::with_capacity(actions_len);
        
        for _ in 0..actions_len {
            if pos + 4 > bytes.len() {
                return Err(Error::new(ErrorCode::InvalidFormat));
            }
            
            let mut action_len_bytes = [0u8; 4];
            action_len_bytes.copy_from_slice(&bytes[pos..pos+4]);
            pos += 4;
            let action_len = u32::from_le_bytes(action_len_bytes) as usize;
            
            if pos + action_len > bytes.len() {
                return Err(Error::new(ErrorCode::InvalidFormat));
            }
            
            let action = ProposalAction::decode(&bytes[pos..pos+action_len])?;
            pos += action_len;
            
            actions.push(action);
        }
        
        // Decode status
        if pos + 1 > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let status = ProposalStatus::decode(&bytes[pos..pos+1])?;
        
        Ok(Proposal {
            id,
            description,
            proposer,
            start_block,
            end_block,
            quorum,
            actions,
            status,
        })
    }
}

/// Action to execute if a proposal passes
pub struct ProposalAction {
    /// Contract to call
    contract_hash: H160,
    /// Method to call
    method: ByteString,
    /// Arguments to pass
    args: Array,
}

impl Codec for ProposalAction {
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        // Encode contract hash
        let hash_bytes = self.contract_hash.encode();
        result.extend_from_slice(&hash_bytes);
        
        // Encode method
        let method_bytes = self.method.encode();
        let method_len = method_bytes.len() as u32;
        result.extend_from_slice(&method_len.to_le_bytes());
        result.extend_from_slice(&method_bytes);
        
        // Encode args (Array already has its own encoding)
        let args_bytes = self.args.encode();
        result.extend_from_slice(&args_bytes);
        
        result
    }
    
    fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 24 { // 20 (H160) + 4 (method length)
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let mut pos = 0;
        
        // Decode contract hash
        let contract_hash = H160::decode(&bytes[pos..pos+20])?;
        pos += 20;
        
        // Decode method
        let mut method_len_bytes = [0u8; 4];
        method_len_bytes.copy_from_slice(&bytes[pos..pos+4]);
        pos += 4;
        let method_len = u32::from_le_bytes(method_len_bytes) as usize;
        
        if pos + method_len > bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let method = ByteString::decode(&bytes[pos..pos+method_len])?;
        pos += method_len;
        
        // Decode args
        if pos >= bytes.len() {
            return Err(Error::new(ErrorCode::InvalidFormat));
        }
        
        let args = Array::decode(&bytes[pos..])?;
        
        Ok(ProposalAction {
            contract_hash,
            method,
            args,
        })
    }
}

/// Voting system for governance
pub struct VotingSystem {
    /// Prefix for storage
    prefix: ByteString,
    /// Token used for voting power
    voting_token: Option<H160>,
}

impl VotingSystem {
    /// Create a new voting system
    pub fn new(prefix: &[u8], voting_token: Option<H160>) -> Self {
        Self {
            prefix: ByteString::from_bytes(prefix),
            voting_token,
        }
    }
    
    /// Create a new proposal
    pub fn create_proposal(
        &self, 
        id: &str, 
        description: &str, 
        proposer: H160, 
        start_block: u32, 
        end_block: u32, 
        quorum: u8, 
        actions: Vec<ProposalAction>
    ) -> Result<()> {
        // Check if caller is the proposer
        if !Runtime::check_witness(&proposer) {
            return Err(Error::with_message(ErrorCode::Unauthorized, "Proposer must be the caller"));
        }

        // Check if proposal already exists
        let proposal_id = ByteString::from(id);
        let proposals_map = self.get_proposals_map();
        
        if proposals_map.get(&proposal_id).unwrap_or_default().is_some() {
            return Err(Error::with_message(ErrorCode::InvalidState, "Proposal already exists"));
        }
        
        // Create the proposal
        let proposal = Proposal {
            id: proposal_id.clone(),
            description: ByteString::from(description),
            proposer,
            start_block,
            end_block,
            quorum,
            actions,
            status: ProposalStatus::Pending,
        };
        
        // Store the proposal
        self.store_proposal(&proposal)?;
        
        // Emit the proposal created event
        self.emit_proposal_created(&proposal);
        
        Ok(())
    }
    
    /// Vote on a proposal
    pub fn vote(&self, proposal_id: &str, voter: H160, vote: VoteValue) -> Result<()> {
        // Check if caller is the voter
        if !Runtime::check_witness(&voter) {
            return Err(Error::with_message(ErrorCode::PermissionDenied, "Voter must be the caller"));
        }
        
        // Get the proposal
        let proposal_id_bs = ByteString::from(proposal_id);
        let proposal = self.get_proposal(&proposal_id_bs)?;
        
        // Check if the proposal is in the voting period
        let current_block = self.get_current_block_height();
        
        if current_block < proposal.start_block {
            return Err(Error::with_message(ErrorCode::InvalidState, "Voting has not started yet"));
        }
        
        if current_block > proposal.end_block {
            return Err(Error::with_message(ErrorCode::InvalidState, "Voting has ended"));
        }
        
        // Check if the proposal is still pending
        if proposal.status != ProposalStatus::Pending {
            return Err(Error::with_message(ErrorCode::InvalidState, "Proposal is not pending"));
        }
        
        // Record the vote
        let votes_map = self.get_votes_map(proposal_id);
        let _ = votes_map.put(&voter, &vote);
        
        // Emit the vote cast event
        self.emit_vote_cast(proposal_id, voter, vote);
        
        Ok(())
    }
    
    /// Check if a proposal has passed
    pub fn check_proposal(&self, proposal_id: &str) -> Result<()> {
        let proposal_id_bs = ByteString::from(proposal_id);
        let mut proposal = self.get_proposal(&proposal_id_bs)?;
        
        // Check if the proposal is still pending
        if proposal.status != ProposalStatus::Pending {
            return Err(Error::with_message(ErrorCode::InvalidState, "Proposal is not pending"));
        }
        
        // Check if the voting period has ended
        let current_block = self.get_current_block_height();
        
        if current_block <= proposal.end_block {
            return Err(Error::with_message(ErrorCode::InvalidState, "Voting period has not ended"));
        }
        
        // Calculate the results
        let (yes_votes, no_votes, total_votes) = self.count_votes(proposal_id);
        
        // Check if quorum was reached
        let quorum_required = (self.get_total_voting_power() * u64::from(proposal.quorum)) / 100;
        
        if total_votes < quorum_required {
            // Not enough votes, mark as rejected
            proposal.status = ProposalStatus::Rejected;
            self.store_proposal(&proposal)?;
            self.emit_proposal_finished(&proposal, false);
            return Ok(());
        }
        
        // Check if the proposal passed
        let passed = yes_votes > no_votes;
        
        // Update the proposal status
        proposal.status = if passed {
            ProposalStatus::Accepted
        } else {
            ProposalStatus::Rejected
        };
        
        self.store_proposal(&proposal)?;
        self.emit_proposal_finished(&proposal, passed);
        
        Ok(())
    }
    
    /// Execute a proposal that has been accepted
    pub fn execute_proposal(&self, proposal_id: &str) -> Result<()> {
        let proposal_id_bs = ByteString::from(proposal_id);
        let mut proposal = self.get_proposal(&proposal_id_bs)?;
        
        // Check if the proposal is accepted
        if proposal.status != ProposalStatus::Accepted {
            return Err(Error::with_message(ErrorCode::InvalidState, "Proposal is not accepted"));
        }
        
        // Execute all actions
        for action in &proposal.actions {
            let result = Runtime::call_contract(
                action.contract_hash.clone(),
                action.method.clone(),
                action.args.clone()
            );
            
            // Check if the call was successful
            let success = match result {
                Any::Boolean(b) => b,
                _ => false
            };
            
            if !success {
                return Err(Error::with_message(ErrorCode::ContractError, "Action execution failed"));
            }
        }
        
        // Update the proposal status
        proposal.status = ProposalStatus::Executed;
        self.store_proposal(&proposal)?;
        
        // Emit the proposal executed event
        self.emit_proposal_executed(&proposal);
        
        Ok(())
    }
    
    /// Cancel a proposal
    pub fn cancel_proposal(&self, proposal_id: &str) -> Result<()> {
        let proposal_id_bs = ByteString::from(proposal_id);
        let mut proposal = self.get_proposal(&proposal_id_bs)?;
        
        // Check if the caller is the proposer
        if !Runtime::check_witness(&proposal.proposer) {
            return Err(Error::with_message(ErrorCode::PermissionDenied, "Only the proposer can cancel"));
        }
        
        // Check if the proposal is still pending
        if proposal.status != ProposalStatus::Pending {
            return Err(Error::with_message(ErrorCode::InvalidState, "Proposal is not pending"));
        }
        
        // Update the proposal status
        proposal.status = ProposalStatus::Canceled;
        self.store_proposal(&proposal)?;
        
        // Emit the proposal canceled event
        self.emit_proposal_canceled(&proposal);
        
        Ok(())
    }
    
    /// Get a proposal by ID
    pub fn get_proposal(&self, proposal_id: &ByteString) -> Result<Proposal> {
        let proposals_map = self.get_proposals_map();
        
        if let Some(proposal) = proposals_map.get(proposal_id).unwrap_or_default() {
            Ok(proposal)
        } else {
            Err(Error::with_message(ErrorCode::NotFound, "Proposal not found"))
        }
    }
    
    /// Helper method to store a proposal
    fn store_proposal(&self, proposal: &Proposal) -> Result<()> {
        let proposals_map = self.get_proposals_map();
        let _ = proposals_map.put(&proposal.id, proposal);
        Ok(())
    }
    
    /// Helper method to get the proposals map
    fn get_proposals_map(&self) -> StorageMap<ByteString, Proposal> {
        let key = format!("{}:proposals", self.prefix);
        StorageMap::new(key.as_bytes())
    }
    
    /// Helper method to get the votes map
    fn get_votes_map(&self, proposal_id: &str) -> StorageMap<H160, VoteValue> {
        let key = format!("{}:votes:{}", self.prefix, proposal_id);
        StorageMap::new(key.as_bytes())
    }
    
    /// Helper method to count votes for a proposal
    fn count_votes(&self, proposal_id: &str) -> (u64, u64, u64) {
        let votes_map = self.get_votes_map(proposal_id);
        let mut yes_votes = 0;
        let mut no_votes = 0;
        let mut total_votes = 0;
        
        // votes_map.iter() returns a Vec<(H160, VoteValue)> directly, not an iterator over Result
        let vote_pairs = votes_map.iter();
        
        for (voter, vote) in vote_pairs {
            let voting_power = self.get_voting_power(&voter);
            
            match vote {
                VoteValue::Yes => yes_votes += voting_power,
                VoteValue::No => no_votes += voting_power,
                VoteValue::Abstain => {}, // Abstentions don't count toward yes or no
            }
            
            total_votes += voting_power;
        }
        
        (yes_votes, no_votes, total_votes)
    }
    
    /// Get the voting power of an address
    fn get_voting_power(&self, address: &H160) -> u64 {
        if let Some(token_hash) = self.voting_token {
            // Call the token contract to get the balance
            let method = ByteString::from("balanceOf");
            let mut args = Array::new();
            args.push(Any::from(address.clone()));
            
            let result = Runtime::call_contract(
                token_hash,
                method,
                args
            );
            
            match result {
                Any::Integer(balance) => balance.as_u64().unwrap_or(0),
                _ => 0,
            }
        } else {
            // If no voting token is set, each address has 1 vote
            1
        }
    }
    
    /// Get the total voting power
    fn get_total_voting_power(&self) -> u64 {
        if let Some(token_hash) = self.voting_token {
            // Call the token contract to get the total supply
            let method = ByteString::from("totalSupply");
            let args = Array::new();
            
            let result = Runtime::call_contract(
                token_hash,
                method,
                args
            );
            
            match result {
                Any::Integer(supply) => supply.as_u64().unwrap_or(0),
                _ => 0,
            }
        } else {
            // If no voting token is set, use a reasonable default
            100_000_000
        }
    }
    
    /// Get the current block height
    fn get_current_block_height(&self) -> u32 {
        let block_hash = Runtime::current_block_hash();
        
        // Call the Ledger contract to get the block
        let method = ByteString::from("getBlock");
        let mut args = Array::new();
        args.push(Any::from(block_hash));
        
        let _result = Runtime::call_contract(
            crate::contract::native::ledger::Ledger::hash(),
            method,
            args
        );
        
        // We don't need to use the block result directly, just get the index
        let height_method = ByteString::from("index");
        let height_result = Runtime::call_contract(
            crate::contract::native::ledger::Ledger::hash(),
            height_method,
            Array::new()
        );
        
        match height_result {
            Any::Integer(int_value) => int_value.as_u32().unwrap_or(0),
            _ => 0,
        }
    }
    
    /// Emit a proposal created event
    fn emit_proposal_created(&self, proposal: &Proposal) {
        let event_name = ByteString::from("ProposalCreated");
        let mut event_data = Array::new();
        
        event_data.push(Any::from(proposal.id.clone()));
        event_data.push(Any::from(proposal.description.clone()));
        event_data.push(Any::from(proposal.proposer.clone()));
        event_data.push(Any::from(proposal.start_block));
        event_data.push(Any::from(proposal.end_block));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    /// Emit a vote cast event
    fn emit_vote_cast(&self, proposal_id: &str, voter: H160, vote: VoteValue) {
        let event_name = ByteString::from("VoteCast");
        let mut event_data = Array::new();
        
        event_data.push(Any::from(ByteString::from(proposal_id)));
        event_data.push(Any::from(voter));
        event_data.push(Any::from(vote.to_storage()));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    /// Emit a proposal finished event
    fn emit_proposal_finished(&self, proposal: &Proposal, passed: bool) {
        let event_name = ByteString::from("ProposalFinished");
        let mut event_data = Array::new();
        
        event_data.push(Any::from(proposal.id.clone()));
        event_data.push(Any::from(if passed { 1 } else { 0 }));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    /// Emit a proposal executed event
    fn emit_proposal_executed(&self, proposal: &Proposal) {
        let event_name = ByteString::from("ProposalExecuted");
        let mut event_data = Array::new();
        
        event_data.push(Any::from(proposal.id.clone()));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    /// Emit a proposal canceled event
    fn emit_proposal_canceled(&self, proposal: &Proposal) {
        let event_name = ByteString::from("ProposalCanceled");
        let mut event_data = Array::new();
        
        event_data.push(Any::from(proposal.id.clone()));
        
        Runtime::notify(&event_name, &event_data);
    }
}
