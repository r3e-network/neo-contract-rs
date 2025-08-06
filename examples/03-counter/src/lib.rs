#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::{IntoByteString, FromByteString};

declare_id!("NeoCounterProgram");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        
        // Set owner
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("owner"),
            ctx.accounts.owner.key().into_byte_string()
        );
        
        // Initialize default counter
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("default_counter"),
            Int256::zero().into_byte_string()
        );
        
        // Initialize statistics
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_ops"),
            Int256::zero().into_byte_string()
        );
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_counters"),
            Int256::zero().into_byte_string()
        );
        
        Runtime::log(ByteString::from_literal("Counter initialized"));
        Ok(())
    }
    
    pub fn increment(ctx: Context<Increment>) -> Result<Int256> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        let current = get_counter_value(&ByteString::from_literal("default_counter"));
        let new_value = current.checked_add(&Int256::one());
        
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("default_counter"),
            new_value.into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Counter incremented"));
        Runtime::notify(ByteString::from_literal("CounterIncremented"), Array::new());
        
        Ok(new_value)
    }
    
    pub fn decrement(ctx: Context<Decrement>) -> Result<Int256> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        let current = get_counter_value(&ByteString::from_literal("default_counter"));
        let new_value = current.checked_sub(&Int256::one());
        
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("default_counter"),
            new_value.into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Counter decremented"));
        Runtime::notify(ByteString::from_literal("CounterDecremented"), Array::new());
        
        Ok(new_value)
    }
    
    pub fn add(ctx: Context<Add>, amount: Int256) -> Result<Int256> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        require!(
            amount > Int256::zero(),
            CounterError::InvalidAmount
        );
        
        let current = get_counter_value(&ByteString::from_literal("default_counter"));
        let new_value = current.checked_add(&amount);
        
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("default_counter"),
            new_value.into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Amount added to counter"));
        
        Ok(new_value)
    }
    
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            CounterError::Unauthorized
        );
        
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("default_counter"),
            Int256::zero().into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Counter reset"));
        Runtime::notify(ByteString::from_literal("CounterReset"), Array::new());
        
        Ok(())
    }
    
    pub fn create_counter(ctx: Context<CreateCounter>, name: ByteString, initial_value: Int256) -> Result<()> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        require!(
            validate_counter_name(&name),
            CounterError::InvalidCounterName
        );
        
        let counter_key = ByteString::from_literal("counter_").concat(&name);
        
        // Check if counter already exists
        require!(
            Storage::get(Storage::get_context(), counter_key.clone()).is_none(),
            CounterError::CounterExists
        );
        
        Storage::put(Storage::get_context(), counter_key, initial_value.into_byte_string());
        
        // Update total counters
        let total = get_counter_value(&ByteString::from_literal("total_counters"));
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_counters"),
            total.checked_add(&Int256::one()).into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Counter created"));
        
        Ok(())
    }
    
    pub fn increment_counter(ctx: Context<IncrementCounter>, name: ByteString) -> Result<Int256> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        require!(
            validate_counter_name(&name),
            CounterError::InvalidCounterName
        );
        
        let counter_key = ByteString::from_literal("counter_").concat(&name);
        let current = get_counter_value(&counter_key);
        let new_value = current.checked_add(&Int256::one());
        
        Storage::put(Storage::get_context(), counter_key, new_value.into_byte_string());
        
        record_operation();
        Runtime::log(ByteString::from_literal("Named counter incremented"));
        
        Ok(new_value)
    }
    
    pub fn decrement_counter(ctx: Context<DecrementCounter>, name: ByteString) -> Result<Int256> {
        require!(
            is_authorized(&ctx.accounts.authority.key()),
            CounterError::Unauthorized
        );
        
        require!(
            validate_counter_name(&name),
            CounterError::InvalidCounterName
        );
        
        let counter_key = ByteString::from_literal("counter_").concat(&name);
        let current = get_counter_value(&counter_key);
        let new_value = current.checked_sub(&Int256::one());
        
        Storage::put(Storage::get_context(), counter_key, new_value.into_byte_string());
        
        record_operation();
        Runtime::log(ByteString::from_literal("Named counter decremented"));
        
        Ok(new_value)
    }
    
    pub fn delete_counter(ctx: Context<DeleteCounter>, name: ByteString) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            CounterError::Unauthorized
        );
        
        require!(
            validate_counter_name(&name),
            CounterError::InvalidCounterName
        );
        
        let counter_key = ByteString::from_literal("counter_").concat(&name);
        
        // Check if counter exists
        require!(
            Storage::get(Storage::get_context(), counter_key.clone()).is_some(),
            CounterError::CounterNotFound
        );
        
        Storage::delete(Storage::get_context(), counter_key);
        
        // Update total counters
        let total = get_counter_value(&ByteString::from_literal("total_counters"));
        Storage::put(
            Storage::get_context(),
            ByteString::from_literal("total_counters"),
            total.checked_sub(&Int256::one()).into_byte_string()
        );
        
        record_operation();
        Runtime::log(ByteString::from_literal("Counter deleted"));
        
        Ok(())
    }
    
    pub fn add_operator(ctx: Context<AddOperator>, operator: H160) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            CounterError::Unauthorized
        );
        
        let operator_key = ByteString::from_literal("op_").concat(&operator.into_byte_string());
        Storage::put(Storage::get_context(), operator_key, ByteString::from_literal("true"));
        
        Runtime::log(ByteString::from_literal("Operator added"));
        Ok(())
    }
    
    pub fn remove_operator(ctx: Context<RemoveOperator>, operator: H160) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            CounterError::Unauthorized
        );
        
        let operator_key = ByteString::from_literal("op_").concat(&operator.into_byte_string());
        Storage::delete(Storage::get_context(), operator_key);
        
        Runtime::log(ByteString::from_literal("Operator removed"));
        Ok(())
    }
    
    pub fn get_value(_ctx: Context<GetValue>) -> Result<Int256> {
        Ok(get_counter_value(&ByteString::from_literal("default_counter")))
    }
    
    pub fn get_counter(_ctx: Context<GetCounter>, name: ByteString) -> Result<Int256> {
        require!(
            validate_counter_name(&name),
            CounterError::InvalidCounterName
        );
        
        let counter_key = ByteString::from_literal("counter_").concat(&name);
        Ok(get_counter_value(&counter_key))
    }
    
    pub fn get_stats(_ctx: Context<GetStats>) -> Result<CounterStats> {
        
        Ok(CounterStats {
            default_counter: get_counter_value(&ByteString::from_literal("default_counter")),
            total_operations: get_counter_value(&ByteString::from_literal("total_ops")),
            total_counters: get_counter_value(&ByteString::from_literal("total_counters")),
        })
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DecrementCounter<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteCounter<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddOperator<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RemoveOperator<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetValue<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCounter<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetStats<'info> {
    pub caller: Signer<'info>,
}

#[derive(Debug, Clone)]
pub struct CounterStats {
    pub default_counter: Int256,
    pub total_operations: Int256,
    pub total_counters: Int256,
}

// Helper functions
fn get_counter_value(key: &ByteString) -> Int256 {
    match Storage::get(Storage::get_context(), key.clone()) {
        Some(value_bytes) => Int256::from_byte_string(value_bytes),
        None => Int256::zero(),
    }
}

fn is_authorized(authority: &H160) -> bool {
    // Check if owner
    if let Some(owner_bytes) = Storage::get(Storage::get_context(), ByteString::from_literal("owner")) {
        let owner = H160::from_byte_string(owner_bytes);
        if check_witness_with_account(owner) {
            return true;
        }
    }
    
    // Check if operator
    let operator_key = ByteString::from_literal("op_").concat(&authority.into_byte_string());
    if Storage::get(Storage::get_context(), operator_key).is_some() {
        return check_witness_with_account(*authority);
    }
    
    false
}

fn validate_counter_name(name: &ByteString) -> bool {
    !name.is_empty() && name.len() <= 32
}

fn record_operation() {
    let total_ops = get_counter_value(&ByteString::from_literal("total_ops"));
    Storage::put(
        Storage::get_context(),
        ByteString::from_literal("total_ops"),
        total_ops.checked_add(&Int256::one()).into_byte_string()
    );
    
    let timestamp = Runtime::get_time();
    Storage::put(
        Storage::get_context(),
        ByteString::from_literal("last_op"),
        ByteString::from_bytes(&timestamp.to_le_bytes())
    );
}

#[derive(Clone, Debug)]
pub enum CounterError {
    Unauthorized,
    InvalidAmount,
    InvalidCounterName,
    CounterExists,
    CounterNotFound,
}

impl From<CounterError> for ContractError {
    fn from(err: CounterError) -> Self {
        match err {
            CounterError::Unauthorized => ContractError::Unauthorized,
            CounterError::InvalidAmount => ContractError::InvalidArgument,
            CounterError::InvalidCounterName => ContractError::InvalidArgument,
            CounterError::CounterExists => ContractError::InvalidArgument,
            CounterError::CounterNotFound => ContractError::InvalidArgument,
        }
    }
}