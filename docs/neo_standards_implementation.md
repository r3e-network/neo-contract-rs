# Implementing NEP Standards with Neo Contract Rust Framework

This guide explains how to implement the various Neo Enhancement Proposal (NEP) standards in your smart contracts using the Neo Contract Rust Framework.

## Introduction to NEP Standards

NEP standards define interfaces and behaviors that smart contracts should implement to ensure interoperability within the Neo ecosystem. Similar to Ethereum's ERC standards, NEPs provide a common language for contracts to interact with each other and with dApps.

## NEP-17: Fungible Token Standard

NEP-17 is the standard for fungible tokens on Neo N3, similar to ERC-20 on Ethereum.

### Required Methods

| Method | Description |
|--------|-------------|
| `symbol() -> String` | Returns the token symbol |
| `decimals() -> u8` | Returns the number of decimals |
| `totalSupply() -> u64` | Returns the total token supply |
| `balanceOf(account: Address) -> u64` | Returns the token balance of an account |
| `transfer(from: Address, to: Address, amount: u64, data: Any) -> bool` | Transfers tokens between addresses |

### Required Events

```rust
#[event]
pub struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    amount: u64,
}
```

### Implementation Example

```rust
use neo_contract::prelude::*;

#[event]
pub struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    amount: u64,
}

#[neo_contract::contract]
#[supported_standards("NEP-17")]
pub mod token {
    use super::*;
    
    #[storage]
    pub struct Token {
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        decimals: StorageItem<u8>,
        total_supply: StorageItem<u64>,
        balances: StorageMap<Address, u64>,
    }
    
    impl Token {
        #[constructor]
        pub fn new(name: String, symbol: String, decimals: u8, total_supply: u64, owner: Address) -> Self {
            let mut this = Self {
                name: StorageItem::new(),
                symbol: StorageItem::new(),
                decimals: StorageItem::new(),
                total_supply: StorageItem::new(),
                balances: StorageMap::new(),
            };
            
            this.name.set(name);
            this.symbol.set(symbol);
            this.decimals.set(decimals);
            this.total_supply.set(total_supply);
            this.balances.insert(&owner, total_supply);
            
            emit!(Transfer {
                from: None,
                to: Some(owner),
                amount: total_supply,
            });
            
            this
        }
        
        #[safe]
        pub fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            self.decimals.get().unwrap_or(8)
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        #[method]
        pub fn transfer(&mut self, to: Address, amount: u64, data: Option<ByteArray>) -> bool {
            let from = runtime::get_calling_scripthashs().first().unwrap().clone();
            
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false;
            }
            
            if amount == 0 {
                emit!(Transfer {
                    from: Some(from),
                    to: Some(to),
                    amount: 0,
                });
                return true;
            }
            
            let new_from_balance = from_balance - amount;
            if new_from_balance > 0 {
                self.balances.insert(&from, new_from_balance);
            } else {
                self.balances.remove(&from);
            }
            
            let to_balance = self.balance_of(to);
            let new_to_balance = to_balance + amount;
            self.balances.insert(&to, new_to_balance);
            
            emit!(Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            });
            
            // If the recipient is a contract, call onNEP17Payment
            if runtime::is_contract(to) {
                contract::call_contract(
                    to, 
                    "onNEP17Payment", 
                    CallFlags::ALL, 
                    &[
                        from.into_value(),
                        amount.into_value(),
                        data.unwrap_or_default().into_value()
                    ]
                ).ok();
            }
            
            true
        }
        
        // Optional: Implement onNEP17Payment for receiving tokens
        #[method]
        pub fn onNEP17Payment(&mut self, from: Address, amount: u64, data: ByteArray) {
            // Handle incoming NEP-17 payments
        }
    }
}
```

## NEP-11: Non-Fungible Token Standard

NEP-11 is the standard for non-fungible tokens (NFTs) on Neo N3, similar to ERC-721 on Ethereum. NEP-11 supports both divisible and non-divisible tokens.

### Required Methods for All NEP-11 Tokens

| Method | Description |
|--------|-------------|
| `symbol() -> String` | Returns the token symbol |
| `decimals() -> u8` | Returns the number of decimals (0 for non-divisible tokens) |
| `totalSupply() -> u64` | Returns the total token supply |
| `balanceOf(owner: Address) -> u64` | Returns the number of tokens owned by an address |
| `tokensOf(owner: Address) -> Iterator<ByteArray>` | Returns an iterator of token IDs owned by an address |
| `ownerOf(tokenId: ByteArray) -> Address` | Returns the owner of a specific token |
| `properties(tokenId: ByteArray) -> Map<String, Any>` | Returns the properties of a specific token |
| `transfer(to: Address, tokenId: ByteArray, data: Any) -> bool` | Transfers a token to an address |

### Required Events

```rust
#[event]
pub struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    #[index]
    token_id: ByteArray,
    amount: u64,  // Only for divisible tokens
}
```

### Non-Divisible NEP-11 Implementation

```rust
use neo_contract::prelude::*;

#[event]
pub struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    #[index]
    token_id: ByteArray,
}

#[neo_contract::contract]
#[supported_standards("NEP-11")]
pub mod nft {
    use super::*;
    
    #[storage]
    pub struct NFTContract {
        name: StorageItem<String>,
        symbol: StorageItem<String>,
        total_supply: StorageItem<u64>,
        
        // Token ownership
        tokens: StorageMap<ByteArray, Address>,
        
        // Owner token lists
        owner_tokens: StorageMap<Address, Vec<ByteArray>>,
        
        // Token properties - can store metadata like IPFS hashes
        properties: StorageMap<ByteArray, HashMap<String, ByteArray>>,
    }
    
    impl NFTContract {
        #[constructor]
        pub fn new(name: String, symbol: String) -> Self {
            let mut this = Self {
                name: StorageItem::new(),
                symbol: StorageItem::new(),
                total_supply: StorageItem::new(),
                tokens: StorageMap::new(),
                owner_tokens: StorageMap::new(),
                properties: StorageMap::new(),
            };
            
            this.name.set(name);
            this.symbol.set(symbol);
            this.total_supply.set(0);
            
            this
        }
        
        #[safe]
        pub fn symbol(&self) -> String {
            self.symbol.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            0  // Non-divisible tokens have 0 decimals
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            self.total_supply.get().unwrap_or_default()
        }
        
        #[safe]
        pub fn balance_of(&self, owner: Address) -> u64 {
            let tokens = self.owner_tokens.get(&owner).unwrap_or_default();
            tokens.len() as u64
        }
        
        #[safe]
        pub fn tokens_of(&self, owner: Address) -> Vec<ByteArray> {
            self.owner_tokens.get(&owner).unwrap_or_default()
        }
        
        #[safe]
        pub fn owner_of(&self, token_id: ByteArray) -> Address {
            self.tokens.get(&token_id).unwrap_or_default()
        }
        
        #[safe]
        pub fn properties(&self, token_id: ByteArray) -> HashMap<String, ByteArray> {
            self.properties.get(&token_id).unwrap_or_default()
        }
        
        #[method]
        pub fn mint(&mut self, to: Address, token_id: ByteArray, token_properties: HashMap<String, ByteArray>) -> bool {
            // Check if token already exists
            if self.tokens.contains_key(&token_id) {
                return false;
            }
            
            // Update ownership
            self.tokens.insert(&token_id, to);
            
            // Update owner tokens
            let mut owner_tokens = self.owner_tokens.get(&to).unwrap_or_default();
            owner_tokens.push(token_id.clone());
            self.owner_tokens.insert(&to, owner_tokens);
            
            // Update properties
            self.properties.insert(&token_id, token_properties);
            
            // Update total supply
            let current_supply = self.total_supply.get().unwrap_or(0);
            self.total_supply.set(current_supply + 1);
            
            // Emit event
            emit!(Transfer {
                from: None,
                to: Some(to),
                token_id,
            });
            
            true
        }
        
        #[method]
        pub fn transfer(&mut self, to: Address, token_id: ByteArray, data: Option<ByteArray>) -> bool {
            let from = runtime::get_calling_scripthashs().first().unwrap().clone();
            
            // Check ownership
            let current_owner = self.owner_of(token_id.clone());
            if current_owner != from {
                return false;
            }
            
            // Update ownership
            self.tokens.insert(&token_id, to);
            
            // Update from owner tokens
            let mut from_tokens = self.owner_tokens.get(&from).unwrap_or_default();
            if let Some(pos) = from_tokens.iter().position(|t| t == &token_id) {
                from_tokens.remove(pos);
            }
            if from_tokens.is_empty() {
                self.owner_tokens.remove(&from);
            } else {
                self.owner_tokens.insert(&from, from_tokens);
            }
            
            // Update to owner tokens
            let mut to_tokens = self.owner_tokens.get(&to).unwrap_or_default();
            to_tokens.push(token_id.clone());
            self.owner_tokens.insert(&to, to_tokens);
            
            // Emit event
            emit!(Transfer {
                from: Some(from),
                to: Some(to),
                token_id: token_id.clone(),
            });
            
            // If the recipient is a contract, call onNEP11Payment
            if runtime::is_contract(to) {
                contract::call_contract(
                    to,
                    "onNEP11Payment",
                    CallFlags::ALL,
                    &[
                        from.into_value(),
                        u64::from(1).into_value(),
                        token_id.into_value(),
                        data.unwrap_or_default().into_value()
                    ]
                ).ok();
            }
            
            true
        }
        
        // Optional: Implement onNEP11Payment for receiving NFTs
        #[method]
        pub fn onNEP11Payment(&mut self, from: Address, amount: u64, token_id: ByteArray, data: ByteArray) {
            // Handle incoming NFT
        }
    }
}
```

### Divisible NEP-11 Implementation

For divisible NFTs (like semi-fungible tokens), the implementation is a bit more complex:

```rust
// This is a simplified example for divisible NEP-11 tokens
// Divisible NFTs track both the token ID and the amount of each token

#[event]
pub struct Transfer {
    #[index]
    from: Option<Address>,
    #[index]
    to: Option<Address>,
    #[index]
    token_id: ByteArray,
    amount: u64,
}

// Divisible NFT implementation would include additional storage:
// balances: StorageMap<(ByteArray, Address), u64> // (token_id, owner) -> amount
```

## NEP-5: Contract Invocation Standard

NEP-5 standardizes the behavior for contract calls.

### Implementation

```rust
// This is typically implemented automatically by the framework
// when you use the #[method] attribute, but you can customize it:

#[method]
pub fn verify() -> bool {
    // Custom verification logic
    true
}

#[method]
pub fn is_valid(&self) -> bool {
    // Custom validation logic
    true
}
```

## NEP-14: Contract Permission Descriptor

NEP-14 defines a standard for declaring which contracts and methods your contract is allowed to call.

### Implementation

```rust
#[neo_contract::contract]
#[contract_permission(
    contract = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5", // GAS contract
    methods = ["transfer", "balanceOf"]
)]
#[contract_permission(
    contract = "*", // Allow calls to any contract
    methods = ["symbol", "decimals"]
)]
pub mod my_contract {
    // Contract implementation
}
```

## NEP-15: Contract Trust Descriptor

NEP-15 defines a standard for declaring which contracts are trusted by your contract.

### Implementation

```rust
#[neo_contract::contract]
#[contract_trust("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")]
#[contract_trust("0x70e2301955bf1e74cbb31d18c2f96886bc4e5692")]
pub mod my_contract {
    // Contract implementation
}
```

## Custom Standards

You can also implement your own standards:

```rust
#[neo_contract::contract]
#[supported_standards("NEP-17", "NEP-11", "MyCustomStandard")]
pub mod multi_token {
    // Implementation that supports multiple standards
}
```

## Testing Standard Compliance

Ensure your contract properly implements all required methods and events:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use neo_contract::testing::*;
    
    #[test]
    fn test_nep17_compliance() {
        // Create mock environment
        let mut env = TestEnvironment::new();
        let owner = env.create_account([1u8; 20]);
        
        // Create token contract
        let mut token = Token::new("Test".to_string(), "TST".to_string(), 8, 1000, owner.address());
        
        // Test required methods
        assert_eq!(token.symbol(), "TST");
        assert_eq!(token.decimals(), 8);
        assert_eq!(token.total_supply(), 1000);
        assert_eq!(token.balance_of(owner.address()), 1000);
        
        // Test transfer and event emission
        let recipient = env.create_account([2u8; 20]);
        env.set_caller(owner.address());
        
        env.event_recorder().start();
        let result = token.transfer(recipient.address(), 100, None);
        assert!(result);
        
        // Verify event emission
        let events = env.event_recorder().events();
        assert_eq!(events.len(), 1);
        
        let transfer_event = &events[0];
        assert_eq!(transfer_event.name, "Transfer");
        assert_eq!(transfer_event.fields[0].as_address(), Some(owner.address()));
        assert_eq!(transfer_event.fields[1].as_address(), Some(recipient.address()));
        assert_eq!(transfer_event.fields[2].as_u64(), Some(100));
    }
}
```

## Best Practices

1. **Declare Standards**: Use the `#[supported_standards]` attribute to declare which standards your contract implements.

2. **Follow Naming Conventions**: Use the exact method names specified in the standards (e.g., `balance_of` for NEP-17).

3. **Emit Required Events**: Ensure your contract emits all required events with the correct parameters.

4. **Handle Contract Calls**: Implement methods like `onNEP17Payment` to handle token receipts.

5. **Complete Implementation**: Implement all required methods, even if some have default behaviors.

6. **Test Compliance**: Write tests that verify your contract correctly implements all standard methods and events.

7. **Permissions Management**: Use `#[contract_permission]` to declare which contracts your contract needs to call.

8. **Trust Relationships**: Use `#[contract_trust]` to declare which contracts your contract trusts.

## Conclusion

Implementing NEP standards ensures your smart contracts can interact seamlessly with other contracts and dApps in the Neo ecosystem. By following the guidelines in this document, you can create standard-compliant contracts that leverage the full power of the Neo blockchain.

The Neo Contract Rust Framework provides the tools and abstractions needed to implement these standards with minimal boilerplate, allowing you to focus on your contract's unique business logic while ensuring interoperability.