# Variables and Constants in Neo Contract RS

This document explains how to define and use different types of variables and constants in Neo N3 smart contracts written with Neo Contract RS.

## Variable Types Overview

In Neo Contract RS, there are three main categories of variables:

1. **Storage Variables** - Persist on the blockchain, part of contract state
2. **Non-Storage Variables** - Temporary variables that exist only during execution
3. **Static Constants** - Fixed values defined at compile-time

## Non-Storage Variables

Non-storage variables are regular Rust variables that exist only during the execution of a method. They don't persist on the blockchain and are reset with each contract invocation.

### Defining Non-Storage Variables

Define non-storage variables within your methods like regular Rust variables:

```rust
#[message]
pub fn calculate_reward(&self, stake_amount: Int256) -> Int256 {
    // Non-storage variables
    let rate = Int256::from(5); // 5% rate
    let duration = 365; // days
    let multiplier = Int256::from(duration) / Int256::from(365);
    
    // Calculation using non-storage variables
    stake_amount * rate * multiplier / Int256::from(100)
}
```

These variables:
- Exist only during method execution
- Don't consume blockchain storage
- Are efficient for intermediate calculations
- Cannot be accessed from other method calls

## Static Constants

Static constants are fixed values that don't change throughout the contract's lifetime. They're defined at compile-time and don't consume blockchain storage.

### Regular Rust Constants

For simple constants, use standard Rust `const` definitions:

```rust
#[contract]
mod token_contract {
    use super::*;
    
    // Regular Rust constants
    const TOKEN_NAME: &[u8] = b"Example Token";
    const DECIMALS: u8 = 8;
    const MAX_SUPPLY: u64 = 1_000_000_00000000; // 1M tokens with 8 decimals
    
    #[storage]
    pub struct Token {
        // Storage variables
        supply: Int256,
        balances: Map<H160, Int256>,
    }
    
    impl Token {
        #[message]
        pub fn name(&self) -> ByteString {
            ByteString::from(TOKEN_NAME)
        }
        
        #[message]
        pub fn decimals(&self) -> u8 {
            DECIMALS
        }
    }
}
```

### Special Blockchain Static Fields

Neo Contract RS provides specialized attributes for defining blockchain-specific static fields. These are optimized for Neo VM and handled by the framework's macros.

#### Basic Static Fields

```rust
#[contract]
mod my_contract {
    use super::*;
    
    // String static field
    #[string]
    static TOKEN_NAME: &str = "Example Token";
    
    // Integer static field
    #[integer]
    static MAX_TRANSFER: u64 = 1_000_000;
    
    // ByteArray static field
    #[byte_array]
    static LOGO_BYTES: &[u8] = &[0x01, 0x02, 0x03, 0x04];
    
    #[storage]
    pub struct MyContract {
        // Storage fields
    }
    
    impl MyContract {
        #[message]
        #[safe]
        pub fn get_name(&self) -> ByteString {
            TOKEN_NAME.to_string()
        }
        
        #[message]
        #[safe]
        pub fn get_max_transfer(&self) -> Int256 {
            MAX_TRANSFER.into()
        }
    }
}
```

#### Blockchain Address Static Fields

For blockchain-specific address types:

```rust
#[contract]
mod my_contract {
    use super::*;
    
    // Contract hash static field
    #[contract_hash]
    static TOKEN_CONTRACT: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";
    
    // Hash160 static field (for addresses)
    #[hash160]
    static TREASURY_ACCOUNT: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";
    
    // Public key static field
    #[public_key]
    static VALIDATOR_KEY: &str = "03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c";
    
    #[storage]
    pub struct MyContract {
        // Storage fields
    }
    
    impl MyContract {
        #[message]
        pub fn send_to_treasury(&mut self, amount: Int256) -> bool {
            // Use the static TREASURY_ACCOUNT
            self.transfer_to(TREASURY_ACCOUNT.into(), amount)
        }
        
        #[message]
        pub fn call_token_contract(&self, method: ByteString, args: Array<Any>) -> Any {
            // Use the static TOKEN_CONTRACT
            contract::call(
                TOKEN_CONTRACT.into(),
                method,
                CallFlags::All,
                args
            )
        }
    }
}
```

### Fixed vs. Dynamic Static Fields

Neo Contract RS provides two variants for static fields:

1. **Dynamic** (e.g., `#[hash160]`) - The value is computed at runtime using the provided string
2. **Fixed** (e.g., `#[hash160_fixed]`) - The value is pre-computed at compile time

Fixed variants have slightly better performance but can't be easily verified by examining the contract code.

```rust
// Dynamic variant - computed at runtime
#[hash160]
static OWNER: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";

// Fixed variant - pre-computed at compile time
#[hash160_fixed]
static OWNER: [u8; 20] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
    0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44
];
```

## Combining Variable Types

A complete example using all variable types:

```rust
#[contract]
mod token_contract {
    use super::*;
    
    // Static constants
    const SECONDS_PER_DAY: u32 = 86400;
    
    // Static fields with specialized attributes
    #[string]
    static TOKEN_NAME: &str = "Example Token";
    
    #[string]
    static TOKEN_SYMBOL: &str = "EXMP";
    
    #[integer]
    static DECIMALS: u8 = 8;
    
    #[hash160]
    static OWNER: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";
    
    // Contract storage
    #[storage]
    pub struct Token {
        total_supply: Int256,
        balances: Map<H160, Int256>,
        is_paused: bool,
    }
    
    impl Token {
        #[constructor]
        pub fn new(initial_supply: Int256) -> Self {
            // Non-storage variable in constructor
            let owner = OWNER.into();
            
            let mut balances = Map::new();
            balances.insert(owner, initial_supply.clone());
            
            Self {
                total_supply: initial_supply,
                balances,
                is_paused: false,
            }
        }
        
        #[message]
        #[safe]
        pub fn name(&self) -> ByteString {
            TOKEN_NAME.to_string()
        }
        
        #[message]
        #[safe]
        pub fn symbol(&self) -> ByteString {
            TOKEN_SYMBOL.to_string()
        }
        
        #[message]
        #[safe]
        pub fn decimals(&self) -> u8 {
            DECIMALS.into()
        }
        
        #[message]
        pub fn transfer(&mut self, to: H160, amount: Int256) -> bool {
            // Check if contract is paused (storage variable)
            if self.is_paused {
                return false;
            }
            
            // Get sender (non-storage variable)
            let from = runtime::calling_script_hash();
            
            // Check authorization
            assert!(runtime::check_witness(from), "Unauthorized");
            
            // Get balance (non-storage variable from storage)
            let from_balance = self.balances.get(&from).unwrap_or_default();
            
            // Check if enough balance (using non-storage variable)
            if from_balance < amount {
                return false;
            }
            
            // Calculate new balances (non-storage variables)
            let new_from_balance = from_balance - amount.clone();
            let to_balance = self.balances.get(&to).unwrap_or_default();
            let new_to_balance = to_balance + amount.clone();
            
            // Update storage
            if new_from_balance.is_zero() {
                self.balances.remove(&from);
            } else {
                self.balances.insert(from, new_from_balance);
            }
            
            self.balances.insert(to, new_to_balance);
            
            true
        }
        
        #[message]
        pub fn pause(&mut self) -> bool {
            // Use static OWNER for authorization
            let caller = runtime::calling_script_hash();
            assert!(caller == OWNER.into(), "Only owner can pause");
            
            // Update storage
            self.is_paused = true;
            
            true
        }
    }
}
```

## Best Practices

### When to Use Each Type

- **Storage Variables**
  - Use for data that must persist between contract calls
  - Use for contract state that can be modified
  - Examples: balances, allowances, ownership, contract settings

- **Non-Storage Variables**
  - Use for intermediate calculations
  - Use for temporary values during method execution
  - Examples: method arguments, calculated values, converted types

- **Static Constants**
  - Use for values that never change
  - Use for contract metadata, addresses, configuration
  - Examples: token name, decimals, predefined addresses, protocol constants

### Optimizing Gas Costs

- Prefer static constants over storage variables when values don't change
- Use fixed variants of static fields for slightly better performance
- Keep non-storage variables in local scope to minimize stack usage

## Advanced Static Field Usage

### Conditional Compilation

Static fields can be combined with Rust's conditional compilation:

```rust
#[contract]
mod contract {
    use super::*;
    
    // Different owner address based on compilation target
    #[cfg(feature = "testnet")]
    #[hash160]
    static OWNER: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj"; // TestNet address
    
    #[cfg(feature = "mainnet")]
    #[hash160]
    static OWNER: &str = "NYtmqHmDPWeuN2Pucb6CE2wcUjHYGgKD3W"; // MainNet address
}
```

### Complex Static Structures

For more complex static data, combine regular Rust data structures with static fields:

```rust
#[contract]
mod whitelist_contract {
    use super::*;
    
    // Static array of whitelisted addresses
    #[hash160]
    static ADMIN_1: &str = "NZNos2WqTbu5oCgyfss9kUJgBXJqhuYAaj";
    
    #[hash160]
    static ADMIN_2: &str = "NYtmqHmDPWeuN2Pucb6CE2wcUjHYGgKD3W";
    
    #[hash160]
    static ADMIN_3: &str = "NLnyLtep7jwyq1qhNPkwXbJpuLcB1dkfpX";
    
    // Regular Rust constant array using static fields
    static ADMINS: [H160; 3] = [
        ADMIN_1.into(),
        ADMIN_2.into(),
        ADMIN_3.into()
    ];
    
    impl WhitelistContract {
        #[message]
        #[safe]
        pub fn is_admin(&self, address: H160) -> bool {
            // Check if address is in the static ADMINS array
            ADMINS.contains(&address)
        }
    }
}
```

## Conclusion

Neo Contract RS provides a flexible system for working with different types of variables and constants in your smart contracts. By choosing the appropriate variable type based on your needs, you can create efficient, gas-optimized contracts with clear, maintainable code.

- **Storage Variables** provide persistent contract state
- **Non-Storage Variables** handle temporary calculations
- **Static Constants** define fixed values efficiently

Using a combination of these types allows for well-structured contracts that follow best practices for Neo N3 development.