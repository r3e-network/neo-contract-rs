//! # MOON DOGE - Meme Coin Example
//!
//! A NEP-17 token implementation with typical meme coin features:
//! - Tax on transfers for buyback and burn
//! - Reward distribution system
//! - Anti-whale mechanics
//! - Liquidity generation
//! - Deflationary tokenomics

#[neo_contract::contract]
mod moon_doge {
    use neo_contract::prelude::*;
    
    /// Events emitted by the token contract
    #[event]
    struct Transfer {
        #[index]
        from: Option<Address>,
        #[index]
        to: Option<Address>,
        amount: u64,
    }
    
    #[event]
    struct TokensBurned {
        #[index]
        amount: u64,
    }
    
    #[event]
    struct RewardsDistributed {
        total_amount: u64,
        recipients: u64,
    }
    
    #[event]
    struct TaxRatesUpdated {
        liquidity_tax: u16,
        marketing_tax: u16,
        buyback_tax: u16,
        reflection_tax: u16,
    }
    
    /// Token storage
    #[storage]
    struct MoonDoge {
        /// Token metadata
        name: Item<String>,
        symbol: Item<String>,
        decimals: Item<u8>,
        
        /// Total supply tracking
        total_supply: Item<u64>,
        circulating_supply: Item<u64>,
        
        /// Balances
        balances: Map<Address, u64>,
        
        /// Tax settings in basis points (100 = 1%)
        liquidity_tax: Item<u16>,
        marketing_tax: Item<u16>,
        buyback_tax: Item<u16>,
        reflection_tax: Item<u16>,
        
        /// Special addresses
        owner: Item<Address>,
        marketing_wallet: Item<Address>,
        team_wallet: Item<Address>,
        
        /// DEX pair contract (for automatic liquidity)
        dex_pair: Item<Hash160>,
        
        /// Tax exemption status
        tax_exempt: Map<Address, bool>,
        
        /// Reward tracking
        total_rewards_distributed: Item<u64>,
        excluded_from_rewards: Map<Address, bool>,
        reward_cycle_blocks: Item<u64>,
        last_reward_block: Item<u64>,
        
        /// Anti-whale settings
        max_transaction_amount: Item<u64>,
        max_wallet_balance: Item<u64>,
        
        /// Trading control
        trading_enabled: Item<bool>,
        
        /// Transaction counters (for analytics)
        total_transactions: Item<u64>,
        total_holders: Item<u64>,
    }
    
    impl MoonDoge {
        /// Initialize the token contract
        #[constructor]
        fn new(
            owner: Address,
            marketing_wallet: Address,
            team_wallet: Address,
            initial_supply: u64,
        ) -> Self {
            // Token metadata
            let name = "MOON DOGE";
            let symbol = "MDOGE";
            let decimals = 8;
            
            // Set up tax rates (default: 10% total tax)
            let liquidity_tax = 300;  // 3%
            let marketing_tax = 200;  // 2%
            let buyback_tax = 200;    // 2%
            let reflection_tax = 300; // 3%
            
            // Anti-whale defaults
            let max_tx_pct = 100;     // 1% of total supply
            let max_wallet_pct = 200; // 2% of total supply
            
            // Calculate limits
            let max_tx = (initial_supply * max_tx_pct) / 10000;
            let max_wallet = (initial_supply * max_wallet_pct) / 10000;
            
            // Create instance with initial values
            let mut instance = Self {
                name: Item::new(name.to_string()),
                symbol: Item::new(symbol.to_string()),
                decimals: Item::new(decimals),
                total_supply: Item::new(initial_supply),
                circulating_supply: Item::new(initial_supply),
                balances: Map::new(),
                liquidity_tax: Item::new(liquidity_tax),
                marketing_tax: Item::new(marketing_tax),
                buyback_tax: Item::new(buyback_tax), 
                reflection_tax: Item::new(reflection_tax),
                owner: Item::new(owner),
                marketing_wallet: Item::new(marketing_wallet),
                team_wallet: Item::new(team_wallet),
                dex_pair: Item::new(Hash160::zero()),
                tax_exempt: Map::new(),
                total_rewards_distributed: Item::new(0),
                excluded_from_rewards: Map::new(),
                reward_cycle_blocks: Item::new(5000), // Distribute rewards every ~5000 blocks
                last_reward_block: Item::new(runtime::get_block().index),
                max_transaction_amount: Item::new(max_tx),
                max_wallet_balance: Item::new(max_wallet),
                trading_enabled: Item::new(false),
                total_transactions: Item::new(0),
                total_holders: Item::new(0),
            };
            
            // Set up tax exemptions for key addresses
            instance.tax_exempt.insert(owner, true);
            instance.tax_exempt.insert(marketing_wallet, true);
            instance.tax_exempt.insert(team_wallet, true);
            
            // Exclude contract and key addresses from rewards
            let contract_address = runtime::executing_script_hash();
            instance.excluded_from_rewards.insert(contract_address, true);
            instance.excluded_from_rewards.insert(marketing_wallet, true);
            instance.excluded_from_rewards.insert(team_wallet, true);
            
            // Mint initial supply to owner
            instance.balances.insert(owner, initial_supply);
            instance.emit(Transfer {
                from: None,
                to: Some(owner),
                amount: initial_supply,
            });
            
            instance.total_holders.set(1);
            
            instance
        }
        
        /// NEP-17 methods
        
        #[safe]
        fn symbol(&self) -> String {
            self.symbol.get().clone()
        }
        
        #[safe]
        fn decimals(&self) -> u8 {
            *self.decimals.get()
        }
        
        #[safe]
        fn total_supply(&self) -> u64 {
            *self.total_supply.get()
        }
        
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        #[method]
        fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Check signatures
            assert!(runtime::check_witness(&from), "Invalid signature");
            
            // Self-transfers are allowed but pointless
            if from == to {
                return true;
            }
            
            // Check if trading is enabled
            let owner = *self.owner.get();
            if from != owner && to != owner {
                assert!(*self.trading_enabled.get(), "Trading not yet enabled");
            }
            
            // Check if transfer amount is valid
            assert!(amount > 0, "Invalid amount");
            
            // Check sender balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Check anti-whale limits
            if !self.is_tax_exempt(&from) && !self.is_tax_exempt(&to) {
                // Max transaction check
                let max_tx = *self.max_transaction_amount.get();
                assert!(amount <= max_tx, "Transaction exceeds max amount");
                
                // Max wallet check (only for receiving)
                let max_wallet = *self.max_wallet_balance.get();
                let to_balance = self.balances.get(&to).unwrap_or_default();
                assert!(to_balance + amount <= max_wallet, "Would exceed max wallet balance");
            }
            
            // Process transfer with tax if applicable
            if self.is_tax_exempt(&from) || self.is_tax_exempt(&to) {
                // Tax-exempt transfer (direct)
                self.do_transfer(from, to, amount);
            } else {
                // Taxed transfer
                self.taxed_transfer(from, to, amount);
            }
            
            // Update holder count if needed
            if self.balances.get(&to).unwrap_or_default() == amount {
                let holders = *self.total_holders.get();
                self.total_holders.set(holders + 1);
            }
            
            // Update transaction count
            let tx_count = *self.total_transactions.get();
            self.total_transactions.set(tx_count + 1);
            
            // Check if reward distribution is due
            self.try_distribute_rewards();
            
            true
        }
        
        /// Custom token methods
        
        /// Set the DEX pair contract hash
        #[method]
        fn set_dex_pair(&mut self, pair_hash: Hash160) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set DEX pair");
            
            self.dex_pair.set(pair_hash);
            true
        }
        
        /// Set tax exemption for an address
        #[method]
        fn set_tax_exempt(&mut self, address: Address, exempt: bool) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set tax exemption");
            
            self.tax_exempt.insert(address, exempt);
            true
        }
        
        /// Update tax rates
        #[method]
        fn update_tax_rates(
            &mut self, 
            liquidity_tax: u16, 
            marketing_tax: u16, 
            buyback_tax: u16, 
            reflection_tax: u16
        ) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can update tax rates");
            
            // Ensure total tax is not too high
            let total_tax = liquidity_tax + marketing_tax + buyback_tax + reflection_tax;
            assert!(total_tax <= 2000, "Total tax cannot exceed 20%");
            
            self.liquidity_tax.set(liquidity_tax);
            self.marketing_tax.set(marketing_tax);
            self.buyback_tax.set(buyback_tax);
            self.reflection_tax.set(reflection_tax);
            
            self.emit(TaxRatesUpdated {
                liquidity_tax,
                marketing_tax,
                buyback_tax,
                reflection_tax,
            });
            
            true
        }
        
        /// Enable trading
        #[method]
        fn enable_trading(&mut self) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can enable trading");
            
            self.trading_enabled.set(true);
            true
        }
        
        /// Update anti-whale settings
        #[method]
        fn update_whale_limits(&mut self, max_tx_pct: u16, max_wallet_pct: u16) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can update whale limits");
            
            // Validate percentages
            assert!(max_tx_pct >= 50, "Max tx too small"); // At least 0.5%
            assert!(max_wallet_pct >= 100, "Max wallet too small"); // At least 1%
            
            // Calculate limits
            let total_supply = *self.total_supply.get();
            let max_tx = (total_supply * max_tx_pct as u64) / 10000;
            let max_wallet = (total_supply * max_wallet_pct as u64) / 10000;
            
            self.max_transaction_amount.set(max_tx);
            self.max_wallet_balance.set(max_wallet);
            
            true
        }
        
        /// Exclude/include address from rewards
        #[method]
        fn set_reward_exclusion(&mut self, address: Address, excluded: bool) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set reward exclusion");
            
            self.excluded_from_rewards.insert(address, excluded);
            true
        }
        
        /// Update reward cycle
        #[method]
        fn set_reward_cycle(&mut self, blocks: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can set reward cycle");
            
            assert!(blocks >= 1000 && blocks <= 50000, "Invalid cycle length");
            self.reward_cycle_blocks.set(blocks);
            true
        }
        
        /// Force reward distribution
        #[method]
        fn distribute_rewards(&mut self) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can force distribution");
            
            self.do_distribute_rewards();
            true
        }
        
        /// Burn tokens from own balance
        #[method]
        fn burn(&mut self, amount: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(runtime::check_witness(&caller), "Invalid signature");
            
            // Check sender balance
            let balance = self.balances.get(&caller).unwrap_or_default();
            assert!(balance >= amount, "Insufficient balance");
            
            // Update balances
            self.balances.insert(caller, balance - amount);
            
            // Update supplies
            let current_supply = *self.total_supply.get();
            let circulating = *self.circulating_supply.get();
            
            self.total_supply.set(current_supply - amount);
            self.circulating_supply.set(circulating - amount);
            
            // Emit events
            self.emit(Transfer {
                from: Some(caller),
                to: None,
                amount,
            });
            
            self.emit(TokensBurned {
                amount,
            });
            
            true
        }
        
        /// Get token statistics
        #[safe]
        fn get_stats(&self) -> (u64, u64, u64, u64, u64, u64) {
            (
                *self.total_supply.get(),
                *self.circulating_supply.get(),
                *self.total_transactions.get(),
                *self.total_holders.get(),
                *self.total_rewards_distributed.get(),
                *self.last_reward_block.get(),
            )
        }
        
        /// Get tax information
        #[safe]
        fn get_tax_info(&self) -> (u16, u16, u16, u16) {
            (
                *self.liquidity_tax.get(),
                *self.marketing_tax.get(),
                *self.buyback_tax.get(),
                *self.reflection_tax.get(),
            )
        }
        
        // === Helper methods ===
        
        /// Process a transfer without tax
        fn do_transfer(&mut self, from: Address, to: Address, amount: u64) {
            // Update balances
            let from_balance = self.balances.get(&from).unwrap_or_default();
            let to_balance = self.balances.get(&to).unwrap_or_default();
            
            self.balances.insert(from, from_balance - amount);
            self.balances.insert(to, to_balance + amount);
            
            // Emit transfer event
            self.emit(Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            });
        }
        
        /// Process a transfer with tax
        fn taxed_transfer(&mut self, from: Address, to: Address, amount: u64) {
            // Calculate tax amounts
            let liquidity_tax = *self.liquidity_tax.get() as u64;
            let marketing_tax = *self.marketing_tax.get() as u64;
            let buyback_tax = *self.buyback_tax.get() as u64;
            let reflection_tax = *self.reflection_tax.get() as u64;
            
            let total_tax_bps = liquidity_tax + marketing_tax + buyback_tax + reflection_tax;
            let total_tax_amount = (amount * total_tax_bps) / 10000;
            
            // Calculate individual tax portions
            let liquidity_amount = (amount * liquidity_tax) / 10000;
            let marketing_amount = (amount * marketing_tax) / 10000;
            let buyback_amount = (amount * buyback_tax) / 10000;
            let reflection_amount = (amount * reflection_tax) / 10000;
            
            // Transfer net amount to recipient
            let net_amount = amount - total_tax_amount;
            
            // Update balances
            let from_balance = self.balances.get(&from).unwrap_or_default();
            let to_balance = self.balances.get(&to).unwrap_or_default();
            
            self.balances.insert(from, from_balance - amount);
            self.balances.insert(to, to_balance + net_amount);
            
            // Emit main transfer event
            self.emit(Transfer {
                from: Some(from),
                to: Some(to),
                amount: net_amount,
            });
            
            // Process tax allocations
            
            // 1. Liquidity tax
            if liquidity_amount > 0 {
                let dex_pair = *self.dex_pair.get();
                if dex_pair != Hash160::zero() {
                    // If DEX pair is set, send to pair for auto-liquidity
                    let pair_balance = self.balances.get(&dex_pair).unwrap_or_default();
                    self.balances.insert(dex_pair, pair_balance + liquidity_amount);
                    
                    self.emit(Transfer {
                        from: Some(from),
                        to: Some(dex_pair),
                        amount: liquidity_amount,
                    });
                } else {
                    // Otherwise, send to owner
                    let owner = *self.owner.get();
                    let owner_balance = self.balances.get(&owner).unwrap_or_default();
                    self.balances.insert(owner, owner_balance + liquidity_amount);
                    
                    self.emit(Transfer {
                        from: Some(from),
                        to: Some(owner),
                        amount: liquidity_amount,
                    });
                }
            }
            
            // 2. Marketing tax
            if marketing_amount > 0 {
                let marketing_wallet = *self.marketing_wallet.get();
                let wallet_balance = self.balances.get(&marketing_wallet).unwrap_or_default();
                self.balances.insert(marketing_wallet, wallet_balance + marketing_amount);
                
                self.emit(Transfer {
                    from: Some(from),
                    to: Some(marketing_wallet),
                    amount: marketing_amount,
                });
            }
            
            // 3. Buyback tax
            if buyback_amount > 0 {
                // For buyback/burn, tokens go to contract itself
                let contract_address = runtime::executing_script_hash();
                let contract_balance = self.balances.get(&contract_address).unwrap_or_default();
                self.balances.insert(contract_address, contract_balance + buyback_amount);
                
                self.emit(Transfer {
                    from: Some(from),
                    to: Some(contract_address),
                    amount: buyback_amount,
                });
            }
            
            // 4. Reflection tax
            if reflection_amount > 0 {
                // Accumulate in contract for later distribution
                let contract_address = runtime::executing_script_hash();
                let contract_balance = self.balances.get(&contract_address).unwrap_or_default();
                self.balances.insert(contract_address, contract_balance + reflection_amount);
                
                self.emit(Transfer {
                    from: Some(from),
                    to: Some(contract_address),
                    amount: reflection_amount,
                });
            }
        }
        
        /// Check if an address is exempt from taxes
        fn is_tax_exempt(&self, address: &Address) -> bool {
            self.tax_exempt.get(address).unwrap_or_default()
        }
        
        /// Try to distribute rewards if conditions are met
        fn try_distribute_rewards(&mut self) {
            let current_block = runtime::get_block().index;
            let last_reward_block = *self.last_reward_block.get();
            let cycle_blocks = *self.reward_cycle_blocks.get();
            
            if current_block >= last_reward_block + cycle_blocks {
                self.do_distribute_rewards();
            }
        }
        
        /// Distribute rewards to holders
        fn do_distribute_rewards(&mut self) {
            // Get contract address and balance
            let contract_address = runtime::executing_script_hash();
            let reflection_balance = self.balances.get(&contract_address).unwrap_or_default();
            
            // Only distribute if we have tokens to distribute
            if reflection_balance > 0 {
                // Count eligible holders
                let mut eligible_total_balance = 0;
                let mut eligible_holders = 0;
                
                // First pass: count eligible balances
                for (address, balance) in self.balances.iter() {
                    // Skip zero balances, excluded addresses, and contract itself
                    if *balance == 0 
                        || self.excluded_from_rewards.get(&address).unwrap_or_default()
                        || address == contract_address {
                        continue;
                    }
                    
                    eligible_total_balance += balance;
                    eligible_holders += 1;
                }
                
                // Only proceed if we have eligible holders
                if eligible_holders > 0 && eligible_total_balance > 0 {
                    // Calculate how much to distribute (all accumulated reflection tokens)
                    let total_distribution = reflection_balance;
                    
                    // Reset contract balance
                    self.balances.insert(contract_address, 0);
                    
                    // Second pass: distribute proportionally to each holder
                    for (address, balance) in self.balances.iter() {
                        if *balance == 0 
                            || self.excluded_from_rewards.get(&address).unwrap_or_default() 
                            || address == contract_address {
                            continue;
                        }
                        
                        // Calculate holder's share
                        let reward = (total_distribution * (*balance)) / eligible_total_balance;
                        
                        if reward > 0 {
                            // Update holder's balance
                            self.balances.insert(address, balance + reward);
                            
                            // Emit transfer event
                            self.emit(Transfer {
                                from: Some(contract_address),
                                to: Some(address),
                                amount: reward,
                            });
                        }
                    }
                    
                    // Update reward stats
                    let total_rewards = *self.total_rewards_distributed.get();
                    self.total_rewards_distributed.set(total_rewards + total_distribution);
                    
                    // Emit rewards event
                    self.emit(RewardsDistributed {
                        total_amount: total_distribution,
                        recipients: eligible_holders,
                    });
                }
            }
            
            // Update last reward block
            self.last_reward_block.set(runtime::get_block().index);
        }
    }
}