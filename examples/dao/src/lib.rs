#![no_std]
#![allow(unused_imports)]

extern crate alloc;

// Import the primary types needed at the top level
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use neo_contract::prelude::*;
use neo_contract::Runtime;
// Import the specific functions needed by the contract macro
use neo_contract::runtime::{__neo_deploy_entry, __neo_invoke_entry};
use neo_contract::types::storage::StorageItem;
// Import the macros directly from neo_macros
use neo_macros::{contract, contract_author, contract_description, contract_version, supported_standards, event, index, storage, constructor, method, safe, no_reentrant};

/// Event emitted when a member is added to the DAO
#[event]
pub struct MemberAdded {
    #[index]
    pub address: H160,
}

/// Event emitted when a proposal is created
#[event]
pub struct ProposalCreated {
    #[index]
    pub proposal_id: u32,
    pub title: String,
    pub description: String,
    pub creator: H160,
    pub created_at: u64,
}

/// Event emitted when a vote is cast
#[event]
pub struct VoteCast {
    #[index]
    pub proposal_id: u32,
    #[index]
    pub voter: H160,
    pub vote_for: bool,
}

/// Event emitted when a proposal is executed
#[event]
pub struct ProposalExecuted {
    #[index]
    pub proposal_id: u32,
    pub executor: H160,
}

/// Event emitted when a member is removed from the DAO
#[event]
pub struct MemberRemoved {
    #[index]
    pub address: H160,
}

/// Event emitted when ownership is transferred
#[event]
pub struct OwnershipTransferred {
    #[index]
    pub previous_owner: H160,
    #[index]
    pub new_owner: H160,
}

/// Decentralized Autonomous Organization (DAO) Example
/// This contract demonstrates a simple governance system where members can
/// create and vote on proposals.
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Decentralized Autonomous Organization for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
pub struct DAOContract {
    /// Number of proposals created
    #[storage]
    proposal_count: StorageItem<u32>,
    
    /// Mapping of proposal ID to Proposal details
    #[storage]
    proposals_title: StorageMap<u32, String>,
    
    #[storage]
    proposals_description: StorageMap<u32, String>,
    
    #[storage]
    proposals_creator: StorageMap<u32, H160>,
    
    #[storage]
    proposals_votes_for: StorageMap<u32, u32>,
    
    #[storage]
    proposals_votes_against: StorageMap<u32, u32>,
    
    #[storage]
    proposals_end_time: StorageMap<u32, u64>,
    
    #[storage]
    proposals_state: StorageMap<u32, u8>, // 0=Active, 1=Passed, 2=Rejected, 3=Executed
    
    /// Mapping of (proposal ID, voter address) to Vote
    #[storage]
    votes: StorageMap<Vec<u8>, bool>, // true=For, false=Against
    
    /// Mapping of address to whether they are a member
    #[storage]
        members: StorageMap<H160, bool>,
    
        /// DAO owner with admin rights
    #[storage]
    owner: StorageItem<H160>,
    
        /// Default voting period in seconds
    #[storage]
    voting_period: StorageItem<u64>,
    
        /// Minimum votes needed to pass a proposal
    #[storage]
    min_votes_to_pass: StorageItem<u32>,
    }

impl DAOContract {
        /// Create a new DAO contract
        #[constructor]
    pub fn new(voting_period_hours: u32, min_votes_to_pass: u32) -> Self {
        let sender = Runtime::calling_script_hash();
        
        let mut instance = Self {
            proposal_count: StorageItem::new(b"proposal_count"),
            proposals_title: StorageMap::new(b"proposals_title"),
            proposals_description: StorageMap::new(b"proposals_description"),
            proposals_creator: StorageMap::new(b"proposals_creator"),
            proposals_votes_for: StorageMap::new(b"proposals_votes_for"),
            proposals_votes_against: StorageMap::new(b"proposals_votes_against"),
            proposals_end_time: StorageMap::new(b"proposals_end_time"),
            proposals_state: StorageMap::new(b"proposals_state"),
                votes: StorageMap::new(b"votes"),
                members: StorageMap::new(b"members"),
            owner: StorageItem::new(b"owner"),
            voting_period: StorageItem::new(b"voting_period"),
            min_votes_to_pass: StorageItem::new(b"min_votes_to_pass"),
        };
        
        // Initialize with defaults
        instance.proposal_count.set(&0);
        instance.owner.set(&sender);
        
        // Convert hours to seconds
        let period_seconds = voting_period_hours as u64 * 3600;
        instance.voting_period.set(&period_seconds);
        
        instance.min_votes_to_pass.set(&min_votes_to_pass);
        
        // Add creator as first member
        instance.members.set(&sender, &true);
        
        // Emit event for first member
        MemberAdded {
            address: sender
        }.notify();
        
        instance
        }

        /// Helper to create a vote key
        fn create_vote_key(&self, proposal_id: u32, voter: &H160) -> Vec<u8> {
        let mut key = vec![];
            key.extend_from_slice(&proposal_id.to_be_bytes());
            key.extend_from_slice(voter.as_bytes());
            key
        }

        /// Check if a proposal has ended
    fn is_proposal_ended(&self, proposal_id: u32) -> bool {
        let current_time = Runtime::time();
        let end_time = self.proposals_end_time.get(&proposal_id).unwrap_or(0);
        current_time >= end_time
    }
    
    /// Get proposal state
    fn get_proposal_state(&self, proposal_id: u32) -> u8 {
        self.proposals_state.get(&proposal_id).unwrap_or(0)
        }

        /// Update a proposal's state based on voting results
    fn update_proposal_state(&mut self, proposal_id: u32) -> bool {
        // Skip if proposal is not active
        let state = self.get_proposal_state(proposal_id);
        if state != 0 {
            return false;
        }
        
        // Skip if proposal is still active
        if !self.is_proposal_ended(proposal_id) {
            return false;
        }
        
        // Update state based on votes
        let votes_for = self.proposals_votes_for.get(&proposal_id).unwrap_or(0);
        let votes_against = self.proposals_votes_against.get(&proposal_id).unwrap_or(0);
        let total_votes = votes_for + votes_against;
        let min_votes = self.min_votes_to_pass.get().unwrap_or(1);
        
        // Determine if proposal passed
        let new_state = if total_votes >= min_votes && votes_for > votes_against {
            1 // Passed
                } else {
            2 // Rejected
        };

        // Save updated state
        self.proposals_state.set(&proposal_id, &new_state);

        true
        }

        /// Add a new member to the DAO
        #[method]
    #[no_reentrant]
    pub fn add_member(&mut self, address: H160) -> bool {
        // Only owner can add members
        let sender = Runtime::calling_script_hash();
        if sender != self.owner.get().unwrap_or_default() {
                return false;
            }

        // Check if already a member
        if self.is_member(address) {
            return true;
        }
        
        // Add to members
        self.members.set(&address, &true);

            // Emit event
        MemberAdded {
            address
        }.notify();

            true
        }

        /// Create a new proposal
        #[method]
    #[no_reentrant]
    pub fn create_proposal(&mut self, title: String, description: String) -> u32 {
        let sender = Runtime::calling_script_hash();
        
            // Only members can create proposals
        if !self.is_member(sender) {
                return 0;
            }

        // Increment proposal count
        let proposal_id = self.proposal_count.get().unwrap_or(0) + 1;
        self.proposal_count.set(&proposal_id);

            // Calculate end time
        let current_time = Runtime::time();
        let voting_period = self.voting_period.get().unwrap_or(3600 * 24 * 7); // Default 7 days
        let end_time = current_time + voting_period;
        
        // Save proposal data
        self.proposals_title.set(&proposal_id, &title.clone());
        self.proposals_description.set(&proposal_id, &description.clone());
        self.proposals_creator.set(&proposal_id, &sender);
        self.proposals_votes_for.set(&proposal_id, &0u32);
        self.proposals_votes_against.set(&proposal_id, &0u32);
        self.proposals_end_time.set(&proposal_id, &end_time);
        self.proposals_state.set(&proposal_id, &0u8); // Active

            // Emit event
        ProposalCreated {
            proposal_id,
            title,
            description,
            creator: sender,
            created_at: current_time
        }.notify();
        
        proposal_id
        }

        /// Vote on a proposal
        #[method]
    #[no_reentrant]
    pub fn vote(&mut self, proposal_id: u32, vote_for: bool) -> bool {
        let voter = Runtime::calling_script_hash();
        
        // Only members can vote
        if !self.is_member(voter) {
                return false;
            }

        // Verify proposal exists
        if !self.proposal_exists(proposal_id) {
                return false;
            }

        // Check if proposal is active
        if self.get_proposal_state(proposal_id) != 0 {
                return false;
            }

        // Check if proposal is not ended
        if self.is_proposal_ended(proposal_id) {
            // Update state if ended
            self.update_proposal_state(proposal_id);
                return false;
            }

        // Check if voter has already voted
        if self.has_voted(proposal_id, voter) {
                return false;
            }

        // Record vote
        let vote_key = self.create_vote_key(proposal_id, &voter);
        self.votes.set(&vote_key, &vote_for);
        
        // Update proposal vote counts
            if vote_for {
            let votes_for = self.proposals_votes_for.get(&proposal_id).unwrap_or(0) + 1;
            self.proposals_votes_for.set(&proposal_id, &votes_for);
            } else {
            let votes_against = self.proposals_votes_against.get(&proposal_id).unwrap_or(0) + 1;
            self.proposals_votes_against.set(&proposal_id, &votes_against);
            }

            // Emit event
        VoteCast {
            proposal_id,
            voter,
            vote_for
        }.notify();

            true
        }

        /// Execute a passed proposal
        #[method]
    #[no_reentrant]
    pub fn execute_proposal(&mut self, proposal_id: u32) -> bool {
        let executor = Runtime::calling_script_hash();
        
        // Only members can execute proposals
        if !self.is_member(executor) {
                return false;
            }

        // Verify proposal exists
        if !self.proposal_exists(proposal_id) {
                return false;
            }

        // If still active, check if it can be closed and update state
        if self.get_proposal_state(proposal_id) == 0 {
            self.update_proposal_state(proposal_id);
        }
        
        // Check if proposal is in passed state
        if self.get_proposal_state(proposal_id) != 1 {
                return false;
            }

            // Mark as executed
        self.proposals_state.set(&proposal_id, &3u8);

            // Emit event
        ProposalExecuted {
            proposal_id,
            executor
        }.notify();

            true
        }

        /// Remove a member from the DAO
        #[method]
    #[no_reentrant]
    pub fn remove_member(&mut self, address: H160) -> bool {
        // Only owner can remove members
        let sender = Runtime::calling_script_hash();
        if sender != self.owner.get().unwrap_or_default() {
            return false;
        }
        
        // Owner cannot remove themselves
        if address == self.owner.get().unwrap_or_default() {
                return false;
            }

        // Check if a member
        if !self.is_member(address) {
                return false;
            }

        // Remove from members
        self.members.set(&address, &false);

            // Emit event
        MemberRemoved {
            address
        }.notify();

            true
        }

        /// Transfer ownership of the DAO
        #[method]
    #[no_reentrant]
    pub fn transfer_ownership(&mut self, new_owner: H160) -> bool {
        // Only current owner can transfer ownership
        let sender = Runtime::calling_script_hash();
        if sender != self.owner.get().unwrap_or_default() {
                return false;
        }
        
        // New owner must be a member
        if !self.is_member(new_owner) {
                return false;
        }
        
        let current_owner = self.owner.get().unwrap_or_default();
        
        // Update owner
        self.owner.set(&new_owner);
        
        // Emit event
        OwnershipTransferred {
            previous_owner: current_owner,
            new_owner
        }.notify();

            true
        }

        /// Check if an address is a member of the DAO
        #[safe]
    pub fn is_member(&self, address: H160) -> bool { 
        self.members.get(&address).unwrap_or(false) 
    }

    /// Check if a proposal exists
        #[safe]
    pub fn proposal_exists(&self, proposal_id: u32) -> bool {
        self.proposals_title.get(&proposal_id).is_some()
        }

        /// Get the total number of proposals
        #[safe]
    pub fn get_proposal_count(&self) -> u32 { 
        self.proposal_count.get().unwrap_or(0) 
    }

        /// Check if a user has voted on a proposal
        #[safe]
    pub fn has_voted(&self, proposal_id: u32, voter: H160) -> bool {
            let vote_key = self.create_vote_key(proposal_id, &voter);
        self.votes.get(&vote_key).is_some()
        }

        /// Get the DAO owner
        #[safe]
    pub fn get_owner(&self) -> H160 { 
        self.owner.get().unwrap_or_default() 
    }
}
