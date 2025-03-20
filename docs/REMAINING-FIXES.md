# Remaining Fixes for Neo Contract Rust Framework

This document provides detailed instructions on fixing the remaining issues in the Neo Contract Rust Framework, particularly in the `neo-contract` crate.

## 1. Fix neo-contract/Cargo.toml

Add the missing dependencies:

```toml
[dependencies]
neo-macros = { path = "../neo-macros", version = "0.1.0" }
bitflags = "1.3.2"
serde = { version = "1.0", features = ["derive"] }
inventory = "0.3"
```

## 2. Fix the Module Structure

The `builtin` module is missing from the crate root. You need to:

1. Check if there's a `builtin` directory in the `neo-contract/src/types/` directory
2. If not, create it and implement the required types
3. Fix the imports in `neo-contract/src/types/mod.rs`:

```rust
// Either correct the path
pub use crate::types::builtin::any::Any;
pub use crate::types::builtin::array::Array;
// etc.

// Or create the correct module structure and reimport
pub mod builtin;
```

## 3. Fix Method Names in the `Any` Enum

Several method names in the `Any` enum need to be aligned with their usage:

1. In `neo-contract/src/runtime/mod.rs`, you need to either:
   - Rename method calls to match the implementations: `as_i64()` → `as_integer()`, etc.
   - Or add the missing methods to the `Any` enum implementation

For example:
```rust
impl Any {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Any::Integer(val) => Some(*val as i64),
            _ => None,
        }
    }
    
    pub fn is_bool(&self) -> bool {
        matches!(self, Any::Boolean(_))
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Any::Boolean(val) => Some(*val),
            _ => None,
        }
    }
    
    // etc.
}
```

## 4. Fix Vec Type Imports

In files where `Vec` is not recognized, add the appropriate import:

```rust
use alloc::vec::Vec;
// or
use crate::prelude::Vec;
```

## 5. Fix convert_any_to_internal Function

The `convert_any_to_internal` function is missing in the Runtime implementation:

```rust
/// Convert Any to internal Neo representation
fn convert_any_to_internal(value: &Any) -> impl InternalType {
    // Implementation depends on your internal type structure
}
```

Or add the `Self::` prefix to the calls:

```rust
args_converted.push(Self::convert_any_to_internal(arg));
```

## 6. Implement From<Array> for Any

Add the missing implementation:

```rust
impl<T> From<Array<T>> for Any 
where T: Clone + Into<Any>
{
    fn from(array: Array<T>) -> Self {
        let items: Vec<Any> = array.into_iter().map(|item| item.into()).collect();
        Any::Array(items)
    }
}
```

## Testing the Fixes

After implementing these fixes, run:

```bash
cargo check -p neo-contract
```

Continue fixing any remaining issues that appear in the output. 