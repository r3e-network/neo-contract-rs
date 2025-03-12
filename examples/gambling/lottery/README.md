# Lottery Contract Example for Neo N3

This example demonstrates a lottery smart contract implementation for the Neo N3 blockchain using the Neo Contract Rust framework. The lottery contract allows users to purchase tickets for a chance to win prizes, with random winners selected using blockchain data as a source of randomness.

## Features

- **Round-based Lottery System**: Create and manage multiple lottery rounds
- **Configurable Parameters**: Customize ticket prices, duration, and commission rates
- **Fair Winner Selection**: Uses block hashes for transparent randomness
- **Commission System**: Configurable fee structure with separate collector address
- **Ticket Limits**: Optional caps on tickets per user to promote fairness
- **Comprehensive Events**: Detailed event emissions for all operations
- **Owner Controls**: Secure administration functions for lottery management

## Contract Structure

### Storage Model

The contract uses the following storage structure:

```rust
struct NeoLottery {
    /// Contract owner
    owner: Item<Address>,
    
    /// Current active lottery round ID
    current_round_id: Item<u32>,
    
    /// Maps round ID to lottery round data
    rounds: Map<u32, LotteryRound>,
    
    /// Maps (round_id, user) to total tickets purchased by user in the round
    user_tickets: Map<(u32, Address), u32>,
    
    /// Maps (round_id, user, index) to ticket purchase record
    ticket_purchases: Map<(u32, Address, u32), TicketPurchase>,
    
    /// Maps (round_id, ticket_index) to owner address
    ticket_owners: Map<(u32, u32), Address>,
    
    /// Maps round ID to list of participants
    round_participants: Map<u32, Vec<Address>>,
    
    /// GAS token contract hash
    gas_token: Item<Hash160>,
    
    /// Commission collector address (can be owner or separate)
    commission_collector: Item<Address>,
}
```

### Key Data Structures

#### LotteryRound

```rust
struct LotteryRound {
    /// Unique ID for this lottery round
    id: u32,
    /// Ticket price in GAS tokens
    ticket_price: u64,
    /// Start time (timestamp)
    start_time: u64,
    /// End time (timestamp)
    end_time: u64,
    /// Total tickets sold
    tickets_sold: u32,
    /// Total pot size
    pot_size: u64,
    /// Max tickets per user (0 = unlimited)
    max_tickets_per_user: u32,
    /// Lottery commission percentage (basis points: 100 = 1%)
    commission_rate: u16,
    /// Winner(s) address(es)
    winners: Vec<Address>,
    /// Status of this lottery round
    status: LotteryStatus,
    /// Block number used for winner selection
    winning_block: u32,
}
```

#### TicketPurchase

```rust
struct TicketPurchase {
    /// Round ID
    round_id: u32,
    /// User address
    user: Address,
    /// Number of tickets purchased
    ticket_count: u32,
    /// Purchase time
    purchase_time: u64,
    /// Ticket numbers (can be used for additional randomness)
    ticket_numbers: Vec<u32>,
}
```

## Core Functionality

### Creating a Lottery Round

The contract owner can create a new lottery round by specifying:
- Ticket price in GAS
- Duration in hours
- Maximum tickets per user (optional)
- Commission rate (in basis points, 100 = 1%)

```rust
fn create_lottery(
    &mut self, 
    ticket_price: u64, 
    duration_hours: u32, 
    max_tickets_per_user: u32, 
    commission_rate: u16
) -> u32
```

### Purchasing Tickets

Users can purchase tickets for an active lottery round:

```rust
fn buy_tickets(&mut self, round_id: u32, ticket_count: u32) -> bool
```

This function:
1. Verifies the lottery is active and hasn't ended
2. Checks that the user hasn't exceeded the maximum ticket limit
3. Calculates the total cost based on ticket price
4. Transfers GAS tokens from the user to the contract
5. Updates the lottery state and user ticket counts
6. Generates ticket numbers and assigns ownership

### Completing a Lottery Round

When a lottery round ends, the contract owner can complete it:

```rust
fn complete_lottery(&mut self, round_id: u32) -> bool
```

This function:
1. Verifies the lottery has ended and tickets were sold
2. Uses the current block hash as a source of randomness
3. Selects a winner based on the random number
4. Calculates prize amount after deducting commission
5. Transfers the prize to the winner
6. Transfers commission to the designated collector
7. Updates the lottery status to completed

## Randomness Generation

The contract uses block hashes as a source of randomness:

```rust
// Get current block for randomness
let current_block = Ledger::current_index();
lottery.winning_block = current_block;

// Select winner using block hash as randomness
let block_hash = Ledger::hash_at(current_block);
let random_number = self.bytes_to_u32(&block_hash.to_vec()) % lottery.tickets_sold + 1;
```

## Events

The contract emits detailed events for all operations:

- `LotteryCreated`: When a new lottery round is created
- `TicketsPurchased`: When tickets are bought
- `WinnerSelected`: When a winner is selected
- `LotteryCompleted`: When a lottery round is completed

## Security Considerations

- **Randomness Source**: The contract uses block hashes for randomness, which provides reasonable unpredictability for small-scale lotteries
- **Owner Controls**: Only the contract owner can create and complete lottery rounds
- **Time-based Constraints**: Lottery rounds have defined start and end times based on blockchain timestamps
- **Commission Limits**: Commission rates are capped at 20% to ensure fairness

## How to Build

### Development Build

For development and testing:

```bash
cargo check -p neo-lottery --features std
cargo build -p neo-lottery --features std
```

### Production Build

For blockchain deployment:

```bash
cargo build -p neo-lottery --release
```

## Using the Contract

### Deployment

Deploy the contract with initial parameters:

```
deploy neo-lottery.nef neo-lottery.manifest.json <owner_address> <gas_token_hash>
```

### Creating a Lottery Round

```
invoke <contract_hash> create_lottery 50000000 24 10 500
```
This creates a lottery with:
- 0.5 GAS ticket price (50000000 = 0.5 GAS in the smallest unit)
- 24 hours duration
- Maximum 10 tickets per user
- 5% commission rate (500 basis points)

### Purchasing Tickets

```
invoke <contract_hash> buy_tickets 1 5
```
This purchases 5 tickets for lottery round #1.

### Completing a Lottery Round

```
invoke <contract_hash> complete_lottery 1
```
This completes lottery round #1, selects a winner, and distributes the prize.

## Known Issues

1. **Procedural Macro Issues**: The `#[neo_contract::contract]`, `#[method]`, and other macros may not resolve correctly in the current Neo Contract Rust framework version
2. **Storage Trait Issues**: The `Storage` trait implementations might need to be updated
3. **Runtime Function Signature Mismatches**: Runtime API signatures may change between versions

## License

This example is provided under the same license as the Neo Contract Rust framework.