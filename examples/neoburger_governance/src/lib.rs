// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use neo_contract::Runtime;
use neo_contract::builtin::{H160, Int256, ByteString, Array, Any, Map};

/// GovernanceToken is a contract that manages the governance of the BurgerNEO ecosystem
pub struct GovernanceToken {
    /// Owner of the contract
    pub owner: H160,
    /// Total supply of the token
    pub total_supply: Int256,
    /// Balances of the token
    pub balances: Map<H160, Int256>,
    /// Proposals
    pub proposals: Map<Int256, ByteString>,
    /// Proposal votes
    pub proposal_votes: Map<Int256, Int256>,
    /// Proposal status (0 = pending, 1 = approved, 2 = rejected)
    pub proposal_status: Map<Int256, Int256>,
    /// Next proposal ID
    pub next_proposal_id: Int256,
}

impl GovernanceToken {
    /// Initialize the contract
    pub fn new() -> Self {
        let owner = Runtime::calling_script_hash();
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
    pub fn name(&self) -> ByteString {
        ByteString::from("BurgerGovernance")
    }

    /// Get the symbol of the token
    pub fn symbol(&self) -> ByteString {
        ByteString::from("bGOV")
    }

    /// Get the decimals of the token
    pub fn decimals(&self) -> u8 {
        8
    }

    /// Get the total supply of the token
    pub fn total_supply(&self) -> Int256 {
        self.total_supply.clone()
    }

    /// Get the balance of an account
    pub fn balance_of(&self, account: H160) -> Int256 {
        // Map doesn't have a get method in the current implementation
        // We need to iterate through the keys and values
        for i in 0..self.balances.len() {
            if let Some(key) = self.balances.keys.get(i) {
                if *key == account {
                    if let Some(value) = self.balances.values.get(i) {
                        return value.clone();
                    }
                }
            }
        }
        Int256::zero()
    }

    /// Transfer tokens from one account to another
    pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
        if amount <= Int256::zero() {
            return false;
        }

        if !Runtime::check_witness(from.clone()) {
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

        // Emit transfer event
        let mut args = Array::new();
        args.push(Any::from(from));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        Runtime::notify(
            &ByteString::from("Transfer"),
            &args
        );

        true
    }

    /// Create a new proposal
    pub fn create_proposal(&mut self, description: ByteString) -> Int256 {
        if !Runtime::check_witness(self.owner.clone()) {
            return Int256::zero();
        }

        let proposal_id = self.next_proposal_id.clone();
        
        self.proposals.put(proposal_id.clone(), description.clone());
        self.proposal_votes.put(proposal_id.clone(), Int256::zero());
        self.proposal_status.put(proposal_id.clone(), Int256::zero()); // Pending

        self.next_proposal_id = self.next_proposal_id.clone() + Int256::from(1);

        // Emit proposal created event
        let mut args = Array::new();
        args.push(Any::from(proposal_id.clone()));
        args.push(Any::from(description));
        
        Runtime::notify(
            &ByteString::from("ProposalCreated"),
            &args
        );

        proposal_id
    }

    /// Vote on a proposal
    pub fn vote(&mut self, proposal_id: Int256, amount: Int256) -> bool {
        let voter = Runtime::calling_script_hash();
        
        if amount <= Int256::zero() {
            return false;
        }

        let voter_balance = self.balance_of(voter.clone());
        if voter_balance < amount {
            return false;
        }

        // Get proposal status
        let mut status = Int256::zero();
        for i in 0..self.proposal_status.len() {
            if let Some(key) = self.proposal_status.keys.get(i) {
                if *key == proposal_id {
                    if let Some(value) = self.proposal_status.values.get(i) {
                        status = value.clone();
                        break;
                    }
                }
            }
        }
        
        if status != Int256::zero() {
            return false; // Proposal is not pending
        }

        // Get current votes
        let mut current_votes = Int256::zero();
        for i in 0..self.proposal_votes.len() {
            if let Some(key) = self.proposal_votes.keys.get(i) {
                if *key == proposal_id {
                    if let Some(value) = self.proposal_votes.values.get(i) {
                        current_votes = value.clone();
                        break;
                    }
                }
            }
        }

        let new_votes = current_votes + amount.clone();
        self.proposal_votes.put(proposal_id.clone(), new_votes);

        // Lock the tokens by transferring them to the contract
        self.transfer(voter.clone(), Runtime::executing_script_hash(), amount.clone());

        // Emit voted event
        let mut args = Array::new();
        args.push(Any::from(proposal_id));
        args.push(Any::from(voter));
        args.push(Any::from(amount));
        
        Runtime::notify(
            &ByteString::from("Voted"),
            &args
        );

        true
    }

    /// Finalize a proposal
    pub fn finalize_proposal(&mut self, proposal_id: Int256, approve: bool) -> bool {
        if !Runtime::check_witness(self.owner.clone()) {
            return false;
        }

        // Get proposal status
        let mut status = Int256::zero();
        for i in 0..self.proposal_status.len() {
            if let Some(key) = self.proposal_status.keys.get(i) {
                if *key == proposal_id {
                    if let Some(value) = self.proposal_status.values.get(i) {
                        status = value.clone();
                        break;
                    }
                }
            }
        }
        
        if status != Int256::zero() {
            return false; // Proposal is not pending
        }

        let new_status = if approve { Int256::from(1) } else { Int256::from(2) };
        self.proposal_status.put(proposal_id.clone(), new_status.clone());

        // Emit proposal finalized event
        let mut args = Array::new();
        args.push(Any::from(proposal_id));
        args.push(Any::from(new_status));
        
        Runtime::notify(
            &ByteString::from("ProposalFinalized"),
            &args
        );

        true
    }

    /// Mint new tokens
    fn mint(&mut self, to: H160, amount: Int256) -> bool {
        if amount <= Int256::zero() {
            return false;
        }

        let to_balance = self.balance_of(to.clone());
        let new_balance = to_balance + amount.clone();
        self.balances.put(to.clone(), new_balance);
        
        self.total_supply = self.total_supply.clone() + amount.clone();

        // Emit transfer event (mint from null address)
        let mut args = Array::new();
        args.push(Any::from(H160::zero()));
        args.push(Any::from(to));
        args.push(Any::from(amount));
        
        Runtime::notify(
            &ByteString::from("Transfer"),
            &args
        );

        true
    }
}

// Required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
