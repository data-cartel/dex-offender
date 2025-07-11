# Ethers to Alloy Migration Guide

This document outlines the migration of this CTF project from ethers-rs to alloy v1.0, including completed work and remaining tasks.

## Migration Overview

Alloy is the successor to ethers-rs, built from the ground up with significant performance improvements, better type safety, and modern Rust patterns. Key benefits include:

- **60% faster U256 arithmetic operations**
- **10x faster ABI encoding**
- **Simplified provider architecture** with fillers and layers
- **Better ergonomics** with the `sol!` macro
- **Type-safe network abstractions**

## Completed Work

### 1. Dependencies Updated
- ✅ `Cargo.toml`: Replaced `ethers = "2.0.14"` with `alloy = { version = "1.0", features = ["full"] }`
- ✅ `flake.nix`: Updated nixpkgs to 24.11 for latest Rust toolchain support
- ✅ Both `ctf/Cargo.toml` and `attack/Cargo.toml` updated

### 2. Core Infrastructure
- ✅ **Roles Module** (`ctf/src/roles.rs`): 
  - Migrated from `SignerMiddleware` to alloy's `RootProvider` with fillable wallets
  - Added address tracking (`deployer_address`, `offender_address`, etc.)
  - Updated provider creation to use `ProviderBuilder` with recommended fillers

- ✅ **Attack Roles** (`attack/src/roles.rs`): 
  - Mirrored the ctf roles structure for consistency
  - Updated to use alloy patterns

### 3. Contract Definitions
- ✅ **Fallback Contract ABI** (`ctf/src/abi/fallback.rs`):
  - Replaced 532-line ethers-generated code with clean 15-line `sol!` macro
  - Much more readable and maintainable
  - Example before/after shows dramatic improvement

### 4. Level Implementation 
- ✅ **Fallback Level** (`ctf/src/ethernaut/lvl01_fallback.rs`):
  - Updated contract deployment and interaction patterns
  - Fixed provider method calls (e.g., `get_balance`, `send_transaction`)
  - Updated to use role addresses instead of provider account queries

### 5. Attack Implementation
- ✅ **Fallback Attack** (`attack/src/ethernaut/hack01_fallback.rs`):
  - Updated contract interaction patterns
  - Added transaction receipt logging
  - Fixed value passing with proper U256 types

### 6. Library Updates
- ✅ **Main Library** (`ctf/src/lib.rs`):
  - Updated imports to use alloy primitives
  - Changed `deploy()` function signature to accept RPC URL instead of provider
  - Updated `set_up_ethernaut()` function

- ✅ **Ethernaut Module** (`ctf/src/ethernaut/mod.rs`):
  - Updated to pass RPC URL to roles creation
  - Removed ethers-specific imports

- ✅ **Deploy Script** (`ctf/src/bin/deploy_levels.rs`):
  - Updated to use alloy's Anvil bindings
  - Simplified provider creation

## Key Pattern Changes

### Before (Ethers):
```rust
// Complex provider with middleware
pub type Actor = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

// Verbose contract interaction
let balance = contract.contributions(address).await?;
let tx = contract.contribute().value(1).send().await?.await?;
```

### After (Alloy):
```rust
// Clean provider with built-in wallet support
pub type Actor = Arc<RootProvider<Http<Client>>>;

// Clean contract interaction with explicit receipts
let balance = contract.contributions(address).call().await?._0;
let receipt = contract.contribute().value(U256::from(1)).send().await?.get_receipt().await?;
```

## Remaining Work

### 1. ABI Contract Definitions (High Priority)
The following contract ABIs need to be converted from ethers-generated code to alloy `sol!` macros:

- `ctf/src/abi/*.rs` - **Approximately 100+ files** need conversion
- Pattern: Replace large ethers-generated code with concise `sol!` definitions
- Example template in `ctf/src/abi/fallback.rs`

### 2. Level Implementations (High Priority)  
Update all level implementations in `ctf/src/ethernaut/`:
- `lvl02_fallout.rs` through `lvl23_dex_two.rs`
- Update provider method calls and transaction patterns
- Use role addresses instead of provider account queries
- Add proper error handling and receipt logging

### 3. Attack Implementations (Medium Priority)
Update all attack implementations in `attack/src/ethernaut/`:
- Follow the pattern established in `hack01_fallback.rs`
- Update contract interaction patterns
- Add transaction receipt logging

### 4. Additional Components (Low Priority)
- **Damn Vulnerable DeFi**: If/when these modules are enabled
- **Testing**: Update any test files that use ethers patterns
- **Documentation**: Update any ethers-specific documentation

## Migration Commands

### With Nix (Recommended):
```bash
# Update flake
nix flake update

# Enter development environment  
nix develop

# Rebuild contracts and bindings
bind-ctf
bind-attack

# Test deployment
deploy-levels
```

### Without Nix:
```bash
# Install latest Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Test compilation
cargo check
cargo build

# Run deployment test
cargo run --bin deploy_levels
```

## Next Steps

1. **Start with Contract ABIs**: Convert the remaining contract ABI files using the fallback.rs pattern
2. **Convert Levels**: Update each level implementation following the lvl01_fallback.rs pattern  
3. **Test Incrementally**: Test each conversion to ensure functionality is preserved
4. **Update Attacks**: Convert attack implementations once their corresponding levels work

## Benefits Realized

Once migration is complete, the project will benefit from:
- **Faster compilation** due to cleaner generated code
- **Better performance** in U256 operations and ABI encoding
- **Improved maintainability** with concise `sol!` macro definitions
- **Future-proof codebase** built on alloy's stable v1.0 foundation
- **Better developer experience** with alloy's improved error messages and documentation

The migration represents a significant modernization of the codebase that will provide long-term benefits for development and maintenance.