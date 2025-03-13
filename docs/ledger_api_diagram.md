# Neo N3 Blockchain Structure Diagrams

This document provides visual diagrams of the Neo N3 blockchain structure to help developers understand the relationships between blocks, transactions, and how to navigate them using the Ledger API.

## Block Chain Structure

```
                            ┌───────────────┐                         ┌───────────────┐
                            │   Block N-1   │                         │    Block N    │
                            │               │                         │               │
                            │ hash: H256    │◄───────prev_hash────────│ hash: H256    │
                            │ index: N-1    │                         │ index: N      │
                            │ timestamp: T1 │                         │ timestamp: T2 │
                            │               │                         │               │
                            └───────┬───────┘                         └───────┬───────┘
                                    │                                         │
                                    │                                         │
                                    ▼                                         ▼
                            ┌───────────────┐                         ┌───────────────┐
                            │ Transactions  │                         │ Transactions  │
                            │ merkle_root   │                         │ merkle_root   │
                            └───────────────┘                         └───────────────┘
                                    │                                         │
                                    │                                         │
            ┌─────────────┬─────────┼─────────┬─────────────┐    ┌─────────────┬─────────┬─────────────┐
            │             │         │         │             │    │             │         │             │
            ▼             ▼         ▼         ▼             ▼    ▼             ▼         ▼             ▼
  ┌─────────────┐ ┌─────────────┐ ... ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ... ┌─────────────┐
  │    Tx 0     │ │    Tx 1     │     │   Tx M-1    │ │    Tx 0     │ │    Tx 1     │     │   Tx K-1    │
  └─────────────┘ └─────────────┘     └─────────────┘ └─────────────┘ └─────────────┘     └─────────────┘
```

## Accessing Blocks and Transactions

```
                  ┌─────────────────────────────────────────────────┐
                  │                  Smart Contract                  │
                  └──────────────────────┬──────────────────────────┘
                                         │
                                         │ API Calls
                                         │
                                         ▼
                  ┌─────────────────────────────────────────────────┐
                  │                   Ledger API                     │
                  └──────────┬────────────────────────────┬─────────┘
                             │                            │
                             │                            │
                             ▼                            ▼
         ┌────────────────────────────────┐   ┌────────────────────────────────┐
         │         Block Access           │   │       Transaction Access        │
         │                                │   │                                 │
         │  • current_index()             │   │  • get_transaction(hash)        │
         │  • current_hash()              │   │  • get_transaction_height(hash) │
         │  • hash_at(index)              │   │  • get_transaction_vm_state     │
         │  • get_block(index_or_hash)    │   │  • get_transaction_signers      │
         │  • current_timestamp()         │   │                                 │
         └────────────────┬───────────────┘   └─────────────────┬───────────────┘
                          │                                     │
                          │                                     │
                          ▼                                     ▼
         ┌────────────────────────────────┐   ┌────────────────────────────────┐
         │         Block Data             │   │         Transaction Data        │
         │                                │   │                                 │
         │ struct Block {                 │   │ struct Transaction {            │
         │   hash: H256,                  │   │   hash: H256,                   │
         │   version: u32,                │   │   version: u8,                  │
         │   prev_hash: H256,             │   │   nonce: u32,                   │
         │   merkle_root: H256,           │   │   sender: H160,                 │
         │   timestamp: u64,              │   │   system_fee: Int256,           │
         │   index: u32,                  │   │   network_fee: Int256,          │
         │   primary: u8,                 │   │   valid_until_block: u32,       │
         │   next_consensus: H160,        │   │   script: ByteString,           │
         │   transactions: Array,         │   │ }                               │
         │ }                              │   │                                 │
         └────────────────────────────────┘   └────────────────────────────────┘
```

## Block-to-Transaction Navigation

```
                             ┌────────────────────────────┐
                             │        Smart Contract      │
                             └──────────────┬─────────────┘
                                            │
                                            │ Query
                                            │
                       ┌───────────────────┐│┌───────────────────┐
                       │                   ▼▼                    │
                       │    ┌────────────────────────────┐       │
                       │    │         Ledger API         │       │
                       │    └────────────────────────────┘       │
                       │                                         │
          ┌────────────▼─────────────┐         ┌─────────────────▼──────────┐
          │                          │         │                            │
┌─────────▼──────────┐    ┌──────────▼─────────┐        ┌───────────────────▼───┐
│  current_index()   │    │    get_block()     │        │   get_transaction()   │
└──────────┬─────────┘    └──────────┬─────────┘        └───────────┬───────────┘
           │                         │                              │
           │                         │                              │
           │                         │                              │
           │                         ▼                              │
           │              ┌─────────────────────┐                   │
           │              │    Block Object     │                   │
           │              │ - hash: H256        │                   │
           │              │ - index: u32        │◄──────────────────┘
           │              │ - transactions: Array│                  │
           │              └─────────┬───────────┘                   │
           │                        │                               │
           │                        │                               │
           │                        ▼                               │
           │              ┌─────────────────────┐                   │
           │              │    Transaction at   │                   │
           │              │      Index X        │                   │
           │              └─────────┬───────────┘                   │
           │                        │                               │
           ▼                        ▼                               ▼
┌──────────────────────────────────────────────────────────────────────────┐
│                   Neo N3 Blockchain (Ledger State)                        │
└──────────────────────────────────────────────────────────────────────────┘
```

## Time-Based Logic in Neo N3

```
                   ┌───────────┐     ┌───────────┐     ┌───────────┐
                   │  Block 1  │     │  Block 2  │     │  Block 3  │
                   │ Time: T1  │────►│ Time: T2  │────►│ Time: T3  │─ ─ ─►
                   └───────────┘     └───────────┘     └───────────┘

Contract retrieves current timestamp with Ledger::current_timestamp()

┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│  ┌───────────────────┐         ┌───────────────────┐         ┌─────────────┴───┐
│  │ Vesting Schedule  │         │   Rate Limiting   │         │  Time-dependent  │
│  │                   │         │                   │         │    Execution     │
│  │ ┌───────────────┐ │         │ ┌───────────────┐ │         │ ┌───────────────┐│
│  │ │start_time: T1 │ │         │ │last_action: T2│ │         │ │deadline: T3   ││
│  │ │end_time: T4   │ │         │ │cooldown: 3600s│ │         │ │               ││
│  │ │               │ │         │ │               │ │         │ │               ││
│  │ └───────┬───────┘ │         │ └───────┬───────┘ │         │ └───────┬───────┘│
│  │         │         │         │         │         │         │         │        │
│  │         ▼         │         │         ▼         │         │         ▼        │
│  │ ┌───────────────┐ │         │ ┌───────────────┐ │         │ ┌───────────────┐│
│  │ │Calculate      │ │         │ │Check if       │ │         │ │Execute if     ││
│  │ │vested amount  │ │         │ │T_current >    │ │         │ │T_current <    ││
│  │ │based on       │ │         │ │last_action+   │ │         │ │deadline       ││
│  │ │current time   │ │         │ │cooldown       │ │         │ │               ││
│  │ └───────────────┘ │         │ └───────────────┘ │         │ └───────────────┘│
│  └───────────────────┘         └───────────────────┘         └───────────────────┘
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Transaction Confirmation Pattern

```
                  ┌───────────┐     ┌───────────┐     ┌───────────┐
                  │  Block N  │     │ Block N+1 │     │ Block N+2 │
                  │           │────►│           │────►│           │─ ─ ─►
                  └─────┬─────┘     └─────┬─────┘     └─────┬─────┘
                        │                 │                 │
                        ▼                 ▼                 ▼
                  ┌───────────┐     ┌───────────┐     ┌───────────┐
                  │  Tx Hash  │     │  Tx Hash  │     │  Tx Hash  │
                  │    A, B   │     │    C, D   │     │    E, F   │
                  └───────────┘     └───────────┘     └───────────┘
                        │
                        │ Transaction A is included in Block N
                        │  
                        ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                              Smart Contract                                 │
│                                                                             │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │                                                                       │ │
│  │            verify_transaction_confirmations(tx_hash_A, 3)             │ │
│  │                                                                       │ │
│  │  ┌─────────────────────────┐     ┌──────────────────────────────┐    │ │
│  │  │                         │     │                              │    │ │
│  │  │ get_transaction_height  │────►│ Check: current_index - height│    │ │
│  │  │        (tx_hash_A)      │     │         >= required          │    │ │
│  │  │                         │     │                              │    │ │
│  │  └─────────────────────────┘     └──────────────────────────────┘    │ │
│  │                                                                       │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘


Current block: N          ➜ 0 confirmations (not enough)
Current block: N+1        ➜ 1 confirmation (not enough)
Current block: N+2        ➜ 2 confirmations (not enough)
Current block: N+3 (or after) ➜ 3+ confirmations (valid)
```

These diagrams illustrate the key concepts of the Neo N3 blockchain structure and how to navigate it using the Ledger API in your smart contracts. Understanding these relationships is essential for implementing blockchain data dependent logic correctly. 