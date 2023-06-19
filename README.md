# DEX OFFENDER

## How to

### Run local blockchain

```sh
anvil
```

### Run tests

```sh
cargo test -p solutions -- --nocapture
```

### Build contracts

```sh
forge build
```

### Create Rust bindings for contracts

```sh
forge build
forge bind -b ./bindings --crate-name bindings --overwrite
```

## Anvil

| Field             | Value                                                       |
| ----------------- | ----------------------------------------------------------- |
| Port              | 8545                                                        |
| Mnemonic:         | test test test test test test test test test test test junk |
| Derivation path   | m/44'/60'/0'/0/                                             |
| Base Fee          | 1000000000                                                  |
| Gas Limit         | 30000000                                                    |
| Genesis Timestamp | 1686684032                                                  |

| Role      | Address                                    | Private key                                                        |
| --------- | ------------------------------------------ | ------------------------------------------------------------------ |
| Offender  | 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720 | 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6 |
| Deployer  | 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 | 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 |
| Some user | 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 | 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d |
