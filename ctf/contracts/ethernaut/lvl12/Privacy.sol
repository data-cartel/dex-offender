// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Ethernaut Level 12: Privacy
 *
 * The creator of this contract was careful enough to protect the sensitive areas of its storage.
 *
 * Unlock this contract to beat the level.
 *
 * Things that might help:
 * Understanding how storage works
 * Understanding how parameter parsing works
 * Understanding how casting works
 *
 * Tips:
 * Remember that metamask is just a commodity. Use another tool if it is presenting problems.
 *  Advanced gameplay could involve using remix, or your own web3 provider.
 */
contract Privacy {
    bool public locked = true; // 1 byte | slot 0
    uint256 public ID = block.timestamp; // 32 bytes | slot 1
    uint8 private flattening = 10; // 1 byte | slot 2
    uint8 private denomination = 255; // 1 byte | slot 2
    uint16 private awkwardness = uint16(block.timestamp); // 2 bytes | slot 2
    bytes32[3] private data; //32 * 3 = 96 bytes | slots 3, 4, 5, 6

    constructor(bytes32[3] memory _data) {
        data = _data;
    }

    function unlock(bytes16 _key) public {
        require(_key == bytes16(data[2]));
        locked = false;
    }

    /*
    A bunch of super advanced solidity algorithms...

      ,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`
      .,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,
      *.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^         ,---/V\
      `*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.    ~|__(o.o)
      ^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'^`*.,*'  UU  UU
    */
}
