#![no_std]

extern crate alloc;

//! # Decentralized Exchange (DEX) Contract for Neo N3
//! 
//! This contract implements a constant product AMM (Automated Market Maker) DEX
//! that allows users to:
//! 1. Create liquidity pools for any pair of tokens
//! 2. Add liquidity to existing pools
//! 3. Remove liquidity from pools
//! 4. Swap tokens using pools
//! 
//! ## Event Handling
//! This contract uses the standardized Neo N3 event pattern:
//! - Events are defined as structs with the `#[event]` attribute
//! - Event parameters that need to be indexed for efficient filtering use the `#[index]` attribute
//! - Events are emitted using the `EventName::emit(params)` method
//! 
//! This approach is automatically provided by the neo-contract framework and is the
//! recommended way to handle events in Neo N3 smart contracts.

#[contract]
#[contract_author("R3E Network")]
#[contract_description("Decentralized Exchange (DEX) for Neo N3")]
#[contract_version("0.1.0")]
#[supported_standards("NEP-17")]
mod neo_dex {
    use neo_contract::prelude::*;
    use alloc::vec::Vec;
    
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
    
    /// Event emitted when a pool is created
    #[event]
    struct PoolCreated {
        #[index]
        pool_id: u32,
        token_a: Hash160,
        token_b: Hash160,
        fee_rate: u16,
    }
    
    /// Event emitted when liquidity is added to a pool
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
    
    /// Event emitted when liquidity is removed from a pool
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
    
    /// Event emitted when a swap occurs
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
            let mut instance = Self {
                owner: Item::new("owner"),
                next_pool_id: Item::new("next_pool_id"),
                pools: Map::new(),
                pool_by_tokens: Map::new(),
                positions: Map::new(),
                provider_pools: Map::new(),
                pool_providers: Map::new(),
                recent_swaps: Map::new(),
                next_swap_id: Item::new("next_swap_id"),
                max_recent_swaps: Item::new("max_recent_swaps"),
                default_fee_rate: Item::new("default_fee_rate"),
                min_liquidity: Item::new("min_liquidity"),
            };
            
            // Initialize values
            instance.owner.set(owner);
            instance.next_pool_id.set(1);
            instance.next_swap_id.set(1);
            instance.max_recent_swaps.set(100);  // Store last 100 swaps
            instance.default_fee_rate.set(30);   // 0.3% default fee
            instance.min_liquidity.set(1_000);   // Minimum liquidity
            
            instance
        }
        
        /// Create a new liquidity pool for a token pair
        /// 
        /// # Arguments
        /// * `token_a` - The first token in the pair
        /// * `token_b` - The second token in the pair
        /// * `fee_rate` - Optional fee rate in basis points (30 = 0.3%)
        /// 
        /// # Returns
        /// The ID of the newly created pool
        #[method]
        #[no_reentrant]
        fn create_pool(&mut self, token_a: Hash160, token_b: Hash160, fee_rate: Option<u16>) -> u32 {
            // Ensure tokens are different
            assert!(token_a != token_b, "Tokens must be different");
            
            // Order tokens for consistent lookup
            let (first_token, second_token) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };
            
            // Check if pool already exists
            let token_pair = (first_token, second_token);
            assert!(self.pool_by_tokens.get(&token_pair).is_none(), "Pool already exists");
            
            // Get the caller's address
            let caller = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&caller), "No authorization");
            
            // Get the next pool ID
            let pool_id = self.next_pool_id.get().unwrap_or_default();
            self.next_pool_id.set(pool_id + 1);
            
            // Set default fee rate if not provided
            let fee = fee_rate.unwrap_or(self.default_fee_rate.get().unwrap_or_default());
            
            // Create the pool
            let pool = LiquidityPool {
                token_a: first_token,
                token_b: second_token,
                reserve_a: 0,
                reserve_b: 0,
                total_liquidity: 0,
                fee_rate: fee,
                last_update: Runtime::time(),
            };
            
            // Save the pool
            self.pools.insert(pool_id, pool);
            self.pool_by_tokens.insert(token_pair, pool_id);
            
            // Emit pool created event
            PoolCreated::emit(pool_id, first_token, second_token, fee);
            
            pool_id
        }
        
        /// Add liquidity to a pool
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        /// * `amount_a_desired` - The desired amount of token A to add
        /// * `amount_b_desired` - The desired amount of token B to add
        /// * `amount_a_min` - The minimum acceptable amount of token A
        /// * `amount_b_min` - The minimum acceptable amount of token B
        /// 
        /// # Returns
        /// A tuple of (amount_a_added, amount_b_added, liquidity_minted)
        #[method]
        #[no_reentrant]
        fn add_liquidity(
            &mut self,
            pool_id: u32,
            amount_a_desired: u64,
            amount_b_desired: u64,
            amount_a_min: u64,
            amount_b_min: u64,
        ) -> (u64, u64, u64) {
            // Verify non-zero amounts
            assert!(amount_a_desired > 0 && amount_b_desired > 0, "Amounts must be greater than zero");
            
            // Get pool data
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let provider = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&provider), "No authorization");
            
            // Calculate amounts to add
            let (amount_a, amount_b, liquidity) = if pool.total_liquidity == 0 {
                // First liquidity provision
                // Calculate initial liquidity as sqrt(amount_a * amount_b)
                let initial_liquidity = (amount_a_desired as f64 * amount_b_desired as f64).sqrt() as u64;
                assert!(initial_liquidity > self.min_liquidity.get().unwrap_or_default(), 
                       "Initial liquidity too low");
                
                (amount_a_desired, amount_b_desired, initial_liquidity)
            } else {
                // Subsequent liquidity provision
                // Calculate based on current reserves
                let amount_b_optimal = self.quote(amount_a_desired, pool.reserve_a, pool.reserve_b);
                
                if amount_b_optimal <= amount_b_desired {
                    // amount_b_optimal is the limiting factor
                    assert!(amount_b_optimal >= amount_b_min, "Insufficient B amount");
                    
                    // Calculate liquidity tokens to mint
                    let liquidity = (amount_a_desired * pool.total_liquidity) / pool.reserve_a;
                    
                    (amount_a_desired, amount_b_optimal, liquidity)
                } else {
                    // amount_a is the limiting factor
                    let amount_a_optimal = self.quote(amount_b_desired, pool.reserve_b, pool.reserve_a);
                    assert!(amount_a_optimal <= amount_a_desired, "Calculated amount exceeds desired");
                    assert!(amount_a_optimal >= amount_a_min, "Insufficient A amount");
                    
                    // Calculate liquidity tokens to mint
                    let liquidity = (amount_b_desired * pool.total_liquidity) / pool.reserve_b;
                    
                    (amount_a_optimal, amount_b_desired, liquidity)
                }
            };
            
            // Transfer tokens from user to contract
            self.transfer_token_to_contract(&pool.token_a, &provider, amount_a);
            self.transfer_token_to_contract(&pool.token_b, &provider, amount_b);
            
            // Update pool reserves
            pool.reserve_a += amount_a;
            pool.reserve_b += amount_b;
            pool.total_liquidity += liquidity;
            pool.last_update = Runtime::time();
            
            // Update pool data
            self.pools.insert(pool_id, pool);
            
            // Update or create provider position
            let position_key = (provider, pool_id);
            let mut position = match self.positions.get(&position_key) {
                Some(pos) => pos,
                None => LiquidityPosition {
                    provider,
                    pool_id,
                    liquidity_tokens: 0,
                    timestamp: Runtime::time(),
                }
            };
            
            position.liquidity_tokens += liquidity;
            position.timestamp = Runtime::time();
            
            self.positions.insert(position_key, position);
            
            // Update provider pools list if first position
            let mut provider_pools = self.provider_pools.get(&provider).unwrap_or_default();
            if !provider_pools.contains(&pool_id) {
                provider_pools.push(pool_id);
                self.provider_pools.insert(provider, provider_pools);
            }
            
            // Update pool providers list if first position
            let mut pool_providers = self.pool_providers.get(&pool_id).unwrap_or_default();
            if !pool_providers.contains(&provider) {
                pool_providers.push(provider);
                self.pool_providers.insert(pool_id, pool_providers);
            }
            
            // Emit liquidity added event
            LiquidityAdded::emit(pool_id, provider, amount_a, amount_b, liquidity);
            
            (amount_a, amount_b, liquidity)
        }
        
        /// Remove liquidity from a pool
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        /// * `liquidity` - The amount of liquidity tokens to burn
        /// * `amount_a_min` - The minimum acceptable amount of token A
        /// * `amount_b_min` - The minimum acceptable amount of token B
        /// 
        /// # Returns
        /// A tuple of (amount_a_removed, amount_b_removed)
        #[method]
        #[no_reentrant]
        fn remove_liquidity(
            &mut self,
            pool_id: u32,
            liquidity: u64,
            amount_a_min: u64,
            amount_b_min: u64,
        ) -> (u64, u64) {
            // Get pool data
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let provider = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&provider), "No authorization");
            
            // Get the provider's position
            let position_key = (provider, pool_id);
            let mut position = self.positions.get(&position_key).expect("No liquidity position found");
            
            // Ensure they have enough liquidity tokens
            assert!(position.liquidity_tokens >= liquidity, "Insufficient liquidity tokens");
            
            // Calculate token amounts to return
            let amount_a = (liquidity * pool.reserve_a) / pool.total_liquidity;
            let amount_b = (liquidity * pool.reserve_b) / pool.total_liquidity;
            
            // Ensure minimum amounts are met
            assert!(amount_a >= amount_a_min, "Insufficient token A output");
            assert!(amount_b >= amount_b_min, "Insufficient token B output");
            
            // Update pool reserves
            pool.reserve_a -= amount_a;
            pool.reserve_b -= amount_b;
            pool.total_liquidity -= liquidity;
            pool.last_update = Runtime::time();
            
            // Update pool data
            self.pools.insert(pool_id, pool);
            
            // Update provider position
            position.liquidity_tokens -= liquidity;
            position.timestamp = Runtime::time();
            
            if position.liquidity_tokens == 0 {
                // Remove the position if no liquidity left
                self.positions.remove(&position_key);
                
                // Remove from provider pools list
                let mut provider_pools = self.provider_pools.get(&provider).unwrap_or_default();
                if let Some(idx) = provider_pools.iter().position(|&id| id == pool_id) {
                    provider_pools.remove(idx);
                    if provider_pools.is_empty() {
                        self.provider_pools.remove(&provider);
                    } else {
                        self.provider_pools.insert(provider, provider_pools);
                    }
                }
                
                // Remove from pool providers list
                let mut pool_providers = self.pool_providers.get(&pool_id).unwrap_or_default();
                if let Some(idx) = pool_providers.iter().position(|&addr| addr == provider) {
                    pool_providers.remove(idx);
                    if pool_providers.is_empty() {
                        self.pool_providers.remove(&pool_id);
                    } else {
                        self.pool_providers.insert(pool_id, pool_providers);
                    }
                }
            } else {
                // Update the position
                self.positions.insert(position_key, position);
            }
            
            // Transfer tokens to the provider
            self.transfer_token_from_contract(&pool.token_a, &provider, amount_a);
            self.transfer_token_from_contract(&pool.token_b, &provider, amount_b);
            
            // Emit liquidity removed event
            LiquidityRemoved::emit(pool_id, provider, amount_a, amount_b, liquidity);
            
            (amount_a, amount_b)
        }
        
        /// Get the amount of token B needed when providing token A
        /// 
        /// # Arguments
        /// * `amount_a` - The amount of token A
        /// * `reserve_a` - The reserve of token A
        /// * `reserve_b` - The reserve of token B
        ///
        /// # Returns
        /// The amount of token B needed
        #[safe]
        fn quote(&self, amount_a: u64, reserve_a: u64, reserve_b: u64) -> u64 {
            assert!(amount_a > 0, "Amount must be positive");
            assert!(reserve_a > 0 && reserve_b > 0, "Reserves must be positive");
            
            (amount_a * reserve_b) / reserve_a
        }
        
        /// Swap exact tokens for tokens
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        /// * `amount_in` - The exact amount of input tokens
        /// * `amount_out_min` - The minimum acceptable amount of output tokens
        /// * `is_token_a_in` - Whether the input token is token A
        /// 
        /// # Returns
        /// The amount of output tokens received
        #[method]
        #[no_reentrant]
        fn swap_exact_tokens_for_tokens(
            &mut self,
            pool_id: u32,
            amount_in: u64,
            amount_out_min: u64,
            is_token_a_in: bool,
        ) -> u64 {
            // Verify non-zero input
            assert!(amount_in > 0, "Input amount must be positive");
            
            // Get pool data
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let user = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&user), "No authorization");
            
            // Determine input and output tokens and reserves
            let (token_in, token_out, reserve_in, reserve_out) = if is_token_a_in {
                (pool.token_a, pool.token_b, pool.reserve_a, pool.reserve_b)
            } else {
                (pool.token_b, pool.token_a, pool.reserve_b, pool.reserve_a)
            };
            
            // Calculate output amount with fee
            let fee_numerator = 10000 - pool.fee_rate as u64;
            let amount_in_with_fee = amount_in * fee_numerator;
            let fee_amount = amount_in - (amount_in_with_fee / 10000);
            let numerator = amount_in_with_fee * reserve_out;
            let denominator = reserve_in * 10000 + amount_in_with_fee;
            let amount_out = numerator / denominator;
            
            // Ensure minimum output amount is met
            assert!(amount_out >= amount_out_min, "Insufficient output amount");
            
            // Transfer input tokens from user to contract
            self.transfer_token_to_contract(&token_in, &user, amount_in);
            
            // Transfer output tokens from contract to user
            self.transfer_token_from_contract(&token_out, &user, amount_out);
            
            // Update reserves
            if is_token_a_in {
                pool.reserve_a += amount_in;
                pool.reserve_b -= amount_out;
            } else {
                pool.reserve_b += amount_in;
                pool.reserve_a -= amount_out;
            }
            
            pool.last_update = Runtime::time();
            
            // Update pool data
            self.pools.insert(pool_id, pool);
            
            // Record the swap operation
            let swap_id = self.next_swap_id.get().unwrap_or_default();
            self.next_swap_id.set(swap_id + 1);
            
            let swap_op = SwapOperation {
                user,
                pool_id,
                token_in,
                token_out,
                amount_in,
                amount_out,
                fee_amount,
                timestamp: Runtime::time(),
            };
            
            // Store the swap (with limited history)
            self.recent_swaps.insert(swap_id, swap_op);
            
            // Remove old swaps if we exceed the maximum
            let max_swaps = self.max_recent_swaps.get().unwrap_or_default();
            if swap_id > max_swaps {
                self.recent_swaps.remove(&(swap_id - max_swaps));
            }
            
            // Emit swap event
            Swap::emit(pool_id, user, token_in, token_out, amount_in, amount_out, fee_amount);
            
            amount_out
        }
        
        /// Swap tokens for exact tokens
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        /// * `amount_out` - The exact amount of output tokens
        /// * `amount_in_max` - The maximum acceptable amount of input tokens
        /// * `is_token_a_in` - Whether the input token is token A
        /// 
        /// # Returns
        /// The amount of input tokens used
        #[method]
        #[no_reentrant]
        fn swap_tokens_for_exact_tokens(
            &mut self,
            pool_id: u32,
            amount_out: u64,
            amount_in_max: u64,
            is_token_a_in: bool,
        ) -> u64 {
            // Verify non-zero output
            assert!(amount_out > 0, "Output amount must be positive");
            
            // Get pool data
            let mut pool = self.pools.get(&pool_id).expect("Pool not found");
            
            // Get user address
            let user = Runtime::calling_script_hash();
            assert!(Runtime::check_witness(&user), "No authorization");
            
            // Determine input and output tokens and reserves
            let (token_in, token_out, reserve_in, reserve_out) = if is_token_a_in {
                (pool.token_a, pool.token_b, pool.reserve_a, pool.reserve_b)
            } else {
                (pool.token_b, pool.token_a, pool.reserve_b, pool.reserve_a)
            };
            
            // Ensure output amount is available
            assert!(amount_out < reserve_out, "Insufficient reserve");
            
            // Calculate input amount required with fee
            let fee_numerator = 10000 - pool.fee_rate as u64;
            let numerator = reserve_in * amount_out * 10000;
            let denominator = (reserve_out - amount_out) * fee_numerator;
            let amount_in = (numerator / denominator) + 1; // Add 1 to round up
            let fee_amount = (amount_in * pool.fee_rate as u64) / 10000;
            
            // Ensure maximum input amount is not exceeded
            assert!(amount_in <= amount_in_max, "Excessive input amount required");
            
            // Transfer input tokens from user to contract
            self.transfer_token_to_contract(&token_in, &user, amount_in);
            
            // Transfer output tokens from contract to user
            self.transfer_token_from_contract(&token_out, &user, amount_out);
            
            // Update reserves
            if is_token_a_in {
                pool.reserve_a += amount_in;
                pool.reserve_b -= amount_out;
            } else {
                pool.reserve_b += amount_in;
                pool.reserve_a -= amount_out;
            }
            
            pool.last_update = Runtime::time();
            
            // Update pool data
            self.pools.insert(pool_id, pool);
            
            // Record the swap operation
            let swap_id = self.next_swap_id.get().unwrap_or_default();
            self.next_swap_id.set(swap_id + 1);
            
            let swap_op = SwapOperation {
                user,
                pool_id,
                token_in,
                token_out,
                amount_in,
                amount_out,
                fee_amount,
                timestamp: Runtime::time(),
            };
            
            // Store the swap (with limited history)
            self.recent_swaps.insert(swap_id, swap_op);
            
            // Remove old swaps if we exceed the maximum
            let max_swaps = self.max_recent_swaps.get().unwrap_or_default();
            if swap_id > max_swaps {
                self.recent_swaps.remove(&(swap_id - max_swaps));
            }
            
            // Emit swap event
            Swap::emit(pool_id, user, token_in, token_out, amount_in, amount_out, fee_amount);
            
            amount_in
        }
        
        /// Get pool ID by token pair
        /// 
        /// # Arguments
        /// * `token_a` - The first token in the pair
        /// * `token_b` - The second token in the pair
        ///
        /// # Returns
        /// The pool ID, if it exists
        #[safe]
        fn get_pool_id(&self, token_a: Hash160, token_b: Hash160) -> Option<u32> {
            // Order tokens to ensure consistent lookup
            let (first_token, second_token) = if token_a < token_b {
                (token_a, token_b)
            } else {
                (token_b, token_a)
            };
            
            self.pool_by_tokens.get(&(first_token, second_token))
        }
        
        /// Get pool details
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        ///
        /// # Returns
        /// The liquidity pool details
        #[safe]
        fn get_pool(&self, pool_id: u32) -> Option<LiquidityPool> {
            self.pools.get(&pool_id)
        }
        
        /// Get user's liquidity position
        /// 
        /// # Arguments
        /// * `provider` - The address of the liquidity provider
        /// * `pool_id` - The ID of the pool
        ///
        /// # Returns
        /// The liquidity position details
        #[safe]
        fn get_position(&self, provider: Address, pool_id: u32) -> Option<LiquidityPosition> {
            self.positions.get(&(provider, pool_id))
        }
        
        /// Get all pools a user has liquidity in
        /// 
        /// # Arguments
        /// * `provider` - The address of the liquidity provider
        ///
        /// # Returns
        /// A list of pool IDs
        #[safe]
        fn get_provider_pools(&self, provider: Address) -> Vec<u32> {
            self.provider_pools.get(&provider).unwrap_or_default()
        }
        
        /// Get all providers for a specific pool
        /// 
        /// # Arguments
        /// * `pool_id` - The ID of the pool
        ///
        /// # Returns
        /// A list of provider addresses
        #[safe]
        fn get_pool_providers(&self, pool_id: u32) -> Vec<Address> {
            self.pool_providers.get(&pool_id).unwrap_or_default()
        }
        
        /// Get the details of a recent swap
        /// 
        /// # Arguments
        /// * `swap_id` - The ID of the swap
        ///
        /// # Returns
        /// The swap operation details
        #[safe]
        fn get_swap(&self, swap_id: u32) -> Option<SwapOperation> {
            self.recent_swaps.get(&swap_id)
        }
        
        /// Get the current contract owner
        ///
        /// # Returns
        /// The owner's address
        #[safe]
        fn get_owner(&self) -> Address {
            self.owner.get().unwrap_or_default()
        }
        
        /// Set a new contract owner
        /// 
        /// # Arguments
        /// * `new_owner` - The address of the new owner
        ///
        /// # Returns
        /// `true` if successful
        #[method]
        #[no_reentrant]
        fn set_owner(&mut self, new_owner: Address) -> bool {
            let current_owner = self.owner.get().unwrap_or_default();
            
            // Only current owner can change ownership
            assert!(Runtime::check_witness(&current_owner), "Only owner can transfer ownership");
            
            self.owner.set(new_owner);
            true
        }
        
        /// Set the default fee rate for new pools
        /// 
        /// # Arguments
        /// * `fee_rate` - The new default fee rate in basis points
        ///
        /// # Returns
        /// `true` if successful
        #[method]
        #[no_reentrant]
        fn set_default_fee_rate(&mut self, fee_rate: u16) -> bool {
            let owner = self.owner.get().unwrap_or_default();
            
            // Only owner can change the default fee rate
            assert!(Runtime::check_witness(&owner), "Only owner can change default fee rate");
            
            // Ensure the fee rate is reasonable
            assert!(fee_rate <= 1000, "Fee rate cannot exceed 10%");
            
            self.default_fee_rate.set(fee_rate);
            true
        }
        
        /// Set the maximum number of recent swaps to store
        /// 
        /// # Arguments
        /// * `max_swaps` - The maximum number of recent swaps
        ///
        /// # Returns
        /// `true` if successful
        #[method]
        #[no_reentrant]
        fn set_max_recent_swaps(&mut self, max_swaps: u32) -> bool {
            let owner = self.owner.get().unwrap_or_default();
            
            // Only owner can change the max recent swaps
            assert!(Runtime::check_witness(&owner), "Only owner can change max recent swaps");
            
            self.max_recent_swaps.set(max_swaps);
            true
        }
        
        /// Set the minimum liquidity requirement
        /// 
        /// # Arguments
        /// * `min_liquidity` - The minimum liquidity required
        ///
        /// # Returns
        /// `true` if successful
        #[method]
        #[no_reentrant]
        fn set_min_liquidity(&mut self, min_liquidity: u64) -> bool {
            let owner = self.owner.get().unwrap_or_default();
            
            // Only owner can change the minimum liquidity
            assert!(Runtime::check_witness(&owner), "Only owner can change minimum liquidity");
            
            self.min_liquidity.set(min_liquidity);
            true
        }
        
        /// Internal helper to transfer tokens from a user to the contract
        fn transfer_token_to_contract(&self, token: &Hash160, from: &Address, amount: u64) -> bool {
            // Create proper NEP-17 transfer arguments
            let mut transfer_args = Array::<Any>::new();
            transfer_args.push(Any::from(*from));
            transfer_args.push(Any::from(Runtime::executing_script_hash()));
            transfer_args.push(Any::from(amount));
            transfer_args.push(Any::from(ByteArray::new())); // data parameter
            
            // Call the token's transfer method
            let result = Runtime::call_contract(token, "transfer", &transfer_args)
                .expect("Failed to call transfer")
                .as_bool()
                .expect("Invalid transfer response");
            
            assert!(result, "Token transfer to contract failed");
            result
        }
        
        /// Internal helper to transfer tokens from the contract to a user
        fn transfer_token_from_contract(&self, token: &Hash160, to: &Address, amount: u64) -> bool {
            // Get the contract's script hash
            let contract_addr = Runtime::executing_script_hash();
            
            // Create proper NEP-17 transfer arguments
            let mut transfer_args = Array::<Any>::new();
            transfer_args.push(Any::from(contract_addr));
            transfer_args.push(Any::from(*to));
            transfer_args.push(Any::from(amount));
            transfer_args.push(Any::from(ByteArray::new())); // data parameter
            
            // Call the token's transfer method
            let result = Runtime::call_contract(token, "transfer", &transfer_args)
                .expect("Failed to call transfer")
                .as_bool()
                .expect("Invalid transfer response");
            
            assert!(result, "Token transfer from contract failed");
            result
        }
    }
}