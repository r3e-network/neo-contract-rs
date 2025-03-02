// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

#[allow(unused_imports)]
use crate::{env, types::*};
use crate::builtin::{H160, ByteString, Array, Any, Int256};
use crate::Runtime;
use core::marker::PhantomData;

/// Oracle contract for accessing off-chain data
pub struct Oracle;

/// Minimum fee for Oracle responses
pub const MINIMUM_RESPONSE_FEE: u64 = 10_000_000;

/// Callback type that will be invoked when the Oracle request is fulfilled
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
        unsafe { env::contract::native_oracle_contract_hash() }

        #[cfg(not(target_family = "wasm"))]
        H160::hex_decode("0xfe924b7cfe89ddd271abaf7210a80a7e11178758").unwrap_or_else(H160::zero)
    }
    
    /// Make a request to the Oracle service
    pub fn request(options: OracleRequestOptions) -> bool {
        let method = ByteString::from("request");
        let mut args = Array::<Any>::new();
        
        // Add all the parameters
        args.push(Any::from(options.url));
        args.push(Any::from(options.filter));
        args.push(Any::from(options.callback));
        args.push(Any::from(options.callback_method));
        args.push(Any::from(options.gas_for_response));
        args.push(Any::from(options.timeout));
        
        if let Some(user_data) = options.user_data {
            args.push(Any::from(user_data));
        } else {
            args.push(Any::new());
        }
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Set the minimum Oracle response fee
    pub fn set_price(price: Int256) -> bool {
        let method = ByteString::from("setPrice");
        let mut args = Array::<Any>::new();
        args.push(Any::from(price));
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
        }
    }
    
    /// Get the current Oracle response fee
    pub fn get_price() -> Int256 {
        let method = ByteString::from("getPrice");
        let args = Array::<Any>::new();
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        match Int256::try_from(result) {
            Ok(price) => price,
            Err(_) => Int256::zero(),
        }
    }
    
    /// Create a builder for an Oracle request
    pub fn build_request<T>(url: &str) -> OracleRequestBuilder<T> {
        OracleRequestBuilder::new(url)
    }
    
    /// Make a request to the Oracle service with simplified parameters
    pub fn request_simple(url: &str, filter: &str, callback: &str, user_data: Option<Any>, gas_for_response: i64) -> bool {
        let method = ByteString::from("request");
        let mut args = Array::<Any>::new();
        
        args.push(Any::from(ByteString::from(url)));
        args.push(Any::from(ByteString::from(filter)));
        args.push(Any::from(ByteString::from(callback)));
        
        if let Some(data) = user_data {
            args.push(Any::from(data));
        } else {
            args.push(Any::new());
        }
        
        args.push(Any::from(gas_for_response));
        
        let result = Runtime::call_contract(
            Oracle::hash(),
            method,
            args
        );
        
        match bool::try_from(result) {
            Ok(success) => success,
            Err(_) => false,
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
            gas_for_response: 100_000_000, // Default gas
            timeout: 10, // Default timeout in seconds
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
        self.user_data = Some(ByteString::from_bytes(data));
        self
    }
    
    /// Send the request
    pub fn send(self) -> bool {
        // Ensure required fields are set
        if self.filter.is_none() || self.callback.is_none() || self.callback_method.is_none() {
            return false;
        }
        
        let options = OracleRequestOptions {
            url: self.url,
            filter: self.filter.unwrap(),
            callback: self.callback.unwrap(),
            callback_method: self.callback_method.unwrap(),
            gas_for_response: self.gas_for_response,
            timeout: self.timeout,
            user_data: self.user_data,
        };
        
        Oracle::request(options)
    }
}
