#![no_std]

extern crate alloc;

use alloc::string::String;
use alloc::format;
use neo_contract::prelude::*;

#[contract]
pub struct HelloWorld;

#[contract_impl]
impl HelloWorld {
    pub fn hello() -> ByteString {
        Runtime::log("Hello, Neo N3!");
        ByteString::from("Hello, Neo N3!")
    }
    
    pub fn greet(name: ByteString) -> ByteString {
        let message = format!("Hello, {}!", name);
        Runtime::log(&message);
        ByteString::from(message)
    }
}
