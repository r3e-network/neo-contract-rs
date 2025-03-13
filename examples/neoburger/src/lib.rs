// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

extern crate alloc;
extern crate wee_alloc;

// Clean up unused imports
use core::panic::PanicInfo;

// Import neo contract properly
use neo_contract::{
    builtin::{H160, Int256, ByteString, Map, Any, Array},
    Runtime,
    contract_author, contract_email, contract_description,
    contract_version, contract_permission, contract_trust,
    storage, constructor, message,
};
use neo_contract::contract::native::legder::Ledger;

// Use wee_alloc as the global allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Helper function to emit a Transfer event
pub fn emit_transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {
    let event_name = ByteString::from("Transfer");
    let mut event_data = Array::<Any>::new();
    
    match from {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    match to {
        Some(addr) => event_data.push(Any::from(addr)),
        None => event_data.push(Any::new()),
    }
    
    event_data.push(Any::from(amount));
    
    Runtime::notify(&event_name, &event_data);
}

// Helper function to emit a CancelStake event
pub fn emit_cancel_stake(from: H160, value: Int256) {
    let event_name = ByteString::from("CancelStake");
    let mut event_data = Array::<Any>::new();
    
    event_data.push(Any::from(from));
    event_data.push(Any::from(value));
    
    Runtime::notify(&event_name, &event_data);
}

// Helper function to emit a CancelVote event
pub fn emit_cancel_vote(from: H160, to: H160, value: Int256) {
    let event_name = ByteString::from("CancelVote");
    let mut event_data = Array::<Any>::new();
    
    event_data.push(Any::from(from));
    event_data.push(Any::from(to));
    event_data.push(Any::from(value));
    
    Runtime::notify(&event_name, &event_data);
}

// Helper function to emit a Distribute event
pub fn emit_distribute() {
    let event_name = ByteString::from("Distribute");
    let event_data = Array::<Any>::new();
    
    Runtime::notify(&event_name, &event_data);
}

// Helper function to emit a Stake event
pub fn emit_stake(from: H160, value: Int256) {
    let event_name = ByteString::from("Stake");
    let mut event_data = Array::<Any>::new();
    
    event_data.push(Any::from(from));
    event_data.push(Any::from(value));
    
    Runtime::notify(&event_name, &event_data);
}

// Helper function to emit a Vote event
pub fn emit_vote(from: H160, to: H160, value: Int256) {
    let event_name = ByteString::from("Vote");
    let mut event_data = Array::<Any>::new();
    
    event_data.push(Any::from(from));
    event_data.push(Any::from(to));
    event_data.push(Any::from(value));
    
    Runtime::notify(&event_name, &event_data);
}

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("BurgerNEO contract example")]
#[contract_version("0.1.0")]
#[contract_permission("*", "*")]
#[contract_trust("*")]
mod burger_contract {
    use super::*;

    #[storage]
    pub struct BurgerContract {
        token_name: ByteString,
        token_symbol: ByteString,
        token_decimals: Int256,
        total: Int256,
        owner: H160,
        balances: Map<H160, Int256>,
        gov_token: H160,
        dao_address: H160,
        unstake_blocks: Int256,
        staking_balance: Map<H160, Int256>,
        staking_height: Map<H160, Int256>,
        burgers: Map<H160, Int256>,
        stake_supply: Int256,
        committee: Map<H160, H160>,
        candidate_votes: Map<H160, Int256>,
        candidate_vote_address_balance: Map<H160, Map<H160, Int256>>,
        total_staked_by_voter: Map<H160, Int256>,
    }

    impl BurgerContract {
        #[constructor]
        pub fn new(owner: H160, gov_token: H160, dao_address: H160) -> Self {
            let balances = Map::new();
            let committee = Map::new();
            let candidate_votes = Map::new();
            let staking_balance = Map::new();
            let staking_height = Map::new();
            let burgers = Map::new();
            let candidate_vote_address_balance = Map::new();
            let total_staked_by_voter = Map::new();
            
            let token_name = ByteString::from("BurgerNEO");
            let token_symbol = ByteString::from("BNEO");
            let unstake_blocks = Int256::from(10);
            
            Self {
                token_name,
                token_symbol,
                token_decimals: Int256::from(8),
                total: Int256::from(0),
                owner,
                balances,
                gov_token,
                dao_address,
                unstake_blocks,
                staking_balance,
                staking_height,
                burgers,
                stake_supply: Int256::from(0),
                committee,
                candidate_votes,
                candidate_vote_address_balance,
                total_staked_by_voter,
            }
        }

        #[safe]
        pub fn name(&self) -> ByteString {
            self.token_name.clone()
        }

        #[safe]
        pub fn symbol(&self) -> ByteString {
            self.token_symbol.clone()
        }

        #[safe]
        pub fn decimals(&self) -> Int256 {
            self.token_decimals.clone()
        }

        #[safe]
        pub fn total_supply(&self) -> Int256 {
            self.total.clone()
        }

        #[method]
        pub fn transfer(&mut self, from: H160, to: H160, amount: Int256, _data: ByteString) -> bool {
            if !Runtime::check_witness(from) {
                return false;
            }
            
            if amount <= Int256::from(0) {
                return false;
            }
            
            let from_balance = match self.balances.get(&from) {
                Some(balance) => *balance,
                None => Int256::from(0),
            };
            
            if from_balance < amount {
                return false;
            }
            
            // Handle burning
            if to == H160::zero() {
                self.balances.put(from.clone(), from_balance - amount);
                self.total = self.total - amount;
                emit_transfer(Some(from), Some(to), amount);
                return true;
            }
            
            // Regular transfer
            let to_balance = match self.balances.get(&to) {
                Some(balance) => *balance,
                None => Int256::from(0),
            };
            
            self.balances.put(from.clone(), from_balance - amount);
            self.balances.put(to.clone(), to_balance + amount);
            
            emit_transfer(Some(from), Some(to), amount);
            true
        }

        #[method]
        pub fn vote(&mut self, addr: H160, candidate: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(addr) {
                return false;
            }
            
            if amount <= Int256::from(0) {
                return false;
            }
            
            // Get current staked balance
            let staked_balance = match self.staking_balance.get(&addr) {
                Some(balance) => *balance,
                None => Int256::from(0),
            };
            
            // Verify staked balance is sufficient
            if staked_balance < amount {
                return false;
            }
            
            // Calculate remaining balance after vote
            let new_staked_balance = staked_balance - amount;
            
            // Update staking balance
            if new_staked_balance.is_zero() {
                self.staking_balance.delete(&addr);
            } else {
                self.staking_balance.put(addr.clone(), new_staked_balance);
            }
            
            // Update total staked by voter
            let total_staked = match self.total_staked_by_voter.get(&addr) {
                Some(total) => {
                    let new_total = *total + amount;
                    if new_total.is_zero() {
                        self.total_staked_by_voter.delete(&addr);
                    } else {
                        self.total_staked_by_voter.put(addr.clone(), new_total);
                    }
                    new_total
                },
                None => amount,
            };
            
            // Update candidate vote balance
            let candidate_vote_balance = match self.candidate_votes.get(&candidate) {
                Some(balance) => *balance + amount,
                None => amount,
            };
            self.candidate_votes.put(candidate.clone(), candidate_vote_balance);
            
            // Update individual vote record
            let mut voter_balances = match self.candidate_vote_address_balance.get(&candidate) {
                Some(balances) => balances.clone(),
                None => Map::new(),
            };
            
            let voter_balance = match voter_balances.get(&addr) {
                Some(balance) => *balance + amount,
                None => amount,
            };
            
            voter_balances.put(addr.clone(), voter_balance);
            self.candidate_vote_address_balance.put(candidate.clone(), voter_balances);
            
            emit_vote(addr, candidate, amount);
            true
        }

        #[method]
        pub fn cancel_vote(&mut self, addr: H160, candidate: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(addr) {
                return false;
            }
            
            if amount <= Int256::from(0) {
                return false;
            }
            
            // Verify candidate has votes
            let candidate_vote_balance = match self.candidate_votes.get(&candidate) {
                Some(balance) => *balance,
                None => return false,
            };
            
            // Verify candidate has votes from this voter
            let voter_balances = match self.candidate_vote_address_balance.get(&candidate) {
                Some(balances) => balances.clone(),
                None => return false,
            };
            
            let voter_balance = match voter_balances.get(&addr) {
                Some(balance) => *balance,
                None => return false,
            };
            
            // Verify voter has enough votes to cancel
            if voter_balance < amount {
                return false;
            }
            
            // Update voter balance for this candidate
            let new_voter_balance = voter_balance - amount;
            let mut updated_voter_balances = voter_balances.clone();
            
            if new_voter_balance.is_zero() {
                updated_voter_balances.delete(&addr);
            } else {
                updated_voter_balances.put(addr.clone(), new_voter_balance);
            }
            
            if updated_voter_balances.is_empty() {
                self.candidate_vote_address_balance.delete(&candidate);
            } else {
                self.candidate_vote_address_balance.put(candidate.clone(), updated_voter_balances);
            }
            
            // Update candidate votes
            let new_candidate_vote_balance = candidate_vote_balance - amount;
            if new_candidate_vote_balance.is_zero() {
                self.candidate_votes.delete(&candidate);
            } else {
                self.candidate_votes.put(candidate.clone(), new_candidate_vote_balance);
            }
            
            // Update total staked by voter
            let total_staked = match self.total_staked_by_voter.get(&addr) {
                Some(total) => {
                    let new_total = *total - amount;
                    if new_total.is_zero() {
                        self.total_staked_by_voter.delete(&addr);
                    } else {
                        self.total_staked_by_voter.put(addr.clone(), new_total);
                    }
                    new_total
                },
                None => Int256::from(0),
            };
            
            // Return tokens to staking balance
            let staking_balance = match self.staking_balance.get(&addr) {
                Some(balance) => *balance + amount,
                None => amount,
            };
            self.staking_balance.put(addr.clone(), staking_balance);
            
            emit_cancel_vote(addr, candidate, amount);
            true
        }

        #[method]
        pub fn stake(&mut self, addr: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(addr) {
                return false;
            }
            
            let balance = match self.balances.get(&addr) {
                Some(balance) => *balance,
                None => Int256::from(0),
            };
            
            if balance < amount {
                return false;
            }
            
            // Update balance
            let new_balance = balance - amount;
            if new_balance.is_zero() {
                self.balances.delete(&addr);
            } else {
                self.balances.put(addr.clone(), new_balance);
            }
            
            self.total = self.total - amount;
            
            let stake_balance = match self.staking_balance.get(&addr) {
                Some(balance) => *balance + amount,
                None => amount,
            };
            
            self.staking_balance.put(addr.clone(), stake_balance);
            
            // Use Ledger::current_index() instead of Runtime::current_block_index()
            let current_height = Ledger::current_index();
            self.staking_height.put(addr.clone(), Int256::from(current_height as i64));
            
            self.stake_supply = self.stake_supply + amount;
            
            emit_stake(addr, amount);
            true
        }

        #[method]
        pub fn cancel_stake(&mut self, addr: H160, amount: Int256) -> bool {
            if !Runtime::check_witness(addr) {
                return false;
            }
            
            let stake_height = match self.staking_height.get(&addr) {
                Some(height) => *height,
                None => return false,
            };
            
            // Use Ledger::current_index() instead of Runtime::current_block_index()
            let current_height = Int256::from(Ledger::current_index() as i64);
            
            if current_height - stake_height < self.unstake_blocks {
                return false;
            }
            
            let staked_amount = match self.staking_balance.get(&addr) {
                Some(balance) => *balance,
                None => return false,
            };
            
            if staked_amount < amount {
                return false;
            }
            
            let voting_amount = match self.total_staked_by_voter.get(&addr) {
                Some(voted) => *voted,
                None => Int256::from(0),
            };
            
            let remaining_staked = staked_amount - amount;
            
            if remaining_staked < voting_amount {
                return false;
            }
            
            if remaining_staked.is_zero() {
                self.staking_balance.delete(&addr);
                self.staking_height.delete(&addr);
            } else {
                let new_staked = staked_amount - amount;
                self.staking_balance.put(addr.clone(), new_staked);
            }
            
            let balance = match self.balances.get(&addr) {
                Some(balance) => *balance,
                None => Int256::from(0),
            };
            
            let new_balance = balance + amount;
            self.balances.put(addr.clone(), new_balance);
            
            self.total = self.total + amount;
            self.stake_supply = self.stake_supply - amount;
            
            emit_cancel_stake(addr, amount);
            true
        }

        #[safe]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance.clone(),
                None => Int256::from(0),
            }
        }
    }
}
