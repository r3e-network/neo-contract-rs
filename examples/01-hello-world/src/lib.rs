#![no_std]
#![no_main]

use neo_contract::prelude::*;

declare_id!("NeoHelloWorldProgram");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // Get storage context
        let context = Storage::get_context();
        
        // Initialize the greeting
        Storage::put(
            context,
            ByteString::from_literal("greeting"),
            ByteString::from_literal("Hello, Neo N3!")
        );
        
        Runtime::log(ByteString::from_literal("Contract initialized"));
        Ok(())
    }
    
    pub fn say_hello(_ctx: Context<SayHello>, _name: ByteString) -> Result<ByteString> {
        let context = Storage::get_context();
        let greeting = Storage::get(context, ByteString::from_literal("greeting"))
            .unwrap_or(ByteString::from_literal("Hello"));
        
        Runtime::log(ByteString::from_literal("Say hello called"));
        Runtime::notify(ByteString::from_literal("HelloEvent"), Array::new());
        
        Ok(greeting)
    }
    
    pub fn set_greeting(ctx: Context<SetGreeting>, new_greeting: ByteString) -> Result<()> {
        // Check authorization using address
        let owner = ctx.accounts.owner;
        require!(
            check_witness_with_account(owner.key()),
            ContractError::Unauthorized
        );
        
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("greeting"),
            new_greeting
        );
        
        Runtime::log(ByteString::from_literal("Greeting updated"));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct SayHello<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetGreeting<'info> {
    pub owner: Signer<'info>,
}

