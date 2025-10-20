// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "../lib/forge-std/src/Script.sol";
import {SapiCoin} from "../src/sapicoin.sol";
import {console} from "../lib/forge-std/src/Console.sol";
contract CounterScript is Script {
    SapiCoin public contra;
    address public luca;
    function setUp() public {}

    // TODO: questo viene eseguito ogni round?
    function run() public {
        vm.startBroadcast();
        luca = 0x03237997256f8088aC568b4A66F307A7A001D3a6;
        contra = new SapiCoin(100);
        bool result = contra.transfer(luca, 10);
        require(result, "very wrong");
        console.log("balance:%d",contra.balanceOf(luca));

        vm.stopBroadcast();
    }
}
