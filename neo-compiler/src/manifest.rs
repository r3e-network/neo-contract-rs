use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// NEO contract manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub groups: Vec<Group>,
    pub features: Features,
    pub supported_standards: Vec<String>,
    pub abi: Abi,
    pub permissions: Vec<Permission>,
    pub trusts: Vec<String>,
    pub extra: Option<serde_json::Value>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            name: "Contract".to_string(),
            groups: vec![],
            features: Features::default(),
            supported_standards: vec![],
            abi: Abi::default(),
            permissions: vec![Permission::default()],
            trusts: vec!["*".to_string()],
            extra: None,
        }
    }
}

impl Manifest {
    /// Load manifest from a JSON file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let manifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    /// Save manifest to a JSON file
    pub fn to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Create a manifest for a Solana-style contract
    pub fn for_solana_style(program_name: &str, methods: Vec<Method>) -> Self {
        Self {
            name: program_name.to_string(),
            groups: vec![],
            features: Features {
                storage: true,
                payable: false,
            },
            supported_standards: vec![],
            abi: Abi {
                methods,
                events: vec![],
            },
            permissions: vec![Permission::default()],
            trusts: vec!["*".to_string()],
            extra: Some(serde_json::json!({
                "compiler": "neo-compiler",
                "style": "solana"
            })),
        }
    }

    /// Add NEP-17 standard support
    pub fn add_nep17_standard(&mut self) {
        self.supported_standards.push("NEP-17".to_string());
        
        // Add standard NEP-17 methods if not present
        let standard_methods = vec![
            ("symbol", vec![], "string"),
            ("decimals", vec![], "integer"),
            ("totalSupply", vec![], "integer"),
            ("balanceOf", vec![("account", "Hash160")], "integer"),
            ("transfer", vec![
                ("from", "Hash160"),
                ("to", "Hash160"),
                ("amount", "Integer"),
                ("data", "Any"),
            ], "boolean"),
        ];

        for (name, params, return_type) in standard_methods {
            if !self.abi.methods.iter().any(|m| m.name == name) {
                self.abi.methods.push(Method {
                    name: name.to_string(),
                    parameters: params.into_iter()
                        .map(|(n, t)| Parameter {
                            name: n.to_string(),
                            type_: t.to_string(),
                        })
                        .collect(),
                    return_type: return_type.to_string(),
                    offset: 0,
                    safe: name != "transfer",
                });
            }
        }

        // Add Transfer event
        if !self.abi.events.iter().any(|e| e.name == "Transfer") {
            self.abi.events.push(Event {
                name: "Transfer".to_string(),
                parameters: vec![
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
            });
        }
    }

    /// Add NEP-11 (NFT) standard support
    pub fn add_nep11_standard(&mut self) {
        self.supported_standards.push("NEP-11".to_string());
        
        // Add standard NEP-11 methods
        let standard_methods = vec![
            ("symbol", vec![], "string"),
            ("decimals", vec![], "integer"),
            ("totalSupply", vec![], "integer"),
            ("balanceOf", vec![("account", "Hash160")], "integer"),
            ("ownerOf", vec![("tokenId", "ByteArray")], "Hash160"),
            ("properties", vec![("tokenId", "ByteArray")], "Map"),
            ("tokens", vec![], "Iterator"),
            ("tokensOf", vec![("owner", "Hash160")], "Iterator"),
            ("transfer", vec![
                ("to", "Hash160"),
                ("tokenId", "ByteArray"),
                ("data", "Any"),
            ], "boolean"),
        ];

        for (name, params, return_type) in standard_methods {
            if !self.abi.methods.iter().any(|m| m.name == name) {
                self.abi.methods.push(Method {
                    name: name.to_string(),
                    parameters: params.into_iter()
                        .map(|(n, t)| Parameter {
                            name: n.to_string(),
                            type_: t.to_string(),
                        })
                        .collect(),
                    return_type: return_type.to_string(),
                    offset: 0,
                    safe: name != "transfer",
                });
            }
        }

        // Add Transfer event
        if !self.abi.events.iter().any(|e| e.name == "Transfer") {
            self.abi.events.push(Event {
                name: "Transfer".to_string(),
                parameters: vec![
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
                    Parameter {
                        name: "tokenId".to_string(),
                        type_: "ByteArray".to_string(),
                    },
                ],
            });
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub pubkey: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub storage: bool,
    pub payable: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            storage: true,
            payable: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abi {
    pub methods: Vec<Method>,
    pub events: Vec<Event>,
}

impl Default for Abi {
    fn default() -> Self {
        Self {
            methods: vec![],
            events: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    #[serde(rename = "returntype")]
    pub return_type: String,
    pub offset: u32,
    pub safe: bool,
}

impl Method {
    /// Create a new method
    pub fn new(name: String, safe: bool) -> Self {
        Self {
            name,
            parameters: vec![],
            return_type: "Void".to_string(),
            offset: 0,
            safe,
        }
    }

    /// Add a parameter
    pub fn add_param(mut self, name: &str, type_: &str) -> Self {
        self.parameters.push(Parameter {
            name: name.to_string(),
            type_: type_.to_string(),
        });
        self
    }

    /// Set return type
    pub fn returns(mut self, type_: &str) -> Self {
        self.return_type = type_.to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub contract: String,
    pub methods: Vec<String>,
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            contract: "*".to_string(),
            methods: vec!["*".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_manifest() {
        let manifest = Manifest::default();
        assert_eq!(manifest.name, "Contract");
        assert!(manifest.features.storage);
        assert!(!manifest.features.payable);
    }

    #[test]
    fn test_solana_style_manifest() {
        let methods = vec![
            Method::new("initialize".to_string(), false)
                .add_param("owner", "Hash160")
                .returns("Void"),
            Method::new("transfer".to_string(), false)
                .add_param("from", "Hash160")
                .add_param("to", "Hash160")
                .add_param("amount", "Integer")
                .returns("Boolean"),
        ];

        let manifest = Manifest::for_solana_style("MyToken", methods);
        assert_eq!(manifest.name, "MyToken");
        assert_eq!(manifest.abi.methods.len(), 2);
        
        let extra = manifest.extra.unwrap();
        assert_eq!(extra["style"], "solana");
    }

    #[test]
    fn test_nep17_standard() {
        let mut manifest = Manifest::default();
        manifest.add_nep17_standard();
        
        assert!(manifest.supported_standards.contains(&"NEP-17".to_string()));
        assert!(manifest.abi.methods.iter().any(|m| m.name == "transfer"));
        assert!(manifest.abi.methods.iter().any(|m| m.name == "balanceOf"));
        assert!(manifest.abi.events.iter().any(|e| e.name == "Transfer"));
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest = Manifest::default();
        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.name, deserialized.name);
    }

    #[test]
    fn test_manifest_file_io() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("test.manifest.json");
        
        let manifest = Manifest::default();
        manifest.to_file(&manifest_path).unwrap();
        
        let loaded = Manifest::from_file(&manifest_path).unwrap();
        assert_eq!(manifest.name, loaded.name);
    }
}