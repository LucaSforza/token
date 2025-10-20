// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {SapiCoin} from "../src/sapicoin.sol";

contract TokenTest is Test {
    SapiCoin public contra;
    address public luca;

    function setUp() public {
        luca = 0x03237997256f8088aC568b4A66F307A7A001D3a6;
        contra = new SapiCoin(100);
        bool result = contra.transfer(msg.sender, 10);
        require(result, "wrong");
    }


}
