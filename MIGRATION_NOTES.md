# Ethers-rs to Alloy Migration Progress

## Completed
✅ Updated nix flake to nixos-unstable for latest Rust toolchain
✅ Updated workspace dependencies from ethers 2.0.14 to alloy 0.8 with full features
✅ Updated both attack and ctf Cargo.toml to use alloy
✅ Migrated roles.rs core types to alloy (Provider, Signer, Wallet)
✅ Updated lib.rs imports and deploy function signature
✅ Updated ethernaut/mod.rs to pass RPC URL instead of Provider
✅ Updated deploy_levels binary to use alloy::node_bindings::Anvil
✅ Modified forge bind scripts to use --alloy flag
✅ Regenerated all contract ABI bindings with alloy (164 contracts in ctf, 1 in attack)

## Remaining Work

### Actor Type
The Actor type needs to be finalized. Current approach uses a complex nested type.
Consider using a simpler approach or trait object:
```rust
pub type Actor = Arc<dyn alloy::providers::Provider<alloy::network::Ethereum> + Send + Sync>;
```

### Contract API Updates
All level implementations need to be updated to use the new alloy contract API:

**Ethers pattern:**
```rust
let contract = Fallback::new(address, provider.clone());
let owner = contract.owner().await?;
contract.contribute().value(1).send().await?.await?;
```

**Alloy pattern:**
```rust
let contract = Fallback::new(address, &provider);
let Fallback::ownerReturn { _0: owner } = contract.owner().call().await?;
let receipt = contract.contribute().value(U256::from(1)).send().await?.get_receipt().await?;
```

Key differences:
- Contract::new() takes `&Provider` not `Arc<Provider>`
- Method calls need `.call()` for view functions
- Returns are structs with named fields matching Solidity returns
- `.send()` returns `PendingTransactionBuilder`, need `.get_receipt()` for receipt
- U256 is from alloy::primitives not ethers::types
- Address is from alloy::primitives not ethers::types

### Files Needing Updates
**Core (partially done):**
- [x] ctf/src/roles.rs - Provider creation (needs Actor type fix)
- [x] ctf/src/lib.rs - Imports and signatures
- [ ] ctf/src/level.rs - Trait definitions might need updates

**Levels (all need updates):**
- [ ] ctf/src/ethernaut/lvl01_fallback.rs
- [ ] ctf/src/ethernaut/lvl02_fallout.rs
- [ ] ctf/src/ethernaut/lvl03_coin_flip.rs
- [ ] ... (20+ more Ethernaut levels)
- [ ] attack/src/ethernaut/hack01_fallback.rs
- [ ] attack/src/lib.rs

## Testing
After migration:
1. Run `cargo build --all` to verify compilation
2. Test `deploy_levels` binary with Anvil
3. Run example exploit to verify end-to-end functionality
4. Update CI/CD if present

## Resources
- Alloy documentation: https://alloy.rs
- Alloy migration guide: https://github.com/alloy-rs/alloy/blob/main/MIGRATING.md
- Alloy examples: https://github.com/alloy-rs/examples
