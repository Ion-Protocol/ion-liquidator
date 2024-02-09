// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidatorForkSharedSetup } from "../../helpers/weeth/LiquidatorForkSharedSetup.sol";

import { LiquidationTypeArgs } from "../../../src/helpers/LiquidationHelpers.sol";
import { WeEthCurveLiquidator } from "../../../src/WeEthCurveLiquidator.sol";

import { MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";
import { WSTETH_ADDRESS, WETH_ADDRESS, WEETH_ADDRESS } from "@ionprotocol/src/Constants.sol";
import { EtherFiLibrary } from "@ionprotocol/src/libraries/EtherFiLibrary.sol";
import { IWeEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";
import { GemJoin } from "@ionprotocol/src/join/GemJoin.sol";

import { ISwapRouter } from "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";
import { ICurvePool } from "../../../src/interfaces/ICurvePool.sol";

contract WeEthCurveLiquidatorTest is LiquidatorForkSharedSetup {
    using EtherFiLibrary for IWeEth;

    ISwapRouter internal constant UNISWAP_V3_ROUTER = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);
    ICurvePool internal constant WEETH_WETH_CURVE_POOL = ICurvePool(0x13947303F63b363876868D070F14dc865C36463b);

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

        // Pump WEETH price
        WETH_ADDRESS.approve(address(WEETH_WETH_CURVE_POOL), type(uint256).max);
        (success,) = address(WETH_ADDRESS).call{ value: 400e18 }("");
        require(success, "Weth mint failed for pump");
        WEETH_WETH_CURVE_POOL.exchange(1, 0, 400e18, 0);

        weEthCurveLiquidator.liquidate(address(this), WEETH_ADDRESS, gemJoins[0], address(WEETH_WETH_CURVE_POOL), false);
    }
}
