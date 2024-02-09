// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { Liquidation } from "@ionprotocol/src/Liquidation.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { UniswapFlashswapBaseLiquidator } from "./UniswapFlashswapBaseLiquidator.sol";
import { ICurvePool } from "./interfaces/ICurvePool.sol";

contract WeEthCurveLiquidator is UniswapFlashswapBaseLiquidator {
    constructor(Liquidation _liquidation, address _treasury) UniswapFlashswapBaseLiquidator(_liquidation, _treasury) { }

    function _executeSecondSwap(
        address pool,
        IERC20 collateralToken,
        uint256 amountCollateralToSwap,
        bool wethIsToken0
    )
        internal
        override
        returns (uint256 wethOutput)
    {
        // Swap WEETH to ETH
        // This will be an exact input swap... swap all the input for as much
        // WETH as possible.
        ICurvePool curvePool = ICurvePool(pool);

        (int128 i, int128 j) = wethIsToken0 ? (int128(1), int128(0)) : (int128(0), int128(1));
        collateralToken.approve(pool, amountCollateralToSwap);

        return curvePool.exchange(i, j, amountCollateralToSwap, 0);
    }
}
