// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

import "../../contracts/ethernaut/Example.sol";

contract ExampleTest is Test {
    address public offender = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720;
    Example public example;

    // this function is run before each test
    function setUp() public {
        vm.prank(address(0)); // sets msg.sender for the next call

        example = new Example();
    }

    function testExample() public {
        assertEq(uint160(example.owner()), uint160(offender) - uint160(tx.origin));
    }

    function testTxOrigin() public {
        assertEq(offender, tx.origin); // this was configured in .env
    }
}
