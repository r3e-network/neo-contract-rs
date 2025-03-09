#![no_std]

#[neo_contract::contract]
pub mod contract_call {
    use neo_contract::prelude::*;

    // Event to log calls
    #[event]
    struct ContractCalled {
        #[index]
        target: Address,
        method: String,
        result: bool,
    }

    #[storage]
    struct ContractCaller {
        owner: Item<Address>,
        call_count: Item<u64>,
        last_called: Item<Address>,
    }

    impl ContractCaller {
        #[constructor]
        fn new(owner: Address) -> Self {
            Self {
                owner: Item::new(owner),
                call_count: Item::new(0),
                last_called: Item::new(Address::zero()),
            }
        }

        // Call another contract's method
        #[method]
        fn call_contract_method(
            &mut self, 
            target: Address, 
            method: String, 
            args: Vec<u8>
        ) -> bool {
            // Ensure only the owner can call this method
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can call contracts");

            // Increment call count
            let count = *self.call_count.get();
            self.call_count.set(count + 1);
            
            // Update last called contract
            self.last_called.set(target);
            
            // Make the contract call with the serialized arguments
            let result: bool = self.call_contract(&target, &method, args).unwrap_or(false);
            
            // Emit event with the call result
            runtime::emit_event(ContractCalled {
                target,
                method,
                result,
            });
            
            result
        }
        
        // Call a token contract's transfer method
        #[method]
        fn call_token_transfer(
            &mut self, 
            token_contract: Address, 
            from: Address, 
            to: Address, 
            amount: u64
        ) -> bool {
            // Ensure the caller is authorized for the from address
            assert!(runtime::check_witness(&from), "Not authorized to transfer from this address");
            
            // Call the token contract's transfer method
            let result: bool = self.call_contract(
                &token_contract,
                "transfer",
                (from, to, amount, Option::<Vec<u8>>::None)
            ).unwrap_or(false);
            
            // Update state on successful transfer
            if result {
                let count = *self.call_count.get();
                self.call_count.set(count + 1);
                self.last_called.set(token_contract);
                
                // Emit event
                runtime::emit_event(ContractCalled {
                    target: token_contract,
                    method: "transfer".to_string(),
                    result,
                });
            }
            
            result
        }
        
        // Get the number of successful calls made
        #[safe]
        fn get_call_count(&self) -> u64 {
            *self.call_count.get()
        }
        
        // Get the last called contract
        #[safe]
        fn get_last_called(&self) -> Address {
            self.last_called.get().clone()
        }
        
        // Get the contract owner
        #[safe]
        fn get_owner(&self) -> Address {
            self.owner.get().clone()
        }
    }
}
