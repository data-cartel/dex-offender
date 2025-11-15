# Ethers-rs to Alloy Migration Progress

## Status: Infrastructure Complete, Bindings Issue Blocking Final Compilation

### ✅ Completed Work

**Infrastructure & Dependencies:**
- Updated nix flake from `nixos-24.05` to `nixos-unstable` (latest Rust)
- Migrated from `ethers 2.0.14` to `alloy 0.8.3`
- Updated `serde` to `1.0.215` with derive feature
- Updated all workspace dependencies to latest compatible versions

**Code Migration:**
- **All 24 Ethernaut level implementations** (lvl01-lvl23 + lvl24_puzzle_wallet) migrated to alloy patterns
- **roles.rs**: Complete migration with correct `ActorProvider` type matching `ProviderBuilder` output
- **lib.rs**: Updated to use `alloy::primitives::U256` and accept RPC URLs
- **ethernaut/mod.rs**: Updated `set_up_ethernaut()` to accept `rpc_url` parameter
- **deploy_levels.rs**: Migrated to `alloy::node_bindings::Anvil`
- **Removed all `.as_ref()` calls** from level implementations (provider is used directly)

**Build Configuration:**
- Updated forge bind scripts to use `--alloy --alloy-version 0.8.3`
- Disabled linting in foundry.toml files
- Generated bindings for 165 contracts (164 in ctf, 1 in attack) with matching alloy version

### ⚠️ Current Blocker

**Bindings Compatibility Issue:**
The nightly forge (1.4.4-nightly) generates bindings with methods that don't exist in alloy 0.8.3's sol-types:
- `tokenize_returns` (method not in trait)
- `abi_decode_returns_validate` (method not in trait)
- `abi_decode_sequence_validate` (method not found)
- Parameter count mismatches in trait implementations

**Root Cause:**
Forge nightly is ahead of alloy 0.8.3 API. Options to resolve:
1. Use older stable forge matching alloy 0.8.3
2. Upgrade to alloy 1.x (but may require API changes)
3. Wait for alloy 0.8.x patch with updated sol-types

### Migration Patterns Applied

**Contract Deployment:**
```rust
let contract = Contract::deploy(&provider).await?;
```

**Contract Instantiation:**
```rust
let contract = Contract::new(address, &provider);
```

**View Function Calls:**
```rust
let result = contract.method().call().await?._0;
```

**State-Changing Transactions:**
```rust
let pending = contract.method().send().await?;
let receipt = pending.get_receipt().await?;
```

**Provider Methods:**
- `provider.default_signer_address()` (for wallet address)
- `provider.get_balance(address).await?`
- `provider.send_transaction(tx).await?.get_receipt().await?`

**Transaction Building:**
```rust
let tx = TransactionRequest::default()
    .to(address)
    .value(amount);
```

**Type Imports:**
```rust
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use alloy::primitives::keccak256;  // for keccak usage
```

### Actor Type (Resolved)

```rust
pub type ActorProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    alloy::network::Ethereum,
>;
```

This matches the exact type produced by `ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http()`.

### Remaining Work

1. **Resolve bindings compatibility** - Choose approach:
   - Option A: Install forge stable 1.3.x matching alloy 0.8.3
   - Option B: Upgrade to alloy 1.1.x and regenerate with nightly forge
   - Option C: Generate ethers bindings and create alloy wrappers

2. **Fix `default_signer_address` calls** - This method doesn't exist on Provider in alloy 0.8.3.
   Need to either:
   - Store addresses separately during Roles construction
   - Use a different approach to get signer addresses

3. **Migrate attack code** - `attack/src/ethernaut/hack01_fallback.rs` and others need same patterns

4. **Test compilation** - Full cargo build should succeed

5. **Runtime testing** - Run deploy_levels and verify exploits work

### Files Changed (3 commits)

**Commit 1 - Infrastructure:**
- 183 files (bindings regenerated)
- Added MIGRATION_NOTES.md

**Commit 2 - Level Migration:**
- 135 files (all level implementations)

**Commit 3 - Type fixes and binding regeneration:**
- Pending commit with resolved Actor type and alloy 0.8.3 bindings

### Next Steps for Completion

1. Determine best path forward for bindings (likely alloy 1.x upgrade)
2. Fix default_signer_address issue
3. Migrate attack implementations
4. Final build verification
5. Runtime test with Anvil
