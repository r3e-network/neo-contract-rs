# Test Fixtures

This directory contains WebAssembly files for testing the neo-compiler. 

## test_contract.wasm

This is a minimal WebAssembly module with a single exported function named "main". It's used by the integration tests to verify that the compiler correctly processes WebAssembly files and produces valid Neo N3 smart contracts.

To generate this test WASM file, you can compile a simple Rust contract or use the minimal WASM binary provided in the integration test code.

The integration tests will skip if this file is not present, making the tests more robust in different development environments.