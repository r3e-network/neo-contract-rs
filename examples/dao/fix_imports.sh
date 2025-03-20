#!/bin/bash

# This script fixes the imports in the DAO example by:
# 1. Adding use statements for String and Vec at the top of the file
# 2. Adding alloc to the allowed module list

# Back up the original file
cp src/lib.rs src/lib.rs.bak

# Add the missing imports at the top
sed -i '' '1s/^/#![feature(rustc_attrs)]\n/' src/lib.rs
sed -i '' '4s/^/use alloc::string::String;\nuse alloc::vec::Vec;\n/' src/lib.rs

# Make the script executable
chmod +x fix_imports.sh

echo "Imports fixed in src/lib.rs" 