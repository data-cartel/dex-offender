// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Ethernaut Level 8: Vault
 *
 * Unlock the vault to pass the level!
 */
contract Vault {
    bool public locked;
    bytes32 private password;

    constructor(bytes32 _password) {
        locked = true;
        password = _password;
    }

    function unlock(bytes32 _password) public {
        if (password == _password) {
            locked = false;
        }
    }
}
