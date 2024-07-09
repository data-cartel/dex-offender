// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Ethernaut {
    address private _deployer;
    address private _offender;

    mapping(uint8 => bool) public passed;
    mapping(uint8 => bool) public deployed;
    mapping(uint8 => address) public targetContracts;

    constructor(address offender_) {
        _deployer = msg.sender;
        _offender = offender_;
    }

    modifier onlyDeployer() {
        require(msg.sender == _deployer, "Only level deployer allowed!");
        _;
    }

    function markAsPassed(uint8 lvl_) public onlyDeployer {
        passed[lvl_] = true;
    }

    function deployLevel(uint8 lvl_, address target_) public onlyDeployer {
        targetContracts[lvl_] = target_;
        deployed[lvl_] = true;
        passed[lvl_] = false;
    }
}
