/// Utility macros for no_std environment

// Note: vec! macro is already provided by alloc crate which is imported elsewhere
// We don't need to redefine it

/// format! macro for no_std - creates ByteString
#[macro_export]
macro_rules! format {
    ($fmt:expr) => {{
        $crate::types::ByteString::from_literal($fmt)
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        // For now, just return the format string without interpolation
        // In production, you'd want proper formatting
        $crate::types::ByteString::from_literal($fmt)
    }};
}

/// emit! macro for events (Solana-style)
#[macro_export]
macro_rules! emit {
    ($event:expr) => {{
        use $crate::services::runtime::Runtime;
        use $crate::types::{ByteString, Array};
        
        // Get event type name using the helper function
        let event_name = $crate::utils::type_name_of_val(&$event);
        let event_name = if let Some(pos) = event_name.rfind("::") {
            &event_name[pos+2..]
        } else {
            event_name
        };
        
        // Emit as Neo notification
        Runtime::notify(
            ByteString::from_literal(event_name),
            Array::new()
        );
    }};
}

// Helper function for emit! macro
#[doc(hidden)]
pub fn type_name_of_val<T: ?Sized>(_: &T) -> &'static str {
    core::any::type_name::<T>()
}

/// Simple println! for debugging (maps to Runtime::log)
#[macro_export]
macro_rules! println {
    ($msg:expr) => {{
        use $crate::services::runtime::Runtime;
        use $crate::types::ByteString;
        Runtime::log(ByteString::from_literal($msg));
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        use $crate::services::runtime::Runtime;
        use $crate::types::ByteString;
        // For now, just log the format string
        Runtime::log(ByteString::from_literal($fmt));
    }};
}

/// msg! macro for debugging
#[macro_export]
macro_rules! msg {
    ($msg:expr) => {{
        use $crate::services::runtime::Runtime;
        use $crate::types::ByteString;
        Runtime::log(ByteString::from_literal($msg));
    }};
}