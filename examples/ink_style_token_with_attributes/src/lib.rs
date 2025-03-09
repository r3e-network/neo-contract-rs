#![no_std]

#[neo_contract::contract]
#[manifest_extra(
    author = "Neo Project",
    email = "contact@neo.org",
    description = "A sample NEP-17 token implementation using ink!-style syntax with attributes",
    version = "1.0.0"
)]
#[supported_standards("NEP-17")]
pub mod token {
    use neo_contract::prelude::*;

    // Define the Transfer event
    #[event]
    pub struct Transfer {
        #[index]
        pub from: Option<Address>,
        #[index]
        pub to: Option<Address>,
        pub amount: u64,
    }
    
    // Define static values
    #[contract_hash]
    const GAS_TOKEN_HASH: Hash160 = hex!("d2a4cff31913016155e38e474a2c06d08be276cf");
    
    #[string]
    const TOKEN_NAME: &str = "Ink Style Token";
    
    #[string]
    const TOKEN_SYMBOL: &str = "IST";
    
    #[integer]
    const TOKEN_DECIMALS: u8 = 8;
    
    #[integer]
    const INITIAL_SUPPLY: u64 = 100_000_000;

    // Token storage with attributes
    #[storage]
    pub struct TokenContract {
        // Token metadata
        pub name: Item<String>,
        pub symbol: Item<String>,
        pub decimals: Item<u8>,
        pub total_supply: Item<u64>,
        
        // Owner with admin rights
        pub owner: Item<Address>,
        
        // Token balances
        pub balances: Map<Address, u64>,
        
        // Record of frozen accounts
        pub frozen_accounts: Map<Address, bool>,
        
        // Minting allowed flag
        pub minting_allowed: Item<bool>,
    }

    // Implementation with security attributes
    impl TokenContract {
        #[constructor]
        pub fn new(owner: Address) -> Self {
            // Create balance for the owner with the initial supply
            let mut balances = Map::new();
            balances.insert(owner, INITIAL_SUPPLY);
            
            // Emit transfer event (mint)
            runtime::emit_event(Transfer {
                from: None,
                to: Some(owner),
                amount: INITIAL_SUPPLY,
            });
            
            Self {
                name: Item::new(TOKEN_NAME.to_string()),
                symbol: Item::new(TOKEN_SYMBOL.to_string()),
                decimals: Item::new(TOKEN_DECIMALS),
                total_supply: Item::new(INITIAL_SUPPLY),
                owner: Item::new(owner),
                balances,
                frozen_accounts: Map::new(),
                minting_allowed: Item::new(true),
            }
        }
        
        // NEP-17 standard methods
        
        #[safe]
        pub fn symbol(&self) -> String {
            self.symbol.get().clone()
        }
        
        #[safe]
        pub fn name(&self) -> String {
            self.name.get().clone()
        }
        
        #[safe]
        pub fn decimals(&self) -> u8 {
            *self.decimals.get()
        }
        
        #[safe]
        pub fn total_supply(&self) -> u64 {
            *self.total_supply.get()
        }
        
        #[safe]
        pub fn balance_of(&self, account: Address) -> u64 {
            self.balances.get(&account).unwrap_or_default()
        }
        
        // Transfer with security attributes
        #[method]
        #[no_reentrant]
        pub fn transfer(&mut self, from: Address, to: Address, amount: u64, data: Option<Vec<u8>>) -> bool {
            // Check that the sender is authorized
            assert!(runtime::check_witness(&from), "No authorization");
            
            // Check that accounts are not frozen
            assert!(!self.frozen_accounts.get(&from).unwrap_or_default(), "Sender account is frozen");
            assert!(!self.frozen_accounts.get(&to).unwrap_or_default(), "Recipient account is frozen");
            
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
                runtime::emit_event(Transfer {
                    from: Some(from),
                    to: Some(to),
                    amount,
                });
                
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
        
        // Admin methods with access control
        
        #[method]
        pub fn mint(&mut self, to: Address, amount: u64) -> bool {
            // Ensure only the contract owner can mint
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can mint");
            
            // Ensure minting is allowed
            assert!(*self.minting_allowed.get(), "Minting is disabled");
            
            // Ensure recipient account is not frozen
            assert!(!self.frozen_accounts.get(&to).unwrap_or_default(), "Recipient account is frozen");
            
            // Update total supply
            let current_supply = *self.total_supply.get();
            self.total_supply.set(current_supply + amount);
            
            // Update recipient balance
            let balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(to, balance + amount);
            
            // Emit transfer event (mint = transfer from None)
            runtime::emit_event(Transfer {
                from: None,
                to: Some(to),
                amount,
            });
            
            true
        }
        
        #[method]
        pub fn burn(&mut self, from: Address, amount: u64) -> bool {
            // Ensure the token owner is authorizing the burn
            assert!(runtime::check_witness(&from), "No authorization");
            
            // Ensure account is not frozen
            assert!(!self.frozen_accounts.get(&from).unwrap_or_default(), "Account is frozen");
            
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
            runtime::emit_event(Transfer {
                from: Some(from),
                to: None,
                amount,
            });
            
            true
        }
        
        #[method]
        pub fn freeze_account(&mut self, account: Address, frozen: bool) -> bool {
            // Ensure only the contract owner can freeze accounts
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can freeze accounts");
            
            // Update frozen status
            self.frozen_accounts.insert(account, frozen);
            true
        }
        
        #[method]
        pub fn set_minting_allowed(&mut self, allowed: bool) -> bool {
            // Ensure only the contract owner can change minting status
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can change minting status");
            
            self.minting_allowed.set(allowed);
            true
        }
        
        #[method]
        pub fn transfer_ownership(&mut self, new_owner: Address) -> bool {
            // Ensure only the current owner can transfer ownership
            let owner = self.owner.get().clone();
            assert!(runtime::check_witness(&owner), "Only owner can transfer ownership");
            
            // Ensure the new owner is valid
            assert!(new_owner != Address::zero(), "Invalid new owner address");
            
            self.owner.set(new_owner);
            true
        }
        
        // Helper methods
        
        #[safe]
        pub fn is_frozen(&self, account: Address) -> bool {
            self.frozen_accounts.get(&account).unwrap_or_default()
        }
        
        #[safe]
        pub fn is_minting_allowed(&self) -> bool {
            *self.minting_allowed.get()
        }
        
        #[safe]
        pub fn get_owner(&self) -> Address {
            self.owner.get().clone()
        }
        
        // Internal helper method
        fn is_contract(&self, address: &Address) -> bool {
            address != &Address::zero() && 
            runtime::contract_exists(address)
        }
    }
}
