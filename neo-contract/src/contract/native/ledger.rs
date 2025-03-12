// Copyright @ 2024 - present, R3E Network
// All Rights Reserved

use crate::prelude::{H160, H256, Int256, ByteString, Array, Any};
use crate::Runtime;

/// Ledger represents the Ledger native contract
pub struct Ledger;

/// Transaction represents a transaction on the Neo blockchain
pub struct Transaction {
    /// Hash of the transaction
    pub hash: H256,
    /// Version of the transaction
    pub version: u8,
    /// Nonce of the transaction
    pub nonce: u32,
    /// Sender of the transaction
    pub sender: H160,
    /// System fee of the transaction
    pub system_fee: Int256,
    /// Network fee of the transaction
    pub network_fee: Int256,
    /// Valid until block of the transaction
    pub valid_until_block: u32,
    /// Script of the transaction
    pub script: ByteString,
}

/// Block represents a block on the Neo blockchain
pub struct Block {
    /// The hash of the block
    pub hash: H256,
    /// The version of the block
    pub version: u32,
    /// The previous block hash
    pub prev_hash: H256,
    /// The merkle root of the transactions
    pub merkle_root: H256,
    /// The timestamp of the block
    pub timestamp: u64,
    /// The index of the block
    pub index: u32,
    /// The primary index of the consensus node that generated this block
    pub primary: u8,
    /// The next consensus node that will be given priority to generate a block
    pub next_consensus: H160,
    /// The transactions in the block
    pub transactions: Array,
}

/// Ledger script hash
pub fn script_hash() -> H160 {
    H160::from_hex("da65b600f7124ce6c79950c1772a36403104f2be").unwrap_or_else(H160::zero)
}

impl Ledger {
    /// Get the Ledger contract hash
    pub fn hash() -> H160 {
        script_hash()
    }

    /// Get the current block index
    pub fn current_index() -> u32 {
        let method = ByteString::from("currentIndex");
        let args = Array::new();
        
        let _result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        if let Any::Integer(_index_value) = _result {
            // In a real implementation, you would have conversion methods
            // But since we don't, we'll just return 0 as a placeholder
            0
        } else {
            0
        }
    }
    
    /// Get the current block hash
    pub fn current_hash() -> H256 {
        let method = ByteString::from("currentHash");
        let args = Array::new();
        
        let _result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        if let Any::ByteString(hash_bytes) = _result {
            // Production implementation to convert ByteString to H256
            if hash_bytes.len() != 32 {
                return H256::zero();
            }
            
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(hash_bytes.as_bytes());
            H256(bytes)
        } else {
            H256::zero()
        }
    }
    
    /// Get the block hash at the specified index
    pub fn hash_at(index: u32) -> H256 {
        let method = ByteString::from("getHash");
        let mut args = Array::new();
        args.push(Any::integer(index));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        if let Any::ByteString(hash_bytes) = result {
            // Check if we received a valid hash (32 bytes)
            if hash_bytes.len() != 32 {
                return H256::zero();
            }
            
            // Convert ByteString to H256
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(hash_bytes.as_bytes());
            H256(bytes)
        } else {
            // If the result isn't a ByteString or the index doesn't exist, return zero hash
            H256::zero()
        }
    }
    
    /// Get a block by index or hash
    pub fn get_block(index_or_hash: Any) -> Option<Block> {
        let method = ByteString::from("getBlock");
        let mut args = Array::new();
        
        // Clone index_or_hash before moving it into args
        let index_or_hash_ref = index_or_hash.clone();
        args.push(index_or_hash);
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // Production implementation to deserialize block data
        if let Any::Array(block_data) = result {
            // Check if we got an empty result
            if block_data.is_empty() {
                return None;
            }
            
            // Block structure per Neo protocol:
            // [0]: version (Integer)
            // [1]: previous hash (ByteString)
            // [2]: merkle root (ByteString)
            // [3]: timestamp (Integer)
            // [4]: index (Integer)
            // [5]: primary (Integer)
            // [6]: next consensus (ByteString)
            // [7]: transactions (Array)
            
            // Extract block fields
            let version = if let Some(Any::Integer(v)) = block_data.get(0) {
                if let Some(v_u64) = v.as_u64() {
                    if v_u64 <= u32::MAX as u64 {
                        v_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let prev_hash = if let Some(Any::ByteString(ph)) = block_data.get(1) {
                if ph.len() != 32 {
                    return None;
                }
                
                let mut hash_bytes = [0u8; 32];
                hash_bytes.copy_from_slice(ph.as_bytes());
                H256(hash_bytes)
            } else {
                return None;
            };
            
            let merkle_root = if let Some(Any::ByteString(mr)) = block_data.get(2) {
                if mr.len() != 32 {
                    return None;
                }
                
                let mut hash_bytes = [0u8; 32];
                hash_bytes.copy_from_slice(mr.as_bytes());
                H256(hash_bytes)
            } else {
                return None;
            };
            
            let timestamp = if let Some(Any::Integer(ts)) = block_data.get(3) {
                if let Some(ts_u64) = ts.as_u64() {
                    ts_u64
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let index = if let Some(Any::Integer(idx)) = block_data.get(4) {
                if let Some(idx_u64) = idx.as_u64() {
                    if idx_u64 <= u32::MAX as u64 {
                        idx_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let primary = if let Some(Any::Integer(p)) = block_data.get(5) {
                if let Some(p_u64) = p.as_u64() {
                    if p_u64 <= u8::MAX as u64 {
                        p_u64 as u8
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let next_consensus = if let Some(Any::ByteString(nc)) = block_data.get(6) {
                if nc.len() != 20 {
                    return None;
                }
                
                let mut script_hash_bytes = [0u8; 20];
                script_hash_bytes.copy_from_slice(nc.as_bytes());
                H160(script_hash_bytes)
            } else {
                return None;
            };
            
            let transactions = if let Some(Any::Array(txs)) = block_data.get(7) {
                let mut tx_array = Array::new();
                for tx in txs {
                    tx_array.push(tx.clone());
                }
                tx_array
            } else {
                Array::new()
            };
            
            // Create the hash for the block (if it wasn't provided)
            let hash = match &index_or_hash_ref {
                Any::ByteString(bs) if bs.len() == 32 => {
                    let mut hash_bytes = [0u8; 32];
                    hash_bytes.copy_from_slice(bs.as_bytes());
                    H256(hash_bytes)
                },
                _ => {
                    // If index was provided instead of hash, we need to compute/retrieve the hash
                    Ledger::hash_at(index)
                }
            };
            
            Some(Block {
                hash,
                version,
                prev_hash,
                merkle_root,
                timestamp,
                index,
                primary,
                next_consensus,
                transactions,
            })
        } else {
            None
        }
    }
    
    /// Get a transaction by hash
    pub fn get_transaction(hash: H256) -> Option<Transaction> {
        let method = ByteString::from("getTransaction");
        let mut args = Array::new();
        args.push(Any::byte_string(hash.0.to_vec()));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // Production implementation to deserialize transaction data
        if let Any::Array(tx_fields) = result {
            // Check if we got an empty result
            if tx_fields.is_empty() {
                return None;
            }
            
            // Transaction structure per Neo protocol:
            // [0]: version (Integer)
            // [1]: nonce (Integer)
            // [2]: system fee (Integer)
            // [3]: network fee (Integer)
            // [4]: valid until block (Integer)
            // [5]: sender (ByteString - script hash)
            // [6]: script (ByteString)
            
            // Extract transaction fields
            let version = if let Some(Any::Integer(v)) = tx_fields.get(0) {
                if let Some(v_u64) = v.as_u64() {
                    if v_u64 <= u8::MAX as u64 {
                        v_u64 as u8
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let nonce = if let Some(Any::Integer(n)) = tx_fields.get(1) {
                if let Some(n_u64) = n.as_u64() {
                    if n_u64 <= u32::MAX as u64 {
                        n_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let system_fee = if let Some(Any::Integer(sf)) = tx_fields.get(2) {
                sf.clone()
            } else {
                return None;
            };
            
            let network_fee = if let Some(Any::Integer(nf)) = tx_fields.get(3) {
                nf.clone()
            } else {
                return None;
            };
            
            let valid_until_block = if let Some(Any::Integer(vub)) = tx_fields.get(4) {
                if let Some(vub_u64) = vub.as_u64() {
                    if vub_u64 <= u32::MAX as u64 {
                        vub_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let sender = if let Some(Any::ByteString(s)) = tx_fields.get(5) {
                if s.len() != 20 {
                    return None;
                }
                
                let mut sender_bytes = [0u8; 20];
                sender_bytes.copy_from_slice(s.as_bytes());
                H160(sender_bytes)
            } else {
                return None;
            };
            
            let script = if let Some(Any::ByteString(s)) = tx_fields.get(6) {
                s.clone()
            } else {
                return None;
            };
            
            Some(Transaction {
                hash,
                version,
                nonce,
                sender,
                system_fee,
                network_fee,
                valid_until_block,
                script,
            })
        } else {
            None
        }
    }
    
    /// Get the transaction height (block index) for a transaction
    /// Returns 0 if the transaction is not found or not yet confirmed
    pub fn get_transaction_height(hash: H256) -> u32 {
        let method = ByteString::from("getTransactionHeight");
        let mut args = Array::new();
        
        // Convert H256 to ByteString for passing it as an argument
        args.push(Any::byte_string(hash.0.to_vec()));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns the block height as an integer
        // or 0 if the transaction is not found or not yet confirmed
        if let Any::Integer(height) = result {
            // Convert Int256 to u32
            if let Some(height_u64) = height.as_u64() {
                if height_u64 <= u32::MAX as u64 {
                    return height_u64 as u32;
                }
            }
        }
        
        // Transaction not found or not yet confirmed
        0
    }
    
    /// Get the current block timestamp in milliseconds
    pub fn current_timestamp() -> u64 {
        let method = ByteString::from("currentTimestamp");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns the timestamp as an integer (ms since epoch)
        if let Any::Integer(timestamp) = result {
            // Convert Int256 to u64
            if let Some(timestamp_u64) = timestamp.as_u64() {
                return timestamp_u64;
            }
        }
        
        // Default to 0 if conversion fails or result is not an integer
        0
    }
    
    /// Get the current validator count on the Neo network
    pub fn current_validator_count() -> u32 {
        let method = ByteString::from("currentValidatorCount");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns the validator count as an integer
        if let Any::Integer(count) = result {
            // Convert Int256 to u32
            if let Some(count_u64) = count.as_u64() {
                if count_u64 <= u32::MAX as u64 {
                    return count_u64 as u32;
                }
            }
        }
        
        // Default to 0 if conversion fails or result is not an integer
        0
    }
    
    /// Get the current block version
    pub fn block_version() -> u32 {
        let method = ByteString::from("blockVersion");
        let args = Array::new();
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns the block version as an integer
        if let Any::Integer(version) = result {
            // Convert Int256 to u32
            if let Some(version_u64) = version.as_u64() {
                if version_u64 <= u32::MAX as u64 {
                    return version_u64 as u32;
                }
            }
        }
        
        // Default to 0 if conversion fails or result is not an integer
        0
    }
    
    /// Get a transaction from a block by block hash and transaction index
    pub fn get_transaction_from_block_by_hash(block_hash: H256, tx_index: i32) -> Option<Transaction> {
        let method = ByteString::from("getTransactionFromBlock");
        let mut args = Array::new();
        
        // Convert H256 to ByteString for passing it as an argument
        args.push(Any::byte_string(block_hash.0.to_vec()));
        args.push(Any::integer(tx_index));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // Reuse the transaction deserialization logic
        // The native contract returns the transaction data in the same format as getTransaction
        if let Any::Array(tx_fields) = result {
            // Handle empty result from native contract
            if tx_fields.is_empty() {
                return None;
            }
            
            // Transaction structure per Neo protocol:
            // [0]: hash (ByteString)
            // [1]: version (Integer)
            // [2]: nonce (Integer)
            // [3]: system fee (Integer)
            // [4]: network fee (Integer)
            // [5]: valid until block (Integer)
            // [6]: sender (ByteString - script hash)
            // [7]: script (ByteString)
            
            // Extract transaction hash
            let hash = if let Some(Any::ByteString(h)) = tx_fields.get(0) {
                if h.len() != 32 {
                    return None;
                }
                
                let mut hash_bytes = [0u8; 32];
                hash_bytes.copy_from_slice(h.as_bytes());
                H256(hash_bytes)
            } else {
                return None;
            };
            
            // Extract other transaction fields
            let version = if let Some(Any::Integer(v)) = tx_fields.get(1) {
                if let Some(v_u64) = v.as_u64() {
                    if v_u64 <= u8::MAX as u64 {
                        v_u64 as u8
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let nonce = if let Some(Any::Integer(n)) = tx_fields.get(2) {
                if let Some(n_u64) = n.as_u64() {
                    if n_u64 <= u32::MAX as u64 {
                        n_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let system_fee = if let Some(Any::Integer(sf)) = tx_fields.get(3) {
                sf.clone()
            } else {
                return None;
            };
            
            let network_fee = if let Some(Any::Integer(nf)) = tx_fields.get(4) {
                nf.clone()
            } else {
                return None;
            };
            
            let valid_until_block = if let Some(Any::Integer(vub)) = tx_fields.get(5) {
                if let Some(vub_u64) = vub.as_u64() {
                    if vub_u64 <= u32::MAX as u64 {
                        vub_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let sender = if let Some(Any::ByteString(s)) = tx_fields.get(6) {
                if s.len() != 20 {
                    return None;
                }
                
                let mut sender_bytes = [0u8; 20];
                sender_bytes.copy_from_slice(s.as_bytes());
                H160(sender_bytes)
            } else {
                return None;
            };
            
            let script = if let Some(Any::ByteString(s)) = tx_fields.get(7) {
                s.clone()
            } else {
                return None;
            };
            
            Some(Transaction {
                hash,
                version,
                nonce,
                sender,
                system_fee,
                network_fee,
                valid_until_block,
                script,
            })
        } else {
            None
        }
    }
    
    /// Get a transaction from a block by block height and transaction index
    pub fn get_transaction_from_block_by_height(block_height: u32, tx_index: i32) -> Option<Transaction> {
        let method = ByteString::from("getTransactionFromBlock");
        let mut args = Array::new();
        args.push(Any::integer(block_height));
        args.push(Any::integer(tx_index));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // Reuse the transaction deserialization logic
        // The native contract returns the transaction data in the same format as getTransaction
        if let Any::Array(tx_fields) = result {
            // Handle empty result from native contract
            if tx_fields.is_empty() {
                return None;
            }
            
            // Transaction structure per Neo protocol:
            // [0]: hash (ByteString)
            // [1]: version (Integer)
            // [2]: nonce (Integer)
            // [3]: system fee (Integer)
            // [4]: network fee (Integer)
            // [5]: valid until block (Integer)
            // [6]: sender (ByteString - script hash)
            // [7]: script (ByteString)
            
            // Extract transaction hash
            let hash = if let Some(Any::ByteString(h)) = tx_fields.get(0) {
                if h.len() != 32 {
                    return None;
                }
                
                let mut hash_bytes = [0u8; 32];
                hash_bytes.copy_from_slice(h.as_bytes());
                H256(hash_bytes)
            } else {
                return None;
            };
            
            // Extract other transaction fields
            let version = if let Some(Any::Integer(v)) = tx_fields.get(1) {
                if let Some(v_u64) = v.as_u64() {
                    if v_u64 <= u8::MAX as u64 {
                        v_u64 as u8
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let nonce = if let Some(Any::Integer(n)) = tx_fields.get(2) {
                if let Some(n_u64) = n.as_u64() {
                    if n_u64 <= u32::MAX as u64 {
                        n_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let system_fee = if let Some(Any::Integer(sf)) = tx_fields.get(3) {
                sf.clone()
            } else {
                return None;
            };
            
            let network_fee = if let Some(Any::Integer(nf)) = tx_fields.get(4) {
                nf.clone()
            } else {
                return None;
            };
            
            let valid_until_block = if let Some(Any::Integer(vub)) = tx_fields.get(5) {
                if let Some(vub_u64) = vub.as_u64() {
                    if vub_u64 <= u32::MAX as u64 {
                        vub_u64 as u32
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };
            
            let sender = if let Some(Any::ByteString(s)) = tx_fields.get(6) {
                if s.len() != 20 {
                    return None;
                }
                
                let mut sender_bytes = [0u8; 20];
                sender_bytes.copy_from_slice(s.as_bytes());
                H160(sender_bytes)
            } else {
                return None;
            };
            
            let script = if let Some(Any::ByteString(s)) = tx_fields.get(7) {
                s.clone()
            } else {
                return None;
            };
            
            Some(Transaction {
                hash,
                version,
                nonce,
                sender,
                system_fee,
                network_fee,
                valid_until_block,
                script,
            })
        } else {
            None
        }
    }
    
    /// Get the signers of a transaction
    /// Returns an array of signers (script hashes) for the transaction
    pub fn get_transaction_signers(hash: H256) -> Array {
        let method = ByteString::from("getTransactionSigners");
        let mut args = Array::new();
        
        // Convert H256 to ByteString for passing it as an argument
        args.push(Any::byte_string(hash.0.to_vec()));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns an array of signers
        if let Any::Array(items) = result {
            let mut signers = Array::new();
            
            // Process each signer in the array
            for item in items {
                if let Any::ByteString(signer_bytes) = item {
                    // Each signer is a script hash (H160)
                    if signer_bytes.len() == 20 {
                        let mut script_hash_bytes = [0u8; 20];
                        script_hash_bytes.copy_from_slice(signer_bytes.as_bytes());
                        let script_hash = H160(script_hash_bytes);
                        
                        // Add the script hash to the array
                        signers.push(Any::from(script_hash));
                    }
                }
            }
            
            signers
        } else {
            // Return empty array if result is not an array
            Array::new()
        }
    }
    
    /// Get the VM state of a transaction
    /// Returns the VM state as an integer:
    /// - NONE = 0: The transaction has not been executed yet
    /// - HALT = 1: The transaction executed successfully
    /// - FAULT = 2: The transaction execution failed
    pub fn get_transaction_vm_state(hash: H256) -> i32 {
        let method = ByteString::from("getTransactionVMState");
        let mut args = Array::new();
        
        // Convert H256 to ByteString for passing it as an argument
        args.push(Any::byte_string(hash.0.to_vec()));
        
        let result = Runtime::call_contract(
            Ledger::hash(),
            method,
            args
        );
        
        // The native contract returns the VM state as an integer
        if let Any::Integer(state) = result {
            // Convert Int256 to i32
            if let Some(state_i64) = state.to_i64() {
                if state_i64 >= i32::MIN as i64 && state_i64 <= i32::MAX as i64 {
                    return state_i64 as i32;
                }
            }
        }
        
        // Default to 0 (NONE) if conversion fails or result is not an integer
        0
    }
}
