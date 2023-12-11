// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidationHelpers, LiquidationTypeArgs } from "../../../src/helpers/LiquidationHelpers.sol";

import { LiquidationHelpersSharedSetup } from "../../helpers/LiquidationHelpersSharedSetup.sol";

import { WadRayMath } from "@ionprotocol/src/libraries/math/WadRayMath.sol";

contract LiqudationHelpers_FuzzTest is LiquidationHelpersSharedSetup {
    using WadRayMath for uint256;

    struct Locs {
        uint256 collateralAmount;
        uint256 normalizedDebt;
        uint256 rate;
        uint256 maxDiscount;
        uint256 baseDiscount;
        uint256 targetHealth;
        uint256 dust;
        uint256 liquidationThreshold;
    }

    function testFuzz_GetLiquidationTypeBounds(
        uint256 collateralAmount,
        uint256 normalizedDebt,
        uint256 rate,
        uint256 baseDiscount,
        uint256 dust
    )
        public
    {
        // Let's assume a very conservative exchange rate of 10
        $.normalizedDebt = bound(normalizedDebt, 1e18, type(uint104).max);
        $.collateralAmount = bound(collateralAmount, $.normalizedDebt, $.normalizedDebt * 10);
        $.rate = bound(rate, 1e27, 100e27);
        $.baseDiscount = bound(baseDiscount, 1, 6) * 0.01e27;
        $.dust = bound(dust, 0, 10) * 1e45;

        (uint256 partialLiquidationBound, uint256 fullLiquidationBound, uint256 protocolLiquidationBound) =
            liquidationHelpers.getLiquidationTypeBounds($);

        _checkValidBounds(partialLiquidationBound, fullLiquidationBound, protocolLiquidationBound);
    }

    function test_GetLiquidationTypeBounds1() public {
        $.normalizedDebt = 300_140_319_009_334_513;
        $.collateralAmount = 497_668_533_102_280_745;
        $.rate = 1_000_000_000_000_000_000_000_000_000;
        $.baseDiscount = 0;
        $.dust = 0;

        (uint256 partialLiquidationBound, uint256 fullLiquidationBound, uint256 protocolLiquidationBound) =
            liquidationHelpers.getLiquidationTypeBounds($);

        _checkValidBounds(partialLiquidationBound, fullLiquidationBound, protocolLiquidationBound);
    }
}
