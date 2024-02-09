// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { BaseScript } from "@ionprotocol/script/Base.s.sol";

import { ICurvePool } from "../src/interfaces/ICurvePool.sol";

contract DeployInitialGemJoinsScript is BaseScript {
    function run() public broadcast {
        ICurvePool curvePool = ICurvePool(0xDC24316b9AE028F1497c275EB9192a3Ea0f67022);

        curvePool.exchange{ value: 46_000 ether }(0, 1, 46_000 ether, 0);
    }
}
