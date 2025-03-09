#![no_std]

#[neo_contract::contract]
pub mod nep17_token {
    use neo_contract::prelude::*;

    // Define the Transfer event
    #[event]
    struct Transfer {
        #[index]
        from: Option<Address>,
        #[index]
        to: Option<Address>,
        amount: u64,
    }

    // Token storage structure
    #[storage]
    struct Nep17Token {
        // Token metadata
        name: Item<String>,
        symbol: Item<String>,
        decimals: Item<u8>,
        total_supply: Item<u64>,
        
        // Owner of the contract
        owner: Item<Address>,
        
        // Balances mapping
        balances: Map<Address, u64>,
    }

    impl Nep17Token {
        #[constructor]
        fn new(
            owner: Address,
            name: String, 
            symbol: String, 
            decimals: u8, 
            total_supply: u64
        ) -> Self {
            // Create balance for the owner with the total supply
            let mut balances = Map::new();
            balances.insert(owner, total_supply);
            
            // Emit transfer event from None (mint) to owner
            Self::emit_transfer(None, Some(owner), total_supply);
            
            Self {
                name: Item::new(name),
                symbol: Item::new(symbol),
                decimals: Item::new(decimals),
                total_supply: Item::new(total_supply),
                owner: Item::new(owner),
                balances,
            }
        }
        
        // NEP-17 methods
        
        // Transfer tokens from one address to another
        #[method]
        fn transfer(&mut self, from: Address, to: Address, amount: u64, data: Option<Vec<u8>>) -> bool {
            // Check that the sender is authorized
            assert!(runtime::check_witness(&from), "No authorization");
            
            // Check that the recipient is valid
            assert!(to != Address::zero(), "Invalid recipient address");
            
            // Get the sender's balance
            let from_balance = self.balances.get(&from).unwrap_or_default();
            
            // Check that the sender has enough tokens
            assert!(from_balance >= amount, "Insufficient balance");
            
            // Update balances
            if amount > 0 {
                // Reduce sender's balance
                let new_from_balance = from_balance - amount;
                if new_from_balance > 0 {
                    self.balances.insert(from, new_from_balance);
                } else {
                    self.balances.remove(&from);
                }
                
                // Increase recipient's balance
                let to_balance = self.balances.get(&to).unwrap_or_default();
                self.balances.insert(to, to_balance + amount);
                
                // Emit transfer event
                Self::emit_transfer(Some(from), Some(to), amount);
                
                // If the recipient is a contract, call onNEP17Payment
                if self.is_contract(&to) {
                    let _ = self.call_contract::<bool>(
                        &to,
                        "onNEP17Payment",
                        (from, amount, data.unwrap_or_default()),
                    );
                }
            }
            
            true
        }
        
        // Get the token symbol
        #[safe]
        fn symbol(&self) -> String {
            self.symbol.get().clone()
        }
        
        // Get the token name
        #[safe]
        fn name(&self) -> String {
            self.name.get().clone()
        }
        
        // Get the token decimals
        #[safe]
        fn decimals(&self) -> u8 {
            *self.decimals.get()
        }
        
        // Get the total supply of tokens
        #[safe]
        fn total_supply(&self) -> u64 {
            *self.total_supply.get()
        }
        
        // Get the balance of an account
        #[safe]
        fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        // Mint new tokens (only owner)
        #[method]
        fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Ensure only the contract owner can mint
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can mint");
            
            // Update total supply
            let current_supply = *self.total_supply.get();
            self.total_supply.set(current_supply + amount);
            
            // Update recipient balance
            let balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to, balance + amount);
            
            // Emit transfer event (mint = transfer from None)
            Self::emit_transfer(None, Some(to), amount);
            
            true
        }
        
        // Burn tokens
        #[method]
        fn burn(&mut self, from: Address, amount: u64) -> bool {
            // Ensure the token owner is authorizing the burn
            assert!(runtime::check_witness(&from), "No authorization");
            
            // Get current balance
            let balance = self.balances.get(&from).unwrap_or_default();
            assert!(balance >= amount, "Insufficient balance to burn");
            
            // Update balance
            let new_balance = balance - amount;
            if new_balance > 0 {
                self.balances.insert(from, new_balance);
            } else {
                self.balances.remove(&from);
            }
            
            // Update total supply
            let current_supply = *self.total_supply.get();
            self.total_supply.set(current_supply - amount);
            
            // Emit transfer event (burn = transfer to None)
            Self::emit_transfer(Some(from), None, amount);
            
            true
        }
        
        // Helper method to check if an address is a contract
        fn is_contract(&self, address: &Address) -> bool {
            address != &Address::zero() && 
            runtime::contract_exists(address)
        }
        
        // Helper to emit transfer event
        fn emit_transfer(from: Option<Address>, to: Option<Address>, amount: u64) {
            runtime::emit_event(Transfer { from, to, amount });
        }
    }
}
