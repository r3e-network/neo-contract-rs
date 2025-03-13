# Gambling Contracts for Neo N3

This directory contains examples of gambling/lottery smart contracts built with the Neo Contract Rust framework for the Neo N3 blockchain.

## Available Examples

### 1. [Lottery](./lottery/)

A blockchain-based lottery system demonstrating:
- Ticket purchasing
- Random winner selection
- Prize distribution
- Time-based drawing periods

## Gambling Contracts on Neo N3

Neo N3 provides several features that make it suitable for gambling applications:

1. **Verifiable Randomness**: Blockchain data as a source of pseudorandomness
2. **Transparent Operation**: All game mechanics visible on-chain
3. **Automated Payouts**: Smart contract-managed prize distribution
4. **Low Fees**: Economical gas costs for gaming operations
5. **Fast Finality**: Quick confirmation times for responsive gameplay

## Legal Considerations

**Important Note**: Gambling applications may be subject to legal restrictions in many jurisdictions. These examples are provided for educational purposes only. Developers should:

1. Consult legal experts before deploying gambling applications
2. Implement age verification and region blocking where required
3. Follow all applicable KYC/AML regulations
4. Consider licensing requirements for gaming operators

## Common Gambling Patterns

These examples demonstrate several essential gambling contract patterns:

### Randomness Generation

```rust
// Generate a pseudorandom number using blockchain data
fn generate_random(max: u32) -> u32 {
    // Use block hash and timestamp as randomness sources
    let block_hash = Ledger::current_hash();
    let timestamp = Ledger::current_timestamp();
    
    // Combine sources of randomness
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&block_hash.to_vec());
    bytes.extend_from_slice(&timestamp.to_le_bytes());
    
    // Create a hash of the combined values
    let hash = runtime::sha256(&bytes);
    
    // Convert first 4 bytes to u32 and mod by max
    let random_value = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]) % max;
    
    random_value
}
```

### Escrow and Payout

```rust
// Distribute prizes to winners
fn distribute_prize(&mut self, winner: H160) -> bool {
    // Get the current prize pool
    let prize_pool = self.prize_pool.get();
    if prize_pool == 0 {
        return false;
    }
    
    // Calculate winner's prize (95% of pool)
    let winner_prize = prize_pool * 95 / 100;
    
    // Calculate operator fee (5% of pool)
    let operator_fee = prize_pool - winner_prize;
    
    // Reset prize pool
    self.prize_pool.set(0);
    
    // Transfer funds to winner
    if !self.transfer_neo(winner, winner_prize) {
        return false;
    }
    
    // Transfer fee to operator
    if operator_fee > 0 {
        let operator = self.operator.get();
        if !self.transfer_neo(operator, operator_fee) {
            return false;
        }
    }
    
    // Emit winner event
    let mut event_data = Array::new();
    event_data.push(Any::from(winner));
    event_data.push(Any::from(winner_prize));
    Runtime::notify(&ByteString::from("Winner"), &event_data);
    
    true
}
```

### Time-Based Actions

```rust
// Check if lottery drawing period has ended
fn is_drawing_time(&self) -> bool {
    let current_time = Ledger::current_timestamp();
    let end_time = self.drawing_end_time.get();
    
    current_time >= end_time
}
```

## Security Considerations

Gambling contracts require special security considerations:

1. **Randomness Manipulation**: Miners/validators could potentially manipulate on-chain randomness
2. **Front-Running Protection**: Prevent players from seeing winning numbers before committing
3. **Reentrancy Protection**: Ensure payment functions cannot be exploited
4. **Fair Game Mechanics**: Provably fair algorithms with transparent odds
5. **Self-Exclusion**: Allow players to opt-out for responsible gambling

## Building Gambling Contracts

To build a gambling contract:

```bash
# Development build
cargo build -p lottery-example --features std

# Production build
cargo build -p lottery-example --release
```

## Fairness Verification

Blockchain gambling should be provably fair:

1. **Transparency**: All game logic visible in the contract code
2. **Seed Verification**: Allow verification of randomness sources
3. **Odds Disclosure**: Clearly display winning probabilities
4. **Transaction Records**: Maintain complete history of game outcomes

## License

These examples are provided under the same license as the Neo Contract Rust framework. 