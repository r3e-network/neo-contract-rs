# Integrating the Escrow Contract with NEP-17 Tokens

This guide demonstrates how to extend the Escrow Contract to handle NEP-17 token transfers instead of just native GAS tokens. This allows the escrow contract to facilitate secure trade of any standard-compliant fungible tokens on the Neo N3 blockchain.

## Overview

To handle NEP-17 tokens, we need to:

1. Add token contract storage and transfer methods
2. Implement token deposit and withdrawal functions
3. Update the escrow creation process
4. Modify the release and refund processes to handle tokens

## Required Modifications

### 1. Add Token-Related Storage and Events

First, update the `Escrow` struct to include token-related storage and define token-specific events:

```rust
// Token-related events
#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    token_contract: H160,
    amount: u64,
    release_time: u64
)]
struct TokenEscrowCreated { }

#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    token_contract: H160,
    amount: u64,
    release_time: u64
)]
struct TokenEscrowReleased { }

#[neo_contract::event(
    escrow_id: u64,
    sender: H160,
    recipient: H160,
    token_contract: H160,
    amount: u64
)]
struct TokenEscrowRefunded { }

#[neo_contract::contract]
pub struct Escrow {
    // Existing storage items...
    
    // Token-specific storage
    token_deposits: StorageMap<u64, TokenDeposit>,  // escrow_id => token deposit info
}

pub struct TokenDeposit {
    token_contract: H160,
    amount: u64,
}
```

### 2. Add Token Transfer Functions

Add helper functions to handle token transfers:

```rust
#[neo_contract::manifest]
impl Escrow {
    // Existing methods...
    
    fn transfer_token_from_sender(&self, token_contract: &H160, from: &H160, to: &H160, amount: u64) -> bool {
        // Ensure the sender has authorized this contract to transfer tokens
        assert!(Runtime::check_witness(from), "Sender authorization required");
        
        // Call the transfer method on the token contract
        let params = [
            from,              // from_address
            to,                // to_address
            amount,           // amount
            "Escrow deposit"   // optional data
        ];
        
        // Invoke the transfer method on the token contract
        let result = Runtime::call_contract(token_contract, "transfer", params);
        
        // Check if the transfer was successful (should return true)
        if let Some(value) = result {
            return value == true;
        }
        
        false
    }

    fn transfer_token_from_contract(&self, token_contract: &H160, to: &H160, amount: u64) -> bool {
        // Call the transfer method on the token contract
        let params = [
            Runtime::executing_script_hash(),  // from_address (this contract)
            to,                                // to_address
            amount,                           // amount
            "Escrow release"                   // optional data
        ];
        
        // Invoke the transfer method on the token contract
        let result = Runtime::call_contract(token_contract, "transfer", params);
        
        // Check if the transfer was successful (should return true)
        if let Some(value) = result {
            return value == true;
        }
        
        false
    }
}
```

### 3. Update Escrow Creation to Handle Tokens

Modify the `create_escrow` function to handle token deposits:

```rust
#[method]
#[no_reentry]
pub fn create_token_escrow(
    &mut self, 
    sender: H160, 
    recipient: H160, 
    token_contract: H160,
    amount: u64, 
    lock_duration: u64
) -> u64 {
    // Ensure sender is the one calling this method
    assert!(Runtime::check_witness(&sender), "Sender must initiate escrow");
    
    // Check rate limit
    assert!(self.check_rate_limit(&sender, ActionType::CreateEscrow), "Rate limit exceeded");
    
    // Get and increment the escrow ID
    let escrow_id = self.next_escrow_id.get();
    self.next_escrow_id.set(escrow_id + 1);
    
    // Transfer tokens from sender to this contract
    let success = self.transfer_token_from_sender(
        &token_contract,
        &sender,
        &Runtime::executing_script_hash(),
        amount
    );
    assert!(success, "Token transfer failed");
    
    // Store token deposit information
    let token_deposit = TokenDeposit {
        token_contract,
        amount,
    };
    self.token_deposits.insert(&escrow_id, token_deposit);
    
    // Store escrow details
    self.senders.insert(&escrow_id, sender);
    self.recipients.insert(&escrow_id, recipient);
    self.statuses.insert(&escrow_id, EscrowStatus::Active);
    
    // Set release time to current time + lock duration
    let current_time = Ledger::current_timestamp();
    let release_time = current_time + lock_duration;
    self.release_times.insert(&escrow_id, release_time);
    
    // Emit escrow creation event
    TokenEscrowCreated {}.fire(
        &escrow_id,
        &sender,
        &recipient,
        &token_contract,
        &amount,
        &release_time
    );
    
    escrow_id
}
```

### 4. Update Release and Refund Functions

Modify the release and refund functions to handle token transfers:

```rust
#[method]
#[no_reentry]
pub fn release_token_escrow(&mut self, escrow_id: u64) -> bool {
    // Get escrow details
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is still active
        if status != EscrowStatus::Active {
            return false;
        }
        
        // Get token deposit
        let token_deposit = self.token_deposits.get(&escrow_id)
            .expect("Token deposit not found");
        
        // Existing authorization checks from the original release_escrow...
        
        // Transfer tokens to recipient
        let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
        let success = self.transfer_token_from_contract(
            &token_deposit.token_contract,
            &recipient,
            token_deposit.amount
        );
        assert!(success, "Token transfer failed");
        
        // Update status
        self.statuses.insert(&escrow_id, EscrowStatus::Completed);
        
        // Emit event
        let sender = self.senders.get(&escrow_id).expect("Sender not found");
        let current_time = Ledger::current_timestamp();
        
        TokenEscrowReleased {}.fire(
            &escrow_id,
            &sender,
            &recipient,
            &token_deposit.token_contract,
            &token_deposit.amount,
            &current_time
        );
        
        return true;
    }
    
    false
}

#[method]
#[no_reentry]
pub fn refund_token_escrow(&mut self, escrow_id: u64) -> bool {
    // Get escrow details
    if let Some(status) = self.statuses.get(&escrow_id) {
        // Check if escrow is still active
        if status != EscrowStatus::Active {
            return false;
        }
        
        // Get token deposit
        let token_deposit = self.token_deposits.get(&escrow_id)
            .expect("Token deposit not found");
        
        // Existing authorization checks from the original refund_escrow...
        
        // Transfer tokens back to sender
        let sender = self.senders.get(&escrow_id).expect("Sender not found");
        let success = self.transfer_token_from_contract(
            &token_deposit.token_contract,
            &sender,
            token_deposit.amount
        );
        assert!(success, "Token transfer failed");
        
        // Update status
        self.statuses.insert(&escrow_id, EscrowStatus::Refunded);
        
        // Emit event
        let recipient = self.recipients.get(&escrow_id).expect("Recipient not found");
        
        TokenEscrowRefunded {}.fire(
            &escrow_id,
            &sender,
            &recipient,
            &token_deposit.token_contract,
            &token_deposit.amount
        );
        
        return true;
    }
    
    false
}
```

### 5. Add Token Balance Checking Function

Add a function to check token balances:

```rust
#[method]
#[safe]
pub fn check_token_balance(&self, token_contract: H160) -> u64 {
    let params = [
        Runtime::executing_script_hash()  // address to check
    ];
    
    // Call the balanceOf method on the token contract
    let result = Runtime::call_contract(&token_contract, "balanceOf", params);
    
    if let Some(value) = result {
        // Convert the result to a u64 balance
        return value.try_into().unwrap_or(0);
    }
    
    0
}
```

## Implementation Example: Mock NEP-17 Token Contract

For testing, you can create a mock NEP-17 token contract:

```rust
#[neo_contract::event(
    from: H160,
    to: H160,
    amount: u64
)]
struct Transfer { }

#[neo_contract::contract]
pub struct MockToken {
    balances: StorageMap<H160, u64>,
    total_supply: StorageItem<u64>,
}

#[neo_contract::manifest]
impl MockToken {
    #[constructor]
    pub fn new() -> Self {
        Self {
            balances: StorageMap::new(),
            total_supply: StorageItem::new(1_000_000),
        }
    }
    
    #[method]
    #[safe]
    pub fn balanceOf(&self, account: H160) -> u64 {
        self.balances.get(&account).unwrap_or(0)
    }
    
    #[method]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64, _data: String) -> bool {
        // Check if the sender has enough tokens
        let from_balance = self.balanceOf(from);
        if from_balance < amount {
            return false;
        }
        
        // Update balances
        let to_balance = self.balanceOf(to);
        self.balances.insert(&from, from_balance - amount);
        self.balances.insert(&to, to_balance + amount);
        
        // Emit transfer event
        Transfer {}.fire(&from, &to, &amount);
        
        true
    }
    
    #[method]
    #[safe]
    pub fn totalSupply(&self) -> u64 {
        self.total_supply.get()
    }
    
    #[method]
    #[safe]
    pub fn decimals(&self) -> u8 {
        8
    }
    
    #[method]
    #[safe]
    pub fn symbol(&self) -> String {
        "TKN".to_string()
    }
}
```

## Security Considerations

When handling tokens in escrow contracts, consider these additional security measures:

1. **Token Verification**: Verify that the token contract adheres to the NEP-17 standard
2. **Amount Validation**: Validate that amounts are reasonable and not affected by overflow
3. **Re-entrancy Protection**: Use the `#[no_reentry]` attribute to prevent re-entrancy attacks
4. **Failed Transfer Handling**: Account for the possibility of token transfers failing
5. **Safe Methods**: Mark read-only methods with `#[safe]` to improve gas efficiency

## Client Interaction

Interacting with the token escrow from a client application:

```javascript
// JavaScript example
async function createTokenEscrow(
  senderAddress,
  recipientAddress,
  tokenContractHash,
  tokenAmount,
  lockDurationSeconds
) {
  const operation = 'create_token_escrow';
  const params = [
    sc.ContractParam.hash160(senderAddress),
    sc.ContractParam.hash160(recipientAddress),
    sc.ContractParam.hash160(tokenContractHash),
    sc.ContractParam.integer(tokenAmount),
    sc.ContractParam.integer(lockDurationSeconds)
  ];
  
  // Create and sign transaction
  const script = sc.createScript({
    scriptHash: escrowContractHash,
    operation,
    args: params
  });
  
  const result = await rpcClient.invokeScript(script).catch(err => {
    console.error('Transaction failed', err);
    return null;
  });
  
  // Parse results - escrow ID will be returned
  if (result && result.state === 'HALT') {
    const escrowId = parseInt(result.stack[0].value);
    console.log(`Token escrow created with ID: ${escrowId}`);
    return escrowId;
  }
  
  return null;
}
```

## Conclusion

By integrating with NEP-17 tokens and using Neo's annotation syntax, the Escrow Contract becomes significantly more versatile, secure, and easier to integrate with the Neo ecosystem. The annotations provide better error handling, improved security with re-entrancy protection, and clear method visibility that helps clients properly interact with the contract. 