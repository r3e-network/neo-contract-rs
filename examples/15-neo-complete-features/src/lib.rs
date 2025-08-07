#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

use neo_contract::prelude::*;

declare_id!("NeoCompleteFeatures");

/// Comprehensive Neo N3 Features Demonstration
/// Shows all native contracts, NEP standards, and advanced features
#[program]
pub mod neo_complete_features {
    use super::*;

    /// Initialize the contract
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.data.owner = ctx.accounts.authority.key();
        config.data.is_initialized = true;
        
        Runtime::log(ByteString::from_literal("Contract initialized with all Neo N3 features"));
        Ok(())
    }

    /// Demonstrate CryptoLib features
    pub fn test_cryptolib(ctx: Context<TestCrypto>, data: ByteString) -> Result<()> {
        // SHA256 hash
        let sha256_hash = CryptoLib::sha256(data.clone());
        Runtime::log(ByteString::from_literal("SHA256 computed"));
        
        // RIPEMD160 hash
        let ripemd_hash = CryptoLib::ripemd160(data.clone());
        Runtime::log(ByteString::from_literal("RIPEMD160 computed"));
        
        // Keccak256 hash (Ethereum compatible)
        let keccak_hash = CryptoLib::keccak256(data.clone());
        Runtime::log(ByteString::from_literal("Keccak256 computed"));
        
        // Murmur32 hash
        let murmur_hash = CryptoLib::murmur32(data, 42);
        Runtime::log(ByteString::from_literal("Murmur32 computed"));
        
        emit!(CryptoTestEvent {
            sha256: sha256_hash,
            ripemd160: ripemd_hash,
            keccak256: keccak_hash,
            murmur32: murmur_hash,
        });
        
        Ok(())
    }

    /// Demonstrate NEO governance features
    pub fn test_governance(ctx: Context<TestGovernance>) -> Result<()> {
        // Register as candidate
        let pubkey = ctx.accounts.candidate.key_as_public_key()?;
        let registered = NeoGovernance::register_candidate(pubkey);
        
        if registered {
            Runtime::log(ByteString::from_literal("Registered as consensus candidate"));
        }
        
        // Vote for candidate
        let voter = ctx.accounts.voter.key();
        let voted = NeoGovernance::vote(voter, Some(pubkey));
        
        if voted {
            Runtime::log(ByteString::from_literal("Voted for candidate"));
        }
        
        // Get candidates list
        let candidates = NeoGovernance::get_candidates();
        Runtime::log(ByteString::from_literal("Retrieved candidates list"));
        
        // Get committee members
        let committee = NeoGovernance::get_committee();
        Runtime::log(ByteString::from_literal("Retrieved committee members"));
        
        // Get GAS per block
        let gas_per_block = NeoGovernance::get_gas_per_block();
        
        emit!(GovernanceEvent {
            registered,
            voted,
            candidate_count: candidates.length() as u32,
            committee_size: committee.length() as u32,
            gas_per_block,
        });
        
        Ok(())
    }

    /// Demonstrate Oracle features
    pub fn request_oracle_data(ctx: Context<OracleRequest>, url: ByteString) -> Result<()> {
        use crate::neo_features::oracle;
        
        // Request external data from oracle
        oracle::request(
            url.clone(),
            oracle::OracleFilter::JsonPath(ByteString::from_literal("$.price")),
            ByteString::from_literal("oracleResponse"),
            Any::null(),
            Int256::from(1000000), // 0.01 GAS for response
        )?;
        
        Runtime::log(ByteString::from_literal("Oracle request sent"));
        
        emit!(OracleRequestEvent {
            url,
            callback: ByteString::from_literal("oracleResponse"),
        });
        
        Ok(())
    }

    /// Oracle response callback
    pub fn oracle_response(ctx: Context<OracleResponse>, url: ByteString, data: Any, code: u8) -> Result<()> {
        require_eq!(
            code,
            0,
            ContractError::OracleError
        );
        
        // Store oracle data
        let storage = Storage::get_context();
        Storage::put(
            storage,
            ByteString::from_literal("oracle_data"),
            data.as_bytes().unwrap_or_default()
        );
        
        emit!(OracleResponseEvent {
            url,
            success: true,
        });
        
        Ok(())
    }

    /// Demonstrate StdLib extended features
    pub fn test_stdlib_extended(ctx: Context<TestStdLib>, data: ByteString) -> Result<()> {
        // Base58 encoding
        let base58_encoded = StdLibExtended::base58_encode(data.clone());
        let base58_decoded = StdLibExtended::base58_decode(base58_encoded.clone());
        
        // Base58 check encoding (with checksum)
        let base58_check = StdLibExtended::base58_check_encode(data.clone());
        
        // String operations
        let separator = ByteString::from_literal(",");
        let parts = StdLibExtended::string_split(data.clone(), separator);
        
        // Memory operations
        let comparison = StdLibExtended::memory_compare(data.clone(), data.clone());
        
        emit!(StdLibEvent {
            base58: base58_encoded,
            parts_count: parts.length() as u32,
            comparison,
        });
        
        Ok(())
    }

    /// Demonstrate NEP-24 royalty features
    pub fn setup_royalty(
        ctx: Context<SetupRoyalty>,
        token_id: ByteString,
        recipient: Pubkey,
        amount: u16,
    ) -> Result<()> {
        require!(
            NEP24Implementation::validate_royalty(amount),
            ContractError::InvalidRoyalty
        );
        
        NEP24Implementation::store_royalty(token_id.clone(), recipient, amount);
        
        emit!(RoyaltySetEvent {
            token_id,
            recipient,
            amount,
        });
        
        Ok(())
    }

    /// Demonstrate NEP-26/27 transfer callbacks
    pub fn safe_transfer_with_callback(
        ctx: Context<SafeTransfer>,
        to: Pubkey,
        amount: Int256,
        data: Any,
    ) -> Result<()> {
        let from = ctx.accounts.from.key();
        
        // Perform safe transfer with callback
        TransferCallback::safe_token_transfer(from, to, amount, data)?;
        
        Runtime::log(ByteString::from_literal("Safe transfer with callback completed"));
        
        Ok(())
    }

    /// Demonstrate iterator features
    pub fn test_iterators(ctx: Context<TestIterators>) -> Result<()> {
        use crate::services::iterator::Iterator;
        
        let storage = Storage::get_context();
        
        // Store some test data
        for i in 0..5 {
            let key = ByteString::from_literal("item_")
                .concat(&ByteString::from(i.to_string().as_bytes()));
            let value = Int256::from(i as i64);
            Storage::put(storage.clone(), key, value.into_any());
        }
        
        // Use iterator to find items
        let prefix = ByteString::from_literal("item_");
        let iter = Storage::find(storage, prefix, FindOptions::default());
        
        let mut count = 0;
        while Iterator::next(iter.clone()) {
            count += 1;
            let key = Iterator::key(iter.clone());
            let value = Iterator::value(iter.clone());
            
            Runtime::log(ByteString::from_literal("Found item"));
        }
        
        emit!(IteratorEvent {
            items_found: count,
        });
        
        Ok(())
    }

    /// Demonstrate all native contracts
    pub fn test_all_natives(ctx: Context<TestNatives>) -> Result<()> {
        // NEO contract
        let neo_balance = neo::get_balance(ctx.accounts.account.key());
        let neo_name = neo::get_name();
        
        // GAS contract
        let gas_balance = gas::get_balance(ctx.accounts.account.key());
        let gas_name = gas::get_name();
        
        // Policy contract
        use crate::contract::native::Policy;
        let fee_per_byte = Policy::get_fee_per_byte();
        let storage_price = Policy::get_storage_price();
        
        // Oracle contract
        use crate::contract::native::Oracle;
        let oracle_price = Oracle::get_price();
        
        // Management contract
        use crate::contract::native::ContractManagement;
        let min_deployment_fee = ContractManagement::get_minimum_deployment_fee();
        
        emit!(NativeContractsEvent {
            neo_balance,
            gas_balance,
            fee_per_byte,
            storage_price,
            oracle_price,
            min_deployment_fee,
        });
        
        Ok(())
    }

    /// Demonstrate multi-signature features
    pub fn test_multisig(ctx: Context<TestMultiSig>) -> Result<()> {
        let pubkeys = ctx.accounts.signers.iter()
            .map(|s| s.key_as_public_key())
            .collect::<Result<Vec<_>>>()?;
        
        // Create multi-sig account (2 of 3)
        let multisig_account = Contract::create_multisig_account(2, pubkeys.into());
        
        Runtime::log(ByteString::from_literal("Multi-sig account created"));
        
        emit!(MultiSigEvent {
            account: multisig_account,
            threshold: 2,
            signers: 3,
        });
        
        Ok(())
    }

    /// Demonstrate BLS12-381 cryptography
    pub fn test_bls12_381(ctx: Context<TestBLS>, x: ByteString, y: ByteString) -> Result<()> {
        // BLS12-381 operations
        let sum = CryptoLib::bls12_381_add(x.clone(), y.clone());
        let product = CryptoLib::bls12_381_mul(x.clone(), y.clone(), false);
        let pairing = CryptoLib::bls12_381_pairing(x, y);
        
        emit!(BLSEvent {
            computed: true,
        });
        
        Ok(())
    }
}

// Account structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 64)]
    pub config: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TestCrypto<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TestGovernance<'info> {
    pub candidate: Signer<'info>,
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
pub struct OracleRequest<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct OracleResponse<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TestStdLib<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetupRoyalty<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SafeTransfer<'info> {
    pub from: Signer<'info>,
}

#[derive(Accounts)]
pub struct TestIterators<'info> {
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TestNatives<'info> {
    pub account: Signer<'info>,
}

#[derive(Accounts)]
pub struct TestMultiSig<'info> {
    pub signers: Vec<Signer<'info>>,
}

#[derive(Accounts)]
pub struct TestBLS<'info> {
    pub authority: Signer<'info>,
}

// Account data structures
#[account]
pub struct ConfigAccount {
    pub owner: Pubkey,
    pub is_initialized: bool,
}

// Events
#[event]
pub struct CryptoTestEvent {
    pub sha256: ByteString,
    pub ripemd160: ByteString,
    pub keccak256: ByteString,
    pub murmur32: ByteString,
}

#[event]
pub struct GovernanceEvent {
    pub registered: bool,
    pub voted: bool,
    pub candidate_count: u32,
    pub committee_size: u32,
    pub gas_per_block: Int256,
}

#[event]
pub struct OracleRequestEvent {
    pub url: ByteString,
    pub callback: ByteString,
}

#[event]
pub struct OracleResponseEvent {
    pub url: ByteString,
    pub success: bool,
}

#[event]
pub struct StdLibEvent {
    pub base58: ByteString,
    pub parts_count: u32,
    pub comparison: i32,
}

#[event]
pub struct RoyaltySetEvent {
    pub token_id: ByteString,
    pub recipient: Pubkey,
    pub amount: u16,
}

#[event]
pub struct IteratorEvent {
    pub items_found: u32,
}

#[event]
pub struct NativeContractsEvent {
    pub neo_balance: Int256,
    pub gas_balance: Int256,
    pub fee_per_byte: Int256,
    pub storage_price: Int256,
    pub oracle_price: Int256,
    pub min_deployment_fee: Int256,
}

#[event]
pub struct MultiSigEvent {
    pub account: H160,
    pub threshold: u32,
    pub signers: u32,
}

#[event]
pub struct BLSEvent {
    pub computed: bool,
}

// Error codes
#[error_code]
pub enum ContractError {
    #[msg("Oracle request failed")]
    OracleError,
    #[msg("Invalid royalty amount")]
    InvalidRoyalty,
}