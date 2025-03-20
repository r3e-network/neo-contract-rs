// This is an example of what the neo-macros-core crate should look like
// It shows the structure and implementation of declarative macros that
// should be moved from neo-contract into this separate crate.

//! Neo Smart Contract Declarative Macros
//!
//! This crate provides declarative macros for Neo N3 smart contract development.
//! It complements the procedural macros in the neo-macros crate.

// NO proc-macro = true in Cargo.toml for this crate!

/// Emit an event with the specified name and arguments
/// 
/// This macro creates an Array of arguments and emits a notification
/// through the Neo runtime system.
/// 
/// # Examples
/// 
/// ```
/// // Emit a Transfer event with three arguments
/// emit_event!("Transfer", from_address, to_address, amount);
/// ```
#[macro_export]
macro_rules! emit_event {
    ($name:expr, $arg1:expr) => {
        {
            let mut args = ::neo_contract::types::builtin::array::Array::new();
            args.push(::neo_contract::types::builtin::any::Any::from($arg1));
            ::neo_contract::runtime::notify_event($name, &args);
        }
    };
    
    ($name:expr, $arg1:expr, $arg2:expr) => {
        {
            let mut args = ::neo_contract::types::builtin::array::Array::new();
            args.push(::neo_contract::types::builtin::any::Any::from($arg1));
            args.push(::neo_contract::types::builtin::any::Any::from($arg2));
            ::neo_contract::runtime::notify_event($name, &args);
        }
    };
    
    ($name:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        {
            let mut args = ::neo_contract::types::builtin::array::Array::new();
            args.push(::neo_contract::types::builtin::any::Any::from($arg1));
            args.push(::neo_contract::types::builtin::any::Any::from($arg2));
            args.push(::neo_contract::types::builtin::any::Any::from($arg3));
            ::neo_contract::runtime::notify_event($name, &args);
        }
    };
}

/// Implement the EventEmitter trait for an event type
/// 
/// This macro automatically implements the EventEmitter trait for a struct,
/// allowing it to be used with the Neo event system.
/// 
/// # Examples
/// 
/// ```
/// struct TransferEvent { from: H160, to: H160, amount: Int256 }
/// 
/// implement_event!(TransferEvent, "Transfer");
/// ```
#[macro_export]
macro_rules! implement_event {
    ($event_type:ty, $event_name:expr) => {
        impl ::neo_contract::event::EventEmitter for $event_type {
            fn emit(&self) {
                let mut args = ::neo_contract::types::builtin::array::Array::new();
                
                // Add all fields to the event args
                let event_data = ::neo_contract::event::event_to_array(self);
                
                // Emit the notification through Runtime
                ::neo_contract::runtime::notify($event_name, &event_data);
            }
        }
    };
}

/// Extend an event type with the StandardEventEmitter trait
/// 
/// This macro adds implementation of StandardEventEmitter for a struct,
/// providing a standardized event emission interface.
/// 
/// # Examples
/// 
/// ```
/// struct TransferEvent { from: H160, to: H160, amount: Int256 }
/// 
/// extend_event!(TransferEvent);
/// ```
#[macro_export]
macro_rules! extend_event {
    ($event_type:ty) => {
        impl ::neo_contract::event::StandardEventEmitter for $event_type {
            fn notify(&self) {
                let name = stringify!($event_type);
                let mut args = ::neo_contract::types::builtin::array::Array::new();
                
                // Convert each struct field to a stack item
                let event_args = ::neo_contract::runtime::to_event_args(self);
                
                // Emit the notification
                ::neo_contract::runtime::notify(name, &event_args);
            }
        }
    };
}

/// Profile a function and return its result
/// 
/// This macro creates a profiler, starts it, executes the code,
/// stops the profiler, and returns the result of the code execution.
/// 
/// # Examples
/// 
/// ```
/// let result = profile!("my_function", {
///     // Code to profile
///     calculate_result()
/// });
/// ```
#[macro_export]
macro_rules! profile {
    ($name:expr, $body:expr) => {{
        let mut profiler = ::neo_contract::profiling::Profiler::new($name);
        profiler.start();
        let result = $body;
        profiler.stop();
        result
    }};
}

/// Create a profiling scope
/// 
/// This macro creates a ProfileScope that automatically stops
/// when it goes out of scope, useful for profiling code blocks.
/// 
/// # Examples
/// 
/// ```
/// {
///     profile_scope!("critical_section");
///     // Code in this scope will be profiled
/// } // Profile stops here automatically
/// ```
#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        let _profiler = ::neo_contract::profiling::ProfileScope::new($name);
    };
}

/// Benchmark a function execution and return the result
/// 
/// This macro creates a benchmark, runs the code multiple times,
/// and returns the result of the first execution.
/// 
/// # Examples
/// 
/// ```
/// let result = benchmark!("my_benchmark", 100, {
///     // Code to benchmark
///     calculate_result()
/// });
/// ```
#[macro_export]
macro_rules! benchmark {
    ($name:expr, $iterations:expr, $body:expr) => {{
        let mut benchmark = ::neo_contract::profiling::Benchmark::new($name, $iterations);
        benchmark.run(|| $body)
    }};
}

// In neo-contract/src/lib.rs, the macros would be re-exported like this:
// pub use neo_macros_core::{
//     emit_event, implement_event, extend_event, 
//     profile, profile_scope, benchmark,
// };

// And in Cargo.toml, the dependency would be:
// [dependencies]
// neo-macros-core = { path = "../neo-macros-core", version = "0.1.0" }
// neo-macros = { path = "../neo-macros", version = "0.1.0" }

// This file is just an example of what a separate crate would contain.
// In a real implementation, this would be a separate crate.
// Here we add a dummy main function to make the compiler happy when
// this file is treated as an example.
fn main() {
    println!("This is an example of what the neo-macros-core crate should look like.");
    println!("In a real implementation, this would be a separate crate.");
} 