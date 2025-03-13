# Blockchain Games on Neo N3

This directory contains examples of blockchain games built with the Neo Contract Rust framework for the Neo N3 blockchain.

## Available Examples

### 1. [RPG Game](./rpg/)

A blockchain-based role-playing game demonstrating:
- Character NFTs with attributes and inventory
- In-game items as transferable assets
- Quest and achievement systems
- Experience and leveling mechanics

## Blockchain Gaming on Neo N3

Neo N3 provides several features that make it suitable for blockchain gaming:

1. **Low Fees**: Economical gas costs for frequent interactions
2. **Fast Finality**: Quick confirmation times for responsive gameplay
3. **NEP-11 Support**: Native NFT standard for in-game assets
4. **High TPS**: Throughput to support many concurrent players
5. **Rich Contract API**: Tools for randomization and complex game logic

## Common Gaming Patterns

These examples demonstrate several essential blockchain gaming patterns:

### Asset Management

```rust
#[storage]
struct GameAssets {
    // Map player address to array of owned item IDs
    player_inventory: StorageMap<H160, StorageList<u32>>,
    
    // Map item ID to item details
    items: StorageMap<u32, Item>,
    
    // Map player address to character data
    characters: StorageMap<H160, Character>,
}
```

### Random Number Generation

```rust
// Generate a pseudorandom number using blockchain data as seed
fn generate_random(player: H160, max: u32) -> u32 {
    // Use block hash, timestamp, and player address as randomness sources
    let block_hash = Ledger::current_hash();
    let timestamp = Ledger::current_timestamp();
    
    // Combine sources of randomness
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&block_hash.to_vec());
    bytes.extend_from_slice(&timestamp.to_le_bytes());
    bytes.extend_from_slice(&player.to_vec());
    
    // Create a hash of the combined values
    let hash = runtime::sha256(&bytes);
    
    // Convert first 4 bytes to u32 and mod by max
    let random_value = u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]]) % max;
    
    random_value
}
```

### Game State Transitions

```rust
// Handle player actions and state transitions
fn process_action(&mut self, player: H160, action_type: u8, target: u32) -> bool {
    // Verify player ownership of character
    if !self.owns_character(player) {
        return false;
    }
    
    // Process based on action type
    match action_type {
        // Attack action
        1 => self.process_attack(player, target),
        
        // Use item action
        2 => self.use_item(player, target),
        
        // Complete quest action
        3 => self.complete_quest(player, target),
        
        // Unknown action
        _ => false,
    }
}
```

## Building Blockchain Games

To build a blockchain game contract:

```bash
# Development build
cargo build -p game-example --features std

# Production build
cargo build -p game-example --release
```

## Gaming Frontend Integration

Blockchain games typically combine on-chain logic with off-chain frontends:

1. **Web Frontend**: React/Vue applications to visualize game state
2. **Game Engines**: Unity or Unreal Engine with Neo SDK integration
3. **Mobile Apps**: Native mobile clients that call contract methods
4. **Backend Services**: Optional centralized components for enhanced UX

Example JavaScript integration:

```javascript
// Connect to game contract
const gameContract = new neo.sc.Contract('0xYourGameContractHash');

// Get player character data
async function getCharacter(address) {
  return await gameContract.call('getCharacter', [address]);
}

// Execute game action (requires signed transaction)
async function executeAction(actionType, targetId) {
  const account = neo.wallet.Account.fromWIF('playerPrivateKey');
  
  const tx = await gameContract.invoke(
    'processAction',
    [actionType, targetId],
    account
  );
  
  return await tx.send();
}
```

## Economic Design Considerations

Blockchain games require careful economic design:

1. **Token Economics**: Balance between sinks and sources
2. **Secondary Markets**: Supporting player-to-player trading
3. **Anti-Exploitation**: Preventing gameplay exploits
4. **Value Proposition**: Ensuring items retain real-world value
5. **Governance**: How game rules can evolve over time

## License

These examples are provided under the same license as the Neo Contract Rust framework. 