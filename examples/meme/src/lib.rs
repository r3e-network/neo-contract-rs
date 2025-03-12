#![no_std]

extern crate alloc;

//! # MOON DOGE - Meme Coin Example for Neo N3
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
    use alloc::string::String;
    
    /// Events emitted by the token contract
    #[event]
    struct Transfer {
        #[index]
        from: Option<Address>,
        #[index]
        to: Option<Address>,
        amount: u64,
    }
    
    /// Implementation for properly emitting the Transfer event using Neo N3 standards
    impl Transfer {
        /// Static method to emit the Transfer event in Neo N3 format
        pub fn emit(from: Option<Address>, to: Option<Address>, amount: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("Transfer");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            match from {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::from(ByteArray::new())), // null for minting
            }
            
            match to {
                Some(addr) => event_data.push(Any::from(addr)),
                None => event_data.push(Any::from(ByteArray::new())), // null for burning
            }
            
            event_data.push(Any::from(amount));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct TokensBurned {
        #[index]
        amount: u64,
    }
    
    /// Implementation for properly emitting the TokensBurned event using Neo N3 standards
    impl TokensBurned {
        /// Static method to emit the TokensBurned event in Neo N3 format
        pub fn emit(amount: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("TokensBurned");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(amount));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct RewardsDistributed {
        total_amount: u64,
        recipients: u64,
    }
    
    /// Implementation for properly emitting the RewardsDistributed event using Neo N3 standards
    impl RewardsDistributed {
        /// Static method to emit the RewardsDistributed event in Neo N3 format
        pub fn emit(total_amount: u64, recipients: u64) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("RewardsDistributed");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(total_amount));
            event_data.push(Any::from(recipients));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
    }
    
    #[event]
    struct TaxRatesUpdated {
        liquidity_tax: u16,
        marketing_tax: u16,
        buyback_tax: u16,
        reflection_tax: u16,
    }
    
    /// Implementation for properly emitting the TaxRatesUpdated event using Neo N3 standards
    impl TaxRatesUpdated {
        /// Static method to emit the TaxRatesUpdated event in Neo N3 format
        pub fn emit(liquidity_tax: u16, marketing_tax: u16, buyback_tax: u16, reflection_tax: u16) {
            // Create event name as ByteString (required for Neo N3)
            let event_name = ByteString::from("TaxRatesUpdated");
            
            // Create Array to hold event parameters (required for Neo N3)
            let mut event_data = Array::<Any>::new();
            
            // Add parameters with proper Neo N3 format
            event_data.push(Any::from(liquidity_tax));
            event_data.push(Any::from(marketing_tax));
            event_data.push(Any::from(buyback_tax));
            event_data.push(Any::from(reflection_tax));
            
            // Emit the event using Runtime::notify (required for Neo N3)
            Runtime::notify(&event_name, &event_data);
        }
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
                name: Item::new("name"),
                symbol: Item::new("symbol"),
                decimals: Item::new("decimals"),
                total_supply: Item::new("total_supply"),
                circulating_supply: Item::new("circulating_supply"),
                balances: Map::new(),
                liquidity_tax: Item::new("liquidity_tax"),
                marketing_tax: Item::new("marketing_tax"),
                buyback_tax: Item::new("buyback_tax"),
                reflection_tax: Item::new("reflection_tax"),
                owner: Item::new("owner"),
                marketing_wallet: Item::new("marketing_wallet"),
                team_wallet: Item::new("team_wallet"),
                dex_pair: Item::new("dex_pair"),
                tax_exempt: Map::new(),
                total_rewards_distributed: Item::new("total_rewards_distributed"),
                excluded_from_rewards: Map::new(),
                reward_cycle_blocks: Item::new("reward_cycle_blocks"),
                last_reward_block: Item::new("last_reward_block"),
                max_transaction_amount: Item::new("max_transaction_amount"),
                max_wallet_balance: Item::new("max_wallet_balance"),
                trading_enabled: Item::new("trading_enabled"),
                total_transactions: Item::new("total_transactions"),
                total_holders: Item::new("total_holders"),
            };
            
            // Set initial values
            instance.name.set(name.to_string());
            instance.symbol.set(symbol.to_string());
            instance.decimals.set(decimals);
            instance.total_supply.set(initial_supply);
            instance.circulating_supply.set(initial_supply);
            instance.liquidity_tax.set(liquidity_tax);
            instance.marketing_tax.set(marketing_tax);
            instance.buyback_tax.set(buyback_tax);
            instance.reflection_tax.set(reflection_tax);
            instance.owner.set(owner);
            instance.marketing_wallet.set(marketing_wallet);
            instance.team_wallet.set(team_wallet);
            instance.dex_pair.set(Hash160::zero());
            instance.total_rewards_distributed.set(0);
            instance.reward_cycle_blocks.set(5000); // Distribute rewards every ~5000 blocks
            instance.last_reward_block.set(Runtime::get_block().index);
            instance.max_transaction_amount.set(max_tx);
            instance.max_wallet_balance.set(max_wallet);
            instance.trading_enabled.set(false);
            instance.total_transactions.set(0);
            instance.total_holders.set(1);
            
            // Set up tax exemptions for key addresses
            instance.tax_exempt.insert(owner, true);
            instance.tax_exempt.insert(marketing_wallet, true);
            instance.tax_exempt.insert(team_wallet, true);
            
            // Exclude contract and key addresses from rewards
            let contract_address = Runtime::executing_script_hash();
            instance.excluded_from_rewards.insert(contract_address, true);
            instance.excluded_from_rewards.insert(marketing_wallet, true);
            instance.excluded_from_rewards.insert(team_wallet, true);
            
            // Mint initial supply to owner
            instance.balances.insert(owner, initial_supply);
            
            // Emit transfer event with proper Neo N3 format
            Transfer::emit(None, Some(owner), initial_supply);
            
            instance
        }
        
        /// NEP-17 methods
        
        /// Get the token symbol
        #[method]
        #[safe]
        fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_default()
        }
        
        /// Get the token decimals
        #[method]
        #[safe]
        fn decimals(&self) -> u8 {
            self.decimals.get().unwrap_or_default()
        }
        
        /// Get the total token supply
        #[method]
        #[safe]
        fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        /// Get the token balance for an account
        #[method]
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        /// Transfer tokens from one account to another
        #[method]
        #[no_reentry]
        fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
            // Check signatures
            assert!(Runtime::check_witness(&from), "Invalid signature");
            
            // Self-transfers are allowed but pointless
            if from == to {
                return true;
            }
            
            // Check if trading is enabled
            let owner = self.owner.get().unwrap_or_default();
            if from != owner && to != owner {
                assert!(self.trading_enabled.get().unwrap_or_default(), "Trading not yet enabled");
            }
            
            // Check if transfer amount is valid
            assert!(amount > 0, "Invalid amount");
            
            // Check sender balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Check anti-whale limits
            if !self.is_tax_exempt(&from) && !self.is_tax_exempt(&to) {
                // Max transaction check
                let max_tx = self.max_transaction_amount.get().unwrap_or_default();
                assert!(amount <= max_tx, "Transaction exceeds max amount");
                
                // Max wallet check (only for receiving)
                let max_wallet = self.max_wallet_balance.get().unwrap_or_default();
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
                let holders = self.total_holders.get().unwrap_or_default();
                self.total_holders.set(holders + 1);
            }
            
            // Update transaction count
            let tx_count = self.total_transactions.get().unwrap_or_default();
            self.total_transactions.set(tx_count + 1);
            
            // Check if reward distribution is due
            self.try_distribute_rewards();
            
            true
        }
        
        /// Custom token methods
        
        /// Set the DEX pair contract hash
        #[method]
        #[no_reentry]
        fn set_dex_pair(&mut self, pair_hash: Hash160) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can set DEX pair");
            
            self.dex_pair.set(pair_hash);
            true
        }
        
        /// Set tax exemption for an address
        #[method]
        #[no_reentry]
        fn set_tax_exempt(&mut self, address: Address, exempt: bool) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can set tax exemption");
            
            self.tax_exempt.insert(address, exempt);
            true
        }
        
        /// Update tax rates
        #[method]
        #[no_reentry]
        fn update_tax_rates(
            &mut self, 
            liquidity_tax: u16, 
            marketing_tax: u16, 
            buyback_tax: u16, 
            reflection_tax: u16
        ) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can update tax rates");
            
            // Ensure total tax is not too high
            let total_tax = liquidity_tax + marketing_tax + buyback_tax + reflection_tax;
            assert!(total_tax <= 2000, "Total tax cannot exceed 20%");
            
            self.liquidity_tax.set(liquidity_tax);
            self.marketing_tax.set(marketing_tax);
            self.buyback_tax.set(buyback_tax);
            self.reflection_tax.set(reflection_tax);
            
            // Emit event with proper Neo N3 format
            TaxRatesUpdated::emit(liquidity_tax, marketing_tax, buyback_tax, reflection_tax);
            
            true
        }
        
        /// Enable trading
        #[method]
        #[no_reentry]
        fn enable_trading(&mut self) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can enable trading");
            
            self.trading_enabled.set(true);
            true
        }
        
        /// Update anti-whale settings
        #[method]
        #[no_reentry]
        fn update_whale_limits(&mut self, max_tx_pct: u16, max_wallet_pct: u16) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can update whale limits");
            
            // Validate percentages
            assert!(max_tx_pct >= 50, "Max tx too small"); // At least 0.5%
            assert!(max_wallet_pct >= 100, "Max wallet too small"); // At least 1%
            
            // Calculate limits
            let total_supply = self.total_supply.get().unwrap_or_default();
            let max_tx = (total_supply * max_tx_pct as u64) / 10000;
            let max_wallet = (total_supply * max_wallet_pct as u64) / 10000;
            
            self.max_transaction_amount.set(max_tx);
            self.max_wallet_balance.set(max_wallet);
            
            true
        }
        
        /// Exclude/include address from rewards
        #[method]
        #[no_reentry]
        fn set_reward_exclusion(&mut self, address: Address, excluded: bool) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can set reward exclusion");
            
            self.excluded_from_rewards.insert(address, excluded);
            true
        }
        
        /// Update reward cycle
        #[method]
        #[no_reentry]
        fn set_reward_cycle(&mut self, blocks: u64) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can set reward cycle");
            
            assert!(blocks >= 1000 && blocks <= 50000, "Invalid cycle length");
            self.reward_cycle_blocks.set(blocks);
            true
        }
        
        /// Force reward distribution
        #[method]
        #[no_reentry]
        fn distribute_rewards(&mut self) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(caller == self.owner.get().unwrap_or_default(), "Only owner can force distribution");
            
            self.do_distribute_rewards();
            true
        }
        
        /// Burn tokens from own balance
        #[method]
        #[no_reentry]
        fn burn(&mut self, amount: u64) -> bool {
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "Invalid signature");
            
            // Check sender balance
            let balance = self.balances.get(&caller).unwrap_or_default();
            assert!(balance >= amount, "Insufficient balance");
            
            // Update balances
            self.balances.insert(caller, balance - amount);
            
            // Update supplies
            let current_supply = self.total_supply.get().unwrap_or_default();
            let circulating = self.circulating_supply.get().unwrap_or_default();
            
            self.total_supply.set(current_supply - amount);
            self.circulating_supply.set(circulating - amount);
            
            // Emit transfer event with proper Neo N3 format
            Transfer::emit(Some(caller), None, amount);
            
            // Emit tokens burned event with proper Neo N3 format
            TokensBurned::emit(amount);
            
            true
        }
        
        /// Get token statistics
        #[method]
        #[safe]
        fn get_stats(&self) -> (u64, u64, u64, u64, u64, u64) {
            (
                self.total_supply.get().unwrap_or_default(),
                self.circulating_supply.get().unwrap_or_default(),
                self.total_transactions.get().unwrap_or_default(),
                self.total_holders.get().unwrap_or_default(),
                self.total_rewards_distributed.get().unwrap_or_default(),
                self.last_reward_block.get().unwrap_or_default(),
            )
        }
        
        /// Get tax information
        #[method]
        #[safe]
        fn get_tax_info(&self) -> (u16, u16, u16, u16) {
            (
                self.liquidity_tax.get().unwrap_or_default(),
                self.marketing_tax.get().unwrap_or_default(),
                self.buyback_tax.get().unwrap_or_default(),
                self.reflection_tax.get().unwrap_or_default(),
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
            
            // Emit transfer event with proper Neo N3 format
            Transfer::emit(Some(from), Some(to), amount);
        }
        
        /// Process a transfer with tax
        fn taxed_transfer(&mut self, from: Address, to: Address, amount: u64) {
            // Calculate tax amounts
            let liquidity_tax = self.liquidity_tax.get().unwrap_or_default() as u64;
            let marketing_tax = self.marketing_tax.get().unwrap_or_default() as u64;
            let buyback_tax = self.buyback_tax.get().unwrap_or_default() as u64;
            let reflection_tax = self.reflection_tax.get().unwrap_or_default() as u64;
            
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
            
            // Emit main transfer event with proper Neo N3 format
            Transfer::emit(Some(from), Some(to), net_amount);
            
            // Process tax allocations
            
            // 1. Liquidity tax
            if liquidity_amount > 0 {
                let dex_pair = self.dex_pair.get().unwrap_or_default();
                if dex_pair != Hash160::zero() {
                    // If DEX pair is set, send to pair for auto-liquidity
                    let pair_balance = self.balances.get(&dex_pair).unwrap_or_default();
                    self.balances.insert(dex_pair, pair_balance + liquidity_amount);
                    
                    // Emit transfer event with proper Neo N3 format
                    Transfer::emit(Some(from), Some(dex_pair), liquidity_amount);
                } else {
                    // Otherwise, send to owner
                    let owner = self.owner.get().unwrap_or_default();
                    let owner_balance = self.balances.get(&owner).unwrap_or_default();
                    self.balances.insert(owner, owner_balance + liquidity_amount);
                    
                    // Emit transfer event with proper Neo N3 format
                    Transfer::emit(Some(from), Some(owner), liquidity_amount);
                }
            }
            
            // 2. Marketing tax
            if marketing_amount > 0 {
                let marketing_wallet = self.marketing_wallet.get().unwrap_or_default();
                let wallet_balance = self.balances.get(&marketing_wallet).unwrap_or_default();
                self.balances.insert(marketing_wallet, wallet_balance + marketing_amount);
                
                // Emit transfer event with proper Neo N3 format
                Transfer::emit(Some(from), Some(marketing_wallet), marketing_amount);
            }
            
            // 3. Buyback tax
            if buyback_amount > 0 {
                // For buyback/burn, tokens go to contract itself
                let contract_address = Runtime::executing_script_hash();
                let contract_balance = self.balances.get(&contract_address).unwrap_or_default();
                self.balances.insert(contract_address, contract_balance + buyback_amount);
                
                // Emit transfer event with proper Neo N3 format
                Transfer::emit(Some(from), Some(contract_address), buyback_amount);
            }
            
            // 4. Reflection tax
            if reflection_amount > 0 {
                // Accumulate in contract for later distribution
                let contract_address = Runtime::executing_script_hash();
                let contract_balance = self.balances.get(&contract_address).unwrap_or_default();
                self.balances.insert(contract_address, contract_balance + reflection_amount);
                
                // Emit transfer event with proper Neo N3 format
                Transfer::emit(Some(from), Some(contract_address), reflection_amount);
            }
        }
        
        /// Check if an address is exempt from taxes
        fn is_tax_exempt(&self, address: &Address) -> bool {
            self.tax_exempt.get(address).unwrap_or_default()
        }
        
        /// Try to distribute rewards if conditions are met
        fn try_distribute_rewards(&mut self) {
            let current_block = Runtime::get_block().index;
            let last_reward_block = self.last_reward_block.get().unwrap_or_default();
            let cycle_blocks = self.reward_cycle_blocks.get().unwrap_or_default();
            
            if current_block >= last_reward_block + cycle_blocks {
                self.do_distribute_rewards();
            }
        }
        
        /// Distribute rewards to holders
        fn do_distribute_rewards(&mut self) {
            // Get contract address and balance
            let contract_address = Runtime::executing_script_hash();
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
                            
                            // Emit transfer event with proper Neo N3 format
                            Transfer::emit(Some(contract_address), Some(address), reward);
                        }
                    }
                    
                    // Update reward stats
                    let total_rewards = self.total_rewards_distributed.get().unwrap_or_default();
                    self.total_rewards_distributed.set(total_rewards + total_distribution);
                    
                    // Emit rewards event with proper Neo N3 format
                    RewardsDistributed::emit(total_distribution, eligible_holders);
                }
            }
            
            // Update last reward block
            self.last_reward_block.set(Runtime::get_block().index);
        }
    }
}