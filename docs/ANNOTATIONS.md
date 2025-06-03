# Neo Contract Annotations Guide

This guide provides a comprehensive reference for all annotations available in the neo-contract-rs framework.

## Overview

The neo-contract-rs framework uses Rust annotations (attributes) to provide a declarative way to define smart contracts. These annotations are processed by procedural macros to generate the necessary boilerplate code for Neo N3 contract deployment and execution.

## Contract-Level Annotations

### `#[contract]`

Marks a struct as a Neo smart contract.

```rust
#[contract]
pub struct MyToken {
    #[storage]
    balances: StorageMap<H160, u64>,
    
    #[storage]
    total_supply: StorageItem<u64>,
}
```

**Usage:**
- Must be applied to a struct
- The struct represents the contract's persistent state
- Generates contract initialization and deployment code

### `#[contract_author]`

Specifies the contract author in the manifest.

```rust
#[contract]
#[contract_author("Alice Developer")]
pub struct MyContract {
    // ...
}
```

### `#[contract_description]`

Provides a description for the contract manifest.

```rust
#[contract]
#[contract_description("A NEP-17 token implementation")]
pub struct MyToken {
    // ...
}
```

### `#[contract_version]`

Sets the contract version.

```rust
#[contract]
#[contract_version("1.0.0")]
pub struct MyContract {
    // ...
}
```

### `#[supported_standards]`

Declares which NEP standards the contract implements.

```rust
#[contract]
#[supported_standards("NEP-17")]
pub struct MyToken {
    // ...
}
```

## Storage Annotations

### `#[storage]`

Marks a field as persistent contract storage.

```rust
#[contract]
pub struct MyContract {
    #[storage]
    owner: StorageItem<H160>,
    
    #[storage]
    balances: StorageMap<H160, u64>,
    
    #[storage]
    metadata: StorageMap<String, String>,
}
```

**Storage Types:**
- `StorageItem<T>`: Single value storage
- `StorageMap<K, V>`: Key-value storage
- `StorageSet<T>`: Set storage (if available)

## Method Annotations

### `#[constructor]`

Marks a method as the contract constructor.

```rust
impl MyContract {
    #[constructor]
    pub fn new(owner: H160, initial_supply: u64) -> Self {
        let mut contract = Self::default();
        contract.owner.put(owner);
        contract.total_supply.put(initial_supply);
        contract.balances.put(&owner, initial_supply);
        contract
    }
}
```

**Requirements:**
- Must return `Self`
- Called only once during contract deployment
- Can take parameters for contract initialization

### `#[method]`

Marks a method as a public contract method that can modify state.

```rust
impl MyContract {
    #[method]
    pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
        let caller = Runtime::calling_script_hash();
        // Transfer logic...
        true
    }
}
```

**Characteristics:**
- Can modify contract state (`&mut self`)
- Consumes gas for state changes
- Can return values
- Can emit events

### `#[safe]`

Marks a method as read-only (cannot modify contract state).

```rust
impl MyContract {
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    #[safe]
    pub fn total_supply(&self) -> u64 {
        self.total_supply.get().unwrap_or(0)
    }
}
```

**Characteristics:**
- Read-only access to contract state (`&self`)
- Lower gas cost
- Cannot emit events
- Always returns a value

## Security Annotations

### `#[no_reentrant]`

Provides reentrancy protection for a method.

```rust
impl MyContract {
    #[method]
    #[no_reentrant]
    pub fn withdraw(&mut self, amount: u64) -> bool {
        // Withdrawal logic with reentrancy protection
        true
    }
}
```

**Protection:**
- Prevents recursive calls to the same method
- Automatically manages reentrancy locks
- Essential for financial operations

### `#[owner_only]`

Restricts method access to the contract owner.

```rust
impl MyContract {
    #[method]
    #[owner_only]
    pub fn admin_function(&mut self, new_value: u64) -> bool {
        // Only callable by contract owner
        self.admin_value.put(new_value);
        true
    }
}
```

### `#[require_witness]`

Requires witness verification for method execution.

```rust
impl MyContract {
    #[method]
    #[require_witness]
    pub fn sensitive_operation(&mut self, data: Vec<u8>) -> bool {
        // Requires caller to provide valid witness
        true
    }
}
```

## Event Annotations

### `#[event]`

Defines a contract event structure.

```rust
#[event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    
    #[index]
    pub to: Option<H160>,
    
    pub amount: u64,
}
```

**Generated:**
- Event emission functions
- Manifest event definitions
- Type-safe event parameters

### `#[index]`

Marks an event field as indexed for efficient filtering.

```rust
#[event]
pub struct TokenMinted {
    #[index]
    pub recipient: H160,  // Indexed - can be filtered
    
    #[index]
    pub token_id: u64,    // Indexed - can be filtered
    
    pub metadata: String, // Not indexed - part of event data
}
```

**Limitations:**
- Maximum 3 indexed fields per event
- Indexed fields consume more gas
- Use for fields that will be frequently filtered

## Advanced Annotations

### `#[init]`

Provides custom initialization logic.

```rust
impl MyContract {
    #[init]
    fn initialize(&mut self) {
        // Custom initialization code
        self.created_at.put(Runtime::time());
    }
}
```

### `#[validate]`

Adds input validation to methods.

```rust
impl MyContract {
    #[method]
    #[validate(amount > 0, "Amount must be positive")]
    #[validate(to != H160::zero(), "Invalid recipient")]
    pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
        // Transfer logic with automatic validation
        true
    }
}
```

### `#[gas_limit]`

Sets a gas limit for method execution.

```rust
impl MyContract {
    #[method]
    #[gas_limit(10000)]
    pub fn expensive_operation(&mut self, data: Vec<u8>) -> bool {
        // Operation with gas limit
        true
    }
}
```

## Annotation Combinations

### Common Patterns

```rust
// Standard token transfer
#[method]
#[no_reentrant]
#[validate(amount > 0)]
pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
    // Transfer implementation
}

// Admin-only configuration
#[method]
#[owner_only]
#[require_witness]
pub fn set_config(&mut self, key: String, value: String) -> bool {
    // Configuration logic
}

// Safe read-only query
#[safe]
pub fn get_balance(&self, account: H160) -> u64 {
    // Balance query
}

// Constructor with validation
#[constructor]
#[validate(initial_supply > 0)]
pub fn new(owner: H160, initial_supply: u64) -> Self {
    // Constructor logic
}
```

## Event Emission

Events defined with `#[event]` can be emitted using generated functions:

```rust
impl MyContract {
    #[method]
    pub fn transfer(&mut self, to: H160, amount: u64) -> bool {
        let from = Runtime::calling_script_hash();
        
        // Transfer logic...
        
        // Emit event
        self.emit_transfer(Some(from), Some(to), amount);
        
        true
    }
}
```

## Manifest Generation

Annotations automatically generate the contract manifest:

```json
{
    "name": "MyContract",
    "abi": {
        "methods": [
            {
                "name": "transfer",
                "parameters": [
                    {"name": "to", "type": "Hash160"},
                    {"name": "amount", "type": "Integer"}
                ],
                "returntype": "Boolean",
                "safe": false
            }
        ],
        "events": [
            {
                "name": "Transfer",
                "parameters": [
                    {"name": "from", "type": "Hash160", "indexed": true},
                    {"name": "to", "type": "Hash160", "indexed": true},
                    {"name": "amount", "type": "Integer", "indexed": false}
                ]
            }
        ]
    }
}
```

## Best Practices

### 1. Annotation Order

```rust
// Recommended order:
#[method]              // Method type first
#[no_reentrant]        // Security annotations
#[owner_only]          // Access control
#[require_witness]     // Additional security
#[validate(...)]       // Input validation
#[gas_limit(...)]      // Resource limits
pub fn my_method(&mut self) -> bool {
    // Implementation
}
```

### 2. Storage Design

```rust
#[contract]
pub struct WellDesignedContract {
    // Simple values
    #[storage]
    owner: StorageItem<H160>,
    
    #[storage]
    paused: StorageItem<bool>,
    
    // Mappings for scalable data
    #[storage]
    balances: StorageMap<H160, u64>,
    
    #[storage]
    allowances: StorageMap<(H160, H160), u64>,
}
```

### 3. Event Design

```rust
// Good: Focused events with appropriate indexing
#[event]
pub struct Transfer {
    #[index] pub from: Option<H160>,
    #[index] pub to: Option<H160>,
    pub amount: u64,
}

// Avoid: Too many indexed fields
#[event]
pub struct OverIndexedEvent {
    #[index] pub field1: H160,
    #[index] pub field2: H160,
    #[index] pub field3: u64,
    #[index] pub field4: String,  // Too many indexed fields!
}
```

## Error Handling

Annotations work with Rust's standard error handling:

```rust
impl MyContract {
    #[method]
    pub fn safe_transfer(&mut self, to: H160, amount: u64) -> Result<bool, String> {
        if amount == 0 {
            return Err("Amount cannot be zero".to_string());
        }
        
        if to == H160::zero() {
            return Err("Invalid recipient".to_string());
        }
        
        // Transfer logic...
        Ok(true)
    }
}
```

## See Also

- [Examples README](../examples/README.md): Practical examples using annotations
- [Macro Architecture](../neo-contract/MACROS.md): Implementation details
- [Contract Organization](../neo-contract/CONTRACTS.md): Contract structure
- [Events Guide](events_guide.md): Detailed event handling
- [Storage Guide](storage_guide.md): Storage patterns and optimization 