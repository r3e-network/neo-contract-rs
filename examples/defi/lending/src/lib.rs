//! # Lending Protocol Smart Contract
//!
//! A decentralized lending platform that allows users to supply assets to earn interest
//! and borrow assets by providing collateral.
//!
//! This contract implements:
//! - Multiple asset support
//! - Variable interest rates
//! - Collateralized borrowing
//! - Liquidation mechanisms
//! - Risk parameters management

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Lending Protocol Smart Contract for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod neo_lending {
    use neo_contract::prelude::*;
    
    /// Asset market information
    #[derive(Debug, Clone, Encode, Decode)]
    struct Market {
        /// Token hash of the asset
        token: Hash160,
        
        /// Total amount of the asset supplied to the protocol
        total_supply: u64,
        
        /// Total amount of the asset borrowed from the protocol
        total_borrows: u64,
        
        /// Total amount of reserves of the asset
        total_reserves: u64,
        
        /// Exchange rate between asset and corresponding aToken (scaled by 1e18)
        exchange_rate_mantissa: u64,
        
        /// Interest rate model parameters
        /// Base rate (scaled by 1e18)
        base_rate_per_block: u64,
        
        /// Multiplier for utilization rate (scaled by 1e18)
        multiplier_per_block: u64,
        
        /// Jump multiplier for high utilization (scaled by 1e18)
        jump_multiplier_per_block: u64,
        
        /// Utilization rate at which the jump multiplier is applied (scaled by 1e18)
        kink: u64,
        
        /// Reserve factor (scaled by 1e18)
        reserve_factor_mantissa: u64,
        
        /// Collateral factor (scaled by 1e18)
        collateral_factor_mantissa: u64,
        
        /// Last updated block number
        accrual_block_number: u64,
        
        /// Borrow interest rate (scaled by 1e18)
        borrow_rate_mantissa: u64,
        
        /// Supply interest rate (scaled by 1e18)
        supply_rate_mantissa: u64,
        
        /// aToken representing supply position
        a_token: Hash160,
    }
    
    /// User's supply and borrow information for a specific asset
    #[derive(Debug, Clone, Encode, Decode)]
    struct AccountAssets {
        /// User account
        account: Address,
        
        /// Token hash
        token: Hash160,
        
        /// Amount supplied (in aTokens)
        supplied: u64,
        
        /// Amount borrowed
        borrowed: u64,
        
        /// Whether this asset is used as collateral
        is_collateral: bool,
    }
    
    /// Oracle price data for an asset
    #[derive(Debug, Clone, Encode, Decode)]
    struct PriceData {
        /// Token hash
        token: Hash160,
        
        /// Price in USD (scaled by 1e8)
        price: u64,
        
        /// Last update timestamp
        last_updated: u64,
        
        /// Price source (oracle address)
        source: Address,
    }
    
    /// Liquidation event details
    #[derive(Debug, Clone, Encode, Decode)]
    struct LiquidationEvent {
        /// Liquidator address
        liquidator: Address,
        
        /// Borrower being liquidated
        borrower: Address,
        
        /// Repaid asset
        repay_token: Hash160,
        
        /// Amount repaid
        repay_amount: u64,
        
        /// Seized collateral asset
        collateral_token: Hash160,
        
        /// Amount of collateral seized
        collateral_amount: u64,
        
        /// Timestamp of liquidation
        timestamp: u64,
    }
    
    /// Events emitted by the lending protocol
    struct MarketListed {}
    
    impl MarketListed {
        pub fn emit(token: Hash160, a_token: Hash160) {
            let event_name = ByteString::from("MarketListed");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(token));
            event_data.push(Any::from(a_token));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct Supply {}
    
    impl Supply {
        pub fn emit(supplier: Address, token: Hash160, amount: u64, tokens_minted: u64) {
            let event_name = ByteString::from("Supply");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(supplier));
            event_data.push(Any::from(token));
            event_data.push(Any::from(amount));
            event_data.push(Any::from(tokens_minted));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct Withdraw {}
    
    impl Withdraw {
        pub fn emit(supplier: Address, token: Hash160, amount: u64, tokens_burned: u64) {
            let event_name = ByteString::from("Withdraw");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(supplier));
            event_data.push(Any::from(token));
            event_data.push(Any::from(amount));
            event_data.push(Any::from(tokens_burned));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct Borrow {}
    
    impl Borrow {
        pub fn emit(borrower: Address, token: Hash160, amount: u64) {
            let event_name = ByteString::from("Borrow");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(borrower));
            event_data.push(Any::from(token));
            event_data.push(Any::from(amount));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct Repay {}
    
    impl Repay {
        pub fn emit(payer: Address, borrower: Address, token: Hash160, amount: u64) {
            let event_name = ByteString::from("Repay");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(payer));
            event_data.push(Any::from(borrower));
            event_data.push(Any::from(token));
            event_data.push(Any::from(amount));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct LiquidateBorrow {}
    
    impl LiquidateBorrow {
        pub fn emit(liquidator: Address, borrower: Address, repay_token: Hash160, repay_amount: u64, collateral_token: Hash160, seize_amount: u64) {
            let event_name = ByteString::from("LiquidateBorrow");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(liquidator));
            event_data.push(Any::from(borrower));
            event_data.push(Any::from(repay_token));
            event_data.push(Any::from(repay_amount));
            event_data.push(Any::from(collateral_token));
            event_data.push(Any::from(seize_amount));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct NewReserveFactor {}
    
    impl NewReserveFactor {
        pub fn emit(token: Hash160, old_factor_mantissa: u64, new_factor_mantissa: u64) {
            let event_name = ByteString::from("NewReserveFactor");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(token));
            event_data.push(Any::from(old_factor_mantissa));
            event_data.push(Any::from(new_factor_mantissa));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    struct NewCollateralFactor {}
    
    impl NewCollateralFactor {
        pub fn emit(token: Hash160, old_factor_mantissa: u64, new_factor_mantissa: u64) {
            let event_name = ByteString::from("NewCollateralFactor");
            let mut event_data = Array::<Any>::new();
            
            event_data.push(Any::from(token));
            event_data.push(Any::from(old_factor_mantissa));
            event_data.push(Any::from(new_factor_mantissa));
            
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    /// Lending protocol storage
    #[storage]
    struct NeoLending {
        /// Contract owner
        owner: Item<Address>,
        
        /// List of market tokens
        market_tokens: Item<Vec<Hash160>>,
        
        /// Maps token hash to market
        markets: Map<Hash160, Market>,
        
        /// Maps token to whether it's listed
        market_listed: Map<Hash160, bool>,
        
        /// Maps (account, token) to account assets
        account_assets: Map<(Address, Hash160), AccountAssets>,
        
        /// Maps account to list of supplied assets
        account_supplied_assets: Map<Address, Vec<Hash160>>,
        
        /// Maps account to list of borrowed assets
        account_borrowed_assets: Map<Address, Vec<Hash160>>,
        
        /// Maps token to price data
        prices: Map<Hash160, PriceData>,
        
        /// Maps token to whether it can be used as collateral
        collateral_asset_allowed: Map<Hash160, bool>,
        
        /// Price oracle contract hash
        price_oracle: Item<Hash160>,
        
        /// Liquidation incentive (scaled by 1e18)
        liquidation_incentive_mantissa: Item<u64>,
        
        /// Close factor (scaled by 1e18)
        close_factor_mantissa: Item<u64>,
        
        /// Last liquidation event ID
        next_liquidation_id: Item<u32>,
        
        /// Maps liquidation ID to liquidation event
        liquidation_history: Map<u32, LiquidationEvent>,
        
        /// Protocol share of liquidation (scaled by 1e18)
        protocol_seize_share_mantissa: Item<u64>,
        
        /// Protocol fee collector address
        fee_collector: Item<Address>,
    }
    
    /// Constants for math calculations
    const MANTISSA_DECIMALS: u8 = 18;
    const MANTISSA_ONE: u64 = 1_000_000_000_000_000_000; // 1e18
    const HALF_MANTISSA: u64 = 500_000_000_000_000_000;  // 0.5e18
    
    impl NeoLending {
        /// Initialize the lending protocol
        #[constructor]
        fn new(owner: Address, price_oracle: Hash160) -> Self {
            Self {
                owner: Item::new(owner),
                market_tokens: Item::new(Vec::new()),
                markets: Map::new(),
                market_listed: Map::new(),
                account_assets: Map::new(),
                account_supplied_assets: Map::new(),
                account_borrowed_assets: Map::new(),
                prices: Map::new(),
                collateral_asset_allowed: Map::new(),
                price_oracle: Item::new(price_oracle),
                liquidation_incentive_mantissa: Item::new(MANTISSA_ONE + MANTISSA_ONE / 10), // 1.1e18 (10% bonus)
                close_factor_mantissa: Item::new(MANTISSA_ONE / 2), // 0.5e18 (50%)
                next_liquidation_id: Item::new(1),
                liquidation_history: Map::new(),
                protocol_seize_share_mantissa: Item::new(MANTISSA_ONE / 20), // 0.05e18 (5%)
                fee_collector: Item::new(owner), // Initially set to owner
            }
        }
        
        /// Lists a new asset market (admin only)
        #[method]
        fn list_market(
            &mut self,
            token: Hash160,
            a_token: Hash160,
            base_rate: u64,
            multiplier: u64,
            jump_multiplier: u64,
            kink: u64,
            collateral_factor: u64,
            reserve_factor: u64,
        ) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can list markets");
            
            // Validate parameters
            assert!(!self.market_listed.get(&token).unwrap_or(false), "Market already listed");
            assert!(collateral_factor <= MANTISSA_ONE * 9 / 10, "Collateral factor too high"); // Max 90%
            assert!(reserve_factor <= MANTISSA_ONE / 2, "Reserve factor too high"); // Max 50%
            
            // Create new market
            let market = Market {
                token,
                total_supply: 0,
                total_borrows: 0,
                total_reserves: 0,
                exchange_rate_mantissa: MANTISSA_ONE, // Initially 1:1
                base_rate_per_block: base_rate,
                multiplier_per_block: multiplier,
                jump_multiplier_per_block: jump_multiplier,
                kink,
                reserve_factor_mantissa: reserve_factor,
                collateral_factor_mantissa: collateral_factor,
                accrual_block_number: runtime::get_block().index,
                borrow_rate_mantissa: 0,
                supply_rate_mantissa: 0,
                a_token,
            };
            
            // Add market to storage
            self.markets.insert(token, market);
            self.market_listed.insert(token, true);
            
            // Add to market tokens list
            let mut market_tokens = self.market_tokens.get().clone();
            market_tokens.push(token);
            self.market_tokens.set(market_tokens);
            
            // Set asset as allowed collateral
            self.collateral_asset_allowed.insert(token, true);
            
            // Emit event
            MarketListed::emit(token, a_token);
            
            true
        }
        
        /// Supply assets to the protocol
        #[method]
        fn supply(&mut self, token: Hash160, amount: u64) -> bool {
            let supplier = runtime::calling_script_hash();
            
            // Verify supplier signature
            assert!(runtime::check_witness(&supplier), "Invalid signature");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Update market with accrued interest
            self.accrue_interest(token);
            
            // Get market data
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Transfer tokens from supplier to contract
            self.transfer_token_to_contract(&token, &supplier, amount);
            
            // Calculate aTokens to mint
            let a_tokens_to_mint = if market.total_supply == 0 {
                // First supply, 1:1 rate
                amount
            } else {
                // a_tokens = amount * exchange_rate
                self.div_u64(
                    self.mul_u64(amount, market.exchange_rate_mantissa),
                    MANTISSA_ONE
                )
            };
            
            // Update supplier's account assets
            let account_key = (supplier, token);
            let mut account_assets = self.account_assets.get(&account_key).unwrap_or_else(|| {
                // Add token to account's supplied assets list
                let mut supplied_assets = self.account_supplied_assets.get(&supplier).unwrap_or_default();
                if !supplied_assets.contains(&token) {
                    supplied_assets.push(token);
                    self.account_supplied_assets.insert(supplier, supplied_assets);
                }
                
                // Create new account assets record
                AccountAssets {
                    account: supplier,
                    token,
                    supplied: 0,
                    borrowed: 0,
                    is_collateral: true, // Default to using as collateral
                }
            });
            
            // Update supplied amount
            account_assets.supplied += a_tokens_to_mint;
            self.account_assets.insert(account_key, account_assets);
            
            // Update market
            market.total_supply += amount;
            self.markets.insert(token, market);
            
            // Emit event
            Supply::emit(supplier, token, amount, a_tokens_to_mint);
            
            true
        }
        
        /// Withdraw supplied assets
        #[method]
        fn withdraw(&mut self, token: Hash160, amount: u64) -> bool {
            let supplier = runtime::calling_script_hash();
            
            // Verify supplier signature
            assert!(runtime::check_witness(&supplier), "Invalid signature");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Update market with accrued interest
            self.accrue_interest(token);
            
            // Get market data
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Calculate aTokens to burn
            let a_tokens_to_burn = self.div_u64(
                self.mul_u64(amount, MANTISSA_ONE),
                market.exchange_rate_mantissa
            );
            
            // Get supplier's account assets
            let account_key = (supplier, token);
            let mut account_assets = self.account_assets.get(&account_key).expect("No supply position");
            
            // Verify sufficient aTokens
            assert!(account_assets.supplied >= a_tokens_to_burn, "Insufficient supply");
            
            // Check if withdrawal would leave insufficient collateral
            if account_assets.is_collateral {
                // Calculate hypothetical account state after withdrawal
                account_assets.supplied -= a_tokens_to_burn;
                
                // Check if account would remain solvent
                let is_solvent = self.is_account_solvent(&supplier);
                
                // Revert the hypothetical state change
                account_assets.supplied += a_tokens_to_burn;
                
                // Enforce solvency check
                assert!(is_solvent, "Withdrawal would leave insufficient collateral");
            }
            
            // Update supplier's account assets
            account_assets.supplied -= a_tokens_to_burn;
            
            // If no supply left, remove from supplied assets list
            if account_assets.supplied == 0 && account_assets.borrowed == 0 {
                self.account_assets.remove(&account_key);
                
                // Remove token from supplied assets list
                let mut supplied_assets = self.account_supplied_assets.get(&supplier).unwrap_or_default();
                if let Some(index) = supplied_assets.iter().position(|t| t == &token) {
                    supplied_assets.remove(index);
                    self.account_supplied_assets.insert(supplier, supplied_assets);
                }
            } else {
                self.account_assets.insert(account_key, account_assets);
            }
            
            // Update market
            assert!(market.total_supply >= amount, "Insufficient market liquidity");
            market.total_supply -= amount;
            self.markets.insert(token, market);
            
            // Transfer tokens from contract to supplier
            self.transfer_token_from_contract(&token, &supplier, amount);
            
            // Emit event
            Withdraw::emit(supplier, token, amount, a_tokens_to_burn);
            
            true
        }
        
        /// Borrow assets from the protocol
        #[method]
        fn borrow(&mut self, token: Hash160, amount: u64) -> bool {
            let borrower = runtime::calling_script_hash();
            
            // Verify borrower signature
            assert!(runtime::check_witness(&borrower), "Invalid signature");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Update market with accrued interest
            self.accrue_interest(token);
            
            // Get market data
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Verify sufficient liquidity
            let available_to_borrow = market.total_supply - market.total_borrows;
            assert!(available_to_borrow >= amount, "Insufficient liquidity");
            
            // Update borrower's account assets
            let account_key = (borrower, token);
            let mut account_assets = self.account_assets.get(&account_key).unwrap_or_else(|| {
                // Add token to account's borrowed assets list
                let mut borrowed_assets = self.account_borrowed_assets.get(&borrower).unwrap_or_default();
                if !borrowed_assets.contains(&token) {
                    borrowed_assets.push(token);
                    self.account_borrowed_assets.insert(borrower, borrowed_assets);
                }
                
                // Create new account assets record
                AccountAssets {
                    account: borrower,
                    token,
                    supplied: 0,
                    borrowed: 0,
                    is_collateral: false, // Not used as collateral by default
                }
            });
            
            // Update borrowed amount
            account_assets.borrowed += amount;
            self.account_assets.insert(account_key, account_assets);
            
            // Check if account would remain solvent after borrow
            assert!(self.is_account_solvent(&borrower), "Borrow would exceed collateral limit");
            
            // Update market
            market.total_borrows += amount;
            self.markets.insert(token, market);
            
            // Transfer tokens from contract to borrower
            self.transfer_token_from_contract(&token, &borrower, amount);
            
            // Emit event
            Borrow::emit(borrower, token, amount);
            
            true
        }
        
        /// Repay borrowed assets
        #[method]
        fn repay(&mut self, token: Hash160, amount: u64, borrower: Option<Address>) -> bool {
            let repayer = runtime::calling_script_hash();
            
            // Verify repayer signature
            assert!(runtime::check_witness(&repayer), "Invalid signature");
            
            // Determine the borrower (self or specified account)
            let actual_borrower = borrower.unwrap_or(repayer);
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Update market with accrued interest
            self.accrue_interest(token);
            
            // Get account assets
            let account_key = (actual_borrower, token);
            let mut account_assets = self.account_assets.get(&account_key).expect("No borrow position");
            
            // Determine repay amount (cap at current borrow balance)
            let repay_amount = if amount > account_assets.borrowed {
                account_assets.borrowed
            } else {
                amount
            };
            
            // Transfer tokens from repayer to contract
            self.transfer_token_to_contract(&token, &repayer, repay_amount);
            
            // Update borrower's account assets
            account_assets.borrowed -= repay_amount;
            
            // If no borrow left, remove token from borrowed assets list
            if account_assets.borrowed == 0 {
                let mut borrowed_assets = self.account_borrowed_assets.get(&actual_borrower).unwrap_or_default();
                if let Some(index) = borrowed_assets.iter().position(|t| t == &token) {
                    borrowed_assets.remove(index);
                    self.account_borrowed_assets.insert(actual_borrower, borrowed_assets);
                }
            }
            
            // If no supply and borrow, remove account assets
            if account_assets.supplied == 0 && account_assets.borrowed == 0 {
                self.account_assets.remove(&account_key);
            } else {
                self.account_assets.insert(account_key, account_assets);
            }
            
            // Update market
            let mut market = self.markets.get(&token).expect("Market not found");
            market.total_borrows -= repay_amount;
            self.markets.insert(token, market);
            
            // Emit event
            Repay::emit(repayer, actual_borrower, token, repay_amount);
            
            true
        }
        
        /// Liquidate an undercollateralized borrower
        #[method]
        fn liquidate_borrow(
            &mut self,
            borrower: Address,
            repay_token: Hash160,
            collateral_token: Hash160,
            repay_amount: u64
        ) -> bool {
            let liquidator = runtime::calling_script_hash();
            
            // Verify liquidator signature
            assert!(runtime::check_witness(&liquidator), "Invalid signature");
            
            // Verify both markets exist
            assert!(self.market_listed.get(&repay_token).unwrap_or(false), "Repay market not listed");
            assert!(self.market_listed.get(&collateral_token).unwrap_or(false), "Collateral market not listed");
            
            // Verify different tokens
            assert!(repay_token != collateral_token, "Same token not allowed");
            
            // Update markets with accrued interest
            self.accrue_interest(repay_token);
            self.accrue_interest(collateral_token);
            
            // Verify borrower is underwater
            assert!(!self.is_account_solvent(&borrower), "Account not liquidatable");
            
            // Get borrower's repay asset position
            let repay_key = (borrower, repay_token);
            let mut borrower_repay_assets = self.account_assets.get(&repay_key).expect("No borrow position");
            
            // Get close factor
            let close_factor = *self.close_factor_mantissa.get();
            
            // Calculate maximum repayable amount
            let max_repayable = self.mul_scalar_truncate(borrower_repay_assets.borrowed, close_factor);
            
            // Cap repay amount
            let actual_repay_amount = if repay_amount > max_repayable {
                max_repayable
            } else {
                repay_amount
            };
            
            // Verify non-zero repay
            assert!(actual_repay_amount > 0, "Repay amount too small");
            
            // Get borrower's collateral asset position
            let collateral_key = (borrower, collateral_token);
            let mut borrower_collateral_assets = self.account_assets.get(&collateral_key).expect("No collateral position");
            
            // Verify collateral is enabled
            assert!(borrower_collateral_assets.is_collateral, "Asset not used as collateral");
            
            // Get markets
            let mut repay_market = self.markets.get(&repay_token).expect("Repay market not found");
            let mut collateral_market = self.markets.get(&collateral_token).expect("Collateral market not found");
            
            // Calculate seize amount
            let (seize_tokens, protocol_seize_amount) = self.calculate_seize_amount(
                repay_token,
                collateral_token,
                actual_repay_amount
            );
            
            // Verify sufficient collateral
            assert!(borrower_collateral_assets.supplied >= seize_tokens, "Insufficient collateral");
            
            // Transfer repay tokens from liquidator to contract
            self.transfer_token_to_contract(&repay_token, &liquidator, actual_repay_amount);
            
            // Update borrower's repay asset position
            borrower_repay_assets.borrowed -= actual_repay_amount;
            
            // If no borrow left, remove token from borrowed assets list
            if borrower_repay_assets.borrowed == 0 {
                let mut borrowed_assets = self.account_borrowed_assets.get(&borrower).unwrap_or_default();
                if let Some(index) = borrowed_assets.iter().position(|t| t == &repay_token) {
                    borrowed_assets.remove(index);
                    self.account_borrowed_assets.insert(borrower, borrowed_assets);
                }
            }
            
            // If no supply and borrow, remove borrower's repay asset position
            if borrower_repay_assets.supplied == 0 && borrower_repay_assets.borrowed == 0 {
                self.account_assets.remove(&repay_key);
            } else {
                self.account_assets.insert(repay_key, borrower_repay_assets);
            }
            
            // Update borrower's collateral position
            borrower_collateral_assets.supplied -= seize_tokens;
            
            // If no supply left, remove token from supplied assets list
            if borrower_collateral_assets.supplied == 0 {
                let mut supplied_assets = self.account_supplied_assets.get(&borrower).unwrap_or_default();
                if let Some(index) = supplied_assets.iter().position(|t| t == &collateral_token) {
                    supplied_assets.remove(index);
                    self.account_supplied_assets.insert(borrower, supplied_assets);
                }
            }
            
            // If no supply and borrow, remove borrower's collateral position
            if borrower_collateral_assets.supplied == 0 && borrower_collateral_assets.borrowed == 0 {
                self.account_assets.remove(&collateral_key);
            } else {
                self.account_assets.insert(collateral_key, borrower_collateral_assets);
            }
            
            // Update liquidator's collateral position
            let liquidator_collateral_key = (liquidator, collateral_token);
            let mut liquidator_collateral_assets = self.account_assets.get(&liquidator_collateral_key).unwrap_or_else(|| {
                // Add token to liquidator's supplied assets list
                let mut supplied_assets = self.account_supplied_assets.get(&liquidator).unwrap_or_default();
                if !supplied_assets.contains(&collateral_token) {
                    supplied_assets.push(collateral_token);
                    self.account_supplied_assets.insert(liquidator, supplied_assets);
                }
                
                // Create new account assets record
                AccountAssets {
                    account: liquidator,
                    token: collateral_token,
                    supplied: 0,
                    borrowed: 0,
                    is_collateral: true, // Default to using as collateral
                }
            });
            
            // Calculate liquidator seize amount (excluding protocol share)
            let liquidator_seize_amount = seize_tokens - protocol_seize_amount;
            
            // Update liquidator's collateral position
            liquidator_collateral_assets.supplied += liquidator_seize_amount;
            self.account_assets.insert(liquidator_collateral_key, liquidator_collateral_assets);
            
            // Update markets
            repay_market.total_borrows -= actual_repay_amount;
            self.markets.insert(repay_token, repay_market);
            
            // Handle protocol fee
            if protocol_seize_amount > 0 {
                let fee_collector = *self.fee_collector.get();
                let fee_collector_key = (fee_collector, collateral_token);
                let mut fee_collector_assets = self.account_assets.get(&fee_collector_key).unwrap_or_else(|| {
                    // Add token to fee collector's supplied assets list
                    let mut supplied_assets = self.account_supplied_assets.get(&fee_collector).unwrap_or_default();
                    if !supplied_assets.contains(&collateral_token) {
                        supplied_assets.push(collateral_token);
                        self.account_supplied_assets.insert(fee_collector, supplied_assets);
                    }
                    
                    // Create new account assets record
                    AccountAssets {
                        account: fee_collector,
                        token: collateral_token,
                        supplied: 0,
                        borrowed: 0,
                        is_collateral: true,
                    }
                });
                
                // Update fee collector's position
                fee_collector_assets.supplied += protocol_seize_amount;
                self.account_assets.insert(fee_collector_key, fee_collector_assets);
            }
            
            // Record liquidation event
            self.record_liquidation(liquidator, borrower, repay_token, actual_repay_amount, collateral_token, seize_tokens);
            
            // Emit event
            LiquidateBorrow::emit(liquidator, borrower, repay_token, actual_repay_amount, collateral_token, seize_tokens);
            
            true
        }
        
        /// Toggle using an asset as collateral
        #[method]
        fn set_asset_as_collateral(&mut self, token: Hash160, use_as_collateral: bool) -> bool {
            let account = runtime::calling_script_hash();
            
            // Verify account signature
            assert!(runtime::check_witness(&account), "Invalid signature");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            assert!(self.collateral_asset_allowed.get(&token).unwrap_or(false), "Asset not allowed as collateral");
            
            // Get account assets
            let account_key = (account, token);
            let mut account_assets = self.account_assets.get(&account_key).expect("No supply position");
            
            // Verify has supply position
            assert!(account_assets.supplied > 0, "No supply position");
            
            // If disabling collateral, check if it would leave account insolvent
            if account_assets.is_collateral && !use_as_collateral {
                // Temporarily disable as collateral
                account_assets.is_collateral = false;
                
                // Check if account would remain solvent
                let is_solvent = self.is_account_solvent(&account);
                
                // Revert the change if it would make account insolvent
                if !is_solvent {
                    return false;
                }
            }
            
            // Update collateral flag
            account_assets.is_collateral = use_as_collateral;
            self.account_assets.insert(account_key, account_assets);
            
            true
        }
        
        /// Set new price oracle (admin only)
        #[method]
        fn set_price_oracle(&mut self, new_oracle: Hash160) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set price oracle");
            
            self.price_oracle.set(new_oracle);
            true
        }
        
        /// Set liquidation incentive (admin only)
        #[method]
        fn set_liquidation_incentive(&mut self, new_incentive_mantissa: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set liquidation incentive");
            
            // Validate parameters
            assert!(new_incentive_mantissa >= MANTISSA_ONE, "Incentive must be >= 1");
            assert!(new_incentive_mantissa <= MANTISSA_ONE * 15 / 10, "Incentive too high"); // Max 1.5
            
            self.liquidation_incentive_mantissa.set(new_incentive_mantissa);
            true
        }
        
        /// Set close factor (admin only)
        #[method]
        fn set_close_factor(&mut self, new_close_factor_mantissa: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set close factor");
            
            // Validate parameters
            assert!(new_close_factor_mantissa > 0, "Close factor must be > 0");
            assert!(new_close_factor_mantissa <= MANTISSA_ONE, "Close factor too high"); // Max 1.0
            
            self.close_factor_mantissa.set(new_close_factor_mantissa);
            true
        }
        
        /// Set collateral factor for a market (admin only)
        #[method]
        fn set_collateral_factor(&mut self, token: Hash160, new_collateral_factor_mantissa: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set collateral factor");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Validate parameters
            assert!(new_collateral_factor_mantissa <= MANTISSA_ONE * 9 / 10, "Collateral factor too high"); // Max 90%
            
            // Get market
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Store old value for event
            let old_collateral_factor = market.collateral_factor_mantissa;
            
            // Update collateral factor
            market.collateral_factor_mantissa = new_collateral_factor_mantissa;
            self.markets.insert(token, market);
            
            // Emit event
            NewCollateralFactor::emit(token, old_collateral_factor, new_collateral_factor_mantissa);
            
            true
        }
        
        /// Set reserve factor for a market (admin only)
        #[method]
        fn set_reserve_factor(&mut self, token: Hash160, new_reserve_factor_mantissa: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set reserve factor");
            
            // Verify market exists
            assert!(self.market_listed.get(&token).unwrap_or(false), "Market not listed");
            
            // Validate parameters
            assert!(new_reserve_factor_mantissa <= MANTISSA_ONE / 2, "Reserve factor too high"); // Max 50%
            
            // Get market
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Store old value for event
            let old_reserve_factor = market.reserve_factor_mantissa;
            
            // Update reserve factor
            market.reserve_factor_mantissa = new_reserve_factor_mantissa;
            self.markets.insert(token, market);
            
            // Emit event
            NewReserveFactor::emit(token, old_reserve_factor, new_reserve_factor_mantissa);
            
            true
        }
        
        /// Set fee collector address (admin only)
        #[method]
        fn set_fee_collector(&mut self, new_collector: Address) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set fee collector");
            
            self.fee_collector.set(new_collector);
            true
        }
        
        /// Set protocol seize share (admin only)
        #[method]
        fn set_protocol_seize_share(&mut self, new_share_mantissa: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set protocol seize share");
            
            // Validate parameters
            assert!(new_share_mantissa <= MANTISSA_ONE / 5, "Protocol share too high"); // Max 20%
            
            self.protocol_seize_share_mantissa.set(new_share_mantissa);
            true
        }
        
        // === Safe methods (read-only) ===
        
        /// Get market information
        #[safe]
        fn get_market_info(&self, token: Hash160) -> Option<(u64, u64, u64, u64, u64, u64, u64)> {
            let market = self.markets.get(&token)?;
            
            Some((
                market.total_supply,
                market.total_borrows,
                market.total_reserves,
                market.exchange_rate_mantissa,
                market.collateral_factor_mantissa,
                market.borrow_rate_mantissa,
                market.supply_rate_mantissa
            ))
        }
        
        /// Get account assets information
        #[safe]
        fn get_account_assets(&self, account: Address, token: Hash160) -> Option<(u64, u64, bool)> {
            let assets = self.account_assets.get(&(account, token))?;
            
            Some((
                assets.supplied,
                assets.borrowed,
                assets.is_collateral
            ))
        }
        
        /// Get account's supplied tokens
        #[safe]
        fn get_account_supplied_tokens(&self, account: Address) -> Vec<Hash160> {
            self.account_supplied_assets.get(&account).unwrap_or_default().clone()
        }
        
        /// Get account's borrowed tokens
        #[safe]
        fn get_account_borrowed_tokens(&self, account: Address) -> Vec<Hash160> {
            self.account_borrowed_assets.get(&account).unwrap_or_default().clone()
        }
        
        /// Get account's health factor (scaled by 1e18)
        #[safe]
        fn get_account_health(&self, account: Address) -> u64 {
            let (total_collateral_value, total_borrow_value) = self.get_account_value_internal(&account);
            
            if total_borrow_value == 0 {
                return u64::MAX; // Max health if no borrows
            }
            
            // health factor = total_collateral_value / total_borrow_value
            self.div_u64(
                self.mul_u64(total_collateral_value, MANTISSA_ONE),
                total_borrow_value
            )
        }
        
        /// Check if account can be liquidated
        #[safe]
        fn can_be_liquidated(&self, account: Address) -> bool {
            !self.is_account_solvent(&account)
        }
        
        /// Get all listed market tokens
        #[safe]
        fn get_all_markets(&self) -> Vec<Hash160> {
            self.market_tokens.get().clone()
        }
        
        /// Get current token price from oracle (or cached price if recent)
        fn get_price(&self, token: Hash160) -> Option<u64> {
            // Check cached price first
            if let Some(price_data) = self.prices.get(&token) {
                // Use cached price if less than 1 hour old
                if Ledger::current_timestamp() - price_data.last_updated < 3600 {
                    return Some(price_data.price);
                }
            }
            
            // Call price oracle
            let oracle = self.price_oracle.get()?;
            
            let mut args = Array::new();
            args.push(Any::from(token));
            
            // Call the oracle
            let result = Runtime::call(
                oracle,
                &ByteString::from("getPrice"),
                args,
                CallFlags::READ_ONLY
            )?;
            
            let price = result.as_u64()?;
            
            // Cache the price
            let price_data = PriceData {
                token,
                price,
                last_updated: Ledger::current_timestamp(),
                source: oracle,
            };
            
            // Store in cache
            self.prices.insert(token, price_data);
            
            Some(price)
        }
        
        /// Get liquidation incentive
        #[safe]
        fn get_liquidation_incentive(&self) -> u64 {
            *self.liquidation_incentive_mantissa.get()
        }
        
        /// Get close factor
        #[safe]
        fn get_close_factor(&self) -> u64 {
            *self.close_factor_mantissa.get()
        }
        
        // === Internal methods ===
        
        /// Accrue interest for a market
        fn accrue_interest(&mut self, token: Hash160) -> bool {
            // Get market
            let mut market = self.markets.get(&token).expect("Market not found");
            
            // Get current block
            let current_block = runtime::get_block().index;
            
            // If no blocks have elapsed, no interest to accrue
            if current_block <= market.accrual_block_number {
                return true;
            }
            
            // Calculate blocks elapsed
            let blocks_elapsed = current_block - market.accrual_block_number;
            
            // If no borrows, only update block number
            if market.total_borrows == 0 {
                market.accrual_block_number = current_block;
                market.borrow_rate_mantissa = self.get_borrow_rate(&market);
                market.supply_rate_mantissa = self.get_supply_rate(&market);
                self.markets.insert(token, market);
                return true;
            }
            
            // Calculate interest using borrow rate formula
            let borrow_rate = self.get_borrow_rate(&market);
            
            // interest_factor = borrow_rate * blocks_elapsed
            let interest_factor = self.mul_u64(borrow_rate, blocks_elapsed);
            
            // interest_accumulated = total_borrows * interest_factor
            let interest_accumulated = self.mul_scalar_truncate(market.total_borrows, interest_factor);
            
            // Calculate reserves portion
            let reserves_factor = market.reserve_factor_mantissa;
            let reserves_delta = self.mul_scalar_truncate(interest_accumulated, reserves_factor);
            
            // Update market
            market.total_borrows += interest_accumulated;
            market.total_reserves += reserves_delta;
            market.accrual_block_number = current_block;
            
            // Update exchange rate
            // exchange_rate = (total_supply + total_borrows - total_reserves) / total_supply
            if market.total_supply > 0 {
                let total_cash = market.total_supply + market.total_borrows - market.total_reserves;
                market.exchange_rate_mantissa = self.div_u64(
                    self.mul_u64(total_cash, MANTISSA_ONE),
                    market.total_supply
                );
            }
            
            // Update rates
            market.borrow_rate_mantissa = self.get_borrow_rate(&market);
            market.supply_rate_mantissa = self.get_supply_rate(&market);
            
            // Save market
            self.markets.insert(token, market);
            
            true
        }
        
        /// Calculate borrow rate based on utilization
        fn get_borrow_rate(&self, market: &Market) -> u64 {
            // If no supply, return base rate
            if market.total_supply == 0 {
                return market.base_rate_per_block;
            }
            
            // utilization = total_borrows / (total_supply - total_reserves)
            let available_cash = if market.total_supply > market.total_reserves {
                market.total_supply - market.total_reserves
            } else {
                0
            };
            
            let utilization = if available_cash > 0 {
                self.div_u64(
                    self.mul_u64(market.total_borrows, MANTISSA_ONE),
                    available_cash
                )
            } else if market.total_borrows > 0 {
                MANTISSA_ONE // 100% utilization
            } else {
                0 // No borrows, no utilization
            };
            
            // Calculate borrow rate
            if utilization <= market.kink {
                // Below kink: baseRate + utilization * multiplier
                market.base_rate_per_block + self.mul_scalar_truncate(utilization, market.multiplier_per_block)
            } else {
                // Above kink: baseRate + kink * multiplier + (utilization - kink) * jumpMultiplier
                let normal_rate = market.base_rate_per_block + self.mul_scalar_truncate(market.kink, market.multiplier_per_block);
                let excess_util = utilization - market.kink;
                normal_rate + self.mul_scalar_truncate(excess_util, market.jump_multiplier_per_block)
            }
        }
        
        /// Calculate supply rate based on borrow rate
        fn get_supply_rate(&self, market: &Market) -> u64 {
            // If no supply, no supply rate
            if market.total_supply == 0 {
                return 0;
            }
            
            // utilization = total_borrows / (total_supply - total_reserves)
            let available_cash = if market.total_supply > market.total_reserves {
                market.total_supply - market.total_reserves
            } else {
                0
            };
            
            let utilization = if available_cash > 0 {
                self.div_u64(
                    self.mul_u64(market.total_borrows, MANTISSA_ONE),
                    available_cash
                )
            } else if market.total_borrows > 0 {
                MANTISSA_ONE // 100% utilization
            } else {
                0 // No borrows, no utilization
            };
            
            // supplyRate = borrowRate * utilization * (1 - reserveFactor)
            let borrow_rate = self.get_borrow_rate(market);
            let one_minus_reserve_factor = MANTISSA_ONE - market.reserve_factor_mantissa;
            
            let rate_to_pool = self.mul_scalar_truncate(borrow_rate, one_minus_reserve_factor);
            self.mul_scalar_truncate(utilization, rate_to_pool)
        }
        
        /// Check if an account is solvent
        fn is_account_solvent(&self, account: &Address) -> bool {
            let (total_collateral_value, total_borrow_value) = self.get_account_value_internal(account);
            
            // Account is solvent if no borrows or collateral value >= borrow value
            total_borrow_value == 0 || total_collateral_value >= total_borrow_value
        }
        
        /// Calculate account's collateral and borrow values
        fn get_account_value_internal(&self, account: &Address) -> (u64, u64) {
            let mut total_collateral_value = 0;
            let mut total_borrow_value = 0;
            
            // Process supplied assets (potential collateral)
            let supplied_tokens = self.account_supplied_assets.get(account).unwrap_or_default();
            for token in supplied_tokens.iter() {
                // Get price
                if let Some(price) = self.get_price(*token) {
                    // Get account assets
                    if let Some(assets) = self.account_assets.get(&(*account, *token)) {
                        // Only count assets marked as collateral
                        if assets.is_collateral {
                            // Get market
                            if let Some(market) = self.markets.get(token) {
                                // Calculate value of supplied assets in collateral
                                // value = supplied * exchange_rate * price * collateral_factor
                                let supplied_amount = self.mul_scalar_truncate(
                                    assets.supplied,
                                    market.exchange_rate_mantissa
                                );
                                
                                let collateral_value = self.mul_scalar_truncate(
                                    self.mul_scalar_truncate(supplied_amount, price),
                                    market.collateral_factor_mantissa
                                );
                                
                                total_collateral_value += collateral_value;
                            }
                        }
                    }
                }
            }
            
            // Process borrowed assets
            let borrowed_tokens = self.account_borrowed_assets.get(account).unwrap_or_default();
            for token in borrowed_tokens.iter() {
                // Get price
                if let Some(price) = self.get_price(*token) {
                    // Get account assets
                    if let Some(assets) = self.account_assets.get(&(*account, *token)) {
                        // Calculate value of borrowed assets
                        // value = borrowed * price
                        let borrow_value = self.mul_scalar_truncate(assets.borrowed, price);
                        total_borrow_value += borrow_value;
                    }
                }
            }
            
            (total_collateral_value, total_borrow_value)
        }
        
        /// Calculate amount of collateral to seize in a liquidation
        fn calculate_seize_amount(
            &self,
            repay_token: Hash160,
            collateral_token: Hash160,
            repay_amount: u64
        ) -> (u64, u64) {
            // Get prices
            let repay_price = self.get_price(repay_token).expect("Repay price not available");
            let collateral_price = self.get_price(collateral_token).expect("Collateral price not available");
            
            // Get liquidation incentive
            let liquidation_incentive = *self.liquidation_incentive_mantissa.get();
            
            // Get collateral market for exchange rate
            let collateral_market = self.markets.get(&collateral_token).expect("Collateral market not found");
            
            // Calculate seize amount
            // seizeAmount = repayAmount * repayPrice / collateralPrice * liquidationIncentive / exchangeRate
            let numerator = self.mul_scalar_truncate(
                self.mul_scalar_truncate(
                    self.mul_scalar_truncate(repay_amount, repay_price),
                    liquidation_incentive
                ),
                MANTISSA_ONE
            );
            
            let denominator = self.mul_scalar_truncate(
                collateral_price,
                collateral_market.exchange_rate_mantissa
            );
            
            let seize_amount = self.div_u64(numerator, denominator);
            
            // Calculate protocol's share
            let protocol_share = *self.protocol_seize_share_mantissa.get();
            let protocol_seize_amount = self.mul_scalar_truncate(seize_amount, protocol_share);
            
            (seize_amount, protocol_seize_amount)
        }
        
        /// Safe multiply two u64 values (full precision)
        fn mul_u64(&self, a: u64, b: u64) -> u64 {
            (a as u128 * b as u128) as u64
        }
        
        /// Safe division of two u64 values
        fn div_u64(&self, a: u64, b: u64) -> u64 {
            if b == 0 {
                0
            } else {
                a / b
            }
        }
        
        /// Multiply a scalar by a mantissa, truncating the result
        fn mul_scalar_truncate(&self, a: u64, scalar: u64) -> u64 {
            self.div_u64(self.mul_u64(a, scalar), MANTISSA_ONE)
        }
        
        /// Transfer NEP-17 token from user to contract
        fn transfer_token_to_contract(&self, token_hash: &Hash160, from: &Address, amount: u64) {
            let transferred: bool = self.call_contract(
                token_hash,
                "transfer",
                (*from, runtime::executing_script_hash(), amount, ByteArray::new())
            ).expect("Token transfer failed");
            
            assert!(transferred, "Failed to transfer tokens to contract");
        }
        
        /// Transfer NEP-17 token from contract to user
        fn transfer_token_from_contract(&self, token_hash: &Hash160, to: &Address, amount: u64) {
            let transferred: bool = self.call_contract(
                token_hash,
                "transfer",
                (runtime::executing_script_hash(), *to, amount, ByteArray::new())
            ).expect("Token transfer failed");
            
            assert!(transferred, "Failed to transfer tokens from contract");
        }
        
        /// Record a liquidation event
        fn record_liquidation(&mut self, liquidator: Address, borrower: Address, repay_token: Hash160, repay_amount: u64, collateral_token: Hash160, seize_amount: u64) {
            let liquidation_id = *self.next_liquidation_id.get();
            self.next_liquidation_id.set(liquidation_id + 1);
            
            let event = LiquidationEvent {
                liquidator,
                borrower,
                repay_token,
                repay_amount,
                collateral_token,
                seize_amount,
                timestamp: Ledger::current_timestamp(),
            };
            
            self.liquidation_history.insert(liquidation_id, event);
        }
    }
}