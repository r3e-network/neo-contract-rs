# Escrow Contract Workflow

This document provides a visual explanation of the escrow contract's workflow and interaction with blockchain data.

## Escrow Lifecycle

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │     │                │
│  Create Escrow │────▶│  Active Escrow │────▶│  Funds Released│────▶│    Completed   │
│                │     │                │     │                │     │                │
└────────────────┘     └───────┬────────┘     └────────────────┘     └────────────────┘
                              │
                              │                ┌────────────────┐     ┌────────────────┐
                              │                │                │     │                │
                              └───────────────▶│      Disputed  │────▶│ DisputeResolved│
                              │                │                │     │                │
                              │                └────────────────┘     └────────────────┘
                              │
                              │                ┌────────────────┐
                              │                │                │
                              └───────────────▶│     Refunded   │
                                               │                │
                                               └────────────────┘
```

## Time-Locked Release Process

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │     │                │
│  Create Escrow │────▶│ Time Lock Set  │────▶│  Time Expired  │────▶│ Release Allowed │
│                │     │ record timestamp│     │ check timestamp│     │                │
└────────────────┘     └────────────────┘     └────────────────┘     └────────────────┘
      │                                               ▲
      │                                               │
      │                ┌────────────────┐             │
      │                │                │             │
      └───────────────▶│ Early Release  │─────────────┘
                       │ (Sender only)  │
                       └────────────────┘
```

## Transaction Validation Flow

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │     │                │
│Submit Transaction│───▶│ TX in Blockchain│───▶│ Confirmations  │────▶│Process Payment  │
│                │     │  check tx_hash  │     │ check block height│   │ update escrow  │
└────────────────┘     └────────────────┘     └────────────────┘     └────────────────┘
                                                     │
                                                     │ If not enough confirmations
                                                     ▼
                                              ┌────────────────┐
                                              │                │
                                              │   Wait more    │
                                              │                │
                                              └────────────────┘
```

## Dispute Resolution Process

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │     │                │
│   Open Dispute │────▶│ Set Resolution │────▶│Wait for Blocks │────▶│Owner Resolves   │
│                │     │   Block Height │     │check block height│   │                │
└────────────────┘     └────────────────┘     └────────────────┘     └────────────────┘
                                                     │
                                                     │ If current block < resolution block
                                                     ▼
                                              ┌────────────────┐
                                              │                │
                                              │Resolution Failed│
                                              │                │
                                              └────────────────┘
```

## Rate Limiting Mechanism

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │     │                │
│ Action Request │────▶│ Check Last Time│────▶│Compare with Now│────▶│ Allow Action   │
│                │     │                │     │                │     │                │
└────────────────┘     └────────────────┘     └────────────────┘     └────────────────┘
                                                     │
                                                     │ If (now < last_time + cooldown)
                                                     ▼
                                              ┌────────────────┐
                                              │                │
                                              │ Rate Limited   │
                                              │                │
                                              └────────────────┘
```

## Ledger API Integration Points

| Contract Function | Ledger API Used | Purpose |
|------------------|-----------------|---------|
| `create_escrow` | `current_timestamp()` | Record creation time and calculate release time |
| `release_escrow` | `current_timestamp()` | Check if time lock has expired |
| `open_dispute` | `current_index()` | Set future block for resolution deadline |
| `resolve_dispute` | `current_index()` | Verify dispute resolution period has passed |
| `validate_tx` | `get_transaction_height()` | Verify transaction inclusion |
| `validate_tx` | `current_index()` | Calculate confirmation count |
| `validate_tx` | `get_transaction_vm_state()` | Verify transaction success |
| `check_rate_limit` | `current_timestamp()` | Implement cooldown periods |

## Escrow Entities and Relationships

```
┌────────────────┐      ┌────────────────┐      ┌────────────────┐
│                │      │                │      │                │
│     Sender     │◄────▶│     Escrow     │◄────▶│   Recipient    │
│                │      │                │      │                │
└────────────────┘      └───────┬────────┘      └────────────────┘
                               │
                               │
┌────────────────┐      ┌─────▼──────────┐      ┌────────────────┐
│                │      │                │      │                │
│  Transaction   │◄────▶│  Dispute Case  │◄────▶│  Contract Owner│
│                │      │                │      │                │
└────────────────┘      └────────────────┘      └────────────────┘
```

## Example Timeline

For a 24-hour locked escrow with a dispute:

1. **T=0**: Escrow created with 24h lock (`current_timestamp()` = 1000)
2. **T=12h**: Dispute opened (`current_index()` = 2880)
3. **T=12h+100 blocks**: Dispute resolution available (`current_index()` = 2980) 
4. **T=13h**: Owner resolves dispute

For transaction validation:

1. **Block N**: Transaction included in blockchain
2. **Block N+3**: Transaction has sufficient confirmations for processing
3. **Block N+3+**: Escrow updated based on transaction 