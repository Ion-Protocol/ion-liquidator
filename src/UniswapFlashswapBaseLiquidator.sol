// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { GemJoin } from "@ionprotocol/src/join/GemJoin.sol";
import { MAINNET_WSTETH_WETH_UNISWAP, WETH_ADDRESS, WSTETH_ADDRESS } from "@ionprotocol/src/Constants.sol";

import { IUniswapV3SwapCallback } from "@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @notice Flashswap from ETH to WSTETH (get WSTETH first) -> liquidate (send
 * WSTETH and receive WEETH) -> swap WEETH to ETH -> send ETH to WSTETH/ETH pool
 * to complete first swap
 */
abstract contract UniswapFlashswapBaseLiquidator is IUniswapV3SwapCallback {
    IonPool internal immutable POOL;
    Liquidation internal immutable LIQUIDATION;
    address internal immutable TREASURY;

    uint160 internal constant MIN_SQRT_RATIO = 4_295_128_739;
    uint160 internal constant MAX_SQRT_RATIO = 1_461_446_703_485_210_103_287_273_052_203_988_822_378_723_970_342;

    constructor(Liquidation _liquidation, address _treasury) {
        LIQUIDATION = _liquidation;
        POOL = _liquidation.POOL();
        TREASURY = _treasury;

        WSTETH_ADDRESS.approve(address(LIQUIDATION), type(uint256).max);
    }

    /**
     *
     * @param user vault to liquidate.
     * @param collateralToken to receive as reward.
     * @param gemJoin of the collateral token.
     * @param pool address on which to swap collateral for ETH.
     * @param wethIsToken0 whether WETH is token0 in the `pool`.
     */
    function liquidate(
        address user,
        IERC20 collateralToken,
        GemJoin gemJoin,
        address pool,
        bool wethIsToken0
    )
        external
    {
        int256 repayAmount = int256(LIQUIDATION.getRepayAmt(0, user));

        // token0 is WSTETH, token1 is ETH
        // We will perform an exact output swap here
        MAINNET_WSTETH_WETH_UNISWAP.swap({
            recipient: address(this),
            // Swap ETH to WSTETH
            zeroForOne: false,
            amountSpecified: -repayAmount,
            sqrtPriceLimitX96: MAX_SQRT_RATIO - 1,
            data: abi.encode(user, collateralToken, gemJoin, pool, wethIsToken0)
        });
    }

    function uniswapV3SwapCallback(int256, int256 amount1Delta, bytes calldata data) public virtual {
        (address user, IERC20 collateralToken, GemJoin gemJoin, address pool, bool wethIsToken0) =
            abi.decode(data, (address, IERC20, GemJoin, address, bool));

        // Send wstETH to Liquidation and receieves weETH in return
        (, uint256 gemOut) = LIQUIDATION.liquidate(0, user, address(this));
        gemJoin.exit(address(this), gemOut);

        uint256 wethOwed = uint256(amount1Delta);

        // We skip all slippage control here depending on lack of profitability to
        // cause a revert and using flashbots to prevent frontrunning.
        uint256 wethOutput = _executeSecondSwap(pool, collateralToken, wethOwed, wethIsToken0);
        uint256 profit = wethOutput - wethOwed;

        WETH_ADDRESS.transfer(msg.sender, wethOwed);
        WETH_ADDRESS.transfer(TREASURY, profit);
    }

    /**
     * @notice Executes the second swap (WEETH -> ETH)
     */
    function _executeSecondSwap(
        address pool,
        IERC20 collateralToken,
        uint256 amountCollateralToSwap,
        bool wethIsToken0
    )
        internal
        virtual
        returns (uint256 wethOutput);
}
