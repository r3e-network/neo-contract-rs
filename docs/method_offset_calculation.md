# Method Offset Calculation in Neo-Wasm Compiler

## Overview

This document explains how method offset calculation works in the Neo-Wasm compiler, which is critical for proper manifest generation in Neo N3 smart contracts. The method offset represents the byte position of a method in the compiled NEF script, which is essential for the Neo virtual machine to properly locate and execute methods.

## Technical Implementation

### 1. Method Offset Tracking

The compiler tracks method offsets at three key stages:

1. **Script Building**: As we build the NEF script from the WASM bytecode, we track the byte position of each method's start.
2. **Method Mapping**: We map WASM method names to their corresponding Rust method names.
3. **Manifest Update**: After building the script, we update the manifest with the correct offsets.

### 2. Key Components

The offset calculation process involves these primary components:

- **Builder**: Tracks the current byte position during script generation
- **ExportOffsets Map**: Maps method names to their byte positions in the script
- **MethodOffsets Map**: Transfers the offsets to the manifest update process
- **Method Mappings**: Maps between WASM export names and Rust method names

### 3. Process Flow

1. **Reset Tracking**:
   - Clear the `exportOffsets` map before each build
   - This ensures we don't have stale offsets from previous compilations

2. **WASM Export Analysis**:
   - Analyze all exports in the WASM module
   - Identify function exports that should be included as methods in the manifest

3. **Script Generation**:
   - When processing exported methods, we call `StoreMethodOffset` to record the current byte position
   - The method name is mapped to its byte offset in the `exportOffsets` map

4. **Offset Transfer**:
   - After script generation, the offsets are transferred to the `methodOffsets` map
   - This allows the manifest updater to access the offset information

5. **Method Name Mapping**:
   - The `MapMethodOffset` function maps WASM export names to their Rust method names
   - This handles custom mappings defined in neo-contract.yaml
   - Also handles common export name patterns (prefixes, mangled names)

6. **Manifest Update**:
   - The `updateManifestWithOffsets` function reads the manifest file
   - It maps WASM method offsets to the corresponding Rust method names
   - It updates each method's offset with the correct value from offset mapping
   - Handles both direct mappings and reverse mappings for comprehensive coverage
   - If changes are made, the manifest is written back to disk

### 4. Method Mapping

Method mapping is a critical part of the offset calculation process:

1. **Direct Mapping**: WASM exports directly map to Rust methods with the same name
2. **Mapped Names**: Custom mappings specified in neo-contract.yaml
3. **Pattern Recognition**: Automatic mapping for common export patterns:
   - `smart_contract_export_X` pattern
   - Mangled names with numeric suffixes (e.g., `method_1`)

## Debugging

When debugging method offset issues, look for these common patterns:

1. **Missing Methods**: Check if methods in the manifest don't have corresponding entries in the WASM exports
2. **Offset Mismatch**: Verify that offsets are being properly tracked during script building
3. **Transfer Issues**: Ensure offsets are correctly transferred from `exportOffsets` to `methodOffsets`
4. **Mapping Problems**: Verify that method name mappings are correctly applied

## Logging

The system provides detailed logging during the manifest update process:

- Lists all WASM exports with their type and index
- Reports when method offsets are stored during script building
- Shows all method mappings between WASM and Rust names
- Lists all methods being updated with their old and new offsets
- Provides warnings for methods without corresponding offsets
- Generates a summary of how many methods were updated

## Maintenance Notes

When modifying the code:

1. Maintain the clear separation between script generation and manifest updating
2. Ensure that any new exported method types are properly handled in the `isMethodName` function
3. Keep the offset calculation consistent with how the Neo VM executes methods
4. Update the method mapping logic if new export patterns are identified
5. Use the helper scripts (`fix-manifest.sh`) for troubleshooting offset issues

## Related Files

- `neo-wasm/rosetta/builder.go`: Contains script building and offset recording
- `neo-wasm/rosetta/rosetta.go`: Contains manifest updating and offset transfer logic
- `neo-wasm/cmd/fix_manifest.go`: Contains manifest correction logic
- `fix-manifest.sh`: Script for fixing method offsets in the manifest
- `neo-contract.yaml`: Configuration file for method name mappings

## Tools for Fixing Offset Issues

The framework provides several tools to help fix method offset issues:

1. **fix-manifest.sh**: Comprehensive script that handles method metadata and offset fixing
2. **neo-wasm fix-manifest**: Command that updates manifest metadata from Rust source
3. **Verbose logging**: Enable with `NEO_WASM_DEBUG=1` for detailed offset information
4. **ASM file analysis**: Use the .neo.asm file to verify method offsets manually 