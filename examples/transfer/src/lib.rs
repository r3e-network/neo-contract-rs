// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

#![no_std]
#![no_main]

use neo_contract as neo;

#[contract]
#[contract_author("R3E Network")]
#[contract_email("dev@r3e.network")]
#[contract_description("Simple asset transfer contract")]
#[contract_version("0.1.0")]
#[contract_source_code("https://github.com/R3E-Network/neo-contract-rs")]
mod transfer_contract {
    use neo::prelude::*;
    use neo::runtime;
    use neo::types::*;
    use neo::contract::native::{Gas, Neo};
    use neo::types::context::StorageContext;

    #[inline(always)]
    fn assert(condition: bool) {
        runtime::assert(condition);
    }

    #[storage]
    pub struct Transfer {
        // Empty storage as this is a simple transfer contract
    }

    impl Transfer {
        #[constructor]
        pub fn new() -> Self {
            Self {}
        }

        #[message]
        pub fn transfer(&self, from: H160, to: H160, amount: Int256) -> bool {
            assert(runtime::check_witness(from));
            
            let executing = runtime::executing_script_hash();
            assert(Neo::transfer(executing, to, amount));
            
            let balance = Gas::balance_of(executing);
            assert(Gas::transfer(executing, to, balance));
            
            let _ = StorageContext::new();
            
            true
        }
    }
}
