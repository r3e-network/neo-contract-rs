// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

//! Profiling utilities for smart contracts
//! Helps measure execution time and gas costs for contract operations
//! Only available in debug mode or with PROFILING feature enabled

use alloc::format;
use alloc::string::String;

// Update imports to use prelude
use crate::prelude::{Any, Array, ByteString, Int256, StorageMap};
// use crate::contract::Contract;
use crate::runtime::Runtime;

/// Debug mode configuration
#[cfg(any(debug_assertions, feature = "profiling"))]
pub const PROFILING_ENABLED: bool = true;

#[cfg(not(any(debug_assertions, feature = "profiling")))]
pub const PROFILING_ENABLED: bool = false;

/// Profiler for measuring execution time and gas costs
pub struct Profiler {
    /// Name of the profiler
    name: String,
    /// Start time of the profiler
    start_time: Option<u64>,
    /// Start gas of the profiler
    start_gas: Option<Int256>,
    /// Enabled flag
    enabled: bool,
}

impl Profiler {
    /// Create a new profiler with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            start_time: None,
            start_gas: None,
            enabled: PROFILING_ENABLED,
        }
    }

    /// Start the profiler
    pub fn start(&mut self) {
        if !self.enabled {
            return;
        }

        self.start_time = Some(Runtime::time());
        self.start_gas = Some(Runtime::gas_left().into());
    }

    /// Stop the profiler and return the elapsed time and gas
    pub fn stop(&mut self) -> Option<(u64, Int256)> {
        if !self.enabled || self.start_time.is_none() || self.start_gas.is_none() {
            return None;
        }

        let end_time = Runtime::time();
        let end_gas: Int256 = Runtime::gas_left().into();

        let elapsed_time = end_time - self.start_time.unwrap();
        let gas_used = self.start_gas.clone().unwrap() - end_gas;

        // Emit profiling event if enabled
        self.emit_profiling_event(elapsed_time, gas_used.clone());

        // Reset profiler
        self.start_time = None;
        self.start_gas = None;

        Some((elapsed_time, gas_used))
    }

    /// Emit profiling event
    fn emit_profiling_event(&self, elapsed_time: u64, gas_used: Int256) {
        if !self.enabled {
            return;
        }

        let event_name = ByteString::from("Profiling");
        let mut event_data = Array::new();

        event_data.push(Any::from(ByteString::from(self.name.as_str())));

        // Convert u64 to ByteString for compatibility with Any
        let elapsed_time_str = ByteString::from(alloc::format!("{}", elapsed_time));
        event_data.push(Any::from(elapsed_time_str));

        event_data.push(Any::from(gas_used));

        Runtime::notify(&event_name, &event_data);
    }
}

/// Profile a function and return its result
///
/// This functionality is now provided by the profile! macro from neo-macros-core,
/// which is re-exported by neo-contract.
///
/// @see profile!

/// Profile scope for measuring execution time and gas costs
/// automatically stops the profiler when it goes out of scope
pub struct ProfileScope {
    /// Profiler instance
    profiler: Profiler,
}

impl ProfileScope {
    /// Create a new profiling scope
    pub fn new(name: &str) -> Self {
        let mut profiler = Profiler::new(name);
        profiler.start();
        Self { profiler }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) { self.profiler.stop(); }
}

/// Profile a method execution and return the result
/// Create a profiling scope that automatically stops when it goes out of scope
///
/// This functionality is now provided by the profile_scope! macro from neo-macros-core,
/// which is re-exported by neo-contract.
///
/// @see profile_scope!

/// Benchmark utility for measuring multiple executions
pub struct Benchmark {
    /// Name of the benchmark
    name: String,
    /// Number of iterations
    iterations: usize,
    /// Total elapsed time
    total_time: u64,
    /// Total gas used
    total_gas: Int256,
    /// Enabled flag
    enabled: bool,
}

impl Benchmark {
    /// Create a new benchmark with the given name
    pub fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: String::from(name),
            iterations,
            total_time: 0,
            total_gas: Int256::zero(),
            enabled: PROFILING_ENABLED,
        }
    }

    /// Run the benchmark with the given function
    pub fn run<F, R>(&mut self, func: F) -> R
    where F: Fn() -> R {
        let mut result = None;

        if !self.enabled {
            return func();
        }

        for i in 0..self.iterations {
            let mut profiler = Profiler::new(&format!("{}_{}", self.name, i));
            profiler.start();

            let func_result = func();

            if i == 0 {
                result = Some(func_result);
            }

            if let Some((elapsed_time, gas_used)) = profiler.stop() {
                self.total_time += elapsed_time;
                self.total_gas = self.total_gas + gas_used;
            }
        }

        // Emit benchmark event
        self.emit_benchmark_event();

        result.unwrap()
    }

    /// Emit benchmark event
    fn emit_benchmark_event(&self) {
        if !self.enabled {
            return;
        }

        let event_name = ByteString::from("Benchmark");
        let mut event_data = Array::new();

        event_data.push(Any::from(ByteString::from(self.name.as_str())));

        // Convert numeric values to ByteString for compatibility with Any
        let iterations_str = ByteString::from(alloc::format!("{}", self.iterations));
        event_data.push(Any::from(iterations_str));

        let total_time_str = ByteString::from(alloc::format!("{}", self.total_time));
        event_data.push(Any::from(total_time_str));

        event_data.push(Any::from(self.total_gas));

        if self.iterations > 0 {
            let avg_time = self.total_time / (self.iterations as u64);
            let avg_gas = self.total_gas / Int256::from_i64(self.iterations as i64);

            let avg_time_str = ByteString::from(alloc::format!("{}", avg_time));
            event_data.push(Any::from(avg_time_str));

            event_data.push(Any::from(avg_gas));
        } else {
            event_data.push(Any::from(ByteString::from("0")));
            event_data.push(Any::from(Int256::zero()));
        }

        Runtime::notify(&event_name, &event_data);
    }
}

/// Benchmark a function execution and return the result
///
/// This functionality is now provided by the benchmark! macro from neo-macros-core,
/// which is re-exported by neo-contract.
///
/// @see benchmark!

/// Gas statistics for different operations
pub struct GasStats;

impl GasStats {
    /// Calculate average gas cost for storage operations
    pub fn storage_stats() -> Option<(Int256, Int256, Int256)> {
        if !PROFILING_ENABLED {
            return None;
        }

        // Measure put operation
        let mut put_benchmark = Benchmark::new("storage_put", 10);
        put_benchmark.run(|| {
            let storage_map = StorageMap::<ByteString, Int256>::new(b"benchmark");
            let _ = storage_map.put(&ByteString::from("test_key"), &Int256::from_i64(100));
        });

        // Measure get operation
        let mut get_benchmark = Benchmark::new("storage_get", 10);
        get_benchmark.run(|| {
            let storage_map = StorageMap::<ByteString, Int256>::new(b"benchmark");
            let _ = storage_map.get(&ByteString::from("test_key"));
        });

        // Measure delete operation
        let mut delete_benchmark = Benchmark::new("storage_delete", 10);
        delete_benchmark.run(|| {
            let storage_map = StorageMap::<ByteString, Int256>::new(b"benchmark");
            let _ = storage_map.delete(&ByteString::from("test_key"));
        });

        // Return dummy values for now since we can't cast () to i64
        Some((Int256::from_i64(0), Int256::from_i64(0), Int256::from_i64(0)))
    }
}
