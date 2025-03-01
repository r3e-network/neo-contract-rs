# neo-contract-rs
Writing Neo-Smart-Contract with Rust

## Attribute Macros

Neo Contract RS now supports ink!-style attribute macros for defining smart contracts.
This provides a more unified approach to contract development.

### Example

```rust
#[neo_contract::contract]
#[neo_contract::contract_author("R3E Network")]
#[neo_contract::contract_email("dev@r3e.network")]
#[neo_contract::contract_description("An example token contract")]
#[neo_contract::contract_version("0.1.0")]
#[neo_contract::supported_standards("NEP-17")]
mod my_contract {
    #[neo(storage)]
    pub struct MyContract {
        value: bool,
    }
    
    impl MyContract {
        #[neo(constructor)]
        pub fn new(initial_value: bool) -> Self {
            Self { value: initial_value }
        }
        
        #[neo(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        
        #[neo(event)]
        pub fn value_changed(old_value: bool, new_value: bool) {}
    }
}
```

### Contract Structure Attributes

- `#[neo_contract::contract]` - Defines a Neo N3 smart contract module
- `#[neo(storage)]` - Marks a struct as the contract's storage
- `#[neo(constructor)]` - Marks a method as a contract constructor
- `#[neo(message)]` - Marks a method as a contract message (callable from outside)
- `#[neo(event)]` - Marks a method as a contract event

### Contract Metadata Attributes

- `#[neo_contract::manifest_extra("key", "value")]` - Adds custom metadata to the contract manifest
- `#[neo_contract::contract_author("Author Name")]` - Specifies the contract author
- `#[neo_contract::contract_email("email@example.com")]` - Specifies the author's email
- `#[neo_contract::contract_description("Description")]` - Provides a contract description
- `#[neo_contract::contract_version("1.0.0")]` - Specifies the contract version
- `#[neo_contract::contract_source_code("https://github.com/...")]` - Links to the source code

### Contract Permission and Standards Attributes

- `#[neo_contract::contract_permission("contract_hash", "method1", "method2")]` - Specifies which contracts and methods can be called
- `#[neo_contract::contract_trust("contract_hash")]` - Specifies which contracts are trusted
- `#[neo_contract::supported_standards("NEP-17", "NEP-11")]` - Declares supported standards

The traditional macro approach is still supported for backward compatibility.

## C# Framework Features

Neo Contract RS now supports additional features from the C# Neo framework:

### Static Field Initialization Attributes

```rust
// Initialize a byte array with a hex string
#[neo::byte_array("0123456789ABCDEF")]
static BYTE_ARRAY: [u8; 8] = [0; 8];

// Initialize a Hash160 with a hex string
#[neo::hash160("0x0123456789abcdef0123456789abcdef01234567")]
static HASH160: H160 = H160::zero();

// Initialize an integer with a value
#[neo::integer("1000000")]
static AMOUNT: Int256 = Int256::zero();

// Initialize a public key with a hex string
#[neo::public_key("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c")]
static PUBLIC_KEY: [u8; 33] = [0; 33];

// Initialize a string with a value
#[neo::string("Hello, NEO!")]
static GREETING: &str = "";

// Initialize a contract hash with a hex string
#[neo::contract_hash("0x0123456789abcdef0123456789abcdef01234567")]
static CONTRACT_HASH: H160 = H160::zero();
```

### Contract Safety and Security Attributes

```rust
// Safe method that doesn't modify state
#[neo::safe]
#[neo(message)]
pub fn total_supply(&self) -> Int256 {
    self.total_supply
}

// Method with reentrancy protection
#[neo::no_reentrant]
#[neo(message)]
pub fn transfer(&mut self, from: H160, to: H160, amount: Int256) -> bool {
    // Implementation
}

// Method with specific reentrancy protection
#[neo::no_reentrant_method]
#[neo(message)]
pub fn withdraw(&mut self, account: H160, amount: Int256) -> bool {
    // Implementation
}
```

### Call Flags

```rust
use neo::call_flags::CallFlags;

// Method with call flags
#[neo(message)]
pub fn call_other_contract(&self, contract_hash: H160, method: &str, args: &[Any]) -> Any {
    // Use call flags
    let flags = CallFlags::ReadStates.add(CallFlags::AllowCall);
    runtime::call_contract(contract_hash, method, args, flags)
}
```

### Contract Structure Attributes

```rust
// Mark a struct as stored in the contract storage
#[neo::stored]
pub struct StoredData {
    // Fields
}

// Define a modifier method
#[neo::modifier]
pub fn only_owner() {
    // Implementation
}

// Specify the calling convention of a method
#[neo::calling_convention(Cdecl)]
pub fn external_function() {
    // Implementation
}

// Specify the opcode of a method
#[neo::op_code(SYSCALL, "System.Runtime.GetTime")]
pub fn get_time() -> u64 {
    // Implementation
}

// Specify the syscall of a method
#[neo::syscall("System.Runtime.GetTime")]
pub fn get_time_syscall() -> u64 {
    // Implementation
}
```

## Examples

Check the `examples` directory for complete contract examples:

- `ink_style_token_with_attributes` - A token contract using the ink!-style attribute macros
- `nep17_token` - A NEP-17 token implementation
- `contract_call` - Example of contract-to-contract calls
- `csharp_features` - Example showcasing C# framework features

## NeoBurger Example

The NeoBurger example demonstrates a complex contract system for Neo N3 governance. It consists of three contracts:

1. **BurgerNEO** - Core contract that handles NEO staking and bNEO token issuance
   - Implements NEP-17 token standard
   - Manages NEO deposits and withdrawals
   - Distributes GAS rewards to bNEO holders

2. **BurgerAgent** - Agent contract that handles voting and NEO management
   - Manages voting for consensus nodes
   - Handles NEO transfers on behalf of the core contract
   - Claims GAS rewards and sends them to the core contract

3. **GovernanceToken (NOBUG)** - Governance token contract for the NeoBurger system
   - Implements NEP-17 token standard
   - Provides governance functionality for the NeoBurger ecosystem
   - Allows token holders to submit and execute proposals

### Building the NeoBurger Example

```bash
# Build the core contract
cd examples/neoburger
cargo build --release

# Build the agent contract
cd ../neoburger_agent
cargo build --release

# Build the governance token contract
cd ../neoburger_governance
cargo build --release
```

### Features Demonstrated

- **ink!-style Attribute Macros**: Using the new unified attribute macro system
- **NEP-17 Token Standard**: Implementation of the Neo N3 token standard
- **Storage Management**: Efficient storage of balances, rewards, and governance data
- **Voting Mechanism**: System for voting on Neo consensus nodes
- **Reward Distribution**: GAS reward distribution to token holders
- **Agent Contract System**: Delegation of NEO management to agent contracts
- **Governance Functionality**: Proposal submission and execution system
