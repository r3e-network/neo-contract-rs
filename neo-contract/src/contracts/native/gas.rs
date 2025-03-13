// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// GAS native contract for Neo N3
/// 
/// GAS is the fuel token for the Neo N3 blockchain, used to pay for transaction fees
/// and smart contract execution costs.
/// 
/// Contract Hash: 0xd2a4cff31913016155e38e474a2c06d08be276cf
#[allow(non_snake_case)]
pub struct GAS;

impl GAS {
    /// Returns the contract hash for the GAS native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::gas_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_gas_contract_hash()
        }
    }

    /// Gets the symbol of the GAS token
    /// 
    /// # Returns
    /// 
    /// The symbol as a ByteString, which is "GAS"
    #[safe]
    pub fn symbol() -> ByteString {
        let method = ByteString::from("symbol");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or_else(|_| ByteString::from("GAS"))
    }

    /// Gets the decimals of the GAS token
    /// 
    /// # Returns
    /// 
    /// The number of decimals, which is 8
    #[safe]
    pub fn decimals() -> u8 {
        let method = ByteString::from("decimals");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(8)
    }

    /// Gets the total supply of GAS tokens
    /// 
    /// # Returns
    /// 
    /// The total supply as an integer
    #[safe]
    pub fn total_supply() -> u64 {
        let method = ByteString::from("totalSupply");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets the balance of GAS for an account
    /// 
    /// # Arguments
    /// 
    /// * `account` - The script hash of the account to check
    /// 
    /// # Returns
    /// 
    /// The account balance as an integer
    #[safe]
    pub fn balance_of(account: &H160) -> u64 {
        let method = ByteString::from("balanceOf");
        let mut args = Array::<Any>::new();
        args.push(Any::from(account.clone()));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Transfers GAS from the calling contract to another account
    /// 
    /// # Arguments
    /// 
    /// * `from` - The script hash of the sending account
    /// * `to` - The script hash of the receiving account
    /// * `amount` - The amount of GAS to transfer
    /// * `data` - Optional data to include with the transfer
    /// 
    /// # Returns
    /// 
    /// True if the transfer was successful, false otherwise
    pub fn transfer(from: &H160, to: &H160, amount: u64, data: Option<Any>) -> bool {
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(from.clone()));
        args.push(Any::from(to.clone()));
        args.push(Any::from(amount));
        
        if let Some(transfer_data) = data {
            args.push(transfer_data);
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(false)
    }
}
