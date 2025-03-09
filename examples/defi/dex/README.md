# NEO Decentralized Exchange (DEX) Smart Contract

A decentralized exchange smart contract built on the Neo N3 blockchain using the neo-contract-rs framework. This DEX implements an automated market maker (AMM) model similar to Uniswap, allowing users to swap tokens, provide liquidity, and earn fees.

## Features

- **Automated Market Maker (AMM)**: Constant product formula (x * y = k) for token price determination
- **Liquidity Pools**: Create and manage liquidity pools for any NEP-17 token pair
- **Token Swaps**: Exchange one token for another with minimal slippage
- **Liquidity Provision**: Users can provide liquidity and receive pool tokens representing their share
- **Fee Collection**: Configurable fee rates on swaps that are distributed to liquidity providers
- **Price Oracle**: View current exchange rates between token pairs

## How It Works

### Liquidity Pools

Each liquidity pool contains reserves of two NEP-17 tokens. The product of the reserves (x * y = k) remains constant during swaps, which determines the exchange rate.

### Adding Liquidity

When providing liquidity:
1. Users deposit both tokens in the pool
2. They receive liquidity tokens representing their share of the pool
3. The amount of liquidity tokens is determined by the proportion of liquidity provided

### Removing Liquidity

When removing liquidity:
1. Users burn their liquidity tokens
2. They receive back both tokens proportional to their share of the pool

### Token Swaps

When swapping tokens:
1. User sends token A to the contract
2. The contract calculates the amount of token B to return using the constant product formula
3. A small fee is taken from the input amount (default: 0.3%)
4. The fee remains in the pool and is distributed to liquidity providers

## Contract Methods

### Pool Management

- `create_pool`: Create a new liquidity pool for a token pair
- `update_fee_rate`: Update the fee rate for a specific pool (admin only)
- `update_default_fee_rate`: Change the default fee rate for new pools (admin only)

### Liquidity Operations

- `add_liquidity`: Add liquidity to a pool and receive liquidity tokens
- `remove_liquidity`: Burn liquidity tokens and withdraw tokens from a pool

### Swapping

- `swap`: Exchange one token for another through a liquidity pool

### View Methods

- `get_pool_info`: View pool reserves, liquidity, and fee rate
- `get_pool_id`: Get pool ID for a token pair
- `get_liquidity_position`: View a user's liquidity position in a pool
- `get_user_pools`: Get all pools a user has liquidity in
- `get_pool_providers`: Get all providers for a pool
- `get_swap_quote`: Calculate expected output amount for a swap
- `get_price_impact`: Calculate the price impact percentage for a swap

## Mathematical Formulas

### Constant Product Formula

The core of the AMM is the constant product formula:
```
x * y = k
```
Where:
- x is the reserve of token A
- y is the reserve of token B
- k is a constant

### Swap Calculation

When swapping amount_in of token A for token B:
```
amount_out = (y * amount_in_with_fee) / (x + amount_in_with_fee)
```
Where:
- amount_in_with_fee = amount_in * (1 - fee_rate)

### Liquidity Calculation

When adding liquidity for the first time:
```
liquidity = sqrt(amount_a * amount_b)
```

For subsequent liquidity additions:
```
liquidity = min(
    amount_a * total_liquidity / reserve_a,
    amount_b * total_liquidity / reserve_b
)
```

## Security Considerations

- **Slippage Protection**: Input parameters for minimum output amounts to protect against front-running
- **Signature Verification**: All state-changing methods verify the caller's signature
- **Reentrancy Protection**: Critical operations are designed to prevent reentrancy attacks
- **Integer Overflow Handling**: Safe mathematics to prevent overflow/underflow attacks
- **Minimum Liquidity**: Required minimum liquidity to prevent manipulation of small pools

## Price Impact and Slippage

Large swaps in pools with limited liquidity can experience significant price impact. The contract provides methods to calculate:
- Expected output amount
- Price impact percentage
- These allow users to make informed decisions and set appropriate slippage tolerances

## Usage Example

```python
# Python example using neo-python client
from neo3.api import SmartContract

# Contract hash of the deployed DEX
dex_contract_hash = '0x1234567890abcdef1234567890abcdef12345678'
dex_contract = SmartContract(dex_contract_hash)

# Create a new pool (admin only)
# Gas token and some NEP-17 token
gas_token_hash = '0x1234567890abcdef1234567890abcdef12345678'
token_hash = '0xabcdef1234567890abcdef1234567890abcdef12'

wallet.sign_transaction(
    dex_contract.create_pool(
        token_a=gas_token_hash,
        token_b=token_hash,
        fee_rate=30  # 0.3%
    )
)

# Add liquidity
pool_id = 1
wallet.sign_transaction(
    dex_contract.add_liquidity(
        pool_id=pool_id,
        amount_a_desired=1000000000,  # 10 GAS (assuming 8 decimals)
        amount_b_desired=50000000,    # 50 tokens (assuming 7 decimals)
        amount_a_min=990000000,       # Minimum 9.9 GAS (1% slippage)
        amount_b_min=49500000         # Minimum 49.5 tokens (1% slippage)
    )
)

# Swap tokens
wallet.sign_transaction(
    dex_contract.swap(
        pool_id=pool_id,
        token_in=gas_token_hash,
        amount_in=100000000,        # 1 GAS
        amount_out_min=4900000      # Minimum 4.9 tokens expected
    )
)
```

## License

This code is provided as an example and is licensed under MIT License.