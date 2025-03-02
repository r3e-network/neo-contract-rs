// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use neo_contract as neo;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("BurgerNEO Governance Token")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
#[supported_standards("NEP-17")]
mod burger_governance {
    use neo::prelude::*;
    use neo::runtime;
    use neo::types::*;
    use neo::builtin::{H160, Int256, ByteString, Map, Array, Any};

    // Helper function to emit a Transfer event
    pub fn emit_transfer(from: H160, to: H160, amount: Int256) {
        // Create event name as ByteString
        let event_name = ByteString::from("Transfer");
        
        // Create an Array to hold parameters
        let mut event_data = Array::<Any>::new();
        
        // Add parameters as Any values
        event_data.push(Any::from(from));
        event_data.push(Any::from(to));
        event_data.push(Any::from(amount));
        
        // Emit the event
        runtime::notify(&event_name, &event_data);
    }
    
    // Helper function to emit a ProposalCreated event
    pub fn emit_proposal_created(proposal_id: Int256, description: ByteString) {
        let event_name = ByteString::from("ProposalCreated");
        
        let mut event_data = Array::<Any>::new();
        event_data.push(Any::from(proposal_id));
        event_data.push(Any::from(description));
        
        runtime::notify(&event_name, &event_data);
    }
    
    // Helper function to emit a Voted event
    pub fn emit_voted(proposal_id: Int256, voter: H160, amount: Int256) {
        let event_name = ByteString::from("Voted");
        
        let mut event_data = Array::<Any>::new();
        event_data.push(Any::from(proposal_id));
        event_data.push(Any::from(voter));
        event_data.push(Any::from(amount));
        
        runtime::notify(&event_name, &event_data);
    }
    
    // Helper function to emit a ProposalFinalized event
    pub fn emit_proposal_finalized(proposal_id: Int256, status: Int256) {
        let event_name = ByteString::from("ProposalFinalized");
        
        let mut event_data = Array::<Any>::new();
        event_data.push(Any::from(proposal_id));
        event_data.push(Any::from(status));
        
        runtime::notify(&event_name, &event_data);
    }

    /// GovernanceToken is a contract that manages the governance of the BurgerNEO ecosystem
    #[storage]
    pub struct Governance {
        /// Owner of the contract
        owner: H160,
        /// Total supply of the token
        total_supply: Int256,
        /// Balances of the token
        balances: Map<H160, Int256>,
        /// Proposals
        proposals: Map<Int256, ByteString>,
        /// Proposal votes
        proposal_votes: Map<Int256, Int256>,
        /// Proposal status (0 = pending, 1 = approved, 2 = rejected)
        proposal_status: Map<Int256, Int256>,
        /// Next proposal ID
        next_proposal_id: Int256,
    }

    impl Governance {
        /// Initialize the contract
        #[constructor]
        pub fn new() -> Self {
            let owner = runtime::calling_script_hash();
            let mut instance = Self {
                owner: owner.clone(),
                total_supply: Int256::zero(),
                balances: Map::new(),
                proposals: Map::new(),
                proposal_votes: Map::new(),
                proposal_status: Map::new(),
                next_proposal_id: Int256::from(1),
            };

            // Mint initial supply to the owner
            instance.mint(owner, Int256::from(1000000i64 * 100000000i64)); // 1,000,000 tokens with 8 decimals
            instance
        }

        /// Get the name of the token
        #[message]
        #[safe]
        pub fn name(&self) -> ByteString {
            ByteString::from("BurgerGovernance")
        }

        /// Get the symbol of the token
        #[message]
        #[safe]
        pub fn symbol(&self) -> ByteString {
            ByteString::from("bGOV")
        }

        /// Get the decimals of the token
        #[message]
        #[safe]
        pub fn decimals(&self) -> u8 {
            8
        }

        /// Get the total supply of the token
        #[message]
        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }

        /// Get the balance of an account
        #[message]
        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::zero(),
            }
        }

        /// Transfer tokens from one account to another
        #[message]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
            if amount <= Int256::zero() {
                return false;
            }

            if !runtime::check_witness(from.clone()) {
                return false;
            }

            let from_balance = self.balance_of(from.clone());
            if from_balance < amount {
                return false;
            }

            let to_balance = self.balance_of(to.clone());
            
            let from_new_balance = from_balance - amount.clone();
            if from_new_balance.is_zero() {
                self.balances.delete(&from);
            } else {
                self.balances.put(from.clone(), from_new_balance);
            }
            
            let to_new_balance = to_balance + amount.clone();
            self.balances.put(to.clone(), to_new_balance);

            emit_transfer(from, to, amount);

            true
        }

        /// Create a new proposal
        #[message]
        pub fn create_proposal(&mut self, description: ByteString) -> Int256 {
            if !runtime::check_witness(self.owner.clone()) {
                return Int256::zero();
            }

            let proposal_id = self.next_proposal_id.clone();
            
            self.proposals.put(proposal_id.clone(), description.clone());
            self.proposal_votes.put(proposal_id.clone(), Int256::zero());
            self.proposal_status.put(proposal_id.clone(), Int256::zero()); // Pending

            self.next_proposal_id = self.next_proposal_id.clone() + Int256::from(1);

            emit_proposal_created(proposal_id.clone(), description);

            proposal_id
        }

        /// Vote on a proposal
        #[message]
        pub fn vote(&mut self, proposal_id: Int256, amount: Int256) -> bool {
            let voter = runtime::calling_script_hash();
            
            if amount <= Int256::zero() {
                return false;
            }

            let voter_balance = self.balance_of(voter.clone());
            if voter_balance < amount {
                return false;
            }

            // Get proposal status
            let status = match self.proposal_status.get(&proposal_id) {
                Some(s) => s.clone(),
                None => return false, // Proposal doesn't exist
            };
            
            if status != Int256::zero() {
                return false; // Proposal is not pending
            }

            // Get current votes
            let current_votes = match self.proposal_votes.get(&proposal_id) {
                Some(v) => v.clone(),
                None => Int256::zero(),
            };

            let new_votes = current_votes + amount.clone();
            self.proposal_votes.put(proposal_id.clone(), new_votes);

            // Lock the tokens by transferring them to the contract
            self.transfer(voter.clone(), runtime::executing_script_hash(), amount.clone());

            emit_voted(proposal_id, voter, amount);

            true
        }

        /// Finalize a proposal
        #[message]
        pub fn finalize_proposal(&mut self, proposal_id: Int256, approve: bool) -> bool {
            if !runtime::check_witness(self.owner.clone()) {
                return false;
            }

            // Get proposal status
            let status = match self.proposal_status.get(&proposal_id) {
                Some(s) => s.clone(),
                None => return false, // Proposal doesn't exist
            };
            
            if status != Int256::zero() {
                return false; // Proposal is not pending
            }

            let new_status = if approve {
                Int256::from(1) // Approved
            } else {
                Int256::from(2) // Rejected
            };

            self.proposal_status.put(proposal_id.clone(), new_status.clone());

            emit_proposal_finalized(proposal_id, new_status);

            true
        }

        /// Mint new tokens (internal function)
        fn mint(&mut self, to: H160, amount: Int256) -> bool {
            if amount <= Int256::zero() {
                return false;
            }

            let balance = self.balance_of(to.clone());
            let new_balance = balance + amount.clone();
            
            self.balances.put(to.clone(), new_balance);
            self.total_supply = self.total_supply.clone() + amount.clone();
            
            // Emit transfer from null address to mint destination
            emit_transfer(H160::zero(), to, amount);
            
            true
        }
    }
}
