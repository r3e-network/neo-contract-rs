use neo_contract::prelude::*;

// Define the Transfer event
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<Address>,
    #[index]
    pub to: Option<Address>,
    pub amount: u64,
}

#[neo_contract::contract]
pub struct Token {
    // Storage fields
    #[storage]
    total_supply: StorageItem<u64>,
    #[storage]
    balances: StorageMap<Address, u64>,
}

impl Token {
    // Constants
    const TOKEN_NAME: &'static str = "Example Token";
    const TOKEN_SYMBOL: &'static str = "EXT";
    const TOKEN_DECIMALS: u8 = 8;
    const TOKEN_TOTAL_SUPPLY: u64 = 100_000_000 * 100_000_000; // 100M tokens with 8 decimal places

    #[constructor]
    pub fn new() -> Self {
        let contract_owner = Runtime::calling_script_hash();
        
        let mut instance = Self {
            total_supply: StorageItem::new(b"total_supply"),
            balances: StorageMap::new(b"balances"),
        };
        
        // Set initial supply
        instance.total_supply.set(&Self::TOKEN_TOTAL_SUPPLY);
        
        // Assign all tokens to contract owner
        instance.balances.insert(contract_owner, Self::TOKEN_TOTAL_SUPPLY);
        
        // Emit transfer event (from None to owner)
        Transfer {
            from: None,
            to: Some(contract_owner),
            amount: Self::TOKEN_TOTAL_SUPPLY,
        }.notify();
        
        instance
    }
    
    #[safe]
    pub fn name(&self) -> String {
        Self::TOKEN_NAME.to_string()
    }
    
    #[safe]
    pub fn symbol(&self) -> String {
        Self::TOKEN_SYMBOL.to_string()
    }
    
    #[safe]
    pub fn decimals(&self) -> u8 {
        Self::TOKEN_DECIMALS
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
    #[no_reentry]
    pub fn transfer(&mut self, from: Address, to: Address, amount: u64) -> bool {
        // Verify caller is authorized
        if !Runtime::check_witness(&from) {
            return false;
        }
        
        // Check amount > 0
        if amount == 0 {
            return false;
        }
        
        // Get current balances
        let from_balance = self.balances.get(&from).unwrap_or_default();
        
        // Check if enough balance
        if from_balance < amount {
            return false;
        }
        
        // Handle edge cases
        if from == to {
            return true;
        }
        
        // Update balances
        let to_balance = self.balances.get(&to).unwrap_or_default();
        
        if from_balance == amount {
            self.balances.remove(&from);
        } else {
            self.balances.insert(from, from_balance - amount);
        }
        
        self.balances.insert(to, to_balance + amount);
        
        // Emit transfer event
        Transfer {
            from: Some(from),
            to: Some(to),
            amount,
        }.notify();
        
        true
    }
}