# Challenge #13 - Wallet Mining

There’s a contract that incentivizes users to deploy Gnosis Safe wallets, rewarding them with 1 DVT. It integrates with an upgradeable authorization mechanism. This way it ensures only allowed deployers (a.k.a. wards) are paid for specific deployments. Mind you, some parts of the system have been highly optimized by anon CT gurus.

The deployer contract only works with the official Gnosis Safe factory at 0x76E2cFc1F5Fa8F6a5b3fC4c8F4788F0116861F9B and corresponding master copy at 0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F. Not sure how it’s supposed to work though - those contracts haven’t been deployed to this chain yet.

In the meantime, it seems somebody transferred 20 million DVT tokens to 0x9b6fb606a9f5789444c17768c6dfcf2f83563801. Which has been assigned to a ward in the authorization contract. Strange, because this address is empty as well.

Pass the challenge by obtaining all tokens held by the wallet deployer contract. Oh, and the 20 million DVT tokens too.
