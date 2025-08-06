use anyhow::Result;
use std::collections::HashMap;

use crate::manifest::{Manifest, Method, Parameter};
use crate::WasmModule;

/// Detector for Solana-style smart contracts
pub struct SolanaStyleDetector<'a> {
    module: &'a WasmModule,
    is_solana_style: bool,
    program_module: Option<String>,
    handlers: HashMap<String, HandlerInfo>,
}

/// Information about a Solana-style handler function
#[derive(Debug, Clone)]
pub struct HandlerInfo {
    pub name: String,
    pub func_index: u32,
    pub is_read_only: bool,
    pub has_context: bool,
}

impl<'a> SolanaStyleDetector<'a> {
    /// Create a new detector
    pub fn new(module: &'a WasmModule) -> Self {
        Self {
            module,
            is_solana_style: false,
            program_module: None,
            handlers: HashMap::new(),
        }
    }

    /// Detect if this is a Solana-style contract
    pub fn detect(&mut self) -> Result<bool> {
        let function_names = self.module.get_function_names();
        
        // Look for Solana-style patterns
        for (idx, name) in &function_names {
            // Check for module-prefixed functions (e.g., "program::initialize")
            if name.contains("::") {
                self.is_solana_style = true;
                let parts: Vec<&str> = name.split("::").collect();
                
                if parts.len() >= 2 {
                    let module_name = parts[0];
                    let handler_name = parts[parts.len() - 1];
                    
                    // Store the program module name
                    if self.program_module.is_none() {
                        self.program_module = Some(module_name.to_string());
                    }
                    
                    // Create handler info
                    let handler = HandlerInfo {
                        name: handler_name.to_string(),
                        func_index: *idx,
                        is_read_only: Self::is_read_only_handler(handler_name),
                        has_context: Self::has_context_param(handler_name),
                    };
                    
                    self.handlers.insert(handler_name.to_string(), handler);
                }
            }
            
            // Also check for common Solana handler names
            if Self::is_solana_handler(name) {
                self.is_solana_style = true;
                
                let handler = HandlerInfo {
                    name: name.clone(),
                    func_index: *idx,
                    is_read_only: Self::is_read_only_handler(name),
                    has_context: Self::has_context_param(name),
                };
                
                self.handlers.insert(name.clone(), handler);
            }
        }
        
        // Check for Solana-specific imports
        for import in &self.module.imports {
            if import.module == "neo_contract" {
                if import.name.contains("Context") || 
                   import.name.contains("emit") ||
                   import.name.contains("msg") ||
                   import.name.contains("require") {
                    self.is_solana_style = true;
                }
            }
        }
        
        Ok(self.is_solana_style)
    }

    /// Check if this is a Solana-style contract
    pub fn is_solana_style(&self) -> bool {
        self.is_solana_style
    }

    /// Get the program module name
    pub fn get_program_module(&self) -> Option<&str> {
        self.program_module.as_deref()
    }

    /// Get all detected handlers
    pub fn get_handlers(&self) -> &HashMap<String, HandlerInfo> {
        &self.handlers
    }

    /// Generate a manifest for a Solana-style contract
    pub fn generate_manifest(&self) -> Result<Manifest> {
        if !self.is_solana_style {
            anyhow::bail!("Not a Solana-style contract");
        }
        
        let program_name = self.program_module
            .as_ref()
            .unwrap_or(&"Contract".to_string())
            .clone();
        
        // Convert handlers to manifest methods
        let mut methods = Vec::new();
        for (name, handler) in &self.handlers {
            let method = Method {
                name: name.clone(),
                parameters: self.infer_parameters(name),
                return_type: self.infer_return_type(name),
                offset: 0, // Will be set during compilation
                safe: handler.is_read_only,
            };
            methods.push(method);
        }
        
        // Check for standard implementations
        let mut manifest = Manifest::for_solana_style(&program_name, methods);
        
        // Auto-detect NEP standards
        if self.has_nep17_methods() {
            manifest.add_nep17_standard();
        }
        
        if self.has_nep11_methods() {
            manifest.add_nep11_standard();
        }
        
        Ok(manifest)
    }

    /// Check if handler name indicates a read-only function
    fn is_read_only_handler(name: &str) -> bool {
        let read_only_prefixes = [
            "get_", "query_", "view_", "check_", "is_", "has_",
        ];
        
        let read_only_names = [
            "balance_of", "allowance", "total_supply", "decimals",
            "symbol", "name", "owner_of", "token_uri", "properties",
        ];
        
        let lower_name = name.to_lowercase();
        
        read_only_prefixes.iter().any(|prefix| lower_name.starts_with(prefix)) ||
        read_only_names.iter().any(|n| lower_name == *n)
    }

    /// Check if function name matches common Solana patterns
    fn is_solana_handler(name: &str) -> bool {
        let common_handlers = [
            "initialize", "transfer", "transfer_from", "approve",
            "mint", "burn", "freeze", "thaw", "pause", "unpause",
            "create", "update", "delete", "close",
            "deposit", "withdraw", "stake", "unstake",
            "swap", "add_liquidity", "remove_liquidity",
        ];
        
        let lower_name = name.to_lowercase();
        common_handlers.iter().any(|h| lower_name == *h)
    }

    /// Check if handler likely has a Context parameter
    fn has_context_param(name: &str) -> bool {
        // Most Solana handlers have Context, except some getters
        !Self::is_read_only_handler(name) || name == "initialize"
    }

    /// Infer parameters for a method based on its name
    fn infer_parameters(&self, name: &str) -> Vec<Parameter> {
        match name {
            "initialize" => vec![
                Parameter {
                    name: "owner".to_string(),
                    type_: "Hash160".to_string(),
                },
            ],
            "transfer" | "transfer_from" => vec![
                Parameter {
                    name: "from".to_string(),
                    type_: "Hash160".to_string(),
                },
                Parameter {
                    name: "to".to_string(),
                    type_: "Hash160".to_string(),
                },
                Parameter {
                    name: "amount".to_string(),
                    type_: "Integer".to_string(),
                },
            ],
            "approve" => vec![
                Parameter {
                    name: "spender".to_string(),
                    type_: "Hash160".to_string(),
                },
                Parameter {
                    name: "amount".to_string(),
                    type_: "Integer".to_string(),
                },
            ],
            "balance_of" | "allowance" => vec![
                Parameter {
                    name: "account".to_string(),
                    type_: "Hash160".to_string(),
                },
            ],
            "mint" | "burn" => vec![
                Parameter {
                    name: "account".to_string(),
                    type_: "Hash160".to_string(),
                },
                Parameter {
                    name: "amount".to_string(),
                    type_: "Integer".to_string(),
                },
            ],
            _ => vec![],
        }
    }

    /// Infer return type for a method based on its name
    fn infer_return_type(&self, name: &str) -> String {
        match name {
            "balance_of" | "total_supply" | "decimals" | "allowance" => "Integer",
            "symbol" | "name" => "String",
            "transfer" | "transfer_from" | "approve" | "mint" | "burn" => "Boolean",
            "owner_of" => "Hash160",
            _ => "Void",
        }.to_string()
    }

    /// Check if contract has NEP-17 token methods
    fn has_nep17_methods(&self) -> bool {
        let required = ["transfer", "balance_of", "total_supply"];
        let handler_names: Vec<String> = self.handlers.keys().cloned().collect();
        
        required.iter().all(|method| {
            handler_names.iter().any(|h| h.contains(method))
        })
    }

    /// Check if contract has NEP-11 NFT methods
    fn has_nep11_methods(&self) -> bool {
        let required = ["owner_of", "tokens_of", "transfer"];
        let handler_names: Vec<String> = self.handlers.keys().cloned().collect();
        
        required.iter().all(|method| {
            handler_names.iter().any(|h| h.contains(method))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_only_detection() {
        assert!(SolanaStyleDetector::is_read_only_handler("get_balance"));
        assert!(SolanaStyleDetector::is_read_only_handler("balance_of"));
        assert!(SolanaStyleDetector::is_read_only_handler("view_state"));
        assert!(!SolanaStyleDetector::is_read_only_handler("transfer"));
        assert!(!SolanaStyleDetector::is_read_only_handler("mint"));
    }

    #[test]
    fn test_solana_handler_detection() {
        assert!(SolanaStyleDetector::is_solana_handler("initialize"));
        assert!(SolanaStyleDetector::is_solana_handler("transfer"));
        assert!(SolanaStyleDetector::is_solana_handler("mint"));
        assert!(!SolanaStyleDetector::is_solana_handler("random_function"));
    }

    #[test]
    fn test_context_param_detection() {
        assert!(SolanaStyleDetector::has_context_param("initialize"));
        assert!(SolanaStyleDetector::has_context_param("transfer"));
        assert!(!SolanaStyleDetector::has_context_param("balance_of"));
        assert!(!SolanaStyleDetector::has_context_param("get_symbol"));
    }
}