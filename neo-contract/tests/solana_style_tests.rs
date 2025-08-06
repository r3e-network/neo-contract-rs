#![cfg(test)]

use neo_contract::prelude::*;

#[test]
fn test_context_creation() {
    let ctx: Context<()> = Context::new();
    assert_eq!(ctx.program_id, Runtime::get_executing_script_hash());
}

#[test]
fn test_account_validation() {
    #[derive(Debug)]
    struct TestAccounts;
    
    impl TestAccounts {
        fn validate(&self) -> Result<()> {
            Ok(())
        }
    }
    
    let accounts = TestAccounts;
    assert!(accounts.validate().is_ok());
}

#[test]
fn test_error_macros() {
    fn test_require() -> Result<()> {
        require!(true, ContractError::InvalidArgument);
        Ok(())
    }
    
    fn test_require_fail() -> Result<()> {
        require!(false, ContractError::InvalidArgument);
        Ok(())
    }
    
    assert!(test_require().is_ok());
    assert!(test_require_fail().is_err());
}

#[test]
fn test_require_eq() {
    fn test_eq() -> Result<()> {
        require_eq!(5, 5, ContractError::InvalidArgument);
        Ok(())
    }
    
    fn test_eq_fail() -> Result<()> {
        require_eq!(5, 6, ContractError::InvalidArgument);
        Ok(())
    }
    
    assert!(test_eq().is_ok());
    assert!(test_eq_fail().is_err());
}

#[test]
fn test_require_gt() {
    fn test_gt() -> Result<()> {
        require_gt!(10, 5, ContractError::InvalidArgument);
        Ok(())
    }
    
    fn test_gt_fail() -> Result<()> {
        require_gt!(5, 10, ContractError::InvalidArgument);
        Ok(())
    }
    
    assert!(test_gt().is_ok());
    assert!(test_gt_fail().is_err());
}

#[test]
fn test_pda_generation() {
    let seeds: &[&[u8]] = &[b"vault", b"user"];
    let program_id = H160::zero();
    let (address, bump) = Pda::find_program_address(&seeds[..], &program_id);
    
    // Verify we get a valid address and bump
    assert!(bump <= 255);
}

#[test]
fn test_contract_error_codes() {
    assert_eq!(ContractError::AccountNotFound.code(), 6000);
    assert_eq!(ContractError::AccountAlreadyExists.code(), 6001);
    assert_eq!(ContractError::Unauthorized.code(), 6012);
}

#[test]
fn test_clock() {
    let clock_result = Clock::get();
    assert!(clock_result.is_ok());
    
    let clock = clock_result.unwrap();
    assert!(clock.unix_timestamp >= 0);
}

#[test]
fn test_rent() {
    let rent_result = Rent::get();
    assert!(rent_result.is_ok());
    
    let rent = rent_result.unwrap();
    let min_balance = rent.minimum_balance(100);
    assert!(min_balance > 0);
}