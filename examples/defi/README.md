# DeFi Examples for Neo Smart Contracts

This directory contains examples of Decentralized Finance (DeFi) applications built with the Neo Contract Rust framework for the Neo N3 blockchain.

## Available Examples

### 1. [Lending Platform](./lending/)

A simple lending protocol implementation demonstrating:
- Interest rate calculations
- Collateralized loans
- Liquidation mechanisms
- Token borrowing and repayment

### 2. [Decentralized Exchange (DEX)](./dex/)

A basic decentralized exchange implementation demonstrating:
- Automated Market Maker (AMM) functionality
- Liquidity pool management
- Token swapping mechanisms
- Fee collection and distribution

### 3. [Staking Platform](./staking/)

A token staking platform demonstrating:
- Yield generation
- Time-locked deposits
- Reward distribution
- Staking incentives

## DeFi on Neo N3

Neo N3 blockchain provides several advantages for DeFi applications:

1. **Fast Finality**: ~15 second block times for quick transaction confirmations
2. **Low Fees**: Economical gas fees for DeFi operations
3. **Native Oracle Support**: Built-in oracle functionality for price feeds
4. **Native Assets**: GAS and NEO as first-class citizens
5. **Rich Smart Contract API**: Access to cryptographic functions, time, and randomization

## Common DeFi Patterns

These examples demonstrate several essential DeFi patterns:

### Price Oracle Integration

```rust
// Using an off-chain price oracle
fn get_asset_price(asset_id: H160) -> u64 {
    // Call to Oracle contract to get latest price
    let oracle_contract = H160::from_str("0xYourOracleContractHash").unwrap();
    let mut args = Array::new();
    args.push(Any::from(asset_id));
    
    match Runtime::call(oracle_contract, "getPrice", args, CallFlags::READ_ONLY) {
        Some(result) => result.as_u64().unwrap_or(0),
        None => 0,
    }
}
```

### Time-Based Logic

```rust
// Calculate interest based on time elapsed
fn calculate_interest(principal: u64, rate: u64, deposit_time: u64) -> u64 {
    let current_time = Ledger::current_timestamp();
    let time_elapsed = current_time - deposit_time;
    
    // Calculate interest (simplified)
    // rate is annual interest rate expressed as basis points (e.g. 500 = 5%)
    let seconds_per_year = 31_536_000u64;
    let interest = principal * rate * time_elapsed / (10_000 * seconds_per_year);
    
    interest
}
```

### Liquidity Pool Management

```rust
#[storage]
struct LiquidityPool {
    // Token reserves
    token_a_reserve: StorageItem<u64>,
    token_b_reserve: StorageItem<u64>,
    // LP token supply
    lp_token_supply: StorageItem<u64>,
    // LP token balances
    lp_balances: StorageMap<H160, u64>,
}

// Calculate swap amount using constant product formula (x * y = k)
fn calculate_output_amount(input_amount: u64, input_reserve: u64, output_reserve: u64) -> u64 {
    // Include fee (0.3%)
    let input_amount_with_fee = input_amount * 997;
    
    // Calculate based on constant product formula
    let numerator = input_amount_with_fee * output_reserve;
    let denominator = input_reserve * 1000 + input_amount_with_fee;
    
    numerator / denominator
}
```

## Building DeFi Contracts

To build a DeFi contract:

```bash
# Development build
cargo build -p defi-example --features std

# Production build
cargo build -p defi-example --release
```

## Security Considerations

DeFi applications require special attention to security:

1. **Reentrancy Protection**: Prevent attackers from recursively calling contracts
2. **Integer Overflow/Underflow**: Use checked math operations
3. **Access Control**: Clearly define who can call administrative functions
4. **Price Oracle Security**: Ensure price feeds are reliable and tamper-resistant
5. **Flash Loan Protection**: Be aware of atomic transactions that can manipulate markets
6. **Slippage Protection**: Include deadline and slippage parameters in swap functions

## Testing DeFi Contracts

Testing DeFi contracts requires specialized approaches:

1. **Time Simulation**: Test interest accrual by simulating time passing
2. **Price Movement Simulation**: Test liquidation mechanisms with price fluctuations
3. **Economic Attack Vectors**: Test against various market manipulation scenarios
4. **Formal Verification**: Consider mathematical proofs for critical formulas

## Known Issues and Workarounds

As with other examples, you may encounter:

1. **Procedural Macro Issues**: The `#[contract]` and other macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
3. **Runtime Function Signature Mismatches**: Check for current function signatures in the framework

## License

These examples are provided under the same license as the Neo Contract Rust framework. 