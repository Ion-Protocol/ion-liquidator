// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidationHelpers, LiquidationTypeArgs } from "../../src/helpers/LiquidationHelpers.sol";

import { RAY } from "@ionprotocol/src/libraries/math/WadRayMath.sol";

import { Test } from "forge-std/Test.sol";

contract LiquidationHelpersSharedSetup is Test {
    LiquidationHelpers liquidationHelpers;

    LiquidationTypeArgs $;

    function setUp() public virtual {
        liquidationHelpers = new LiquidationHelpers();

        $.rate = 1e27;
        $.maxDiscount = 0.2e27;
        $.baseDiscount = 0;
        $.targetHealth = 1.25e27;
        $.dust = 0;
        $.liquidationThreshold = 0.92e27;
    }

    function _checkValidBounds(
        uint256 partialLiquidationBound,
        uint256 fullLiquidationBound,
        uint256 protocolLiquidationBound
    )
        internal
    {
        // If partial liquidation can take place
        if (partialLiquidationBound != type(uint256).max) {
            _checkExchangeRateTriggersPartialLiquidation(partialLiquidationBound);

            // Crossing the partial liquidation boundary should mean entering
            // non-liquidatable territory
            _checkExchangeRateIsHealthyVault(partialLiquidationBound + 1);
        }

        // If full liquidation can take place
        if (fullLiquidationBound != type(uint256).max) {
            _checkExchangeRateTriggersFullLiquidation(fullLiquidationBound);

            // This means full liquidation is the first type of liqudation
            // possible. "First" in the sense that is the variant of liquidation
            // requiring the highest exchange rate.
            if (partialLiquidationBound == type(uint256).max) {
                // This means that crossing the boundary will not allow
                // liquidations anymore
                _checkExchangeRateIsHealthyVault(fullLiquidationBound + 1);
            } else {
                // This means partial liquidation is the first type of
                // liquidation possible. So crossing the full liquidation
                // boundary will allow partial liquidations.
                _checkExchangeRateTriggersPartialLiquidation(fullLiquidationBound + 1);
            }

            // // Since full liquidation is the first type of liquidation possible,
            // // crossing the protocol liquidation boundary should mean entering
            // // full liquidation territory.
            _checkExchangeRateTriggersFullLiquidation(protocolLiquidationBound + 1);
            _checkExchangeRateTriggersProtocolLiquidation(protocolLiquidationBound);
        } else {
            // If fullLiquidationBound is max and partialLiquidationBound is
            // also max this means all liquidtable vaults will only be subject
            // to a protocol liquidation. So crossing that boundary should mean
            // vault is no longer liquidatable.
            if (partialLiquidationBound == type(uint256).max) {
                _checkExchangeRateIsHealthyVault(protocolLiquidationBound + 1);
                _checkExchangeRateTriggersProtocolLiquidation(protocolLiquidationBound);
            } else {
                // If fullLiquidationBound is max and partialLiquidationBound is
                // not max this means dust is zero. Protocol liqudation should
                // directly cross into partial liquidation and partial
                // liquidation should cross into healthy territory.
                _checkExchangeRateTriggersPartialLiquidation(protocolLiquidationBound + 1);
                _checkExchangeRateTriggersProtocolLiquidation(protocolLiquidationBound);
            }
        }
    }

    function _checkExchangeRateTriggersPartialLiquidation(uint256 partialLiquidationBound) internal {
        (, uint256 repayRad) = liquidationHelpers.getRepayAmount(
            $.collateralAmount,
            $.normalizedDebt,
            $.rate,
            partialLiquidationBound,
            $.maxDiscount,
            $.baseDiscount,
            $.targetHealth,
            $.liquidationThreshold
        );

        assertFalse(repayRad > $.normalizedDebt * $.rate);
        assertTrue($.normalizedDebt * $.rate >= $.dust + repayRad);
    }

    function _checkExchangeRateTriggersFullLiquidation(uint256 fullLiquidationBound) internal {
        (, uint256 repayRad) = liquidationHelpers.getRepayAmount(
            $.collateralAmount,
            $.normalizedDebt,
            $.rate,
            fullLiquidationBound,
            $.maxDiscount,
            $.baseDiscount,
            $.targetHealth,
            $.liquidationThreshold
        );

        assertFalse(repayRad > $.normalizedDebt * $.rate);
        assertTrue($.normalizedDebt * $.rate < $.dust + repayRad);
    }

    function _checkExchangeRateTriggersProtocolLiquidation(uint256 protocolLiquidationBound) internal {
        (, uint256 repayRad) = liquidationHelpers.getRepayAmount(
            $.collateralAmount,
            $.normalizedDebt,
            $.rate,
            protocolLiquidationBound,
            $.maxDiscount,
            $.baseDiscount,
            $.targetHealth,
            $.liquidationThreshold
        );

        assertTrue(repayRad > $.normalizedDebt * $.rate);
    }

    function _checkExchangeRateIsHealthyVault(uint256 exchangeRate) internal {
        assertGe(
            liquidationHelpers.getHealthRatio(
                $.collateralAmount, exchangeRate, $.liquidationThreshold, $.normalizedDebt * $.rate
            ),
            RAY
        );
    }
}
