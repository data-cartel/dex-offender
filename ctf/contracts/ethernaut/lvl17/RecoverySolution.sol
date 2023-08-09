// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RecoverySolution {
    function solution(address creator) public pure returns (address) {
        address token =
            address(uint160(uint256(keccak256(abi.encodePacked(bytes1(0xd6), bytes1(0x94), creator, bytes1(0x01))))));
        return token;
    }
}
