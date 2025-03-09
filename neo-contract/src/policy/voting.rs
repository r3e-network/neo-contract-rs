// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Voting mechanism for decentralized governance
//! This module provides tools for creating and managing votes and proposals

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// Fix imports to use prelude for all types
use crate::prelude::{H160, ByteString, Int256, Array, Any, StorageMap};
use crate::runtime::Runtime;
use crate::error::{Error, ErrorCode, Result};

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
        let value = data.as_string();
        match value.as_str() {
            "yes" => Some(VoteValue::Yes),
            "no" => Some(VoteValue::No),
            "abstain" => Some(VoteValue::Abstain),
            _ => None,
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
        let value = data.as_string();
        match value.as_str() {
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

/// Action to execute if a proposal passes
pub struct ProposalAction {
    /// Contract to call
    contract_hash: H160,
    /// Method to call
    method: ByteString,
    /// Arguments to pass
    args: Array,
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
            return Err(Error::Unauthorized("Proposer must be the caller"));
        }
        
        // Check if proposal already exists
        let proposal_id = ByteString::from(id);
        let proposals_map = self.get_proposals_map();
        
        if proposals_map.get(&proposal_id).is_some() {
            return Err(Error::AlreadyExists("Proposal already exists"));
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
            return Err(Error::Unauthorized("Voter must be the caller"));
        }
        
        // Get the proposal
        let proposal_id_bs = ByteString::from(proposal_id);
        let proposal = self.get_proposal(&proposal_id_bs)?;
        
        // Check if the proposal is in the voting period
        let current_block = self.get_current_block_height();
        
        if current_block < proposal.start_block {
            return Err(Error::InvalidState("Voting has not started yet"));
        }
        
        if current_block > proposal.end_block {
            return Err(Error::InvalidState("Voting has ended"));
        }
        
        // Check if the proposal is still pending
        if proposal.status != ProposalStatus::Pending {
            return Err(Error::InvalidState("Proposal is not pending"));
        }
        
        // Record the vote
        let votes_map = self.get_votes_map(proposal_id);
        votes_map.put(&voter, &vote);
        
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
            return Err(Error::InvalidState("Proposal is not pending"));
        }
        
        // Check if the voting period has ended
        let current_block = self.get_current_block_height();
        
        if current_block <= proposal.end_block {
            return Err(Error::InvalidState("Voting period has not ended"));
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
            return Err(Error::InvalidState("Proposal is not accepted"));
        }
        
        // Execute all actions
        for action in &proposal.actions {
            let result = Runtime::call_contract(
                action.contract_hash.clone(),
                action.method.clone(),
                action.args.clone()
            );
            
            // Check if the call was successful
            let success = bool::try_from(result).unwrap_or(false);
            
            if !success {
                return Err(Error::ContractCallError("Action execution failed"));
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
            return Err(Error::Unauthorized("Only the proposer can cancel"));
        }
        
        // Check if the proposal is still pending
        if proposal.status != ProposalStatus::Pending {
            return Err(Error::InvalidState("Proposal is not pending"));
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
        
        if let Some(proposal) = proposals_map.get(proposal_id) {
            Ok(proposal)
        } else {
            Err(Error::NotFound("Proposal not found"))
        }
    }
    
    /// Helper method to store a proposal
    fn store_proposal(&self, proposal: &Proposal) -> Result<()> {
        let proposals_map = self.get_proposals_map();
        proposals_map.put(&proposal.id, proposal);
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
        
        for item in votes_map.iter() {
            if let Some((voter, vote)) = item {
                let voting_power = self.get_voting_power(&voter);
                
                match vote {
                    VoteValue::Yes => yes_votes += voting_power,
                    VoteValue::No => no_votes += voting_power,
                    VoteValue::Abstain => {}, // Abstentions don't count toward yes or no
                }
                
                total_votes += voting_power;
            }
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
            
            match Int256::try_from(result) {
                Ok(balance) => balance.as_u64().unwrap_or(0),
                Err(_) => 0,
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
            
            match Int256::try_from(result) {
                Ok(supply) => supply.as_u64().unwrap_or(0),
                Err(_) => 0,
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
        
        let result = Runtime::call_contract(
            crate::contract::native::ledger::Ledger::hash(),
            method,
            args
        );
        
        // Extract the height from the block
        let block = result;
        let height_method = ByteString::from("index");
        let height_result = Runtime::call_contract(
            crate::contract::native::ledger::Ledger::hash(),
            height_method,
            Array::new()
        );
        
        match u32::try_from(height_result) {
            Ok(height) => height,
            Err(_) => 0,
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
