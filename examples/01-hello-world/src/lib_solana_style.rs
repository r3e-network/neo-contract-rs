//! # Hello World Smart Contract - Solana Style
//!
//! A Neo N3 smart contract using Solana-style syntax demonstrating:
//! - Program module pattern
//! - Context-based account access
//! - Account validation with constraints
//! - Type-safe error handling
//! - Event emission

#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use neo_contract::prelude::*;

// Declare the program ID (Neo N3 contract address)
declare_id!("NeoHelloWorldProgram123456789");

/// Main program module using Solana-style syntax
#[program]
pub mod hello_world {
    use super::*;

    /// Initialize the contract with a greeting
    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        let state_account = &mut ctx.accounts.state;
        let authority = &ctx.accounts.authority;
        
        // Ensure not already initialized
        require!(!state_account.is_initialized, ErrorCode::AlreadyInitialized);
        
        // Set initial state
        state_account.greeting = greeting.clone();
        state_account.authority = authority.key();
        state_account.visitor_count = 0;
        state_account.is_initialized = true;
        
        // Emit initialization event
        emit!(ProgramInitialized {
            authority: authority.key(),
            greeting,
        });
        
        msg!("Program initialized with greeting: {}", state_account.greeting);
        Ok(())
    }
    
    /// Get the current greeting (read-only)
    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<String> {
        let state = &ctx.accounts.state;
        Ok(state.greeting.clone())
    }
    
    /// Set a new greeting (authority required)
    pub fn set_greeting(ctx: Context<SetGreeting>, new_greeting: String) -> Result<()> {
        let state = &mut ctx.accounts.state;
        
        // Validate greeting
        require!(
            !new_greeting.is_empty() && new_greeting.len() <= 100,
            ErrorCode::InvalidGreeting
        );
        
        let old_greeting = state.greeting.clone();
        state.greeting = new_greeting.clone();
        
        emit!(GreetingChanged {
            old_greeting,
            new_greeting,
            authority: ctx.accounts.authority.key(),
        });
        
        Ok(())
    }
    
    /// Register a new visitor
    pub fn say_hello(ctx: Context<SayHello>, visitor_name: String) -> Result<String> {
        let state = &mut ctx.accounts.state;
        let visitor = &ctx.accounts.visitor;
        
        // Validate visitor name
        require!(
            !visitor_name.is_empty() && visitor_name.len() <= 50,
            ErrorCode::InvalidVisitorName
        );
        
        // Increment visitor count
        state.visitor_count = state.visitor_count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
        
        // Add to recent visitors
        let visitor_record = VisitorRecord {
            name: visitor_name.clone(),
            address: visitor.key(),
            timestamp: Clock::get()?.unix_timestamp,
            visitor_number: state.visitor_count,
        };
        
        state.recent_visitors.push(visitor_record.clone());
        
        // Keep only last 10 visitors
        if state.recent_visitors.len() > 10 {
            state.recent_visitors.remove(0);
        }
        
        emit!(VisitorRegistered {
            visitor: visitor_record,
        });
        
        // Return personalized greeting
        Ok(format!("Hello, {}! {}", visitor_name, state.greeting))
    }
    
    /// Get visitor count
    pub fn get_visitor_count(ctx: Context<GetVisitorCount>) -> Result<u64> {
        Ok(ctx.accounts.state.visitor_count)
    }
    
    /// Get recent visitors
    pub fn get_recent_visitors(ctx: Context<GetRecentVisitors>) -> Result<Vec<VisitorRecord>> {
        Ok(ctx.accounts.state.recent_visitors.clone())
    }
    
    /// Get contract info
    pub fn get_info(ctx: Context<GetInfo>) -> Result<ContractInfo> {
        let state = &ctx.accounts.state;
        Ok(ContractInfo {
            name: "Hello World Contract".to_string(),
            version: "2.0.0".to_string(),
            author: "Neo Rust Framework".to_string(),
            visitor_count: state.visitor_count,
            current_greeting: state.greeting.clone(),
        })
    }
    
    /// Reset contract state (authority only)
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        
        // Reset to defaults
        state.greeting = "Hello, Neo N3 World!".to_string();
        state.visitor_count = 0;
        state.recent_visitors.clear();
        
        emit!(ContractReset {
            authority: ctx.accounts.authority.key(),
        });
        
        msg!("Contract state has been reset");
        Ok(())
    }
}

// ===== Account Validation Structures =====

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + StateAccount::SIZE)]
    pub state: Account<'info, StateAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetGreeting<'info> {
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct SetGreeting<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, StateAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SayHello<'info> {
    #[account(mut)]
    pub state: Account<'info, StateAccount>,
    pub visitor: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetVisitorCount<'info> {
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct GetRecentVisitors<'info> {
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct GetInfo<'info> {
    pub state: Account<'info, StateAccount>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, StateAccount>,
    pub authority: Signer<'info>,
}

// ===== Account Data Structures =====

#[account]
pub struct StateAccount {
    pub greeting: String,
    pub authority: Pubkey,
    pub visitor_count: u64,
    pub recent_visitors: Vec<VisitorRecord>,
    pub is_initialized: bool,
}

impl StateAccount {
    pub const SIZE: usize = 32 + // authority
        4 + 100 + // greeting string
        8 + // visitor_count
        4 + (10 * VisitorRecord::SIZE) + // recent_visitors
        1; // is_initialized
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VisitorRecord {
    pub name: String,
    pub address: Pubkey,
    pub timestamp: i64,
    pub visitor_number: u64,
}

impl VisitorRecord {
    pub const SIZE: usize = 4 + 50 + // name
        32 + // address
        8 + // timestamp
        8; // visitor_number
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ContractInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub visitor_count: u64,
    pub current_greeting: String,
}

// ===== Error Codes =====

#[error_code]
pub enum ErrorCode {
    #[msg("The contract has already been initialized")]
    AlreadyInitialized,
    
    #[msg("Invalid greeting: must be 1-100 characters")]
    InvalidGreeting,
    
    #[msg("Invalid visitor name: must be 1-50 characters")]
    InvalidVisitorName,
    
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    
    #[msg("Unauthorized: caller is not the authority")]
    Unauthorized,
}

// ===== Events =====

#[event]
pub struct ProgramInitialized {
    pub authority: Pubkey,
    pub greeting: String,
}

#[event]
pub struct GreetingChanged {
    pub old_greeting: String,
    pub new_greeting: String,
    pub authority: Pubkey,
}

#[event]
pub struct VisitorRegistered {
    pub visitor: VisitorRecord,
}

#[event]
pub struct ContractReset {
    pub authority: Pubkey,
}

// ===== Helper Macros =====

/// Emit an event
#[macro_export]
macro_rules! emit {
    ($event:expr) => {
        {
            neo_contract::emit_event($event)
        }
    };
}

/// Log a message
#[macro_export]
macro_rules! msg {
    ($($arg:tt)*) => {
        {
            neo_contract::log_message(&format!($($arg)*))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_record_size() {
        assert!(VisitorRecord::SIZE >= 94);
    }

    #[test]
    fn test_state_account_size() {
        assert!(StateAccount::SIZE >= 1085);
    }
}