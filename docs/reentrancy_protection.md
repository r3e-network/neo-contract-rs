# Reentrancy Protection in Neo N3 Contracts

This document explains reentrancy vulnerabilities and how to implement proper protection mechanisms in Neo N3 smart contracts using the Rust framework.

## Understanding Reentrancy

Reentrancy is a vulnerability that occurs when an external contract call is made before state changes are finalized, allowing the called contract to recursively call back into the original contract and manipulate its state in unexpected ways.

### The Vulnerability Pattern

A typical reentrancy attack follows this pattern:

1. Contract A calls Contract B
2. Before Contract A updates its state, Contract B calls back into Contract A
3. Contract A re-executes the function, operating on stale state
4. This process may repeat multiple times, allowing funds or assets to be extracted

## Reentrancy in Neo N3

Neo N3 smart contracts are vulnerable to reentrancy when they call external contracts. Although Neo N3 provides mechanisms like call flags to help manage contract interactions, developers must still implement proper reentrancy protection in their contracts.

## Protection Strategies

### 1. Mutex Pattern

The mutex pattern implements a lock that prevents functions from being re-entered while they're executing:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod secure_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct ContractStorage {
        locked: Item<bool>,
        balances: Map<H160, u64>,
        total_supply: Item<u64>,
    }
    
    #[method]
    fn withdraw(&mut self, amount: u64) -> bool {
        // Check if contract is locked
        if self.locked.get() {
            return false; // Reentry attempt detected
        }
        
        // Set lock
        self.locked.put(true);
        
        let caller = Runtime::calling_script_hash();
        let balance = self.balances.get(&caller);
        
        // Ensure user has enough balance
        if balance < amount {
            self.locked.put(false); // Release lock
            return false;
        }
        
        // Update state
        self.balances.put(&caller, balance - amount);
        
        // External call (potential reentrancy point)
        let gas_contract = gas_contract_hash();
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(self_address()));
        args.push(Any::from(caller));
        args.push(Any::from(amount));
        
        let call_flags = CallFlags::ALL as u32;
        let result = contract_call(&gas_contract, &method, &args, call_flags);
        
        // Release lock
        self.locked.put(false);
        
        // Return result
        !result.is_empty()
    }
}
```

### 2. Checks-Effects-Interactions Pattern

This pattern structures code to complete all state changes before making external calls:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod secure_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct ContractStorage {
        balances: Map<H160, u64>,
        total_supply: Item<u64>,
    }
    
    #[method]
    fn withdraw(&mut self, amount: u64) -> bool {
        // 1. CHECKS
        let caller = Runtime::calling_script_hash();
        let balance = self.balances.get(&caller);
        
        // Ensure user has enough balance
        if balance < amount {
            return false;
        }
        
        // 2. EFFECTS - Update state before external calls
        self.balances.put(&caller, balance - amount);
        
        // 3. INTERACTIONS - External call happens after state is updated
        let gas_contract = gas_contract_hash();
        let method = ByteString::from("transfer");
        let mut args = Array::<Any>::new();
        args.push(Any::from(self_address()));
        args.push(Any::from(caller));
        args.push(Any::from(amount));
        
        let call_flags = CallFlags::ALL as u32;
        let result = contract_call(&gas_contract, &method, &args, call_flags);
        
        // Return result
        !result.is_empty()
    }
}
```

### 3. Using Safe Methods

Marking methods as read-only with the `#[safe]` attribute can prevent state modifications:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod secure_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct ContractStorage {
        balances: Map<H160, u64>,
    }
    
    // Safe methods cannot modify state and are more resilient to reentrancy
    #[safe]
    fn get_balance(&self, account: H160) -> u64 {
        self.balances.get(&account)
    }
    
    // State-modifying methods should implement reentrancy protection
    #[method]
    fn transfer(&mut self, to: H160, amount: u64) -> bool {
        // Implementation with reentrancy protection
        // ...
    }
}
```

### 4. Call Flag Restrictions

Using appropriate call flags to restrict external contract capabilities:

```rust
// Read-only call that cannot modify state
let read_only_flags = CallFlags::READ_ONLY as u32;
let result = contract_call(&target_contract, &method, &args, read_only_flags);

// Call that can modify state but not make additional calls
let restricted_flags = (CallFlags::ALLOW_STATES | CallFlags::ALLOW_MODIFY_STATES) as u32;
let result = contract_call(&target_contract, &method, &args, restricted_flags);
```

## Advanced Reentrancy Protection

### State Validations

Implement additional state validations after external calls:

```rust
#[method]
fn complex_operation(&mut self) -> bool {
    // Save initial state for validation
    let initial_total = self.total_supply.get();
    
    // Perform operations and external calls
    // ...
    
    // Validate final state
    let final_total = self.total_supply.get();
    assert!(initial_total >= final_total, "State invariant violated");
    
    true
}
```

### Transaction Batching

Batch related operations to minimize external call risks:

```rust
#[method]
fn batch_transfers(&mut self, receivers: Array<H160>, amounts: Array<u64>) -> bool {
    // Validate input
    if receivers.len() != amounts.len() || receivers.len() == 0 {
        return false;
    }
    
    // Apply all state changes first
    let sender = Runtime::calling_script_hash();
    let mut total_amount = 0u64;
    
    for i in 0..receivers.len() {
        total_amount += amounts.get(i);
    }
    
    // Check balance once for the batch
    let sender_balance = self.balances.get(&sender);
    if sender_balance < total_amount {
        return false;
    }
    
    // Update sender balance
    self.balances.put(&sender, sender_balance - total_amount);
    
    // Update receiver balances
    for i in 0..receivers.len() {
        let receiver = receivers.get(i);
        let amount = amounts.get(i);
        let receiver_balance = self.balances.get(&receiver);
        self.balances.put(&receiver, receiver_balance + amount);
    }
    
    // External interactions can come after all state changes
    // ...
    
    true
}
```

## Testing for Reentrancy Vulnerabilities

### Mock Reentrancy Attack

Create test contracts that attempt reentrancy attacks:

```rust
// Test contract that attempts to re-enter the target contract
#[neo_contract::contract]
mod attack_contract {
    use neo_contract::prelude::*;
    
    #[storage]
    struct AttackStorage {
        target: Item<H160>,
    }
    
    #[method]
    fn set_target(&mut self, target_contract: H160) {
        self.target.put(target_contract);
    }
    
    #[method]
    fn attack(&mut self) -> bool {
        // Call the vulnerable function
        let target = self.target.get();
        let method = ByteString::from("withdraw");
        let amount = 100u64;
        
        let mut args = Array::<Any>::new();
        args.push(Any::from(amount));
        
        let call_flags = CallFlags::ALL as u32;
        let result = contract_call(&target, &method, &args, call_flags);
        
        !result.is_empty()
    }
    
    // Callback function that will be triggered during the external call
    #[method]
    pub fn on_payment(&mut self) {
        // Attempt to re-enter the target contract
        let target = self.target.get();
        let method = ByteString::from("withdraw");
        let amount = 100u64;
        
        let mut args = Array::<Any>::new();
        args.push(Any::from(amount));
        
        let call_flags = CallFlags::ALL as u32;
        contract_call(&target, &method, &args, call_flags);
    }
}
```

### Scenario Testing

Test your contracts with various scenarios:

1. Multiple nested contract calls
2. Concurrent calls from different addresses
3. Edge cases with state modifications

## Real-World Example: NEP-17 Token with Reentrancy Protection

A complete NEP-17 token implementation with reentrancy protection:

```rust
use neo_contract::prelude::*;

#[neo_contract::contract]
mod secure_token {
    use neo_contract::prelude::*;
    
    #[storage]
    struct TokenStorage {
        balances: Map<H160, u64>,
        total_supply: Item<u64>,
        locked: Item<bool>,
    }
    
    // Safe (read-only) methods
    #[safe]
    fn symbol(&self) -> ByteString {
        ByteString::from("TKN")
    }
    
    #[safe]
    fn decimals(&self) -> u8 {
        8
    }
    
    #[safe]
    fn total_supply(&self) -> u64 {
        self.total_supply.get()
    }
    
    #[safe]
    fn balance_of(&self, account: H160) -> u64 {
        self.balances.get(&account)
    }
    
    // State-modifying methods with reentrancy protection
    #[method]
    fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Check for reentrancy
        if self.locked.get() {
            return false;
        }
        
        // Set lock
        self.locked.put(true);
        
        // Verify sender
        if !Runtime::check_witness(&from) {
            self.locked.put(false);
            return false;
        }
        
        // Check if addresses are valid
        if from.is_zero() || to.is_zero() {
            self.locked.put(false);
            return false;
        }
        
        // Check amount
        if amount == 0 {
            self.locked.put(false);
            return false;
        }
        
        // Check balance
        let from_balance = self.balances.get(&from);
        if from_balance < amount {
            self.locked.put(false);
            return false;
        }
        
        // Update balances
        if from == to {
            // No need to update balances if sending to self
            self.locked.put(false);
            return true;
        }
        
        // Update sender balance
        if from_balance == amount {
            self.balances.delete(&from);
        } else {
            self.balances.put(&from, from_balance - amount);
        }
        
        // Update receiver balance
        let to_balance = self.balances.get(&to);
        self.balances.put(&to, to_balance + amount);
        
        // Emit transfer event
        self.emit_transfer(Some(from), Some(to), amount);
        
        // If receiver is a contract, notify it about the transfer
        if self.is_contract(to) {
            // We're making an external call, but our state is already updated
            self.call_on_payment(to);
        }
        
        // Release lock
        self.locked.put(false);
        
        true
    }
    
    // Helper methods
    fn emit_transfer(&self, from: Option<H160>, to: Option<H160>, amount: u64) {
        let event_name = ByteString::from("Transfer");
        let mut event_data = Array::<Any>::new();
        
        match from {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        match to {
            Some(addr) => event_data.push(Any::from(addr)),
            None => event_data.push(Any::new()),
        }
        
        event_data.push(Any::from(amount));
        
        Runtime::notify(&event_name, &event_data);
    }
    
    fn is_contract(&self, address: H160) -> bool {
        // Neo N3 provides a way to check if an address is a contract
        // For simplicity, we're using a mock implementation here
        false
    }
    
    fn call_on_payment(&self, receiver: H160) {
        let method = ByteString::from("onPayment");
        let args = Array::<Any>::new();
        
        // Use restricted call flags - don't allow further calls
        let flags = (CallFlags::ALLOW_NOTIFY | CallFlags::ALLOW_STATES) as u32;
        
        contract_call(&receiver, &method, &args, flags);
    }
}
```

## Conclusion

Reentrancy vulnerabilities can have severe consequences for Neo N3 smart contracts. By implementing proper protection mechanisms like the mutex pattern and checks-effects-interactions pattern, and leveraging Neo N3 features like call flags and safe methods, developers can create secure and resilient contracts.

Remember that security is an evolving field, and it's important to stay updated with the latest security best practices in the Neo N3 ecosystem.
