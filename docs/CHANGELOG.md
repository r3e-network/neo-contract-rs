# Changelog

## [Unreleased]

### Added
- Enhanced manifest generation with documentation extraction from Rust source files
- Support for `@safe` annotations in method documentation for marking read-only methods
- Automatic detection of NEP-17 and NEP-11 standard implementations
- New documented token example showcasing best practices
- Documentation style guide for Neo smart contracts
- Efficient smart contracts guide with optimization techniques
- Method descriptions in manifest's Extra field
- Enhanced parameter type handling in manifest generation

### Fixed
- Fixed unused imports in rosetta.go
- Fixed undefined hash issue by implementing sha256Checksum function
- Fixed param.Type undefined issue in manifest event parameter handling
- Added proper error handling in manifest generation

### Improved
- Updated documentation structure with more comprehensive guides
- Enhanced method safety detection with proper annotation support
- Improved contract description extraction from source files
- Better organization of documentation with consistent style
- Expanded examples with efficiency best practices
- Enhanced README files with more detailed information

## [0.1.0] - Initial Release

### Added
- Basic NEO contract implementation with Rust
- NEP-17 token standard support
- Basic manifest generation
- Simple examples:
  - Hello World
  - Simple Storage
  - NEP-17 Token
  - Transfer
  - NEP-11 NFT
  - Oracle Price Feed
- Core documentation structure