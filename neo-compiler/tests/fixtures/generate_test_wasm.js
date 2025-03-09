#!/usr/bin/env node
/**
 * This script generates a minimal valid WebAssembly module file for testing.
 * It creates a WASM file with a single exported function named "main".
 * 
 * Usage: node generate_test_wasm.js [output_path]
 * Default output: ./test_contract.wasm
 */

const fs = require('fs');
const path = require('path');

// Default output path
const outputPath = process.argv[2] || path.join(__dirname, 'test_contract.wasm');

// A minimal valid WebAssembly module with a single exported function
// This is binary representation of (module (func (export "main")))
const wasmBytes = new Uint8Array([
  0x00, 0x61, 0x73, 0x6D, // magic number: "\0asm"
  0x01, 0x00, 0x00, 0x00, // version: 1
  
  // Type section
  0x01, 0x04, 0x01, 0x60, 0x00, 0x00, // (type (func))
  
  // Function section
  0x03, 0x02, 0x01, 0x00, // (func (type 0))
  
  // Export section
  0x07, 0x07, 0x01, 0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00, // (export "main" (func 0))
  
  // Code section
  0x0A, 0x04, 0x01, 0x02, 0x00, 0x0B, // (code (func (body end)))
]);

// Write the WASM file
fs.writeFileSync(outputPath, Buffer.from(wasmBytes));

console.log(`Generated test WebAssembly module at: ${outputPath}`);
console.log('This file contains a minimal valid WASM module with an exported "main" function.');