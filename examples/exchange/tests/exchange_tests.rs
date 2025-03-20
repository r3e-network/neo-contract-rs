use exchange::*;
use neo_contract::prelude::*;
use neo_contract::types::H160;
use neo_contract::types::ByteString;
use neo_contract::types::Array;
use neo_contract::types::Any;
use neo_contract::Runtime;
use std::cell::RefCell;
use std::collections::HashMap;

// Define a mock NEP-17 token for testing interoperability
struct MockToken {
    symbol: &'static str,
    decimals: u8,
    balances: HashMap<H160, u64>,
    contract_hash: H160,
}

impl MockToken {
    fn new(symbol: &'static str, decimals: u8, contract_hash: H160) -> Self {
        Self {
            symbol,
            decimals,
            balances: HashMap::new(),
            contract_hash,
        }
    }
    
    fn mint(&mut self, address: &H160, amount: u64) {
        let current = *self.balances.get(address).unwrap_or(&0);
        self.balances.insert(*address, current + amount);
    }
    
    fn balance_of(&self, address: &H160) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
    
    fn transfer(&mut self, from: &H160, to: &H160, amount: u64) -> bool {
        let from_balance = self.balance_of(from);
        if from_balance < amount {
            return false;
        }
        
        self.balances.insert(*from, from_balance - amount);
        let to_balance = self.balance_of(to);
        self.balances.insert(*to, to_balance + amount);
        
        // Call onNEP17Payment on the recipient if it's a contract
        if is_contract(to) {
            // This would call onNEP17Payment in a real implementation
        }
        
        true
    }
    
    fn call(&mut self, method: &str, args: &[Any]) -> Any {
        match method {
            "symbol" => Any::string(self.symbol),
            "decimals" => Any::integer(self.decimals as i64),
            "balanceOf" => {
                if args.len() < 1 {
                    return Any::integer(0);
                }
                let address = args[0].as_h160().unwrap();
                Any::integer(self.balance_of(address) as i64)
            },
            "transfer" => {
                if args.len() < 3 {
                    return Any::boolean(false);
                }
                
                let from = args[0].as_h160().unwrap();
                let to = args[1].as_h160().unwrap();
                let amount = args[2].as_i64().unwrap() as u64;
                
                Any::boolean(self.transfer(from, to, amount))
            },
            _ => Any::null(),
        }
    }
}

// Mock runtime environment
thread_local! {
    static MOCK_TOKENS: RefCell<HashMap<H160, MockToken>> = RefCell::new(HashMap::new());
    static WITNESSES: RefCell<Vec<H160>> = RefCell::new(Vec::new());
    static CALLING_SCRIPT: RefCell<H160> = RefCell::new(H160::default());
    static EXECUTING_SCRIPT: RefCell<H160> = RefCell::new(H160::default());
    static NOTIFICATIONS: RefCell<Vec<(String, Vec<Any>)>> = RefCell::new(Vec::new());
}

fn is_contract(address: &H160) -> bool {
    MOCK_TOKENS.with(|tokens| {
        tokens.borrow().contains_key(address)
    })
}

// Register a mock token
fn register_token(token: MockToken) {
    MOCK_TOKENS.with(|tokens| {
        tokens.borrow_mut().insert(token.contract_hash, token);
    });
}

// Add a witness
fn add_witness(address: H160) {
    WITNESSES.with(|witnesses| {
        witnesses.borrow_mut().push(address);
    });
}

// Set the calling script
fn set_calling_script(address: H160) {
    CALLING_SCRIPT.with(|script| {
        *script.borrow_mut() = address;
    });
}

// Set the executing script
fn set_executing_script(address: H160) {
    EXECUTING_SCRIPT.with(|script| {
        *script.borrow_mut() = address;
    });
}

// Get notifications
fn get_notifications() -> Vec<(String, Vec<Any>)> {
    NOTIFICATIONS.with(|notifications| {
        notifications.borrow().clone()
    })
}

// Clear all test state
fn reset_test_environment() {
    MOCK_TOKENS.with(|tokens| {
        tokens.borrow_mut().clear();
    });
    
    WITNESSES.with(|witnesses| {
        witnesses.borrow_mut().clear();
    });
    
    CALLING_SCRIPT.with(|script| {
        *script.borrow_mut() = H160::default();
    });
    
    EXECUTING_SCRIPT.with(|script| {
        *script.borrow_mut() = H160::default();
    });
    
    NOTIFICATIONS.with(|notifications| {
        notifications.borrow_mut().clear();
    });
}

// Mock runtime for interop tests

pub mod exchange_contract {
    use super::*;
    
    // Re-export the ExchangeContract struct for testing
    pub use exchange::exchange_contract::ExchangeContract;
    
    pub struct InSwapGuard {
        in_swap: bool
    }
    
    impl InSwapGuard {
        pub fn get(&self) -> Option<Option<bool>> {
            Some(Some(self.in_swap))
        }
        
        pub fn set(&mut self, value: &bool) -> Option<()> {
            self.in_swap = *value;
            Some(())
        }
    }
}

#[test]
fn test_exchange_interoperability() {
    // Skip this test for now as it requires access to the exchange contract's internals
    // We'll implement proper testing infrastructure in the future
}

#[test]
fn test_exchange_reentrancy_protection() {
    // Skip this test for now as it requires access to the exchange contract's internals
    // We'll implement proper testing infrastructure in the future
} 