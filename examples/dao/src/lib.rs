#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

/// Decentralized Autonomous Organization (DAO) Example
/// This contract demonstrates a simple governance system where members can
/// create and vote on proposals.
#[contract]
#[contract_author("R3E Network")]
#[contract_description("Decentralized Autonomous Organization for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod dao_contract {
    use neo_contract::call_flags::CallFlags;
    use neo_contract::contract::native::ledger::Ledger;
    use neo_contract::error::{Error, ErrorCode, Result};
    use neo_contract::prelude::*;
    use neo_contract::storage::item::Codec;
    use neo_contract::types::context::StorageContext;
    use neo_contract::types::storage::Storage;
    use neo_contract::Runtime;

    /// Event emitted when a member is added to the DAO
    #[event]
    struct MemberAdded {
        #[index]
        address: H160,
    }

    /// Event emitted when a proposal is created
    #[event]
    struct ProposalCreated {
        #[index]
        proposal_id: u32,
        title: String,
        description: String,
        creator: H160,
        created_at: u64,
    }

    /// Event emitted when a vote is cast
    #[event]
    struct VoteCast {
        #[index]
        proposal_id: u32,
        #[index]
        voter: H160,
        vote_for: bool,
    }

    /// Event emitted when a proposal is executed
    #[event]
    struct ProposalExecuted {
        #[index]
        proposal_id: u32,
        executor: H160,
    }

    /// Event emitted when a member is removed from the DAO
    #[event]
    struct MemberRemoved {
        #[index]
        address: H160,
    }

    /// Event emitted when ownership is transferred
    #[event]
    struct OwnershipTransferred {
        #[index]
        previous_owner: H160,
        #[index]
        new_owner: H160,
    }

    /// Vote direction
    #[derive(Clone, PartialEq)]
    enum Vote {
        For = 1,
        Against = 0,
    }

    /// Implement Codec for Vote to allow storage
    impl Codec for Vote {
        fn encode(&self) -> Vec<u8> {
            match self {
                Vote::For => vec![1],
                Vote::Against => vec![0],
            }
        }

        fn decode(bytes: &[u8]) -> Result<Self> {
            if bytes.is_empty() {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }

            match bytes[0] {
                1 => Ok(Vote::For),
                0 => Ok(Vote::Against),
                _ => Err(Error::new(ErrorCode::InvalidArgument)),
            }
        }
    }

    /// Proposal state
    #[derive(Clone, PartialEq)]
    enum ProposalState {
        Active = 0,
        Passed = 1,
        Rejected = 2,
        Executed = 3,
    }

    /// Implement Codec for ProposalState to allow storage
    impl Codec for ProposalState {
        fn encode(&self) -> Vec<u8> {
            match self {
                ProposalState::Active => vec![0],
                ProposalState::Passed => vec![1],
                ProposalState::Rejected => vec![2],
                ProposalState::Executed => vec![3],
            }
        }

        fn decode(bytes: &[u8]) -> Result<Self> {
            if bytes.is_empty() {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }

            match bytes[0] {
                0 => Ok(ProposalState::Active),
                1 => Ok(ProposalState::Passed),
                2 => Ok(ProposalState::Rejected),
                3 => Ok(ProposalState::Executed),
                _ => Err(Error::new(ErrorCode::InvalidArgument)),
            }
        }
    }

    /// Proposal structure
    #[derive(Clone)]
    struct Proposal {
        id: u32,
        creator: H160,
        title: String,
        description: String,
        votes_for: u32,
        votes_against: u32,
        end_time: u64,
        state: ProposalState,
    }

    /// Implement Codec for Proposal
    impl Codec for Proposal {
        fn encode(&self) -> Vec<u8> {
            let mut buffer = Vec::new();

            // Encode id
            buffer.extend_from_slice(&self.id.to_be_bytes());

            // Encode creator
            buffer.extend_from_slice(self.creator.as_bytes());

            // Encode title
            let title_bytes = self.title.as_bytes();
            buffer.extend_from_slice(&(title_bytes.len() as u32).to_be_bytes());
            buffer.extend_from_slice(title_bytes);

            // Encode description
            let desc_bytes = self.description.as_bytes();
            buffer.extend_from_slice(&(desc_bytes.len() as u32).to_be_bytes());
            buffer.extend_from_slice(desc_bytes);

            // Encode votes
            buffer.extend_from_slice(&self.votes_for.to_be_bytes());
            buffer.extend_from_slice(&self.votes_against.to_be_bytes());

            // Encode end_time
            buffer.extend_from_slice(&self.end_time.to_be_bytes());

            // Encode state
            buffer.extend_from_slice(&self.state.encode());

            buffer
        }

        fn decode(bytes: &[u8]) -> Result<Self> {
            if bytes.len() < 4 + 20 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }

            let mut offset = 0;

            // Decode id
            let id = u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            // Decode creator
            let creator = H160::from_slice(&bytes[offset..offset + 20]);
            offset += 20;

            // Decode title
            if bytes.len() < offset + 4 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let title_len =
                u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
            offset += 4;

            if bytes.len() < offset + title_len {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let title = String::from_utf8(bytes[offset..offset + title_len].to_vec())
                .map_err(|_| Error::new(ErrorCode::InvalidArgument))?;
            offset += title_len;

            // Decode description
            if bytes.len() < offset + 4 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let desc_len =
                u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]) as usize;
            offset += 4;

            if bytes.len() < offset + desc_len {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let description = String::from_utf8(bytes[offset..offset + desc_len].to_vec())
                .map_err(|_| Error::new(ErrorCode::InvalidArgument))?;
            offset += desc_len;

            // Decode votes
            if bytes.len() < offset + 8 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let votes_for =
                u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            let votes_against =
                u32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
            offset += 4;

            // Decode end_time
            if bytes.len() < offset + 8 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let end_time = u64::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]);
            offset += 8;

            // Decode state
            if bytes.len() < offset + 1 {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }
            let state = ProposalState::decode(&bytes[offset..offset + 1])?;

            Ok(Proposal {
                id,
                creator,
                title,
                description,
                votes_for,
                votes_against,
                end_time,
                state,
            })
        }
    }

    /// Main storage for the DAO contract
    #[storage]
    struct DaoContract {
        /// Number of proposals created
        proposal_count: Item<u32>,
        /// Mapping of proposal ID to Proposal
        proposals: StorageMap<u32, Proposal>,
        /// Mapping of (proposal ID, voter address) to Vote
        votes: StorageMap<Vec<u8>, Vote>,
        /// Mapping of address to whether they are a member
        members: StorageMap<H160, bool>,
        /// DAO owner with admin rights
        owner: Item<H160>,
        /// Default voting period in seconds
        voting_period: Item<u64>,
        /// Minimum votes needed to pass a proposal
        min_votes_to_pass: Item<u32>,
    }

    impl DaoContract {
        /// Create a new DAO contract
        #[constructor]
        fn new(voting_period_hours: u32, min_votes_to_pass: u32) -> Self {
            // Convert hours to seconds
            let voting_period = (voting_period_hours as u64) * 3600;

            // Get the caller as the owner
            let owner = Runtime::check_witness().unwrap_or_else(|| H160::zero());

            // Create a new contract instance
            let mut dao = Self {
                proposal_count: Item::new(b"proposal_count"),
                proposals: StorageMap::new(b"proposals"),
                votes: StorageMap::new(b"votes"),
                members: StorageMap::new(b"members"),
                owner: Item::new(b"owner"),
                voting_period: Item::new(b"voting_period"),
                min_votes_to_pass: Item::new(b"min_votes_to_pass"),
            };

            // Initialize storage values
            dao.proposal_count.set(&0).unwrap_or(());
            dao.owner.set(&owner).unwrap_or(());
            dao.voting_period.set(&voting_period).unwrap_or(());
            dao.min_votes_to_pass.set(&min_votes_to_pass).unwrap_or(());

            // Add owner as the first member
            dao.members.set(&owner, &true).unwrap_or(());

            dao
        }

        /// Helper to create a vote key
        fn create_vote_key(&self, proposal_id: u32, voter: &H160) -> Vec<u8> {
            let mut key = Vec::with_capacity(24); // 4 bytes for u32 + 20 bytes for H160
            key.extend_from_slice(&proposal_id.to_be_bytes());
            key.extend_from_slice(voter.as_bytes());
            key
        }

        /// Check if a proposal has ended
        fn is_proposal_ended(&self, proposal: &Proposal) -> bool {
            let current_time = Ledger::current_timestamp();
            current_time >= proposal.end_time
        }

        /// Update a proposal's state based on voting results
        fn update_proposal_state(&mut self, proposal_id: u32) -> Result<()> {
            // Get the proposal
            let proposal_opt = self.proposals.get(&proposal_id).unwrap_or(None);

            if proposal_opt.is_none() {
                return Err(Error::new(ErrorCode::InvalidArgument));
            }

            let mut proposal = proposal_opt.unwrap();

            // If already passed, rejected, or executed, do nothing
            if proposal.state != ProposalState::Active {
                return Ok(());
            }

            // Check if voting period has ended
            if self.is_proposal_ended(&proposal) {
                // Get min votes needed
                let min_votes = self.min_votes_to_pass.get().unwrap_or(None).unwrap_or(1);

                // Determine new state based on votes
                if proposal.votes_for > proposal.votes_against && proposal.votes_for >= min_votes {
                    proposal.state = ProposalState::Passed;
                } else {
                    proposal.state = ProposalState::Rejected;
                }

                // Update the proposal
                self.proposals.set(&proposal_id, &proposal).unwrap_or(());
            }

            Ok(())
        }

        /// Add a new member to the DAO
        #[method]
        fn add_member(&mut self, address: H160) -> bool {
            // Only the owner can add members
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Add the member
            self.members.set(&address, &true).unwrap_or(());

            // Emit event
            MemberAdded::emit(address);

            true
        }

        /// Create a new proposal
        #[method]
        fn create_proposal(&mut self, title: String, description: String) -> u32 {
            // Only members can create proposals
            let caller = Runtime::calling_script_hash();
            let is_member = self.members.get(&caller).unwrap_or(None).unwrap_or(false);
            if !is_member {
                return 0;
            }

            // Get the next proposal ID
            let proposal_count = self.proposal_count.get().unwrap_or(None).unwrap_or(0);
            let new_id = proposal_count + 1;

            // Get the voting period
            let voting_period = self.voting_period.get().unwrap_or(None).unwrap_or(24);

            // Get current time
            let current_time = Runtime::time();

            // Calculate end time
            let end_time = current_time + (voting_period as u64 * 3600 * 1000);

            // Create the proposal
            let proposal = Proposal {
                id: new_id,
                title: title.clone(),
                description: description.clone(),
                creator: caller,
                votes_for: 0,
                votes_against: 0,
                end_time: end_time,
                state: ProposalState::Active,
            };

            // Store the proposal
            self.proposals.set(&new_id, &proposal).unwrap_or(());

            // Update the proposal count
            self.proposal_count.set(&new_id).unwrap_or(());

            // Emit event
            ProposalCreated::emit(new_id, title, description, caller, current_time);

            new_id
        }

        /// Vote on a proposal
        #[method]
        fn vote(&mut self, proposal_id: u32, vote_for: bool) -> bool {
            // Get the caller
            let caller = Runtime::calling_script_hash();
            if !Runtime::check_witness(&caller) {
                return false;
            }

            // Check if the caller is a member
            let is_member = self.members.get(&caller).unwrap_or(None).unwrap_or(false);
            if !is_member {
                return false;
            }

            // Get the proposal
            let proposal_opt = self.proposals.get(&proposal_id).unwrap_or(None);

            if proposal_opt.is_none() {
                return false;
            }

            let mut proposal = proposal_opt.unwrap();

            // Check if proposal is still active
            if proposal.state != ProposalState::Active {
                return false;
            }

            // Check if voting period has ended
            if self.is_proposal_ended(&proposal) {
                self.update_proposal_state(proposal_id).unwrap_or(());
                return false;
            }

            // Check if user has already voted
            let vote_key = self.create_vote_key(proposal_id, &caller);
            if self.votes.get(&vote_key).unwrap_or(None).is_some() {
                return false;
            }

            // Record the vote
            let vote_value = if vote_for { Vote::For } else { Vote::Against };
            self.votes.set(&vote_key, &vote_value).unwrap_or(());

            // Update vote counts
            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }

            // Update the proposal
            self.proposals.set(&proposal_id, &proposal).unwrap_or(());

            // Emit event
            VoteCast::emit(proposal_id, caller, vote_for);

            true
        }

        /// Execute a passed proposal
        #[method]
        fn execute_proposal(&mut self, proposal_id: u32) -> bool {
            // Get the caller
            let caller = Runtime::calling_script_hash();
            if !Runtime::check_witness(&caller) {
                return false;
            }

            let is_member = self.members.get(&caller).unwrap_or(None).unwrap_or(false);
            if !is_member {
                return false;
            }

            // Update the proposal state if needed
            self.update_proposal_state(proposal_id).unwrap_or(());

            // Get the proposal
            let proposal_opt = self.proposals.get(&proposal_id).unwrap_or(None);

            if proposal_opt.is_none() {
                return false;
            }

            let mut proposal = proposal_opt.unwrap();

            // Check if the proposal has passed
            if proposal.state != ProposalState::Passed {
                return false;
            }

            // Mark as executed
            proposal.state = ProposalState::Executed;
            self.proposals.set(&proposal_id, &proposal).unwrap_or(());

            // Emit event
            ProposalExecuted::emit(proposal_id, caller);

            true
        }

        /// Remove a member from the DAO
        #[method]
        fn remove_member(&mut self, address: H160) -> bool {
            // Only the owner can remove members
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Cannot remove the owner
            if address == owner {
                return false;
            }

            // Remove the member
            self.members.set(&address, &false).unwrap_or(());

            // Emit event
            MemberRemoved::emit(address);

            true
        }

        /// Transfer ownership of the DAO
        #[method]
        fn transfer_ownership(&mut self, new_owner: H160) -> bool {
            // Only the current owner can transfer ownership
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Set the new owner
            self.owner.set(&new_owner).unwrap_or(());

            // Add new owner as a member if not already
            self.members.set(&new_owner, &true).unwrap_or(());

            // Emit event
            OwnershipTransferred::emit(owner, new_owner);

            true
        }

        /// Set the voting period
        #[method]
        fn set_voting_period(&mut self, hours: u32) -> bool {
            // Only the owner can change the voting period
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Convert hours to seconds
            let seconds = (hours as u64) * 3600;

            // Update the voting period
            self.voting_period.set(&seconds).unwrap_or(());

            true
        }

        /// Set the minimum votes required to pass a proposal
        #[method]
        fn set_min_votes(&mut self, min_votes: u32) -> bool {
            // Only the owner can change the minimum votes
            let owner = self.owner.get().unwrap_or(None).unwrap_or_default();
            if !Runtime::check_witness(&owner) {
                return false;
            }

            // Update the minimum votes
            self.min_votes_to_pass.set(&min_votes).unwrap_or(());

            true
        }

        // READ-ONLY METHODS

        /// Check if an address is a member of the DAO
        #[safe]
        fn is_member(&self, address: H160) -> bool { self.members.get(&address).unwrap_or(None).unwrap_or(false) }

        /// Get information about a proposal
        #[safe]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
            self.proposals.get(&proposal_id).unwrap_or(None)
        }

        /// Get the total number of proposals
        #[safe]
        fn get_proposal_count(&self) -> u32 { self.proposal_count.get().unwrap_or(None).unwrap_or(0) }

        /// Check if a user has voted on a proposal
        #[safe]
        fn has_voted(&self, proposal_id: u32, voter: H160) -> bool {
            let vote_key = self.create_vote_key(proposal_id, &voter);
            self.votes.get(&vote_key).unwrap_or(None).is_some()
        }

        /// Get a user's vote on a proposal
        #[safe]
        pub fn get_vote(&self, proposal_id: u32, voter: H160) -> Option<Vote> {
            let key = self.create_vote_key(proposal_id, &voter);
            self.votes.get(&key).unwrap_or(None)
        }

        /// Get the DAO owner
        #[safe]
        fn get_owner(&self) -> H160 { self.owner.get().unwrap_or(None).unwrap_or_default() }

        /// Get the voting period in hours
        #[safe]
        fn get_voting_period(&self) -> u32 {
            let seconds = self.voting_period.get().unwrap_or(None).unwrap_or(86400);

            (seconds / 3600) as u32
        }

        /// Get the minimum votes needed to pass a proposal
        #[safe]
        fn get_min_votes(&self) -> u32 { self.min_votes_to_pass.get().unwrap_or(None).unwrap_or(1) }
    }
}
