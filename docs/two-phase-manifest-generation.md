# Two-Phase Manifest Generation

This document describes the two-phase manifest generation process implemented in neo-contract-rs, designed to ensure accurate method detection and offset calculation in Neo N3 contract manifests.

## The Problem

The manifest file for a Neo N3 smart contract needs to include all methods with their correct offsets in the compiled NEF file. Previously, the manifest generation was done in a single step during the WASM to NEF conversion, which sometimes resulted in:

1. Missing methods that were defined in the Rust code but not properly detected from the WASM binary
2. Incorrect method offsets that didn't match the actual location in the NEF file
3. Inconsistent method parameters and return type information

## The Solution: Two-Phase Manifest Generation

We've implemented a two-phase approach to manifest generation that addresses these issues:

### Phase 1: Source Code Analysis

During the Rust to WASM compilation step:

1. The Rust source code (typically `lib.rs`) is analyzed to extract:
   - All public methods defined in `impl` blocks
   - Documentation comments for methods and the contract itself
   - Safety annotations (`@safe` tags in doc comments)
   - Method parameter and return type information when possible

2. An initial manifest is generated with:
   - Complete method listings based on the source code
   - Accurate method documentation
   - Safety status based on annotations or naming conventions
   - Provisional parameter and return type information
   - Placeholder zero values for method offsets (to be filled in Phase 2)

3. This initial manifest is saved alongside the WASM file

### Phase 2: Offset Calculation

During the WASM to NEF conversion:

1. The initial manifest is loaded
2. The WASM binary is analyzed to extract:
   - Actual exported functions
   - Function offsets in the compiled code

3. The manifest is updated with:
   - Accurate method offsets corresponding to their position in the NEF file
   - Any adjustments to parameter or return types based on WASM analysis
   - Any additional methods that might be found in the WASM but weren't detected in Phase 1

4. The final manifest is saved with the NEF file

## Benefits

This two-phase approach provides several advantages:

1. **Completeness**: Methods defined in the Rust code are properly included in the manifest, even if the WASM analysis can't detect them fully
2. **Accuracy**: Method offsets are correctly calculated based on the actual compiled NEF file
3. **Documentation**: Method documentation from the Rust source is preserved
4. **Standards Detection**: Better detection of supported standards (NEP-17, NEP-11) through source code analysis

## Implementation Details

The implementation consists of:

1. `RustManifestGenerator` in the `rust_parser` package, which handles the Rust source analysis and initial manifest generation
2. Enhanced `saveNeoManifest` method in `rosetta.go` that merges the initial manifest with WASM analysis data
3. Updated compilation script `compile-neo.sh` that orchestrates the two-phase process

## Best Practices

For optimal results with the two-phase manifest generation:

1. Use descriptive doc comments (`///`) for your contract and methods
2. Mark read-only methods with `@safe` in their doc comments
3. Follow standard naming conventions for methods (e.g., `get_*` for getters)
4. Implement standard interfaces completely (NEP-17, NEP-11) to ensure proper detection
5. Review the generated manifest before deployment

## Related Documentation

- [Understanding NEO Manifests](understanding-neo-manifests.md)
- [Documentation Best Practices](documentation-best-practices.md)
- [Code Documentation Style](code-documentation-style.md) 