// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;


import { LiquidationHelpersSharedSetup } from "../../helpers/LiquidationHelpersSharedSetup.sol";

import { WadRayMath } from "@ionprotocol/src/libraries/math/WadRayMath.sol";

import { safeconsole as console } from "forge-std/safeconsole.sol";

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
        $.maxDiscount = 0.2e27;
        $.targetHealth = 1.25e27;
        $.dust = 0;
        $.liquidationThreshold = 0.92e27;
        $.normalizedDebt = 2_312_575_154_794_509_358;
        $.collateralAmount = 3_000_000_000_000_000_000;
        $.rate = 1_000_000_057_141_349_455_516_898_485;
        $.baseDiscount = 0;
        $.dust = 0;

        (uint256 partialLiquidationBound, uint256 fullLiquidationBound, uint256 protocolLiquidationBound) =
            liquidationHelpers.getLiquidationTypeBounds($);

        _checkValidBounds(partialLiquidationBound, fullLiquidationBound, protocolLiquidationBound);

        console.log("partialLiquidationBound", partialLiquidationBound);
        console.log("fullLiquidationBound", fullLiquidationBound);
        console.log("protocolLiquidationBound", protocolLiquidationBound);
    }
}
