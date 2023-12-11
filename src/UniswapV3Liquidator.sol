// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";

import { IUniswapV3Pool } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import { IUniswapV3SwapCallback } from "@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol";

import { IERC20 } from "@openzeppelin/contracts/interfaces/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import { Liquidator } from "./Liquidator.sol";

contract UniswapV3Liquidator is Liquidator, IUniswapV3SwapCallback {
    using SafeERC20 for IERC20;

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4_295_128_739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO = 1_461_446_703_485_210_103_287_273_052_203_988_822_378_723_970_342;

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
        override
        returns (uint256 wethOutput)
    {
        IUniswapV3Pool pool = IUniswapV3Pool(address(uint160(uint256(poolId))));
        bool zeroForOne = !wethIsToken0;

        (int256 amount0, int256 amount1) = pool.swap(
            address(this),
            zeroForOne,
            int256(amountCollateralToSwap),
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
            abi.encode(collateralToken)
        );

        wethOutput = uint256(zeroForOne ? -amount1 : -amount0);
    }

    /**
     * @dev From the perspective of the pool i.e. Negative amount means pool is
     * sending. This contract should NEVER hold funds. Otherwise, funds can we
     * withdrawn by directly calling this function.
     * @param amount0Delta change in token0
     * @param amount1Delta change in token1
     */
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external override {
        IERC20 collateralToken = abi.decode(data, (IERC20));

        uint256 collateralToSend;
        if (amount0Delta > 0) {
            collateralToSend = uint256(amount0Delta);
        } else if (amount1Delta > 0) {
            collateralToSend = uint256(amount1Delta);
        } else {
            revert("UniswapV3Liquidator: no tokens received");
        }

        collateralToken.safeTransfer(msg.sender, collateralToSend);
    }
}
