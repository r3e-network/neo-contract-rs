# Lending Protocol Example for Neo N3

This example demonstrates a simple lending protocol implementation on the Neo N3 blockchain using the Neo Contract Rust framework.

## Lending Protocol Features

- **Interest-Bearing Deposits**: Earn interest by depositing assets
- **Collateralized Loans**: Borrow assets against collateral
- **Dynamic Interest Rates**: Interest rates that adjust based on utilization
- **Liquidation Mechanism**: Process to handle under-collateralized positions
- **Multi-Asset Support**: Support for multiple NEP-17 tokens

## Contract Structure

### Storage Model

The lending protocol maintains several key storage items:

```rust
#[storage]
struct LendingProtocol {
    // Supported assets and their configurations
    supported_assets: StorageMap<H160, AssetConfig>,
    
    // User deposits for each asset
    deposits: StorageMap<Vec<u8>, u64>, // key: asset_id + user_address
    
    // Outstanding loans
    loans: StorageMap<Vec<u8>, Loan>, // key: asset_id + user_address
    
    // Global statistics for each asset
    asset_stats: StorageMap<H160, AssetStats>,
    
    // Price oracle contract
    price_oracle: StorageItem<H160>,
    
    // Protocol administrator
    admin: StorageItem<H160>,
}
```

### Key Data Structures

```rust
struct AssetConfig {
    token_address: H160,
    collateral_factor: u64, // 0-10000 (basis points)
    liquidation_threshold: u64, // 0-10000 (basis points)
    liquidation_penalty: u64, // 0-10000 (basis points)
    base_rate: u64, // Base interest rate in basis points
    slope1: u64, // Interest rate model parameter 1
    slope2: u64, // Interest rate model parameter 2
}

struct AssetStats {
    total_deposits: u64,
    total_borrows: u64,
    total_reserves: u64,
    last_update_timestamp: u64,
    cumulative_interest_rate: u64,
}

struct Loan {
    principal: u64,
    interest_index: u64,
    collateral_asset: H160,
    collateral_amount: u64,
    start_timestamp: u64,
}
```

## Core Functionality

### Depositing Assets

Users can deposit NEP-17 tokens to earn interest:

```rust
#[method]
pub fn deposit(&mut self, asset_id: H160, amount: u64) -> bool {
    // Validate inputs
    assert!(amount > 0, "Amount must be greater than zero");
    assert!(self.is_supported_asset(&asset_id), "Asset not supported");
    
    // Get user address
    let user = Runtime::check_witness(&Runtime::current_sender())
        .expect("Authentication failed");
    
    // Transfer tokens from user to contract
    let success = self.transfer_from(asset_id, &user, &self.contract_hash(), amount);
    assert!(success, "Token transfer failed");
    
    // Update user deposit balance
    let deposit_key = self.get_deposit_key(&asset_id, &user);
    let current_deposit = self.deposits.get(&deposit_key).unwrap_or(0);
    let new_deposit = current_deposit + amount;
    self.deposits.insert(deposit_key, new_deposit);
    
    // Update asset statistics
    self.update_asset_stats(&asset_id, amount, 0);
    
    // Emit deposit event
    self.emit_deposit_event(&user, &asset_id, amount);
    
    true
}
```

### Borrowing Assets

Users can borrow assets by providing collateral:

```rust
#[method]
pub fn borrow(&mut self, asset_id: H160, amount: u64, collateral_asset_id: H160) -> bool {
    // Validate inputs
    assert!(amount > 0, "Amount must be greater than zero");
    assert!(self.is_supported_asset(&asset_id), "Asset not supported");
    assert!(self.is_supported_asset(&collateral_asset_id), "Collateral asset not supported");
    
    // Get user address
    let user = Runtime::check_witness(&Runtime::current_sender())
        .expect("Authentication failed");
    
    // Check collateral value and health factor
    let collateral_value = self.get_user_collateral_value(&user, &collateral_asset_id);
    let borrow_value = self.get_asset_value(&asset_id, amount);
    let config = self.supported_assets.get(&asset_id).unwrap();
    
    // Calculate maximum borrow amount based on collateral
    let max_borrow_value = collateral_value * config.collateral_factor / 10000;
    assert!(borrow_value <= max_borrow_value, "Insufficient collateral");
    
    // Create loan record
    let loan_key = self.get_loan_key(&asset_id, &user);
    let loan = Loan {
        principal: amount,
        interest_index: self.get_current_interest_index(&asset_id),
        collateral_asset: collateral_asset_id,
        collateral_amount: self.get_user_deposit(&user, &collateral_asset_id),
        start_timestamp: Ledger::current_timestamp(),
    };
    self.loans.insert(loan_key, loan);
    
    // Update asset statistics
    self.update_asset_stats(&asset_id, 0, amount);
    
    // Transfer borrowed tokens to user
    let success = self.transfer(asset_id, &user, amount);
    assert!(success, "Token transfer failed");
    
    // Emit borrow event
    self.emit_borrow_event(&user, &asset_id, amount, &collateral_asset_id);
    
    true
}
```

### Interest Rate Model

The protocol uses a dynamic interest rate model based on utilization:

```rust
fn calculate_interest_rate(&self, asset_id: &H160) -> u64 {
    let stats = self.asset_stats.get(asset_id).unwrap();
    let config = self.supported_assets.get(asset_id).unwrap();
    
    // If no deposits, return base rate
    if stats.total_deposits == 0 {
        return config.base_rate;
    }
    
    // Calculate utilization rate (0-10000)
    let utilization = stats.total_borrows * 10000 / stats.total_deposits;
    
    // Two-slope interest rate model
    if utilization <= 8000 {  // 80% utilization
        // Below optimal utilization: base_rate + slope1 * utilization
        config.base_rate + (utilization * config.slope1 / 10000)
    } else {
        // Above optimal utilization: add slope2 with higher weight
        let base_interest = config.base_rate + (8000 * config.slope1 / 10000);
        let excess_utilization = utilization - 8000;
        base_interest + (excess_utilization * config.slope2 / 10000)
    }
}
```

### Liquidation Mechanism

When collateral value falls below the liquidation threshold:

```rust
#[method]
pub fn liquidate(&mut self, borrower: H160, asset_id: H160, repay_amount: u64) -> bool {
    // Validate inputs
    assert!(repay_amount > 0, "Amount must be greater than zero");
    assert!(self.is_supported_asset(&asset_id), "Asset not supported");
    
    // Get liquidator address
    let liquidator = Runtime::check_witness(&Runtime::current_sender())
        .expect("Authentication failed");
    
    // Check if position is liquidatable
    let health_factor = self.calculate_health_factor(&borrower);
    assert!(health_factor < 10000, "Position is not liquidatable");
    
    // Get the loan details
    let loan_key = self.get_loan_key(&asset_id, &borrower);
    let mut loan = self.loans.get(&loan_key).expect("Loan not found");
    assert!(repay_amount <= loan.principal, "Cannot repay more than principal");
    
    // Calculate collateral to seize (including liquidation bonus)
    let config = self.supported_assets.get(&asset_id).unwrap();
    let collateral_config = self.supported_assets.get(&loan.collateral_asset).unwrap();
    
    let repay_value = self.get_asset_value(&asset_id, repay_amount);
    let liquidation_bonus = config.liquidation_penalty;
    let seize_value = repay_value * (10000 + liquidation_bonus) / 10000;
    
    let collateral_price = self.get_asset_price(&loan.collateral_asset);
    let seize_amount = seize_value * 10^8 / collateral_price;
    
    // Update loan state
    loan.principal -= repay_amount;
    if loan.principal == 0 {
        self.loans.remove(&loan_key);
    } else {
        self.loans.insert(loan_key, loan);
    }
    
    // Transfer repaid tokens from liquidator to contract
    let success = self.transfer_from(asset_id, &liquidator, &self.contract_hash(), repay_amount);
    assert!(success, "Token transfer failed");
    
    // Transfer seized collateral to liquidator
    let deposit_key = self.get_deposit_key(&loan.collateral_asset, &borrower);
    let current_deposit = self.deposits.get(&deposit_key).unwrap_or(0);
    assert!(current_deposit >= seize_amount, "Insufficient collateral");
    
    self.deposits.insert(deposit_key, current_deposit - seize_amount);
    
    let liquidator_deposit_key = self.get_deposit_key(&loan.collateral_asset, &liquidator);
    let liquidator_deposit = self.deposits.get(&liquidator_deposit_key).unwrap_or(0);
    self.deposits.insert(liquidator_deposit_key, liquidator_deposit + seize_amount);
    
    // Emit liquidation event
    self.emit_liquidation_event(&liquidator, &borrower, &asset_id, repay_amount, &loan.collateral_asset, seize_amount);
    
    true
}
```

## Building and Deploying

To build this lending protocol example:

```bash
# Development build
cargo build -p defi-lending --features std

# Production build
cargo build -p defi-lending --release
```

## Security Considerations

This lending protocol example demonstrates key security mechanisms:

1. **Collateral Factors**: Conservative collateralization requirements
2. **Price Oracle Integration**: Up-to-date asset prices for accurate valuations
3. **Liquidation Process**: Timely handling of under-collateralized positions
4. **Interest Rate Controls**: Encouraging optimal capital utilization
5. **Access Controls**: Administrative functions restricted to authorized users

## Integration with Front-end

This contract can be integrated with a front-end application to provide users with a complete lending platform experience:

```javascript
// JavaScript example with neo-js
const { rpc, sc, wallet } = require('@cityofzion/neo-js');

// Connect to the lending protocol contract
const lendingContract = new sc.Contract('0xYourContractScriptHash');

// Deposit assets
async function depositAsset(assetId, amount) {
  const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
  
  const tx = await lendingContract.invoke(
    'deposit',
    [assetId, amount],
    account
  );
  
  return await tx.send();
}

// Borrow assets
async function borrowAsset(assetId, amount, collateralAssetId) {
  const account = wallet.Account.fromWIF('YourPrivateKeyWIF');
  
  const tx = await lendingContract.invoke(
    'borrow',
    [assetId, amount, collateralAssetId],
    account
  );
  
  return await tx.send();
}
```

## Educational Value

This example demonstrates several important DeFi concepts:

1. **Risk Management**: Collateralization, health factors, and liquidation
2. **Interest Rate Models**: Dynamic interest rates based on utilization
3. **Price Oracle Integration**: Using external data for asset valuation
4. **Multi-Asset Support**: Managing different token types within a single protocol
5. **Event Emission**: Keeping an auditable history of all protocol activities

## Known Issues and Workarounds

As with other examples in this repository, you may encounter:

1. **Procedural Macro Issues**: The `#[contract]` and other macros may not resolve correctly
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
3. **Runtime Function Signature Mismatches**: Check for current function signatures in the framework

## License

This example is provided under the same license as the Neo Contract Rust framework.