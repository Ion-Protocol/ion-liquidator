// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { BaseScript } from "@ionprotocol/script/Base.s.sol";
import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";
import { IWstEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";
import { CREATEX } from "@ionprotocol/src/Constants.sol";

import { WeEthCurveLiquidator } from "../src/WeEthCurveLiquidator.sol";
import { WeEthUniswapLiquidator } from "../src/WeEthUniswapLiquidator.sol";

import { stdJson as StdJson } from "forge-std/StdJson.sol";

contract DeployLiquidatorsScript is BaseScript {
    using StdJson for string;

    string configPath = "./deployment-config/01_DeployLiquidators.json";
    string config = vm.readFile(configPath);

    function run()
        public
        broadcast
        returns (
            WeEthCurveLiquidator weEthCurveLiquidator,
            WeEthUniswapLiquidator weEthUniswapLiquidator
        )
    {
        Liquidation liquidation = Liquidation(config.readAddress(".liquidation"));
        address treasury = config.readAddress(".treasury");
        bytes32 salt1 = config.readBytes32(".salt1");
        bytes32 salt2 = config.readBytes32(".salt2");

        bytes memory curveLiquidatorInitCode = type(WeEthCurveLiquidator).creationCode;

        weEthCurveLiquidator = WeEthCurveLiquidator(
            CREATEX.deployCreate3(salt1, abi.encodePacked(curveLiquidatorInitCode, abi.encode(liquidation, treasury)))
        );

        bytes memory uniswapLiquidatorInitCode = type(WeEthUniswapLiquidator).creationCode;

        weEthUniswapLiquidator = WeEthUniswapLiquidator(
            CREATEX.deployCreate3(salt2, abi.encodePacked(uniswapLiquidatorInitCode, abi.encode(liquidation, treasury)))
        );
    }
}
