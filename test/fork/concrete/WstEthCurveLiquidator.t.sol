// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidatorForkSharedSetup } from "../../helpers/lrt/LiquidatorForkSharedSetup.sol";

import { LiquidationTypeArgs } from "../../../src/helpers/LiquidationHelpers.sol";
import { ICurvePool } from "../../../src/interfaces/ICurvePool.sol";

import { MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract WstEthCurveLiquidatorTest is LiquidatorForkSharedSetup {
    ICurvePool internal immutable STETH_CURVE_POOL = ICurvePool(0xDC24316b9AE028F1497c275EB9192a3Ea0f67022);

    function test_liquidate() external {
        uint8 ilkIndex = 0;
        uint256 collateralAmount = 1e18;
        uint256 normalizedBorrowAmount = 0.97e18;

        gemJoins[ilkIndex].join(address(this), collateralAmount);
        ionPool.depositCollateral(ilkIndex, address(this), address(this), collateralAmount, new bytes32[](0));
        ionPool.borrow(ilkIndex, address(this), address(this), 0.97e18, new bytes32[](0));

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

        vm.deal(address(this), 47_000 ether);
        STETH_CURVE_POOL.exchange{ value: 46_000 ether }(0, 1, 46_000 ether, 0);

        wstEthCurveLiquidator.liquidate(
            ilkIndex,
            address(this),
            IERC20(address(WSTETH)),
            gemJoins[ilkIndex],
            bytes32(uint256(uint160(address(STETH_CURVE_POOL)))),
            true
        );
    }
}
