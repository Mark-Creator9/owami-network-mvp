// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleToken {
    uint public totalSupply;
    mapping(address => uint) public balanceOf;

    constructor() {
        totalSupply = 1000000;
    }

    function transfer(address to, uint amount) public {
        require(balanceOf[msg.sender] >= amount, "Insufficient balance");
        balanceOf[msg.sender] -= amount;
        balanceOf[to] += amount;
    }
}