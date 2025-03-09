//! # Simple DEX Contract
//!
//! A decentralized exchange implementation supporting token swaps,
//! liquidity provision, and automated market making.

#[neo_contract::contract]
mod neo_dex {
    use neo_contract::prelude::*;
    
    /// Liquidity Pool for a token pair
    #[derive(Debug, Clone, Encode, Decode)]
    struct LiquidityPool {
        /// First token hash in the pair
        token_a: Hash160,
        /// Second token hash in the pair
        token_b: Hash160,
        /// Amount of token A in the pool
        reserve_a: u64,
        /// Amount of token B in the pool
        reserve_b: u64,
        /// Total liquidity tokens issued
        total_liquidity: u64,
        /// Fee rate in basis points (100 = 1%)
        fee_rate: u16,
        /// Last update timestamp
        last_update: u64,
    }
    
    /// Provider's liquidity position
    #[derive(Debug, Clone, Encode, Decode)]
    struct LiquidityPosition {
        /// Provider's address
        provider: Address,
        /// Pool ID
        pool_id: u32,
        /// Amount of liquidity tokens owned
        liquidity_tokens: u64,
        /// Timestamp when position was created/updated
        timestamp: u64,
    }
    
    /// Swap operation details
    #[derive(Debug, Clone, Encode, Decode)]
    struct SwapOperation {
        /// User address
        user: Address,
        /// Pool ID
        pool_id: u32,
        /// Input token hash
        token_in: Hash160,
        /// Output token hash
        token_out: Hash160,
        /// Input amount
        amount_in: u64,
        /// Output amount
        amount_out: u64,
        /// Fee amount
        fee_amount: u64,
        /// Timestamp of the swap
        timestamp: u64,
    }
    
    /// Events emitted by the DEX contract
    #[event]
    struct PoolCreated {
        #[index]
        pool_id: u32,
        token_a: Hash160,
        token_b: Hash160,
        fee_rate: u16,
    }
    
    #[event]
    struct LiquidityAdded {
        #[index]
        pool_id: u32,
        #[index]
        provider: Address,
        amount_a: u64,
        amount_b: u64,
        liquidity_minted: u64,
    }
    
    #[event]
    struct LiquidityRemoved {
        #[index]
        pool_id: u32,
        #[index]
        provider: Address,
        amount_a: u64,
        amount_b: u64,
        liquidity_burned: u64,
    }
    
    #[event]
    struct Swap {
        #[index]
        pool_id: u32,
        #[index]
        user: Address,
        token_in: Hash160,
        token_out: Hash160,
        amount_in: u64,
        amount_out: u64,
        fee_amount: u64,
    }
    
    /// DEX contract storage
    #[storage]
    struct NeoDex {
        /// Contract owner
        owner: Item<Address>,
        
        /// Next pool ID
        next_pool_id: Item<u32>,
        
        /// Maps pool ID to liquidity pool
        pools: Map<u32, LiquidityPool>,
        
        /// Maps (token A, token B) to pool ID
        pool_by_tokens: Map<(Hash160, Hash160), u32>,
        
        /// Maps (provider, pool ID) to liquidity position
        positions: Map<(Address, u32), LiquidityPosition>,
        
        /// Maps provider to list of pool IDs they have positions in
        provider_pools: Map<Address, Vec<u32>>,
        
        /// Maps pool ID to list of providers
        pool_providers: Map<u32, Vec<Address>>,
        
        /// Swap history (limited storage - only recent swaps)
        recent_swaps: Map<u32, SwapOperation>,
        
        /// Next swap ID for recent swaps tracking
        next_swap_id: Item<u32>,
        
        /// Maximum number of recent swaps to store
        max_recent_swaps: Item<u32>,
        
        /// Default fee rate in basis points (30 = 0.3%)
        default_fee_rate: Item<u16>,
        
        /// Minimum liquidity requirement to prevent dust amounts
        min_liquidity: Item<u64>,
    }
    
    impl NeoDex {
        /// Initialize the DEX contract
        #[constructor]
        fn new(owner: Address) -> Self {
            Self {
                owner: Item::new(owner),
                next_pool_id: Item::new(1),
                pools: Map::new(),
                pool_by_tokens: Map::new(),
                positions: Map::new(),
                provider_pools: Map::new(),
                pool_providers: Map::new(),
                recent_swaps: Map::new(),
                next_swap_id: Item::new(1),
                max_recent_swaps: Item::new(100), // Store last 100 swaps
                default_fee_rate: Item::new(30),   // 0.3% default fee
                min_liquidity: Item::new(1_000),   // Minimum liquidity
            }
        }
        
        /// Create a new liquidity pool
        #[method]
        fn create_pool(&mut self, token_a: Hash160, token_b: Hash160, fee_rate: Option<u16>) -> u32 {
            // Ensure tokens are different
            assert!(token_a != token_b, "Tokens must be different");
            
            // Order tokens to ensure consistent lookup
            let (first_token, second_token) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };
            
            // Check if pool already exists
            assert!(
                !self.pool_by_tokens.contains_key(&(first_token, second_token)),
                "Pool already exists"
            );
            
            // Get default fee rate if not specified
            let pool_fee_rate = fee_rate.unwrap_or(*self.default_fee_rate.get());
            assert!(pool_fee_rate <= 1000, "Fee rate too high"); // Max 10%
            
            // Get next pool ID
            let pool_id = *self.next_pool_id.get();
            self.next_pool_id.set(pool_id + 1);
            
            // Create pool
            let pool = LiquidityPool {
                token_a: first_token,
                token_b: second_token,
                reserve_a: 0,
                reserve_b: 0,
                total_liquidity: 0,
                fee_rate: pool_fee_rate,
                last_update: runtime::time(),
            };
            
            // Store pool
            self.pools.insert(pool_id, pool);
            self.pool_by_tokens.insert((first_token, second_token), pool_id);
            
            // Initialize pool providers list
            self.pool_providers.insert(pool_id, Vec::new());
            
            // Emit event
            self.emit(PoolCreated {
                pool_id,
                token_a: first_token,
                token_b: second_token,
                fee_rate: pool_fee_rate,
            });
            
            pool_id
        }
        
        /// Add liquidity to a pool
        #[method]
        fn add_liquidity(
            &mut self,
            pool_id: u32,
            amount_a_desired: u64,
            amount_b_desired: u64,
            amount_a_min: u64,
            amount_b_min: u64,
        ) -> (u64, u64, u64) {
            let provider = runtime::calling_script_hash();
            
            // Verify provider signature
            assert!(runtime::check_witness(&provider), "Invalid signature");
            
            // Get pool
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Calculate optimal amounts
            let (amount_a, amount_b, liquidity_minted) = if pool.total_liquidity == 0 {
                // First liquidity provision
                // The initial liquidity token amount is the geometric mean of the input amounts
                let liquidity = (amount_a_desired as u128 * amount_b_desired as u128)
                    .integer_sqrt() as u64;
                    
                // Ensure minimum liquidity
                assert!(liquidity >= *self.min_liquidity.get(), "Insufficient initial liquidity");
                
                (amount_a_desired, amount_b_desired, liquidity)
            } else {
                // Not first provision - calculate based on existing reserves
                let amount_b_optimal = self.quote(amount_a_desired, pool.reserve_a, pool.reserve_b);
                
                if amount_b_optimal <= amount_b_desired {
                    // amount_b_optimal is the binding constraint
                    assert!(amount_b_optimal >= amount_b_min, "Insufficient B amount");
                    
                    // Calculate liquidity tokens to mint
                    let liquidity = self.min_value(
                        amount_a_desired * pool.total_liquidity / pool.reserve_a,
                        amount_b_optimal * pool.total_liquidity / pool.reserve_b
                    );
                    
                    (amount_a_desired, amount_b_optimal, liquidity)
                } else {
                    // amount_a_desired is the binding constraint
                    let amount_a_optimal = self.quote(amount_b_desired, pool.reserve_b, pool.reserve_a);
                    assert!(amount_a_optimal <= amount_a_desired, "Amounts reversed");
                    assert!(amount_a_optimal >= amount_a_min, "Insufficient A amount");
                    
                    // Calculate liquidity tokens to mint
                    let liquidity = self.min_value(
                        amount_a_optimal * pool.total_liquidity / pool.reserve_a,
                        amount_b_desired * pool.total_liquidity / pool.reserve_b
                    );
                    
                    (amount_a_optimal, amount_b_desired, liquidity)
                }
            };
            
            // Transfer tokens from provider to contract
            self.transfer_token_to_contract(&pool.token_a, &provider, amount_a);
            self.transfer_token_to_contract(&pool.token_b, &provider, amount_b);
            
            // Update pool reserves
            pool.reserve_a += amount_a;
            pool.reserve_b += amount_b;
            pool.total_liquidity += liquidity_minted;
            pool.last_update = runtime::time();
            
            // Save updated pool
            self.pools.insert(pool_id, pool);
            
            // Update or create liquidity position
            let position_key = (provider, pool_id);
            let mut position = self.positions.get(&position_key).unwrap_or_else(|| {
                // New position - add provider to pool_providers list
                let mut providers = self.pool_providers.get(&pool_id).unwrap_or_default();
                if !providers.contains(&provider) {
                    providers.push(provider);
                    self.pool_providers.insert(pool_id, providers);
                }
                
                // Add pool to provider_pools list
                let mut provider_pools = self.provider_pools.get(&provider).unwrap_or_default();
                if !provider_pools.contains(&pool_id) {
                    provider_pools.push(pool_id);
                    self.provider_pools.insert(provider, provider_pools);
                }
                
                // Create new position
                LiquidityPosition {
                    provider,
                    pool_id,
                    liquidity_tokens: 0,
                    timestamp: runtime::time(),
                }
            });
            
            // Update position
            position.liquidity_tokens += liquidity_minted;
            position.timestamp = runtime::time();
            self.positions.insert(position_key, position);
            
            // Emit event
            self.emit(LiquidityAdded {
                pool_id,
                provider,
                amount_a,
                amount_b,
                liquidity_minted,
            });
            
            (amount_a, amount_b, liquidity_minted)
        }
        
        /// Remove liquidity from a pool
        #[method]
        fn remove_liquidity(
            &mut self,
            pool_id: u32,
            liquidity: u64,
            amount_a_min: u64,
            amount_b_min: u64,
        ) -> (u64, u64) {
            let provider = runtime::calling_script_hash();
            
            // Verify provider signature
            assert!(runtime::check_witness(&provider), "Invalid signature");
            
            // Get pool
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get provider position
            let position_key = (provider, pool_id);
            let mut position = self.positions.get(&position_key).expect("No liquidity position");
            
            // Check if provider has enough liquidity tokens
            assert!(position.liquidity_tokens >= liquidity, "Insufficient liquidity");
            
            // Calculate token amounts to return
            let amount_a = pool.reserve_a * liquidity / pool.total_liquidity;
            let amount_b = pool.reserve_b * liquidity / pool.total_liquidity;
            
            // Check minimum amounts
            assert!(amount_a >= amount_a_min, "Insufficient A output");
            assert!(amount_b >= amount_b_min, "Insufficient B output");
            
            // Update pool reserves
            pool.reserve_a -= amount_a;
            pool.reserve_b -= amount_b;
            pool.total_liquidity -= liquidity;
            pool.last_update = runtime::time();
            
            // Save updated pool
            self.pools.insert(pool_id, pool.clone());
            
            // Update position
            position.liquidity_tokens -= liquidity;
            position.timestamp = runtime::time();
            
            if position.liquidity_tokens == 0 {
                // Remove position if no liquidity left
                self.positions.remove(&position_key);
                
                // Remove provider from pool_providers list
                let mut providers = self.pool_providers.get(&pool_id).unwrap_or_default();
                if let Some(index) = providers.iter().position(|p| p == &provider) {
                    providers.remove(index);
                    self.pool_providers.insert(pool_id, providers);
                }
                
                // Remove pool from provider_pools list
                let mut provider_pools = self.provider_pools.get(&provider).unwrap_or_default();
                if let Some(index) = provider_pools.iter().position(|p| p == &pool_id) {
                    provider_pools.remove(index);
                    self.provider_pools.insert(provider, provider_pools);
                }
            } else {
                // Save updated position
                self.positions.insert(position_key, position);
            }
            
            // Transfer tokens to provider
            self.transfer_token_from_contract(&pool.token_a, &provider, amount_a);
            self.transfer_token_from_contract(&pool.token_b, &provider, amount_b);
            
            // Emit event
            self.emit(LiquidityRemoved {
                pool_id,
                provider,
                amount_a,
                amount_b,
                liquidity_burned: liquidity,
            });
            
            (amount_a, amount_b)
        }
        
        /// Swap tokens
        #[method]
        fn swap(&mut self, pool_id: u32, token_in: Hash160, amount_in: u64, amount_out_min: u64) -> u64 {
            let user = runtime::calling_script_hash();
            
            // Verify user signature
            assert!(runtime::check_witness(&user), "Invalid signature");
            
            // Get pool
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Verify token_in is part of the pool
            assert!(
                token_in == pool.token_a || token_in == pool.token_b,
                "Token not in pool"
            );
            
            // Determine token_out
            let token_out = if token_in == pool.token_a { pool.token_b } else { pool.token_a };
            
            // Get current reserves
            let (reserve_in, reserve_out) = if token_in == pool.token_a {
                (pool.reserve_a, pool.reserve_b)
            } else {
                (pool.reserve_b, pool.reserve_a)
            };
            
            // Calculate fee
            let fee_amount = (amount_in * pool.fee_rate as u64) / 10000;
            let amount_in_with_fee = amount_in - fee_amount;
            
            // Calculate output amount using constant product formula
            // (x + dx) * (y - dy) = x * y
            // dy = y * dx / (x + dx)
            let numerator = amount_in_with_fee * reserve_out;
            let denominator = reserve_in + amount_in_with_fee;
            let amount_out = numerator / denominator;
            
            // Check minimum output amount
            assert!(amount_out >= amount_out_min, "Insufficient output amount");
            
            // Transfer token_in from user to contract
            self.transfer_token_to_contract(&token_in, &user, amount_in);
            
            // Update pool reserves
            if token_in == pool.token_a {
                pool.reserve_a += amount_in;
                pool.reserve_b -= amount_out;
            } else {
                pool.reserve_b += amount_in;
                pool.reserve_a -= amount_out;
            }
            
            pool.last_update = runtime::time();
            
            // Save updated pool
            self.pools.insert(pool_id, pool);
            
            // Transfer token_out to user
            self.transfer_token_from_contract(&token_out, &user, amount_out);
            
            // Record swap operation
            let swap = SwapOperation {
                user,
                pool_id,
                token_in,
                token_out,
                amount_in,
                amount_out,
                fee_amount,
                timestamp: runtime::time(),
            };
            
            let swap_id = *self.next_swap_id.get();
            self.next_swap_id.set((swap_id + 1) % *self.max_recent_swaps.get());
            self.recent_swaps.insert(swap_id, swap);
            
            // Emit event
            self.emit(Swap {
                pool_id,
                user,
                token_in,
                token_out,
                amount_in,
                amount_out,
                fee_amount,
            });
            
            amount_out
        }
        
        /// Update fee rate for a pool (owner only)
        #[method]
        fn update_fee_rate(&mut self, pool_id: u32, new_fee_rate: u16) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can update fee rate");
            
            // Validate fee rate
            assert!(new_fee_rate <= 1000, "Fee rate too high"); // Max 10%
            
            // Get pool
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Update fee rate
            pool.fee_rate = new_fee_rate;
            
            // Save updated pool
            self.pools.insert(pool_id, pool);
            
            true
        }
        
        /// Update default fee rate (owner only)
        #[method]
        fn update_default_fee_rate(&mut self, new_fee_rate: u16) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can update default fee rate");
            
            // Validate fee rate
            assert!(new_fee_rate <= 1000, "Fee rate too high"); // Max 10%
            
            // Update default fee rate
            self.default_fee_rate.set(new_fee_rate);
            
            true
        }
        
        /// Update min liquidity requirement (owner only)
        #[method]
        fn update_min_liquidity(&mut self, new_min_liquidity: u64) -> bool {
            let caller = runtime::calling_script_hash();
            assert!(caller == *self.owner.get(), "Only owner can update min liquidity");
            
            // Update min liquidity
            self.min_liquidity.set(new_min_liquidity);
            
            true
        }
        
        // === Safe methods (read-only) ===
        
        /// Get pool information
        #[safe]
        fn get_pool_info(&self, pool_id: u32) -> Option<(Hash160, Hash160, u64, u64, u64, u16)> {
            let pool = self.pools.get(&pool_id)?;
            
            Some((
                pool.token_a,
                pool.token_b,
                pool.reserve_a,
                pool.reserve_b,
                pool.total_liquidity,
                pool.fee_rate
            ))
        }
        
        /// Get pool ID by token pair
        #[safe]
        fn get_pool_id(&self, token_a: Hash160, token_b: Hash160) -> Option<u32> {
            // Order tokens to ensure consistent lookup
            let (first_token, second_token) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };
            
            self.pool_by_tokens.get(&(first_token, second_token)).copied()
        }
        
        /// Get user's liquidity position in a pool
        #[safe]
        fn get_liquidity_position(&self, user: Address, pool_id: u32) -> Option<u64> {
            let position = self.positions.get(&(user, pool_id))?;
            Some(position.liquidity_tokens)
        }
        
        /// Get all pools a user has liquidity in
        #[safe]
        fn get_user_pools(&self, user: Address) -> Vec<u32> {
            self.provider_pools.get(&user).unwrap_or_default().clone()
        }
        
        /// Get all providers for a pool
        #[safe]
        fn get_pool_providers(&self, pool_id: u32) -> Vec<Address> {
            self.pool_providers.get(&pool_id).unwrap_or_default().clone()
        }
        
        /// Calculate expected output amount for a swap
        #[safe]
        fn get_swap_quote(&self, pool_id: u32, token_in: Hash160, amount_in: u64) -> Option<u64> {
            let pool = self.pools.get(&pool_id)?;
            
            // Verify token_in is part of the pool
            if token_in != pool.token_a && token_in != pool.token_b {
                return None;
            }
            
            // Get current reserves
            let (reserve_in, reserve_out) = if token_in == pool.token_a {
                (pool.reserve_a, pool.reserve_b)
            } else {
                (pool.reserve_b, pool.reserve_a)
            };
            
            // Calculate fee
            let fee_amount = (amount_in * pool.fee_rate as u64) / 10000;
            let amount_in_with_fee = amount_in - fee_amount;
            
            // Calculate output amount using constant product formula
            let numerator = amount_in_with_fee * reserve_out;
            let denominator = reserve_in + amount_in_with_fee;
            
            Some(numerator / denominator)
        }
        
        /// Get price impact percentage (basis points) for a swap
        #[safe]
        fn get_price_impact(&self, pool_id: u32, token_in: Hash160, amount_in: u64) -> Option<u16> {
            let pool = self.pools.get(&pool_id)?;
            
            // Verify token_in is part of the pool
            if token_in != pool.token_a && token_in != pool.token_b {
                return None;
            }
            
            // Get current reserves
            let (reserve_in, reserve_out) = if token_in == pool.token_a {
                (pool.reserve_a, pool.reserve_b)
            } else {
                (pool.reserve_b, pool.reserve_a)
            };
            
            // Current price
            let current_price = (reserve_out as f64) / (reserve_in as f64);
            
            // Price after swap
            let amount_in_with_fee = amount_in - (amount_in * pool.fee_rate as u64) / 10000;
            let amount_out = self.get_swap_quote(pool_id, token_in, amount_in).unwrap_or_default();
            let new_reserve_in = reserve_in + amount_in_with_fee;
            let new_reserve_out = reserve_out - amount_out;
            let new_price = (new_reserve_out as f64) / (new_reserve_in as f64);
            
            // Calculate impact as percentage in basis points
            let impact = ((current_price - new_price) / current_price) * 10000.0;
            
            Some(impact as u16)
        }
        
        // === Helper methods ===
        
        /// Calculate proportional token amount
        fn quote(&self, amount_a: u64, reserve_a: u64, reserve_b: u64) -> u64 {
            amount_a * reserve_b / reserve_a
        }
        
        /// Return smaller of two values
        fn min_value(&self, a: u64, b: u64) -> u64 {
            if a < b { a } else { b }
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
    }
}