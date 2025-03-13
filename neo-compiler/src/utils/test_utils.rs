use std::io::Write;
use tempfile::NamedTempFile;
use wat::parse_str;

/// Creates a test WebAssembly module for use in tests.
///
/// Returns a tuple containing:
/// - A temporary file with the WebAssembly binary
/// - The WebAssembly binary as a byte vector
pub fn create_test_wasm() -> (NamedTempFile, Vec<u8>) {
    // Simple WebAssembly module with a single exported function
    let wat = r#"(module
        (func $add (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add)
        (export "add" (func $add))
    )"#;

    // Convert WAT to WASM
    let wasm_binary = parse_str(wat).expect("Failed to parse WAT");

    // Create temporary file
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(&wasm_binary).expect("Failed to write to temp file");
    temp_file.flush().expect("Failed to flush temp file");

    (temp_file, wasm_binary)
}

/// Creates a simple WebAssembly binary without creating a temporary file.
///
/// Returns the WebAssembly binary as a byte vector.
pub fn create_test_wasm_binary() -> Vec<u8> {
    // Simple WebAssembly module with a single exported function
    let wat = r#"(module
        (func $add (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add)
        (export "add" (func $add))
    )"#;

    // Convert WAT to WASM
    parse_str(wat).expect("Failed to parse WAT")
}
