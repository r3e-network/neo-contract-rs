use crate::error::ContractError;

extern crate alloc;
use alloc::vec::Vec;

/// Trait for account serialization
pub trait AccountSerialize {
    fn try_serialize(&self, buf: &mut alloc::vec::Vec<u8>) -> Result<(), ContractError>;
}

/// Trait for account deserialization  
pub trait AccountDeserialize: Sized {
    fn try_deserialize(buf: &[u8]) -> Result<Self, ContractError>;
}

/// Trait for account validation
pub trait AccountValidation {
    fn validate(&self) -> Result<(), ContractError>;
}

/// Account loader for lazy loading
pub struct AccountLoader<'info, T> {
    acc_info: crate::context::AccountInfo<'info>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'info, T> AccountLoader<'info, T> 
where
    T: AccountDeserialize + AccountSerialize
{
    pub fn load(&self) -> Result<T, ContractError> {
        T::try_deserialize(self.acc_info.data)
    }
    
    pub fn load_mut(&mut self) -> Result<T, ContractError> {
        if !self.acc_info.is_writable {
            return Err(ContractError::AccountNotMutable);
        }
        T::try_deserialize(self.acc_info.data)
    }
}

/// Account metadata
#[derive(Debug, Clone)]
pub struct AccountMeta {
    pub pubkey: crate::types::H160,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl AccountMeta {
    pub fn new(pubkey: crate::types::H160, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: true,
        }
    }
    
    pub fn new_readonly(pubkey: crate::types::H160, is_signer: bool) -> Self {
        Self {
            pubkey,
            is_signer,
            is_writable: false,
        }
    }
}

/// Trait for converting to account info
pub trait ToAccountInfo<'info> {
    fn to_account_info(&self) -> crate::context::AccountInfo<'info>;
}

/// Trait for converting to account metas
pub trait ToAccountMetas {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta>;
}

/// Empty account for initialization
#[derive(Debug, Clone)]
pub struct EmptyAccount;

impl AccountSerialize for EmptyAccount {
    fn try_serialize(&self, _buf: &mut Vec<u8>) -> Result<(), ContractError> {
        Ok(())
    }
}

impl AccountDeserialize for EmptyAccount {
    fn try_deserialize(_buf: &[u8]) -> Result<Self, ContractError> {
        Ok(EmptyAccount)
    }
}