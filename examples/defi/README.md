# DeFi Smart Contracts for Neo N3

This directory contains implementations of famous DeFi protocols adapted for Neo N3 blockchain using the Neo Contract Rust framework.

## 📦 Included Protocols

### 1. **Uniswap V2 AMM** (`uniswap-v2-amm/`)
- Automated Market Maker with constant product formula (x * y = k)
- Features:
  - Create liquidity pools for any token pair
  - Add/remove liquidity with LP tokens
  - Token swaps with configurable fees
  - Price quotes and slippage protection

### 2. **Compound Lending Protocol** (`compound-lending/`)
- Over-collateralized lending and borrowing
- Features:
  - Supply assets to earn interest
  - Borrow against collateral
  - Dynamic interest rates based on utilization
  - Liquidation of under-collateralized positions
  - cToken accounting for supplied assets

### 3. **Aave Flash Loans** (`aave-flashloan/`)
- Uncollateralized loans within single transaction
- Features:
  - Flash loan single or multiple assets
  - Configurable flash loan fees
  - Liquidity provision and withdrawal
  - Flash loan receiver callback interface
  - Fee accumulation and distribution

### 4. **Test Tokens** (`test-tokens/`)
- NEP-17 compliant tokens for testing
- Includes USDT, WBTC, and WETH test implementations

## 🚀 Quick Start

### Prerequisites

1. **Install Rust and Neo tools:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Neo Express (requires .NET)
dotnet tool install Neo.Express -g
```

2. **Install Python dependencies:**
```bash
pip install neo-mamba
```

### Setup Neo Express

1. **Initialize and start Neo Express:**
```bash
chmod +x setup-neo-express.sh
./setup-neo-express.sh
```

This will:
- Create a local Neo blockchain instance
- Create test wallets (alice, bob, charlie, defi-deployer)
- Distribute NEO and GAS to test wallets

### Build Contracts

```bash
chmod +x build-all.sh
./build-all.sh
```

This compiles all contracts to WASM format.

### Deploy Contracts

```bash
chmod +x deploy-contracts.py
./deploy-contracts.py
```

This will:
1. Deploy all test tokens (USDT, WBTC, WETH)
2. Deploy Uniswap V2 AMM and create trading pairs
3. Deploy Compound lending markets
4. Deploy Aave flash loan pool
5. Provide initial liquidity to all protocols

### Test DeFi Operations

```bash
chmod +x test-defi-operations.py
./test-defi-operations.py
```

This runs various DeFi operations:
- Token swaps on Uniswap
- Lending and borrowing on Compound
- Flash loans on Aave
- Liquidity provision
- Liquidations

## 📋 Contract Interfaces

### Uniswap V2 AMM

```rust
// Initialize pool
initialize_pool(token_a: H160, token_b: H160, fee_rate: u32)

// Add liquidity
add_liquidity(amount0_desired: Int256, amount1_desired: Int256, amount0_min: Int256, amount1_min: Int256)

// Remove liquidity
remove_liquidity(liquidity: Int256, amount0_min: Int256, amount1_min: Int256)

// Swap tokens
swap(amount_in: Int256, amount_out_min: Int256, token_in: H160)

// Get price quote
get_quote(amount_in: Int256, token_in: H160) -> Int256
```

### Compound Lending

```rust
// Initialize market
initialize_market(asset: H160, collateral_factor: u32, base_rate: u32, multiplier: u32, reserve_factor: u32)

// Supply assets
supply(amount: Int256)

// Withdraw assets
withdraw(amount: Int256)

// Borrow assets
borrow(amount: Int256)

// Repay loan
repay(amount: Int256)

// Liquidate position
liquidate(borrower: H160, repay_amount: Int256, collateral_asset: H160)
```

### Aave Flash Loans

```rust
// Initialize pool
initialize_pool(flash_loan_fee: u32)

// Execute flash loan
flash_loan(asset: H160, amount: Int256, params: ByteString)

// Flash loan multiple assets
flash_loan_multiple(assets: Vec<H160>, amounts: Vec<Int256>, params: ByteString)

// Deposit liquidity
deposit_liquidity(asset: H160, amount: Int256)

// Withdraw liquidity
withdraw_liquidity(asset: H160, amount: Int256)
```

## 🏗️ Architecture

### Smart Contract Design
- **Solana Anchor-style syntax** adapted for Neo N3
- **Account-based model** with explicit account management
- **Event-driven** architecture for tracking state changes
- **Modular design** with separate contracts for each protocol

### Security Features
- **Reentrancy protection** on all state-changing functions
- **Access control** for admin functions
- **Slippage protection** on swaps and liquidity operations
- **Collateral checks** before borrowing
- **Flash loan callbacks** with strict validation

### Gas Optimization
- **Efficient storage patterns** to minimize storage operations
- **Batch operations** where possible
- **Optimized mathematical operations**
- **Minimal external calls**

## 🧪 Testing

### Unit Tests
Each contract includes unit tests in the `tests/` directory:
```bash
cargo test --package uniswap-v2-amm
cargo test --package compound-lending
cargo test --package aave-flashloan
```

### Integration Tests
The `test-defi-operations.py` script runs end-to-end integration tests simulating real DeFi scenarios.

### Manual Testing
You can manually test contracts using Neo Express:

```bash
# Check token balance
neo-express contract invoke test-token-usdt balance_of \
  --arg '{"type":"Hash160","value":"alice"}' \
  alice -i defi-testnet.neo-express

# Perform a swap
neo-express contract invoke uniswap-v2-amm swap \
  --arg '{"type":"Integer","value":"1000000000"}' \
  --arg '{"type":"Integer","value":"900000000"}' \
  --arg '{"type":"Hash160","value":"<token_address>"}' \
  alice -i defi-testnet.neo-express
```

## 📊 Gas Costs

Approximate gas costs for operations (in GAS):
- Token Transfer: ~0.05 GAS
- Uniswap Swap: ~0.15 GAS
- Add Liquidity: ~0.20 GAS
- Compound Supply: ~0.15 GAS
- Compound Borrow: ~0.25 GAS
- Flash Loan: ~0.30 GAS

## 🔒 Security Considerations

1. **Audit Status**: These contracts are examples and have NOT been audited
2. **Test Environment**: Only deploy on testnet or Neo Express
3. **Known Limitations**:
   - Simplified price oracle implementation
   - Basic access control (production needs multi-sig)
   - No upgradability pattern implemented

## 🛠️ Troubleshooting

### Common Issues

1. **Build fails with "target not found"**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Neo Express not found**
   ```bash
   dotnet tool install Neo.Express -g
   export PATH=$PATH:~/.dotnet/tools
   ```

3. **Contract deployment fails**
   - Ensure Neo Express is running
   - Check wallet has sufficient GAS
   - Verify contract compiled successfully

4. **Transaction fails**
   - Check account has sufficient token balance
   - Verify approval was granted for token transfers
   - Ensure parameters are in correct format

## 📚 Resources

- [Neo N3 Documentation](https://docs.neo.org/)
- [Neo Contract Rust Framework](https://github.com/r3e-network/neo-contract-rs)
- [Uniswap V2 Whitepaper](https://uniswap.org/whitepaper.pdf)
- [Compound Protocol Docs](https://compound.finance/docs)
- [Aave Documentation](https://docs.aave.com/)

## 📄 License

These example contracts are provided for educational purposes. Use at your own risk.