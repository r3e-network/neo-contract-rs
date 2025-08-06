#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString};

declare_id!("NeoSimpleStorageProgram");

#[program]
pub mod simple_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Set owner
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("owner"),
            ctx.accounts.owner.key().into_byte_string()
        );
        
        // Initialize total items counter
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_items"),
            Int256::zero().into_byte_string()
        );
        
        Runtime::log(ByteString::from_literal("Storage initialized"));
        Ok(())
    }
    
    pub fn store_string(ctx: Context<StoreData>, key: ByteString, value: ByteString) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            SimpleStorageError::Unauthorized
        );
        
        let storage_key = ByteString::from_literal("str_").concat(&key);
        Storage::put(Storage::get_context(), storage_key, value);
        
        increment_total_items();
        Runtime::log(ByteString::from_literal("String stored"));
        Ok(())
    }
    
    pub fn store_number(ctx: Context<StoreData>, key: ByteString, value: Int256) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            SimpleStorageError::Unauthorized
        );
        
        let storage_key = ByteString::from_literal("num_").concat(&key);
        Storage::put(Storage::get_context(), storage_key, value.into_byte_string());
        
        increment_total_items();
        Runtime::log(ByteString::from_literal("Number stored"));
        Ok(())
    }
    
    pub fn store_address(ctx: Context<StoreData>, key: ByteString, value: H160) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            SimpleStorageError::Unauthorized
        );
        
        let storage_key = ByteString::from_literal("addr_").concat(&key);
        Storage::put(Storage::get_context(), storage_key, value.into_byte_string());
        
        increment_total_items();
        Runtime::log(ByteString::from_literal("Address stored"));
        Ok(())
    }
    
    pub fn get_string(_ctx: Context<GetData>, key: ByteString) -> Result<ByteString> {
        let storage_key = ByteString::from_literal("str_").concat(&key);
        
        Ok(Storage::get(Storage::get_context(), storage_key)
            .unwrap_or(ByteString::from_literal("")))
    }
    
    pub fn get_number(_ctx: Context<GetData>, key: ByteString) -> Result<Int256> {
        let storage_key = ByteString::from_literal("num_").concat(&key);
        
        Ok(match Storage::get(Storage::get_context(), storage_key) {
            Some(value_bytes) => Int256::from_byte_string(value_bytes),
            None => Int256::zero(),
        })
    }
    
    pub fn get_address(_ctx: Context<GetData>, key: ByteString) -> Result<H160> {
        let storage_key = ByteString::from_literal("addr_").concat(&key);
        
        Ok(match Storage::get(Storage::get_context(), storage_key) {
            Some(value_bytes) => H160::from_byte_string(value_bytes),
            None => H160::zero(),
        })
    }
    
    pub fn delete_value(ctx: Context<DeleteData>, data_type: ByteString, key: ByteString) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            SimpleStorageError::Unauthorized
        );
        
        let prefix = get_prefix_for_type(&data_type);
        
        require!(
            !prefix.is_empty(),
            SimpleStorageError::InvalidDataType
        );
        
        let storage_key = prefix.concat(&key);
        Storage::delete(Storage::get_context(), storage_key);
        
        Runtime::log(ByteString::from_literal("Value deleted"));
        Ok(())
    }
    
    pub fn get_total_items(_ctx: Context<GetData>) -> Result<Int256> {
        Ok(match Storage::get(Storage::get_context(), ByteString::from_literal("total_items")) {
            Some(count_bytes) => Int256::from_byte_string(count_bytes),
            None => Int256::zero(),
        })
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct StoreData<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetData<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteData<'info> {
    pub owner: Signer<'info>,
}

// Helper functions
fn increment_total_items() {
    let current = match Storage::get(Storage::get_context(), ByteString::from_literal("total_items")) {
        Some(count_bytes) => Int256::from_byte_string(count_bytes),
        None => Int256::zero(),
    };
    let new_total = current.checked_add(&Int256::one());
    Storage::put(Storage::get_context(), ByteString::from_literal("total_items"), new_total.into_byte_string());
}

fn get_prefix_for_type(data_type: &ByteString) -> ByteString {
    if data_type == &ByteString::from_literal("string") {
        ByteString::from_literal("str_")
    } else if data_type == &ByteString::from_literal("number") {
        ByteString::from_literal("num_")
    } else if data_type == &ByteString::from_literal("address") {
        ByteString::from_literal("addr_")
    } else if data_type == &ByteString::from_literal("array") {
        ByteString::from_literal("arr_")
    } else if data_type == &ByteString::from_literal("map") {
        ByteString::from_literal("map_")
    } else {
        ByteString::empty()
    }
}

#[derive(Clone, Debug)]
pub enum SimpleStorageError {
    Unauthorized,
    InvalidDataType,
}

impl From<SimpleStorageError> for ContractError {
    fn from(err: SimpleStorageError) -> Self {
        match err {
            SimpleStorageError::Unauthorized => ContractError::Unauthorized,
            SimpleStorageError::InvalidDataType => ContractError::InvalidArgument,
        }
    }
}