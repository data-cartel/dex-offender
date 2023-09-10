// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./DoubleEntryPoint.sol";

contract DoubleEntryPointCheck {
    DoubleEntryPoint instance;
    address vault;
    CryptoVault cryptoVault;

    constructor(address _instance) {
        instance = DoubleEntryPoint(_instance);
        vault = instance.cryptoVault();
        cryptoVault = CryptoVault(vault);
    }

    bool ok = false;
    bytes data;
    bool public result;
    bool swept = false;

    function sweep() public {
        try cryptoVault.sweepToken(IERC20(instance.delegatedFrom())) {
            ok = true;
            data = abi.encode(false);
            return;
        } catch {
            ok = false;
            data = abi.encode(instance.balanceOf(instance.cryptoVault()) > 0);
            return;
        }
    }

    function checker() public {
        if (!ok) {
            swept = abi.decode(data, (bool));
        }
        result = swept && (!ok);
    }
}
