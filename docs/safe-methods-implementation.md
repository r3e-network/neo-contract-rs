# Safe Methods Implementation - Technical Details

This document provides a comprehensive overview of how the `#[safe]` attribute flows through the Neo Rust contract framework from source code to the final manifest.

## Implementation Flow

### 1. Rust Code with `#[safe]` Attribute

The process begins when a developer marks a contract method with the `#[safe]` attribute:

```rust
#[neo::contract]
impl TokenContract {
    #[safe]
    pub fn total_supply() -> u64 {
        // Read-only implementation
    }
}
```

### 2. Attribute Processing in Procedural Macros

The `#[neo::contract]` macro processes the implementation block and identifies methods with the `#[safe]` attribute:

**File:** `neo-contract-proc-macros/src/contract.rs`

```rust
// Check if the method has a #[safe] attribute
fn has_safe_attribute(method: &syn::ImplItemFn) -> bool {
    method.attrs.iter().any(|attr| {
        if let Ok(path) = attr.path.clone().into_token_stream().to_string().parse::<String>() {
            path.trim() == "safe"
        } else {
            false
        }
    })
}
```

When processing each method in the implementation:

```rust
fn expand_impl_item(item: &syn::ItemImpl) -> TokenStream {
    // ... code omitted ...
    
    // Check if the method has a #[safe] attribute
    let is_safe = has_safe_attribute(method);
    
    // If the method is marked as safe, add a comment that can be parsed by the WASM to NEF converter
    let safe_comment = if is_safe {
        quote::quote! { /* @safe */ }
    } else {
        quote::quote! {}
    };
    
    quote::quote! {
        #[no_mangle]
        #safe_comment
        pub fn #name(#args) #returns {
            #self_type::#name(#args)
        }
    }
    
    // ... code omitted ...
}
```

The key operation here is that the macro adds a special `/* @safe */` comment to methods marked with the `#[safe]` attribute.

### 3. WebAssembly Generation and Comment Preservation

When the Rust code is compiled to WebAssembly, the `/* @safe */` comments are preserved in the custom sections of the WASM binary.

### 4. WASM Custom Section Decoding

During the conversion from WASM to NEF format, the decoder extracts these comments:

**File:** `neo-wasm/wasm/decoder.go`

```go
func (dec *ModuleDecoder) decodeCustomSection(header SectionHeader) error {
    // ... code omitted ...
    
    sectionName := string(name)
    customSection := CustomSection{
        SectionHeader: header,
        Name:          sectionName,
        FuncComments:  make(map[int]string),
    }
    
    if sectionName == "comment" || sectionName == "rust_metadata" {
        // Try to extract function comments from data
        comments := string(data)
        
        // Parse the comment data for any @safe annotations
        lines := strings.Split(comments, "\n")
        for _, line := range lines {
            line = strings.TrimSpace(line)
            
            // Find function index markers like @0 or @function_0
            if strings.HasPrefix(line, "@") {
                parts := strings.SplitN(line[1:], " ", 2)
                if len(parts) >= 2 {
                    // Try to parse the function index
                    indexStr := parts[0]
                    index, err := strconv.Atoi(indexStr)
                    if err == nil {
                        customSection.FuncComments[index] = parts[1]
                    }
                }
            }
        }
    }
    
    // ... code omitted ...
}
```

### 5. Method Safety Detection in Manifest Generation

During manifest generation, the `hasSafeAnnotation` function checks if a method has the `@safe` annotation:

**File:** `neo-wasm/rosetta/rosetta.go`

```go
// hasSafeAnnotation checks if a function has a @safe annotation in its custom section comments
func (rs *Rosetta) hasSafeAnnotation(module *wasm.Module, index int) bool {
    // Look for the custom sections with comments
    for i := range module.Customs {
        // Initialize the map if it doesn't exist
        if module.Customs[i].FuncComments == nil {
            continue
        }

        // Check if there's a comment for this function
        if comment, ok := module.Customs[i].FuncComments[index]; ok {
            // Check if the comment contains @safe
            if strings.Contains(comment, "@safe") {
                rs.logger.Debug("Found @safe annotation for function at index", index)
                return true
            }
        }
    }
    return false
}
```

### 6. Setting the Safe Flag in the Manifest

Finally, when generating the Neo contract manifest, the `saveNeoManifest` function sets the `Safe` flag based on the presence of the `@safe` annotation:

```go
// Create method
method := neo.Method{
    Name:   fn.Name,
    Offset: uint32(fn.Index),
    Parameters: []neo.Parameter{
        // Parameter info omitted
    },
    ReturnType: neo.Void,
    // By default, assume the method is safe (read-only)
    Safe: true,
}

// Check if method has a @safe annotation in its comment
isSafe := rs.hasSafeAnnotation(&module, fn.Index)

// Read-only methods should be marked with @safe annotation
// Only if the annotation is absent, fall back to name-based checks
if !isSafe {
    // If no @safe annotation, check if it's a known safe method by name
    if !(strings.HasPrefix(fn.Name, "get_") ||
        strings.HasPrefix(fn.Name, "query_") ||
        strings.HasPrefix(fn.Name, "balance_") ||
        strings.HasPrefix(fn.Name, "total_") ||
        strings.Contains(fn.Name, "symbol") ||
        strings.Contains(fn.Name, "decimals") ||
        fn.Name == "contract_info") {
        // If not a known safe method by name and no @safe annotation, mark as unsafe
        method.Safe = false
    }
}
```

## Notes on Backward Compatibility

For backward compatibility, methods with names that conventionally indicate read-only operations (like those starting with `get_`, `query_`, etc.) are still marked as safe even without the `#[safe]` attribute. However, it's recommended to explicitly use the `#[safe]` attribute for all read-only methods for clarity.

## Manifest Format

In the final Neo N3 contract manifest JSON, the safe methods will be represented with:

```json
{
  "methods": [
    {
      "name": "total_supply",
      "parameters": [],
      "returntype": "Integer",
      "safe": true
    },
    {
      "name": "transfer",
      "parameters": [
        {"name": "from", "type": "Hash160"},
        {"name": "to", "type": "Hash160"},
        {"name": "amount", "type": "Integer"}
      ],
      "returntype": "Boolean",
      "safe": false
    }
  ]
}
```

## Best Practices

1. **Always use explicit annotations**: Mark all read-only methods with `#[safe]` rather than relying on naming conventions.

2. **Check method semantics**: Only mark methods as `#[safe]` if they truly don't modify contract state.

3. **Composition awareness**: If a safe method calls other methods, ensure those methods are also safe.

4. **Documentation**: Document the safety characteristics of methods in comments.
