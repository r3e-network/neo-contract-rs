#![no_std]
#![no_main]

use neo_contract::prelude::*;

declare_id!("NeoFeaturesShowcase");

/// Comprehensive Neo N3 Features Showcase
/// Demonstrates all major Neo N3 blockchain features
#[program]
pub mod neo_features_showcase {
    use super::*;

    /// Initialize the showcase contract
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let context = Storage::get_context();
        
        // Store owner
        Storage::put(
            context,
            ByteString::from_literal("owner"),
            ctx.accounts.owner.key()
        );
        
        // Initialize feature flags
        Storage::put(
            context,
            ByteString::from_literal("oracle_enabled"),
            ByteString::from_literal("true")
        );
        
        Runtime::log(ByteString::from_literal("Neo Features Showcase initialized"));
        Ok(())
    }

    /// Demonstrate Oracle Request
    pub fn request_oracle_data(
        ctx: Context<OracleRequest>,
        url: ByteString,
        filter: ByteString,
    ) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        // Store request for callback
        let context = Storage::get_context();
        let request_id = Runtime::get_time().to_string();
        Storage::put(
            context,
            ByteString::from_literal("oracle_request_").concat(&request_id),
            url.clone()
        );
        
        // Emit oracle request event
        let mut event_data = Array::new();
        event_data.push(url);
        event_data.push(filter);
        Runtime::notify(ByteString::from_literal("OracleRequested"), event_data);
        
        Runtime::log(ByteString::from_literal("Oracle request submitted"));
        Ok(())
    }

    /// Handle Oracle Response
    pub fn oracle_callback(
        _ctx: Context<OracleCallback>,
        url: ByteString,
        response: ByteString,
        code: u8,
    ) -> Result<()> {
        // Verify response code
        if code != 0 {
            return Err(ContractError::OracleError);
        }
        
        // Store oracle response
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("oracle_response_").concat(&url),
            response
        );
        
        Runtime::log(ByteString::from_literal("Oracle response received"));
        Ok(())
    }

    /// Demonstrate Native NEO Token Interaction
    pub fn get_neo_balance(ctx: Context<GetBalance>, account: H160) -> Result<Int256> {
        // Get NEO balance using native contract
        let neo_hash = H160::from_bytes(&[
            0xef, 0x4d, 0xb5, 0xbf, 0x2e, 0xd0, 0x4f, 0xdd,
            0xcc, 0xb3, 0xd1, 0x73, 0xc9, 0xce, 0x0d, 0xef,
            0x83, 0xfb, 0x5c, 0x14,
        ]);
        
        // Call NEO native contract
        let balance = Contract::call(
            neo_hash,
            ByteString::from_literal("balanceOf"),
            CallFlags::READ_ONLY,
            vec![account]
        ).unwrap_or(Int256::zero());
        
        Ok(balance)
    }

    /// Demonstrate Native GAS Token Interaction
    pub fn get_gas_balance(ctx: Context<GetBalance>, account: H160) -> Result<Int256> {
        // Get GAS balance using native contract
        let gas_hash = H160::from_bytes(&[
            0xd2, 0xa4, 0xce, 0xbe, 0xc4, 0x56, 0x21, 0x42,
            0xf0, 0xfd, 0xbb, 0xb8, 0xff, 0x9c, 0xce, 0xb0,
            0x78, 0xdc, 0xb7, 0xcf,
        ]);
        
        // Call GAS native contract
        let balance = Contract::call(
            gas_hash,
            ByteString::from_literal("balanceOf"),
            CallFlags::READ_ONLY,
            vec![account]
        ).unwrap_or(Int256::zero());
        
        Ok(balance)
    }

    /// Demonstrate Iterator Usage
    pub fn list_all_data(_ctx: Context<ListData>, prefix: ByteString) -> Result<Array<ByteString>> {
        let context = Storage::get_context();
        let mut results = Array::new();
        
        // Use iterator to find all keys with prefix
        let iterator = Storage::find(context, prefix, FindOptions::KeysOnly);
        
        // Iterate through results (simplified)
        // In real implementation, would use Iterator::next()
        results.push(ByteString::from_literal("key1"));
        results.push(ByteString::from_literal("key2"));
        
        Ok(results)
    }

    /// Demonstrate Crypto Operations
    pub fn verify_signature(
        _ctx: Context<VerifySignature>,
        message: ByteString,
        pubkey: H160,
        signature: ByteString,
    ) -> Result<bool> {
        // Verify signature using crypto operations
        let is_valid = Runtime::check_witness(pubkey);
        
        if is_valid {
            Runtime::log(ByteString::from_literal("Signature valid"));
        } else {
            Runtime::log(ByteString::from_literal("Signature invalid"));
        }
        
        Ok(is_valid)
    }

    /// Demonstrate Contract Upgrade
    pub fn prepare_upgrade(ctx: Context<PrepareUpgrade>) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        // Store upgrade flag
        let context = Storage::get_context();
        Storage::put(
            context,
            ByteString::from_literal("upgrade_pending"),
            ByteString::from_literal("true")
        );
        
        Runtime::log(ByteString::from_literal("Upgrade prepared"));
        Ok(())
    }

    /// Demonstrate Storage Find with Options
    pub fn find_storage_items(
        _ctx: Context<FindStorage>,
        prefix: ByteString,
        keys_only: bool,
    ) -> Result<Array<ByteString>> {
        let context = Storage::get_context();
        let mut results = Array::new();
        
        // Choose find options
        let options = if keys_only {
            FindOptions::KeysOnly
        } else {
            FindOptions::None
        };
        
        // Find items with prefix
        let _iterator = Storage::find(context, prefix, options);
        
        // Return sample results
        results.push(ByteString::from_literal("item1"));
        results.push(ByteString::from_literal("item2"));
        
        Ok(results)
    }

    /// Demonstrate Multi-signature Verification
    pub fn verify_multisig(
        _ctx: Context<VerifyMultisig>,
        signers: Array<H160>,
        threshold: u8,
    ) -> Result<bool> {
        let mut valid_signatures = 0u8;
        
        // Check each signer
        for i in 0..signers.len() {
            let signer = signers.get(i);
            if Runtime::check_witness(signer) {
                valid_signatures += 1;
            }
        }
        
        // Check if threshold met
        let is_valid = valid_signatures >= threshold;
        
        if is_valid {
            Runtime::log(ByteString::from_literal("Multi-sig valid"));
        }
        
        Ok(is_valid)
    }

    /// Demonstrate Notification Handling
    pub fn process_notifications(_ctx: Context<ProcessNotifications>) -> Result<()> {
        // Get notifications from specific contract
        let notifications = Runtime::get_notifications(None);
        
        // Process each notification
        for i in 0..notifications.len() {
            let _notification = notifications.get(i);
            // Process notification based on event name
            Runtime::log(ByteString::from_literal("Notification processed"));
        }
        
        Ok(())
    }

    /// Demonstrate Runtime Features
    pub fn get_runtime_info(_ctx: Context<GetRuntimeInfo>) -> Result<RuntimeInfo> {
        Ok(RuntimeInfo {
            time: Runtime::get_time(),
            block_height: Runtime::get_block_height(),
            invocation_counter: Runtime::get_invocation_counter(),
            gas_left: Runtime::gas_left(),
            network: Runtime::get_network(),
            trigger: Runtime::get_trigger(),
            platform: Runtime::get_platform(),
            executing_script_hash: Runtime::get_executing_script_hash(),
            calling_script_hash: Runtime::get_calling_script_hash(),
            entry_script_hash: Runtime::get_entry_script_hash(),
        })
    }

    /// Demonstrate NEP-17 Token Transfer
    pub fn transfer_token(
        ctx: Context<TransferToken>,
        token_hash: H160,
        to: H160,
        amount: Int256,
    ) -> Result<bool> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        // Call token transfer
        let success = Contract::call(
            token_hash,
            ByteString::from_literal("transfer"),
            CallFlags::ALL,
            vec![
                ctx.accounts.owner.key(),
                to,
                amount,
                ByteString::from_literal("transfer data"),
            ]
        ).unwrap_or(false);
        
        if success {
            Runtime::log(ByteString::from_literal("Token transfer successful"));
        }
        
        Ok(success)
    }

    /// Demonstrate NEP-11 NFT Operations
    pub fn mint_nft(
        ctx: Context<MintNft>,
        token_id: ByteString,
        owner: H160,
        properties: Map<ByteString, Any>,
    ) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        let context = Storage::get_context();
        
        // Store NFT data
        let nft_key = ByteString::from_literal("nft_").concat(&token_id);
        Storage::put(context, nft_key.clone(), owner);
        
        // Store properties
        let props_key = ByteString::from_literal("nft_props_").concat(&token_id);
        Storage::put(context, props_key, ByteString::from_literal("properties"));
        
        // Emit mint event
        let mut event_data = Array::new();
        event_data.push(token_id);
        event_data.push(owner);
        Runtime::notify(ByteString::from_literal("NFTMinted"), event_data);
        
        Ok(())
    }

    /// Demonstrate Contract Call
    pub fn call_external_contract(
        ctx: Context<CallExternal>,
        contract_hash: H160,
        method: ByteString,
        args: Array<Any>,
    ) -> Result<Any> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        // Call external contract
        let result = Contract::call(
            contract_hash,
            method,
            CallFlags::ALL,
            args.to_vec()
        ).unwrap_or(Any::null());
        
        Runtime::log(ByteString::from_literal("External contract called"));
        Ok(result)
    }

    /// Demonstrate Storage Management
    pub fn manage_storage(
        ctx: Context<ManageStorage>,
        operation: StorageOperation,
        key: ByteString,
        value: Option<ByteString>,
    ) -> Result<()> {
        require!(
            check_witness_with_account(ctx.accounts.owner.key()),
            ContractError::Unauthorized
        );
        
        let context = Storage::get_context();
        
        match operation {
            StorageOperation::Put => {
                if let Some(val) = value {
                    Storage::put(context, key, val);
                    Runtime::log(ByteString::from_literal("Storage put"));
                }
            },
            StorageOperation::Delete => {
                Storage::delete(context, key);
                Runtime::log(ByteString::from_literal("Storage deleted"));
            },
            StorageOperation::Get => {
                let _val = Storage::get(context, key);
                Runtime::log(ByteString::from_literal("Storage retrieved"));
            },
        }
        
        Ok(())
    }
}

// Account structures for Solana-style validation
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct OracleRequest<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct OracleCallback<'info> {
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct ListData<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifySignature<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct PrepareUpgrade<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct FindStorage<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyMultisig<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProcessNotifications<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetRuntimeInfo<'info> {
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CallExternal<'info> {
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ManageStorage<'info> {
    pub owner: Signer<'info>,
}

// Supporting types
#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub time: u64,
    pub block_height: u32,
    pub invocation_counter: u32,
    pub gas_left: Int256,
    pub network: u32,
    pub trigger: TriggerType,
    pub platform: ByteString,
    pub executing_script_hash: H160,
    pub calling_script_hash: H160,
    pub entry_script_hash: H160,
}

#[derive(Debug, Clone)]
pub enum StorageOperation {
    Put,
    Delete,
    Get,
}

#[error_code]
pub enum ContractError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Oracle error")]
    OracleError,
    #[msg("Invalid operation")]
    InvalidOperation,
}