#![no_std]
#![no_main]

extern crate alloc;

use neo_contract::prelude::*;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Panic handler for WASM no_std builds
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

declare_id!("NeoHelloWorldContract123456789");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, greeting: ByteString) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        let authority = &ctx.accounts.authority;
        
        greeting_account.data.authority = authority.key();
        greeting_account.data.greeting = greeting;
        greeting_account.data.visitor_count = Int256::zero();
        greeting_account.data.is_initialized = true;
        
        Runtime::notify(
            ByteString::from_literal("GreetingInitialized"),
            Array::new()
        );
        
        Ok(())
    }
    
    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<ByteString> {
        let greeting_account = &ctx.accounts.greeting_account;
        Ok(greeting_account.data.greeting.clone())
    }
    
    pub fn set_greeting(mut ctx: Context<SetGreeting>, new_greeting: ByteString) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        let authority = &ctx.accounts.authority;
        
        require_keys_eq!(
            greeting_account.data.authority,
            authority.key(),
            ContractError::Unauthorized
        );
        
        greeting_account.data.greeting = new_greeting.clone();
        
        Runtime::notify(
            ByteString::from_literal("GreetingChanged"),
            Array::new()
        );
        
        Ok(())
    }
    
    pub fn say_hello(mut ctx: Context<SayHello>, visitor_name: ByteString) -> Result<ByteString> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        
        greeting_account.data.visitor_count = greeting_account.data.visitor_count
            .checked_add(&Int256::one());
        
        let response = ByteString::from_literal("Hello, ");
        
        Runtime::notify(
            ByteString::from_literal("VisitorRegistered"),
            Array::new()
        );
        
        Ok(response)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 256)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetGreeting<'info> {
    pub greeting_account: Account<'info, GreetingAccount>,
}

#[derive(Accounts)]
pub struct SetGreeting<'info> {
    #[account(mut, has_one = authority)]
    pub greeting_account: Account<'info, GreetingAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SayHello<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    pub visitor: Signer<'info>,
}

// Simple struct without Solana-specific attributes
pub struct GreetingAccount {
    pub authority: H160,
    pub greeting: ByteString,
    pub visitor_count: Int256,
    pub is_initialized: bool,
}

// Use the ContractError from prelude instead of defining our own
// ContractError is already available from neo_contract::prelude