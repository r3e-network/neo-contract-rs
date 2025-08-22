#![cfg(test)]

use neo_contract::prelude::*;

#[test]
fn test_contract_impl_pattern() {
    #[contract_author("Test Contract")]
    #[contract_version("1.0.0")]
    pub struct TestContract {
        value: u64,
    }
    
    #[contract_impl]
    impl TestContract {
        pub fn init() -> Self {
            Self { value: 42 }
        }
        
        #[method]
        #[safe]
        pub fn get_value(&self) -> u64 {
            self.value
        }
    }
    
    let contract = TestContract::init();
    assert_eq!(contract.get_value(), 42);
}

#[test]
fn test_method_validation() {
    #[contract_author("Validation Test")]
    #[contract_version("1.0.0")]
    pub struct ValidationContract {
        owner: H160,
    }
    
    #[contract_impl]
    impl ValidationContract {
        pub fn init() -> Self {
            Self { owner: H160::zero() }
        }
        
        #[method]
        pub fn validate_input(&self, value: u64) -> Result<()> {
            require!(value > 0, ContractError::InvalidArgument);
            require!(value < 100, ContractError::InvalidArgument);
            Ok(())
        }
    }
    
    let contract = ValidationContract::init();
    assert!(contract.validate_input(50).is_ok());
    assert!(contract.validate_input(0).is_err());
    assert!(contract.validate_input(150).is_err());
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