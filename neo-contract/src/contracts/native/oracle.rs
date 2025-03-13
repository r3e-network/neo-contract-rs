// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::env::contract;
use crate::types::builtin::h160::H160;
use crate::types::builtin::string::ByteString;
use crate::types::Any;
use crate::types::Array;
use crate::prelude::*;

/// Oracle native contract for Neo N3
/// 
/// The Oracle service allows Neo N3 smart contracts to access resources 
/// outside the blockchain through HTTPS requests.
/// 
/// Contract Hash: 0xfe924b7cfe89ddd271abaf7210a80a7e11178758
#[allow(non_snake_case)]
pub struct Oracle;

/// Oracle response codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleResponseCode {
    /// The request was successful
    Success = 0x00,
    /// The protocol is not supported
    ProtocolNotSupported = 0x10,
    /// The response content type is not supported
    ContentTypeNotSupported = 0x11,
    /// The request timed out
    Timeout = 0x12,
    /// Not found (404)
    NotFound = 0x13,
    /// Forbidden (403)
    Forbidden = 0x14,
    /// The response is too large
    ResponseTooLarge = 0x15,
    /// The price for the request is insufficient
    InsufficientFunds = 0x16,
    /// Error making the request
    Error = 0xFF,
}

impl Oracle {
    /// Returns the contract hash for the Oracle native contract
    pub fn hash() -> H160 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            contract::oracle_contract_hash()
        }
        #[cfg(target_arch = "wasm32")]
        {
            contract::native_oracle_contract_hash()
        }
    }

    /// Makes an Oracle request to an external data source
    /// 
    /// # Arguments
    /// 
    /// * `url` - The URL to request data from (must be HTTPS)
    /// * `callback` - The callback contract hash
    /// * `callback_method` - The method to call when the oracle response is received
    /// * `user_data` - Optional user data to include with the callback
    /// * `gas_for_response` - Amount of GAS to pay for the response processing
    /// 
    /// # Returns
    /// 
    /// The request ID as a u64
    pub fn request(
        url: &ByteString, 
        callback: &H160, 
        callback_method: &ByteString, 
        user_data: Option<Any>,
        gas_for_response: u64
    ) -> u64 {
        let method = ByteString::from("request");
        let mut args = Array::<Any>::new();
        args.push(Any::from(url.clone()));
        args.push(Any::from(callback.clone()));
        args.push(Any::from(callback_method.clone()));
        
        if let Some(data) = user_data {
            args.push(data);
        } else {
            args.push(Any::new()); // Null value for optional parameter
        }
        
        args.push(Any::from(gas_for_response));
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Gets the price of an Oracle request
    /// 
    /// # Returns
    /// 
    /// The price in GAS as a u64
    #[safe]
    pub fn get_price() -> u64 {
        let method = ByteString::from("getPrice");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(&Self::hash(), &method, &args);
        result.try_into().unwrap_or(0)
    }

    /// Sets the price for Oracle requests
    /// 
    /// # Arguments
    /// 
    /// * `price` - The new price in GAS
    pub fn set_price(price: u64) {
        let method = ByteString::from("setPrice");
        let mut args = Array::<Any>::new();
        args.push(Any::from(price));
        
        let _ = Runtime::call_contract(&Self::hash(), &method, &args);
    }

    /// This method should be implemented by contracts that want to receive Oracle responses
    /// 
    /// # Arguments
    /// 
    /// * `request_id` - The ID of the request
    /// * `url` - The URL that was queried
    /// * `response_code` - The response code from the Oracle
    /// * `result` - The result data (if successful)
    pub trait OracleCallback {
        fn on_oracle_response(
            request_id: u64, 
            url: ByteString, 
            response_code: OracleResponseCode, 
            result: ByteString
        );
    }
}
