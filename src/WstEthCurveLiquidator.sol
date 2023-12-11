// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IWstEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";
import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { CurveLiquidator } from "./CurveLiquidator.sol";

/**
 * @dev Since curve only has stEth pool, WstEth will need to be wrapped and
 * unwrapped to be able to use curve as a swap exchange.
 */
contract WstEthCurveLiquidator is CurveLiquidator {
    IWstEth internal immutable WSTETH;

    constructor(
        IonPool ionPool,
        Liquidation liquidation,
        IWETH9 weth,
        IWstEth wstEth,
        IERC20 stEth,
        address _treasury
    )
        CurveLiquidator(ionPool, liquidation, weth, _treasury)
    {
        WSTETH = wstEth;
        // Approve Curve stEth pool
        stEth.approve(0xDC24316b9AE028F1497c275EB9192a3Ea0f67022, type(uint256).max);
    }

    function _executeSwap(
        bytes32 poolToSwap,
        IERC20 collateralToken,
        uint256 amountCollateralToSwap,
        bool wethIsToken0
    )
        internal
        override
        returns (uint256)
    {
        uint256 stEthAmountOut = WSTETH.unwrap(amountCollateralToSwap);

        uint256 ethAmountOut = CurveLiquidator._executeSwap(poolToSwap, collateralToken, stEthAmountOut, wethIsToken0);

        WETH.deposit{ value: ethAmountOut }();

        return ethAmountOut;
    }

    receive() external payable { }
}
