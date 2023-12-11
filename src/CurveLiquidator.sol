// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";
import { ICurvePool } from "./interfaces/ICurvePool.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { Liquidator } from "./Liquidator.sol";

contract CurveLiquidator is Liquidator {
    constructor(
        IonPool ionPool,
        Liquidation liquidation,
        IWETH9 weth,
        address _treasury
    )
        Liquidator(ionPool, liquidation, weth, _treasury)
    { }

    function _executeSwap(
        bytes32 poolId,
        IERC20 collateralToken,
        uint256 amountCollateralToSwap,
        bool wethIsToken0
    )
        internal
        virtual
        override
        returns (uint256)
    {
        address poolToSwap = address(uint160(uint256(poolId)));
        ICurvePool pool = ICurvePool(poolToSwap);

        (int128 i, int128 j) = wethIsToken0 ? (int128(1), int128(0)) : (int128(0), int128(1));
        collateralToken.approve(poolToSwap, amountCollateralToSwap);

        return pool.exchange(i, j, amountCollateralToSwap, 0);
    }
}
