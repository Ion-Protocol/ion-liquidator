// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { Liquidation } from "@ionprotocol/src/Liquidation.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import { IUniswapV3Pool } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";

import { UniswapFlashswapBaseLiquidator } from "./UniswapFlashswapBaseLiquidator.sol";

contract WeEthUniswapLiquidator is UniswapFlashswapBaseLiquidator {
    constructor(Liquidation _liquidation, address _treasury) UniswapFlashswapBaseLiquidator(_liquidation, _treasury) { }

    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) public override {
        // First swap callback
        if (data.length != 64) {
            return super.uniswapV3SwapCallback(amount0Delta, amount1Delta, data);
        }

        // Second swap callback
        (IERC20 collateralToken, bool wethIsToken0) = abi.decode(data, (IERC20, bool));

        uint256 collateralToSend = wethIsToken0 ? uint256(amount1Delta) : uint256(amount0Delta);

        collateralToken.transfer(msg.sender, collateralToSend);
    }

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
        (int256 amount0Out, int256 amount1Out) = IUniswapV3Pool(pool).swap(
            address(this),
            !wethIsToken0,
            int256(amountCollateralToSwap),
            wethIsToken0 ? MAX_SQRT_RATIO - 1 : MIN_SQRT_RATIO + 1,
            abi.encode(collateralToken, wethIsToken0)
        );

        return wethIsToken0 ? uint256(-amount0Out) : uint256(-amount1Out);
    }
}
