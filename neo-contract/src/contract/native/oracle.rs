// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};
// use crate::builtin::{H160, ByteString, Array, Any, Int256};
use crate::prelude::{H160, ByteString, Array, Any, Int256};
use crate::runtime::Runtime;
use core::marker::PhantomData;

/// Oracle contract for accessing off-chain data
pub struct Oracle;

/// Minimum fee for Oracle responses
pub const MINIMUM_RESPONSE_FEE: u64 = 10_000_000;

/// Callback type that will be invoked when the Oracle request is fulfilled
#[allow(dead_code)]
pub struct OracleCallback<T> {
    /// URL for the request
    url: ByteString,
    /// Filter for the request
    filter: ByteString,
    /// Callback method to call when the request is fulfilled
    callback: ByteString,
    /// Gas for the callback
    gas_for_response: i64,
    /// Timeout for the request
    timeout: u64,
    /// User data to pass to the callback
    user_data: Option<ByteString>,
    /// Phantom for the callback result type
    _phantom: PhantomData<T>,
}

/// Response type for the Oracle request
pub struct OracleResponse<T> {
    /// Original request URL
    pub url: ByteString,
    /// Original user data
    pub user_data: Option<ByteString>,
    /// Response code
    pub code: i32,
    /// Response result
    pub result: T,
}

/// Oracle request options
pub struct OracleRequestOptions {
    /// URL for the request
    pub url: ByteString,
    /// Filter for the request (JSONPath or XPath)
    pub filter: ByteString,
    /// Callback contract hash
    pub callback: H160,
    /// Callback method
    pub callback_method: ByteString,
    /// Gas for the callback
    pub gas_for_response: i64,
    /// Timeout for the request in seconds
    pub timeout: u64,
    /// User data to pass to the callback
    pub user_data: Option<ByteString>,
}

impl Oracle {
    /// Get the Oracle contract hash
    #[inline(always)]
    #[rustfmt::skip]
    pub fn hash() -> H160 {
        #[cfg(target_family = "wasm")]
        unsafe { 
            let h160 = env::contract::native_oracle_contract_hash();
            // Convert from types::builtin::H160 to builtin::H160
            H160::try_from(h160.0.as_slice()).unwrap()
        }

        #[cfg(not(target_family = "wasm"))]
        H160::from_hex("fe924b7cfe89ddd271abaf7210a80a7e11178758").unwrap_or_else(H160::zero)
    }
    
    /// Make a request to the Oracle service
    pub fn request(options: OracleRequestOptions) -> bool {
        let method = ByteString::from("request");
        let mut args = Array::new();
        
        // Add all the parameters
        args.push(Any::byte_string(options.url));
        args.push(Any::byte_string(options.filter));
        
        // Convert H160 to bytes for callback
        let callback_bytes = ByteString::from(options.callback.0.as_ref());
        args.push(Any::byte_string(callback_bytes));
        
        args.push(Any::byte_string(options.callback_method));
        
        // Need to convert these to integers, but Int256 is the only available integer type
        // For gas_for_response (i64) and timeout (u64), we'll use integer conversion
        args.push(Any::integer(options.gas_for_response));
        args.push(Any::integer(options.timeout));
        
        if let Some(user_data) = options.user_data {
            args.push(Any::byte_string(user_data));
        } else {
            args.push(Any::null());
        }
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Set the minimum Oracle response fee
    pub fn set_price(price: Int256) -> bool {
        let method = ByteString::from("setPrice");
        let mut args = Array::new();
        args.push(Any::integer(price));
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Get the minimum Oracle response fee
    pub fn get_price() -> Int256 {
        let method = ByteString::from("getPrice");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        if let Any::Integer(price) = result {
            price
        } else {
            Int256::from(0)
        }
    }
    
    /// Create a builder for an Oracle request
    pub fn build_request<T>(url: &str) -> OracleRequestBuilder<T> {
        OracleRequestBuilder::new(url)
    }
    
    /// Make a request to the Oracle service with simplified parameters
    pub fn request_simple(url: &str, filter: &str, callback: &str, user_data: Option<Any>, gas_for_response: i64) -> bool {
        let method = ByteString::from("request");
        let mut args = Array::new();
        
        args.push(Any::byte_string(ByteString::from(url)));
        args.push(Any::byte_string(ByteString::from(filter)));
        args.push(Any::byte_string(ByteString::from(callback)));
        
        if let Some(data) = user_data {
            args.push(data);
        } else {
            args.push(Any::null());
        }
        
        args.push(Any::integer(gas_for_response));
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
    
    /// Add a new URL to the accepted list
    pub fn add_url(url: ByteString, gas_for_response: i64) -> bool {
        let method = ByteString::from("addURL");
        let mut args = Array::new();
        
        args.push(Any::byte_string(url));
        // For gas_for_response (i64), we'll use integer conversion
        args.push(Any::integer(gas_for_response));
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        if let Any::Boolean(success) = result {
            success
        } else {
            false
        }
    }
}

/// Builder for Oracle requests
pub struct OracleRequestBuilder<T> {
    /// URL for the request
    url: ByteString,
    /// Filter for the request
    filter: Option<ByteString>,
    /// Callback contract hash
    callback: Option<H160>,
    /// Callback method
    callback_method: Option<ByteString>,
    /// Gas for the callback
    gas_for_response: i64,
    /// Timeout for the request
    timeout: u64,
    /// User data to pass to the callback
    user_data: Option<ByteString>,
    /// Phantom for the response type
    _phantom: PhantomData<T>,
}

impl<T> OracleRequestBuilder<T> {
    /// Create a new Oracle request builder
    pub fn new(url: &str) -> Self {
        Self {
            url: ByteString::from(url),
            filter: None,
            callback: None,
            callback_method: None,
            gas_for_response: MINIMUM_RESPONSE_FEE as i64,
            timeout: 0,
            user_data: None,
            _phantom: PhantomData,
        }
    }
    
    /// Set the filter for the request
    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = Some(ByteString::from(filter));
        self
    }
    
    /// Set the callback contract and method
    pub fn callback(mut self, callback: H160, method: &str) -> Self {
        self.callback = Some(callback);
        self.callback_method = Some(ByteString::from(method));
        self
    }
    
    /// Set the gas for the callback
    pub fn gas(mut self, gas: i64) -> Self {
        self.gas_for_response = gas;
        self
    }
    
    /// Set the timeout for the request
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set the user data for the request
    pub fn user_data(mut self, data: &[u8]) -> Self {
        self.user_data = Some(ByteString::from(data));
        self
    }
    
    /// Send the request
    pub fn send(self) -> bool {
        let callback = self.callback.unwrap_or_else(H160::zero);
        let filter = self.filter.unwrap_or_else(|| ByteString::from(""));
        let callback_method = self.callback_method.unwrap_or_else(|| ByteString::from(""));
        
        let options = OracleRequestOptions {
            url: self.url,
            filter,
            callback,
            callback_method,
            gas_for_response: self.gas_for_response,
            timeout: self.timeout,
            user_data: self.user_data,
        };
        
        Oracle::request(options)
    }
}
