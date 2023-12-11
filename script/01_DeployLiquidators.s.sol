// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { BaseScript } from "@ionprotocol/script/Base.s.sol";
import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";
import { IWstEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";

import { WstEthCurveLiquidator } from "../src/WstEthCurveLiquidator.sol";
import { CurveLiquidator } from "../src/CurveLiquidator.sol";
import { UniswapV3Liquidator } from "../src/UniswapV3Liquidator.sol";

import { stdJson as StdJson } from "forge-std/StdJson.sol";

contract DeployInitialGemJoinsScript is BaseScript {
    using StdJson for string;

    string configPath = "./deployment-config/01_DeployLiquidators.json";
    string config = vm.readFile(configPath);

    function run()
        public
        broadcast
        returns (
            WstEthCurveLiquidator wstEthCurveLiquidator,
            CurveLiquidator curveLiquidator,
            UniswapV3Liquidator uniswapV3Liquidator
        )
    {
        IonPool ionPool = IonPool(config.readAddress(".ionPool"));
        Liquidation liquidation = Liquidation(config.readAddress(".liquidation"));
        IWETH9 weth = IWETH9(config.readAddress(".weth"));
        IWstEth wstEth = IWstEth(config.readAddress(".wstEth"));
        IERC20 stEth = IERC20(config.readAddress(".stEth"));
        address treasury = config.readAddress(".treasury");

        wstEthCurveLiquidator = new WstEthCurveLiquidator{ salt: bytes32(abi.encode(0)) }(
            ionPool, liquidation, weth, wstEth, stEth, treasury
        );
        curveLiquidator = new CurveLiquidator{ salt: bytes32(abi.encode(0)) }(ionPool, liquidation, weth, treasury);
        uniswapV3Liquidator =
            new UniswapV3Liquidator{ salt: bytes32(abi.encode(0)) }(ionPool, liquidation, weth, treasury);
    }
}
