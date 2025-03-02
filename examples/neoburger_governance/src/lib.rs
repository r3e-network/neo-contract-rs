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

    /// GovernanceToken is a contract that manages the governance of the BurgerNEO ecosystem
    #[storage]
    pub struct Governance {
        /// Owner of the contract
        owner: H160,
        /// Total supply of the token
        total_supply: Int256,
        /// Balances of the token
        balances: builtin::Map,
        /// Proposals
        proposals: builtin::Map,
        /// Proposal votes
        proposal_votes: builtin::Map,
        /// Proposal status (0 = pending, 1 = approved, 2 = rejected)
        proposal_status: builtin::Map,
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
                balances: builtin::Map::new(),
                proposals: builtin::Map::new(),
                proposal_votes: builtin::Map::new(),
                proposal_status: builtin::Map::new(),
                next_proposal_id: Int256::from(1),
            };

            // Mint initial supply to the owner
            instance.mint(owner, Int256::from(1000000i64 * 100000000i64)); // 1,000,000 tokens with 8 decimals
            instance
        }

        /// Get the name of the token
        #[message]
        pub fn name(&self) -> ByteString {
            ByteString::from("BurgerGovernance")
        }

        /// Get the symbol of the token
        #[message]
        pub fn symbol(&self) -> ByteString {
            ByteString::from("bGOV")
        }

        /// Get the decimals of the token
        #[message]
        pub fn decimals(&self) -> u8 {
            8
        }

        /// Get the total supply of the token
        #[message]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply.clone()
        }

        /// Get the balance of an account
        #[message]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance,
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
                self.balances.put(&from, &from_new_balance);
            }
            
            let to_new_balance = to_balance + amount.clone();
            self.balances.put(&to, &to_new_balance);

            self.transfer_event(from, to, amount);

            true
        }

        /// Create a new proposal
        #[message]
        pub fn create_proposal(&mut self, description: ByteString) -> Int256 {
            if !runtime::check_witness(self.owner.clone()) {
                return Int256::zero();
            }

            let proposal_id = self.next_proposal_id.clone();
            
            self.proposals.put(&proposal_id, &description);
            self.proposal_votes.put(&proposal_id, &Int256::zero());
            self.proposal_status.put(&proposal_id, &Int256::zero()); // Pending

            self.next_proposal_id = self.next_proposal_id.clone() + Int256::from(1);

            self.proposal_created_event(proposal_id.clone(), description);

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
                Some(s) => s,
                None => return false, // Proposal doesn't exist
            };
            
            if status != Int256::zero() {
                return false; // Proposal is not pending
            }

            // Get current votes
            let current_votes = match self.proposal_votes.get(&proposal_id) {
                Some(v) => v,
                None => Int256::zero(),
            };

            let new_votes = current_votes + amount.clone();
            self.proposal_votes.put(&proposal_id, &new_votes);

            // Lock the tokens by transferring them to the contract
            self.transfer(voter.clone(), runtime::executing_script_hash(), amount.clone());

            self.voted_event(proposal_id, voter, amount);

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
                Some(s) => s,
                None => return false, // Proposal doesn't exist
            };
            
            if status != Int256::zero() {
                return false; // Proposal is not pending
            }

            let new_status = if approve { Int256::from(1) } else { Int256::from(2) };
            self.proposal_status.put(&proposal_id, &new_status);

            self.proposal_finalized_event(proposal_id, new_status);

            true
        }

        #[event]
        pub fn transfer_event(&self, from: H160, to: H160, amount: Int256) {}

        #[event]
        pub fn proposal_created_event(&self, proposal_id: Int256, description: ByteString) {}

        #[event]
        pub fn voted_event(&self, proposal_id: Int256, voter: H160, amount: Int256) {}

        #[event]
        pub fn proposal_finalized_event(&self, proposal_id: Int256, status: Int256) {}

        /// Mint new tokens (internal function)
        fn mint(&mut self, to: H160, amount: Int256) -> bool {
            if amount <= Int256::zero() {
                return false;
            }

            let to_balance = self.balance_of(to.clone());
            let new_balance = to_balance + amount.clone();
            self.balances.put(&to, &new_balance);
            
            self.total_supply = self.total_supply.clone() + amount.clone();

            self.transfer_event(H160::zero(), to, amount);

            true
        }
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
