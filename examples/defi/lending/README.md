# NEO Lending Protocol Smart Contract

A decentralized lending and borrowing protocol built on the Neo N3 blockchain using the neo-contract-rs framework. This contract enables users to supply assets, earn interest, and borrow against their collateral.

## Features

- **Multi-Asset Support**: Support for any NEP-17 compatible token
- **Variable Interest Rates**: Dynamic rates based on market utilization
- **Collateralized Borrowing**: Secure loans backed by supplied assets
- **Liquidation Mechanism**: Protection against undercollateralized positions
- **Risk Parameter Management**: Configurable collateral and reserve factors per asset
- **Price Oracle Integration**: External price feeds for accurate asset valuation

## How It Works

### Markets

Each supported asset has its own market with specific parameters:
- **Supply and Borrow Rates**: Dynamically adjusted based on market utilization
- **Collateral Factor**: Maximum loan-to-value ratio (e.g., 75% means you can borrow up to 75% of your collateral value)
- **Reserve Factor**: Portion of interest that goes to protocol reserves
- **Interest Rate Model**: Includes base rate, multiplier, and jump multiplier for high utilization

### Supplying Assets

When users supply assets:
1. They transfer tokens to the protocol
2. They receive a proportional amount of aTokens (interest-bearing tokens)
3. Supplied assets can be used as collateral for borrowing
4. Assets earn interest based on the market's supply rate

### Borrowing

Users can borrow assets if:
1. They have supplied enough collateral
2. The total borrow value doesn't exceed their collateral value adjusted by collateral factors
3. The protocol has enough liquidity of the requested asset

### Interest Accrual

- Interest accrues on every block based on utilization
- Borrow rates increase as utilization increases, especially after passing the "kink" point
- Supply rates are derived from borrow rates and market utilization

### Liquidation

If a borrower's position becomes undercollateralized:
1. Anyone can repay part of their debt (up to the close factor amount)
2. The liquidator receives collateral at a discount (liquidation incentive)
3. This process continues until the borrower's position is safe again or fully liquidated

## Contract Methods

### Market Management

- `list_market`: Add support for a new asset (admin only)
- `set_collateral_factor`: Update collateral factor for an asset (admin only)
- `set_reserve_factor`: Update reserve factor for an asset (admin only)

### User Operations

- `supply`: Provide assets to the protocol
- `withdraw`: Withdraw supplied assets 
- `borrow`: Borrow assets using collateral
- `repay`: Repay borrowed assets
- `set_asset_as_collateral`: Toggle using an asset as collateral

### Liquidation

- `liquidate_borrow`: Liquidate an undercollateralized borrower

### Protocol Administration

- `set_price_oracle`: Update price oracle address (admin only)
- `set_liquidation_incentive`: Update liquidation incentive (admin only)
- `set_close_factor`: Update close factor (admin only)
- `set_fee_collector`: Set protocol fee collector address (admin only)
- `set_protocol_seize_share`: Update protocol's share of liquidation (admin only)

### View Methods

- `get_market_info`: View market parameters and state
- `get_account_assets`: View user's supplied and borrowed assets
- `get_account_supplied_tokens`: View all tokens supplied by a user
- `get_account_borrowed_tokens`: View all tokens borrowed by a user
- `get_account_health`: Calculate account health factor
- `can_be_liquidated`: Check if an account can be liquidated
- `get_all_markets`: List all supported markets
- `get_price`: Get current price for an asset
- `get_liquidation_incentive`: Get current liquidation incentive
- `get_close_factor`: Get current close factor

## Interest Rate Model

The protocol uses a piecewise interest rate model:

For utilization < kink:
```
borrow_rate = base_rate + utilization * multiplier
```

For utilization ≥ kink:
```
borrow_rate = base_rate + kink * multiplier + (utilization - kink) * jump_multiplier
```

Supply rate is derived from borrow rate:
```
supply_rate = borrow_rate * utilization * (1 - reserve_factor)
```

## Security Considerations

- **Reentrancy Protection**: Critical operations are protected against reentrancy attacks
- **Access Control**: Admin-only functions are protected with caller verification
- **Solvency Checks**: All borrowing and withdrawal operations verify account solvency
- **Price Oracle Freshness**: Price data can be cached with timestamps to handle oracle failures
- **Integer Math**: Safe math operations to prevent overflow/underflow
- **Liquidation Incentives**: Calibrated to ensure timely liquidations without excessive penalties
- **Protocol Shares**: Protocol can collect a share of liquidation proceeds for sustainability

## Risk Parameters

Default parameters (can be adjusted by governance):
- **Liquidation Incentive**: 10% (1.1 × debt value in collateral)
- **Close Factor**: 50% (maximum portion of a borrow that can be repaid in a single liquidation)
- **Protocol Seize Share**: 5% (portion of liquidation proceeds that goes to the protocol)

Market-specific parameters:
- **Collateral Factor**: 0-90% (determined per asset based on risk profile)
- **Reserve Factor**: 0-50% (determined per asset based on risk profile)

## Usage Example

```python
# Python example using neo-python client
from neo3.api import SmartContract

# Contract hash of the deployed lending protocol
lending_hash = '0x1234567890abcdef1234567890abcdef12345678'
lending_contract = SmartContract(lending_hash)

# Supply GAS to the protocol
gas_token_hash = '0xd2a4cff31913016155e38e474a2c06d08be276cf'
wallet.sign_transaction(
    lending_contract.supply(
        token=gas_token_hash,
        amount=1000000000  # 10 GAS (assuming 8 decimals)
    )
)

# Borrow NEO using GAS as collateral
neo_token_hash = '0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5'
wallet.sign_transaction(
    lending_contract.borrow(
        token=neo_token_hash,
        amount=500000000  # 5 NEO (assuming 8 decimals)
    )
)

# Repay borrow
wallet.sign_transaction(
    lending_contract.repay(
        token=neo_token_hash,
        amount=500000000,  # 5 NEO
        borrower=None  # Repay own borrow
    )
)

# Liquidate undercollateralized borrower
wallet.sign_transaction(
    lending_contract.liquidate_borrow(
        borrower='NbnjKGMBJzJ6j5JwPPBhGGXxTj6qUbueNP',
        repay_token=neo_token_hash,
        collateral_token=gas_token_hash,
        repay_amount=100000000  # 1 NEO
    )
)
```

## License

This code is provided as an example and is licensed under MIT License.