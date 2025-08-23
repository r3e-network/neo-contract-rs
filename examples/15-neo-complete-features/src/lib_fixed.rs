#![no_std]
#![no_main]

extern crate alloc;
use neo_contract::prelude::*;

// WASM global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Simple Neo N3 Complete Features Contract
pub struct NeoCompleteFeatures {
    owner: H160,
    is_initialized: bool,
}

impl NeoCompleteFeatures {
    pub fn new() -> Self {
        Self {
            owner: H160::zero(),
            is_initialized: false,
        }
    }

    pub fn initialize(&mut self, owner: H160) -> bool {
        if self.is_initialized {
            return false;
        }
        
        self.owner = owner;
        self.is_initialized = true;
        true
    }
    
    pub fn get_owner(&self) -> H160 {
        self.owner
    }
    
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}