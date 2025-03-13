// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::h256::H256;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// Ledger native contract for Neo N3
/// 
/// This contract provides access to blockchain information such as blocks,
/// transactions, and contracts on the Neo N3 blockchain.
/// 
/// Contract Hash: 0xda65b600f7124ce6c79950c1772a36403104f2be
#[allow(non_snake_case)]
pub struct Ledger;

impl Ledger {
    /// Returns the contract hash for the Ledger native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::ledger_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_ledger_contract_hash()
        }
    }

    /// Gets the hash of the current block
    /// 
    /// # Returns
    /// 
    /// The current block hash as H256
    #[safe]
    pub fn current_hash() -> H256 {
        let method = ByteString::from("currentHash");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| H256::zero())
    }

    /// Gets the index of the current block
    /// 
    /// # Returns
    /// 
    /// The current block index as u32
    #[safe]
    pub fn current_index() -> u32 {
        let method = ByteString::from("currentIndex");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets a block by its hash
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the block to retrieve
    /// 
    /// # Returns
    /// 
    /// The block as an Any value
    #[safe]
    pub fn get_block(hash: &H256) -> Any {
        let method = ByteString::from("getBlock");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Gets a block by its index
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the block to retrieve
    /// 
    /// # Returns
    /// 
    /// The block as an Any value
    #[safe]
    pub fn get_block_by_index(index: u32) -> Any {
        let method = ByteString::from("getBlock");
        let mut args = Array::<Any>::new();
        args.push(Any::from(index));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Gets a transaction by its hash
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the transaction to retrieve
    /// 
    /// # Returns
    /// 
    /// The transaction as an Any value
    #[safe]
    pub fn get_transaction(hash: &H256) -> Any {
        let method = ByteString::from("getTransaction");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Gets a transaction height by its hash
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the transaction
    /// 
    /// # Returns
    /// 
    /// The height (block index) of the transaction, or -1 if not found
    #[safe]
    pub fn get_transaction_height(hash: &H256) -> i32 {
        let method = ByteString::from("getTransactionHeight");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        match result.try_into::<i32>() {
            Ok(height) => height,
            Err(_) => -1,
        }
    }

    /// Gets a transaction from a block by its index
    /// 
    /// # Arguments
    /// 
    /// * `block_index` - The index of the block
    /// * `tx_index` - The index of the transaction within the block
    /// 
    /// # Returns
    /// 
    /// The transaction as an Any value
    #[safe]
    pub fn get_transaction_from_block(block_index: u32, tx_index: i32) -> Any {
        let method = ByteString::from("getTransactionFromBlock");
        let mut args = Array::<Any>::new();
        args.push(Any::from(block_index));
        args.push(Any::from(tx_index));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }

    /// Gets a contract state by its hash
    /// 
    /// # Arguments
    /// 
    /// * `hash` - The hash of the contract to retrieve
    /// 
    /// # Returns
    /// 
    /// The contract state as an Any value
    #[safe]
    pub fn get_contract_state(hash: &H160) -> Any {
        let method = ByteString::from("getContractState");
        let mut args = Array::<Any>::new();
        args.push(Any::from(hash.clone()));
        
        Runtime::call_contract(&Self::hash(), &method, &args)
    }
}
