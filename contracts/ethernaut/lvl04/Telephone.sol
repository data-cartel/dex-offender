// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Ethernaut Level 4: Telephone
 *
 * The goal of this level is for you to claim ownership of the instance you are given.
 *
 * Things that might help
 * - Look into Solidity's documentation on the delegatecall low level function,
 *     how it works, how it can be used to delegate operations to on-chain libraries,
 *     and what implications it has on execution scope.
 * - Fallback methods
 * - Method ids
 */
contract Telephone {
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    function changeOwner(address _owner) public {
        if (tx.origin != msg.sender) {
            owner = _owner;
        }
    }
}
