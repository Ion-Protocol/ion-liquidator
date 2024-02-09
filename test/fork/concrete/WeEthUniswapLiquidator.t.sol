// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidatorForkSharedSetup } from "../../helpers/weeth/LiquidatorForkSharedSetup.sol";

import { LiquidationTypeArgs } from "../../../src/helpers/LiquidationHelpers.sol";

import { MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";
import { WSTETH_ADDRESS, WETH_ADDRESS, WEETH_ADDRESS } from "@ionprotocol/src/Constants.sol";
import { EtherFiLibrary } from "@ionprotocol/src/libraries/EtherFiLibrary.sol";
import { IWeEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";

import { ISwapRouter } from "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import { IUniswapV3Pool } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";

contract WeEthUniswapLiquidatorTest is LiquidatorForkSharedSetup {
    using EtherFiLibrary for IWeEth;

    ISwapRouter internal constant UNISWAP_V3_ROUTER = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);
    IUniswapV3Pool internal constant WEETH_WETH_UNISWAP_POOL =
        IUniswapV3Pool(0x7A415B19932c0105c82FDB6b720bb01B0CC2CAe3);

    function test_liquidate() external {
        uint8 ilkIndex = 0;
        uint256 collateralAmount = 1e18;
        uint256 normalizedBorrowAmount = 0.87e18;

        gemJoins[ilkIndex].join(address(this), collateralAmount);
        ionPool.depositCollateral(ilkIndex, address(this), address(this), collateralAmount, new bytes32[](0));
        ionPool.borrow(ilkIndex, address(this), address(this), normalizedBorrowAmount, new bytes32[](0));

        LiquidationTypeArgs memory args = LiquidationTypeArgs({
            collateralAmount: collateralAmount,
            normalizedDebt: normalizedBorrowAmount,
            rate: ionPool.rate(ilkIndex),
            maxDiscount: liquidation.MAX_DISCOUNT(),
            baseDiscount: liquidation.BASE_DISCOUNT(),
            targetHealth: liquidation.TARGET_HEALTH(),
            dust: ionPool.dust(ilkIndex),
            liquidationThreshold: liquidation.LIQUIDATION_THRESHOLD(),
            negativeB: 0,
            bSquared: 0,
            fourAc: 0,
            totalDebt: 0
        });

        (uint256 partialLiqudiationBound,, uint256 protocolLiquidationBound) =
            liquidationHelpers.getLiquidationTypeBounds(args);

        uint256 partialLiquidationExchangeRate = (partialLiqudiationBound + protocolLiquidationBound) / 2;

        MockReserveOracle(reserveOracles[ilkIndex]).setExchangeRate(partialLiquidationExchangeRate);

        (bool success,) = address(WSTETH_ADDRESS).call{ value: 2000e18 }("");
        require(success, "WstEth mint failed for dump");
        ISwapRouter.ExactInputSingleParams memory dumpWethWstEthParams = ISwapRouter.ExactInputSingleParams({
            tokenIn: address(WSTETH_ADDRESS),
            tokenOut: address(WETH_ADDRESS),
            fee: 100,
            recipient: address(this),
            deadline: block.timestamp + 100,
            amountIn: WSTETH_ADDRESS.balanceOf(address(this)) - 20e18,
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        WSTETH_ADDRESS.approve(address(UNISWAP_V3_ROUTER), type(uint256).max);
        UNISWAP_V3_ROUTER.exactInputSingle(dumpWethWstEthParams);

        weEthUniswapLiquidator.liquidate(
            address(this), WEETH_ADDRESS, gemJoins[0], address(WEETH_WETH_UNISWAP_POOL), true
        );
    }
}
