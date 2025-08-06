#![no_std]
#![no_main]

use neo_contract::prelude::*;
use neo_contract::types::builtin::IntoAny;
use neo_contract::serialize::{Serialize, Deserialize};

declare_id!("NeoHelloWorldContract123456789");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greeting: ByteString) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        let authority = &ctx.accounts.authority;
        
        greeting_account.authority = *authority.key();
        greeting_account.greeting = greeting;
        greeting_account.visitor_count = Int256::zero();
        greeting_account.is_initialized = true;
        
        Runtime::notify(
            ByteString::from_literal("GreetingInitialized"),
            Array::new()
        );
        
        Ok(())
    }
    
    #[safe_attr]
    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<ByteString> {
        let greeting_account = &ctx.accounts.greeting_account;
        Ok(greeting_account.greeting.clone())
    }
    
    pub fn set_greeting(ctx: Context<SetGreeting>, new_greeting: ByteString) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        let authority = &ctx.accounts.authority;
        
        require_keys_eq!(
            greeting_account.authority,
            *authority.key(),
            ContractError::Unauthorized
        );
        
        greeting_account.greeting = new_greeting.clone();
        
        Runtime::notify(
            ByteString::from_literal("GreetingChanged"),
            Array::new()
        );
        
        Ok(())
    }
    
    pub fn say_hello(ctx: Context<SayHello>, visitor_name: ByteString) -> Result<ByteString> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        
        greeting_account.visitor_count = greeting_account.visitor_count
            .checked_add(&Int256::one())
            .unwrap_or(Int256::zero());
        
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

#[account_attr]
pub struct GreetingAccount {
    pub authority: H160,
    pub greeting: ByteString,
    pub visitor_count: Int256,
    pub is_initialized: bool,
}