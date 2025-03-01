// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use neo_contract::{
    contract::SmartContract,
    types::*,
};

#[neo_contract::contract]
mod token {
    use super::*;
    
    #[neo(storage)]
    pub struct Token {
        total_supply: Int256,
        balances: builtin::Map,
    }
    
    impl Token {
        #[neo(constructor)]
        pub fn new(initial_supply: Int256) -> Self {
            let mut balances = builtin::Map::new();
            let owner = runtime::calling_script_hash();
            
            balances.put(&owner, &initial_supply);
            
            Self {
                total_supply: initial_supply,
                balances,
            }
        }
        
        #[neo(message)]
        pub fn total_supply(&self) -> Int256 {
            self.total_supply
        }
        
        #[neo(message)]
        pub fn balance_of(&self, account: H160) -> Int256 {
            match self.balances.get(&account) {
                Some(balance) => balance,
                None => Int256::zero(),
            }
        }
        
        #[neo(event)]
        pub fn transfer(from: Option<H160>, to: Option<H160>, amount: Int256) {}
    }
    
    impl SmartContract for Token {}
}
