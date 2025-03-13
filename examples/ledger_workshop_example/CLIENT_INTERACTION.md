# Client Interaction Guide for Escrow Contract

This guide explains how to interact with the Escrow Contract from client applications. It provides examples using the Neo SDK for various programming languages, taking advantage of the Neo Contract annotation system.

## Contract Interaction Overview

The escrow contract provides several methods for interaction, all properly annotated with method and safety attributes:

1. **Creating an Escrow**: Setup an escrow between two parties with time lock (modifies state)
2. **Releasing Funds**: Release funds to the recipient (modifies state)
3. **Refunding**: Return funds to the sender (modifies state)
4. **Dispute Management**: Open and resolve disputes (modifies state)
5. **Transaction Validation**: Process payments based on blockchain transactions (modifies state)
6. **Query Functions**: Get escrow details and status information (read-only, marked with `#[safe]`)

## Advantages of Annotation-Based Contracts

The escrow contract uses Neo's modern contract annotation system, providing several advantages:

1. **Clear API Definition**: Methods are clearly marked with `#[method]` annotations
2. **Security Enhancements**: State-modifying methods are protected with `#[no_reentry]` to prevent re-entrancy attacks
3. **Gas Optimization**: Read-only methods are marked with `#[safe]` for reduced gas costs
4. **Structured Events**: Events are formally defined with fields for better client parsing
5. **Formal Constructor**: The contract initialization is handled by a proper `#[constructor]` method

## Prerequisites

- A Neo N3 node (MainNet, TestNet, or private network)
- The escrow contract deployed and its script hash
- A wallet with GAS for transaction fees

## Using the Neo SDK (JavaScript/TypeScript)

### Setup

```javascript
import { rpc, sc, wallet } from '@cityofzion/neon-js';

// Configure connection to Neo network
const rpcClient = new rpc.RPCClient('https://n3seed1.ngd.network:10332');

// Load wallet from WIF or other methods
const account = new wallet.Account('your-private-key-or-WIF');

// Contract hash (update with your deployed contract hash)
const contractHash = '0x1234567890abcdef1234567890abcdef12345678';
```

### Creating an Escrow

```javascript
async function createEscrow(
  senderAddress, 
  recipientAddress, 
  amount, 
  lockDurationSeconds
) {
  const operation = 'create_escrow';  // This matches the method name in the contract
  const params = [
    sc.ContractParam.hash160(senderAddress),
    sc.ContractParam.hash160(recipientAddress),
    sc.ContractParam.integer(amount),
    sc.ContractParam.integer(lockDurationSeconds)
  ];
  
  // Create and sign transaction
  const script = sc.createScript({
    scriptHash: contractHash,
    operation,
    args: params
  });
  
  // Since this modifies contract state, we need a real transaction
  const transaction = new sc.ScriptBuilder()
    .emitAppCall(contractHash, operation, params)
    .build();
  
  const signedTx = await account.signTransaction(transaction);
  const result = await rpcClient.sendRawTransaction(signedTx).catch(err => {
    console.error('Transaction failed', err);
    return null;
  });
  
  if (result) {
    console.log(`Escrow creation transaction sent: ${result}`);
    
    // Wait for transaction to be confirmed
    const txStatus = await waitForTransaction(result);
    
    if (txStatus && txStatus.executions && txStatus.executions[0].vmstate === 'HALT') {
      const escrowId = parseInt(txStatus.executions[0].stack[0].value);
      console.log(`Escrow created with ID: ${escrowId}`);
      return escrowId;
    }
  }
  
  return null;
}

// Helper function to wait for transaction confirmation
async function waitForTransaction(txid, maxAttempts = 10) {
  for (let i = 0; i < maxAttempts; i++) {
    const res = await rpcClient.getApplicationLog(txid).catch(() => null);
    if (res) return res;
    await new Promise(resolve => setTimeout(resolve, 3000)); // Wait 3 seconds
  }
  return null;
}
```

### Querying Escrow Details (Read-Only Safe Method)

```javascript
async function getEscrowDetails(escrowId) {
  // This is a safe (read-only) method, so we can use invokeFunction
  const operation = 'get_escrow_details';
  const params = [sc.ContractParam.integer(escrowId)];
  
  const result = await rpcClient.invokeFunction(
    contractHash,
    operation,
    params
  ).catch(err => {
    console.error('Query failed', err);
    return null;
  });
  
  if (result && result.state === 'HALT' && result.stack[0].type !== 'Null') {
    // Parse the returned array
    const details = result.stack[0].value;
    
    // The escrow details are returned as a tuple
    return {
      sender: details[0].value,
      recipient: details[1].value,
      amount: parseInt(details[2].value),
      status: parseInt(details[3].value), // Map to EscrowStatus enum
      releaseTime: parseInt(details[4].value)
    };
  }
  
  return null;
}
```

### Releasing an Escrow (State-Modifying Method)

```javascript
async function releaseEscrow(escrowId) {
  const operation = 'release_escrow';
  const params = [sc.ContractParam.integer(escrowId)];
  
  // This is a state-modifying method, so it needs a full transaction
  const transaction = new sc.ScriptBuilder()
    .emitAppCall(contractHash, operation, params)
    .build();
  
  const signedTx = await account.signTransaction(transaction);
  const result = await rpcClient.sendRawTransaction(signedTx).catch(err => {
    console.error('Release failed', err);
    return null;
  });
  
  if (result) {
    console.log(`Release transaction sent: ${result}`);
    
    // Wait for transaction to be confirmed
    const txStatus = await waitForTransaction(result);
    
    if (txStatus && txStatus.executions && txStatus.executions[0].vmstate === 'HALT') {
      const success = txStatus.executions[0].stack[0].value === true;
      console.log(`Escrow ${escrowId} release ${success ? 'succeeded' : 'failed'}`);
      return success;
    }
  }
  
  return false;
}
```

## Using the Neo SDK (Python)

### Setup

```python
from neo3.api.wrappers import GenericRPC
from neo3.wallet import Account
from neo3.contracts import SmartContract
from neo3.core import types
from neo3.core.types import UInt160, UInt256
from neo3.network.payloads import Transaction
from neo3.contracts import ScriptBuilder, InvocationTransaction

# Setup connection to Neo network
rpc_client = GenericRPC("https://n3seed1.ngd.network:10332")

# Load account
account = Account.from_wif("your-private-key-or-WIF")

# Contract hash
contract_hash = UInt160.from_string("0x1234567890abcdef1234567890abcdef12345678")
```

### Creating an Escrow

```python
def create_escrow(sender_address, recipient_address, amount, lock_duration_seconds):
    # Convert addresses to UInt160
    sender = UInt160.from_string(sender_address)
    recipient = UInt160.from_string(recipient_address)
    
    # Build the script to invoke the contract
    sb = ScriptBuilder()
    sb.emit_contract_call(
        contract_hash,
        "create_escrow",
        [
            sender,
            recipient,
            types.Integer(amount),
            types.Integer(lock_duration_seconds)
        ]
    )
    
    # Create a transaction
    tx = InvocationTransaction(
        script=sb.to_array(),
        signers=[account]
    )
    
    # Sign and send transaction
    tx.sign(account)
    tx_hash = rpc_client.send_raw_transaction(tx.serialize())
    
    if tx_hash:
        print(f"Transaction sent: {tx_hash}")
        
        # Wait for transaction to be confirmed
        tx_result = wait_for_transaction(tx_hash)
        
        if tx_result and tx_result.vmstate == "HALT":
            escrow_id = int(tx_result.stack[0].value)
            print(f"Escrow created with ID: {escrow_id}")
            return escrow_id
    
    print("Failed to create escrow")
    return None
    
def wait_for_transaction(tx_hash, max_attempts=10):
    # Helper function to wait for transaction confirmation
    import time
    
    for i in range(max_attempts):
        try:
            tx_result = rpc_client.get_application_log(tx_hash)
            if tx_result:
                return tx_result.executions[0]  # Return the first execution result
        except Exception:
            pass
        
        time.sleep(3)  # Wait 3 seconds
    
    return None
```

## Listening for Structured Contract Events

The contract now emits structured events that are easier to parse:

```javascript
// JavaScript example for event subscription
async function subscribeToEscrowEvents() {
  const ws = new rpc.WsRpcClient('wss://n3seed1.ngd.network:10332');
  
  // Subscribe to all contract notifications
  await ws.subscribe('notification_from_contract', (notification) => {
    if (notification.contract === contractHash) {
      const event = notification.event_name;
      const params = notification.state.value;
      
      switch (event) {
        case 'EscrowCreated':
          // The event structure matches our contract event definition
          const escrowId = parseInt(params[0].value);
          const sender = params[1].value;
          const recipient = params[2].value;
          const amount = parseInt(params[3].value);
          const releaseTime = parseInt(params[4].value);
          
          console.log(`New escrow #${escrowId} created: ${amount} from ${sender} to ${recipient}, releasing at ${new Date(releaseTime * 1000).toLocaleString()}`);
          break;
          
        case 'EscrowReleased':
          const releasedId = parseInt(params[0].value);
          console.log(`Escrow #${releasedId} has been released`);
          break;
          
        // Handle other events (DisputeOpened, DisputeResolved, etc.)
      }
    }
  });
}
```

## Working with Safe Methods for Better Performance

Safe methods (read-only) are marked with `#[safe]` in the contract and can be called with less gas:

```javascript
// Get escrow release estimate (safe method)
async function estimateReleaseBlock(escrowId) {
  // Invoke the safe method directly
  const result = await rpcClient.invokeFunction(
    contractHash,
    'estimate_release_block',
    [sc.ContractParam.integer(escrowId)]
  );
  
  if (result && result.state === 'HALT' && result.stack[0].type !== 'Null') {
    const estimatedBlock = parseInt(result.stack[0].value);
    return estimatedBlock;
  }
  
  return null;
}

// Check transaction confirmation status (safe method)
async function checkConfirmationStatus(txHash, requiredConfirmations) {
  const result = await rpcClient.invokeFunction(
    contractHash,
    'check_confirmation_status',
    [
      sc.ContractParam.hash256(txHash),
      sc.ContractParam.integer(requiredConfirmations)
    ]
  );
  
  if (result && result.state === 'HALT') {
    const tuple = result.stack[0].value;
    return {
      confirmed: tuple[0].value === true,
      confirmations: parseInt(tuple[1].value)
    };
  }
  
  return { confirmed: false, confirmations: 0 };
}
```

## Error Handling with Annotations

The contract's annotations provide improved error handling:

1. **Re-entrancy Protection**: Methods with `#[no_reentry]` will fail if called recursively
2. **Method Visibility**: Methods marked as `#[safe]` are clearly intended for read-only access
3. **Constructor Protection**: The `#[constructor]` annotation ensures initialization happens only once

```javascript
// Example of handling re-entrancy protection errors
async function handleEscrowTransactionError(error) {
  if (error.message.includes('reentry')) {
    console.error('Re-entrancy protection triggered: operation already in progress');
    // Wait and retry later
    return 'RETRY_LATER';
  } else if (error.message.includes('witness')) {
    console.error('Authentication error: missing proper signature');
    return 'AUTHENTICATION_FAILED';
  } else if (error.message.includes('Rate limit')) {
    console.error('Rate limiting active: operation blocked temporarily');
    return 'RATE_LIMITED';
  }
  
  console.error('Unknown error:', error);
  return 'UNKNOWN_ERROR';
}
```

## Testing Client Integration

Before deploying to production, test your client integration thoroughly:

1. Create test escrows with small amounts
2. Verify escrow details match expected values using safe methods
3. Test time-locked releases by creating escrows with short lock durations
4. Test dispute resolution process
5. Verify that transaction validation works correctly

## Full Example Application

For a complete example of a client application integrated with the escrow contract, see the [Escrow dApp Example](https://github.com/neo-project/neo-contract-examples/tree/main/js/escrow-dapp) which includes a web interface for interacting with the contract.

## Next Steps

- Extend this contract to handle NEP-17 token transfers using the token integration guide
- Implement a multi-token escrow service
- Create a more complex dispute resolution system with voting
- Add installment-based escrow agreements 