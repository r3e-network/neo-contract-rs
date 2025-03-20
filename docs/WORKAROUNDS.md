# Neo Contract Rust Framework Workarounds

This document outlines current workarounds for known issues in the Neo Contract Rust Framework.

## Known Issues

### 1. Attribute Macro Resolution Issues

The attribute macros like `#[contract]`, `#[event]`, etc. might not be properly resolved due to import issues.

**Workaround**:
- Import the macros directly from `neo-macros` instead of using the re-exports:
  ```rust
  use neo_macros::{
      contract, contract_author, contract_description, contract_version,
      event, index, storage, constructor, method, safe
  };
  ```

### 2. Missing neo_contract_module in Prelude

The `#[contract]` macro looks for `neo_contract_module` in the prelude, but it might not be properly exposed.

**Workaround**:
- Create a simple contract without using the `#[contract]` macro:
  ```rust
  // Instead of:
  #[contract]
  mod my_contract {
      // ...
  }

  // Use:
  mod my_contract {
      // ... contract contents ...
  }

  #[no_mangle]
  pub fn deploying() -> bool {
      true
  }

  #[no_mangle]
  pub fn invoke(operation: String, args: Vec<Any>) -> Any {
      // dispatch operations here
  }
  ```

### 3. Event System Issues

The `#[event]` macro and event emission using `Event::emit()` might not work properly.

**Workaround**:
- Define events as regular structs and emit them using `Runtime::notify()`:
  ```rust
  // Instead of:
  #[event]
  struct Transfer {
      #[index]
      from: H160,
      to: H160,
      amount: u64,
  }
  
  // Use:
  struct Transfer {
      from: H160,
      to: H160,
      amount: u64,
  }
  
  // Instead of:
  Transfer::emit(from, to, amount);
  
  // Use:
  let mut event_args = Array::new();
  event_args.push(Any::from(from));
  event_args.push(Any::from(to));
  event_args.push(Any::from(amount));
  Runtime::notify(&ByteString::from("Transfer"), &event_args);
  ```

### 4. Storage Issues

The `#[storage]` macro might not work properly.

**Workaround**:
- Implement storage manually:
  ```rust
  // Instead of:
  #[storage]
  struct MyContract {
      counter: Item<u32>,
      balances: StorageMap<H160, u32>,
  }
  
  // Use:
  struct MyContract {
      counter: Item<u32>,
      balances: StorageMap<H160, u32>,
  }
  
  impl MyContract {
      fn new() -> Self {
          Self {
              counter: Item::new(b"counter"),
              balances: StorageMap::new(b"balances"),
          }
      }
  }
  ```

### 5. Missing Vec and String Types

The `Vec` and `String` types might not be properly imported.

**Workaround**:
- Import them directly from `alloc`:
  ```rust
  use alloc::vec::Vec;
  use alloc::string::String;
  use alloc::vec; // For the vec! macro
  ```

### 6. Contract Entry Point Issues

The `#[contract]` macro generates entry points, but they might not work.

**Workaround**:
- Implement the entry points manually:
  ```rust
  #[no_mangle]
  pub fn deploying() -> bool {
      // Initialize contract
      true
  }
  
  #[no_mangle]
  pub fn invoke(operation: String, args: Vec<Any>) -> Any {
      // Dispatch operations
      match operation.as_str() {
          "transfer" => {
              // Handle transfer
              Any::boolean(true)
          },
          _ => Any::null()
      }
  }
  ```

## Example Contract Using Workarounds

Here's an example contract that uses the workarounds:

```rust
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use neo_contract::types::builtin::h160::H160;
use neo_contract::types::builtin::string::ByteString;
use neo_contract::types::builtin::any::Any;
use neo_contract::types::builtin::array::Array;
use neo_contract::storage::item::Item;
use neo_contract::storage::map::Map as StorageMap;
use neo_contract::Runtime;

// Contract module
mod my_contract {
    use super::*;
    
    // Event (without macro)
    pub struct Transfer {
        pub from: H160,
        pub to: H160,
        pub amount: u64,
    }
    
    // Contract storage (without macro)
    pub struct MyContract {
        balances: StorageMap<H160, u64>,
        total_supply: Item<u64>,
        owner: Item<H160>,
    }
    
    impl MyContract {
        pub fn new() -> Self {
            Self {
                balances: StorageMap::new(b"balances"),
                total_supply: Item::new(b"total_supply"),
                owner: Item::new(b"owner"),
            }
        }
        
        pub fn init(&mut self) {
            let sender = Runtime::calling_script_hash();
            self.owner.set(&sender).unwrap_or(());
            self.total_supply.set(&0).unwrap_or(());
        }
        
        pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
            // Check witness
            if !Runtime::check_witness(&from) {
                return false;
            }
            
            // Check balance
            let from_balance = self.balances.get(&from).unwrap_or(None).unwrap_or(0);
            if from_balance < amount {
                return false;
            }
            
            // Update balances
            self.balances.set(&from, &(from_balance - amount)).unwrap_or(());
            let to_balance = self.balances.get(&to).unwrap_or(None).unwrap_or(0);
            self.balances.set(&to, &(to_balance + amount)).unwrap_or(());
            
            // Emit event
            let mut event_args = Array::new();
            event_args.push(Any::from(from));
            event_args.push(Any::from(to));
            event_args.push(Any::integer(amount));
            Runtime::notify(&ByteString::from("Transfer"), &event_args);
            
            true
        }
        
        pub fn balance_of(&self, address: H160) -> u64 {
            self.balances.get(&address).unwrap_or(None).unwrap_or(0)
        }
        
        pub fn get_owner(&self) -> H160 {
            self.owner.get().unwrap_or(None).unwrap_or_default()
        }
    }
}

// Manual entry points
#[no_mangle]
pub fn deploying() -> bool {
    let mut contract = my_contract::MyContract::new();
    contract.init();
    true
}

#[no_mangle]
pub fn invoke(operation: String, args: Vec<Any>) -> Any {
    let mut contract = my_contract::MyContract::new();
    
    match operation.as_str() {
        "transfer" => {
            if args.len() != 3 {
                return Any::boolean(false);
            }
            
            let from = args[0].as_h160().unwrap_or_default();
            let to = args[1].as_h160().unwrap_or_default();
            let amount = args[2].as_integer().unwrap_or_default() as u64;
            
            Any::boolean(contract.transfer(from, to, amount))
        },
        "balanceOf" => {
            if args.len() != 1 {
                return Any::integer(0);
            }
            
            let address = args[0].as_h160().unwrap_or_default();
            
            Any::integer(contract.balance_of(address) as i64)
        },
        "getOwner" => {
            Any::h160(contract.get_owner())
        },
        _ => Any::null()
    }
}
```

## Future Improvements

We are working on fixing these issues in the framework. Future releases will:

1. Properly implement all attribute macros
2. Provide better error messages
3. Fix import issues
4. Improve documentation

In the meantime, these workarounds should help you build Neo smart contracts with the current version of the framework. 