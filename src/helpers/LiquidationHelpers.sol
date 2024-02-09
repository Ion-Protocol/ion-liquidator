// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { RAY, WadRayMath } from "@ionprotocol/src/libraries/math/WadRayMath.sol";

import { ud60x18 } from "prb-math/UD60x18.sol";

import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import { Math } from "@openzeppelin/contracts/utils/math/Math.sol";

struct LiquidationTypeArgs {
    uint256 collateralAmount; // [WAD]
    uint256 normalizedDebt; // [WAD]
    uint256 rate; // [RAY]
    uint256 maxDiscount; // [RAY]
    uint256 baseDiscount; // [RAY]
    uint256 targetHealth; // [RAY]
    uint256 dust; // [RAD]
    uint256 liquidationThreshold; // [RAY]
    uint256 negativeB;
    uint256 bSquared;
    int256 fourAc;
    uint256 totalDebt;
}

/**
 * @notice This contract contains a set of helper function that may be helpful
 * for designing, operating, and testing a liquidator bot on Ion Protocol.
 */
contract LiquidationHelpers {
    using SafeCast for uint256;
    using WadRayMath for uint256;
    using Math for uint256;

    /**
     * @notice A helper function to calculate the health ratio of a vault.
     * @param collateralAmount The amount of collateral in the vault. [WAD]
     * @param exchangeRate The current exchange rate of the collateral. [RAY]
     * @param liquidationThreshold The liquidationThreshold as configured in the Liquidation contract. [RAY]
     * @param totalDebt The total amount of debt in the vault. [RAD]
     */
    function getHealthRatio(
        uint256 collateralAmount,
        uint256 exchangeRate,
        uint256 liquidationThreshold,
        uint256 totalDebt
    )
        public
        pure
        returns (uint256)
    {
        uint256 collateralValue = (collateralAmount * exchangeRate).rayMulDown(liquidationThreshold);
        return collateralValue.radDivDown(totalDebt);
    }

    /**
     *
     * @param collateralAmount The amount of collateral in the vault. [WAD]
     * @param normalizedDebt The amount of normalized debt in the vault. [WAD]
     * @param rate The current rate of the collateral. [RAY]
     * @param exchangeRate The current exchange rate of the collateral. [WAD]
     * @param maxDiscount The maxDiscount as configured in the Liquidation contract. [RAY]
     * @param baseDiscount The baseDiscount as configured in the Liquidation contract. [RAY]
     * @param targetHealth THe targetHealth as configured in the Liquidation contract. [RAY]
     * @param liquidationThreshold The liquidationThreshold as configured in the Liquidation contract. [RAY]
     * @return repayWad The amount of debt to repay by the liquidator. [WAD]
     * @return repayRad The amount of debt to repay by the liquidator in RAD. [RAD]
     */
    function getRepayAmount(
        uint256 collateralAmount,
        uint256 normalizedDebt,
        uint256 rate,
        uint256 exchangeRate,
        uint256 maxDiscount,
        uint256 baseDiscount,
        uint256 targetHealth,
        uint256 liquidationThreshold
    )
        public
        pure
        returns (uint256 repayWad, uint256 repayRad)
    {
        exchangeRate = exchangeRate.scaleUpToRay(18);

        uint256 collateralValue = (collateralAmount * exchangeRate).rayMulDown(liquidationThreshold);

        uint256 totalDebt = normalizedDebt * rate;
        if (totalDebt == 0) return (0, 0);
        uint256 healthRatio = collateralValue.rayDivDown(totalDebt); // round down in protocol favor

        if (healthRatio >= RAY) return (0, 0);

        uint256 discount = baseDiscount + (RAY - healthRatio); // [ray] + ([ray] - [ray])
        discount = discount <= maxDiscount ? discount : maxDiscount;

        uint256 debtValue = normalizedDebt * rate;

        uint256 repayNum = debtValue.rayMulUp(targetHealth) - collateralValue; // [rad] - [rad] = [rad]
        uint256 repayDen = targetHealth - liquidationThreshold.rayDivUp(RAY - discount); // [ray]
        repayRad = repayNum.rayDivUp(repayDen); // [rad] * RAY / [ray] = [rad]

        repayWad = repayRad / RAY;
        if (repayRad % RAY != 0) ++repayWad;
    }

    /**
     * @notice A helper function to calculate the amount of collateral that will
     * be rewarded to the liquidator and the amount of debt that must be repaid
     * by the liquidator.
     * @param collateralAmount Amount of collateral in vault. [WAD]
     * @param normalizedDebt Amount of normalized debt in vault. [WAD]
     * @param rate The current rate of the collateral. [RAY]
     * @param exchangeRate The current exchange rate of the collateral. [RAY]
     * @param maxDiscount The maxDiscount as configured in the Liquidation contract. [RAY]
     * @param baseDiscount The baseDiscount as configured in the Liquidation contract. [RAY]
     * @param targetHealth THe targetHealth as configured in the Liquidation contract. [RAY]
     * @param liquidationThreshold The liquidationThreshold as configured in the Liquidation contract. [RAY]
     * @param dust The `dust` parameter for the collateral. [RAD]
     * @return repayWad Amount of debt to repay. [WAD]
     * @return collateralRewardAmount Amount of collateral that will be reward to liquidator. [WAD]
     */
    function getRepayAndCollateralRewardAmount(
        uint256 collateralAmount,
        uint256 normalizedDebt,
        uint256 rate,
        uint256 exchangeRate,
        uint256 maxDiscount,
        uint256 baseDiscount,
        uint256 targetHealth,
        uint256 liquidationThreshold,
        uint256 dust
    )
        external
        pure
        returns (uint256 repayWad, uint256 collateralRewardAmount)
    {
        uint256 repayRad;
        (repayWad, repayRad) = getRepayAmount(
            collateralAmount,
            normalizedDebt,
            rate,
            exchangeRate,
            maxDiscount,
            baseDiscount,
            targetHealth,
            liquidationThreshold
        );

        exchangeRate = exchangeRate.scaleUpToRay(18);

        uint256 collateralValue = (collateralAmount * exchangeRate).rayMulDown(liquidationThreshold);

        uint256 totalDebt = normalizedDebt * rate;
        if (totalDebt == 0) return (0, 0);
        uint256 healthRatio = collateralValue.rayDivDown(totalDebt); // round down in protocol favor

        if (healthRatio >= RAY) return (0, 0);

        uint256 discount = baseDiscount + (RAY - healthRatio); // [RAY] + ([RAY] - [RAY])
        discount = discount <= maxDiscount ? discount : maxDiscount;
        uint256 price = exchangeRate.rayMulUp(RAY - discount); // [RAY]

        if (repayRad > totalDebt) {
            return (0, 0);
        } else if (totalDebt < dust + repayRad) {
            collateralRewardAmount = totalDebt / price; // [WAD]
        } else {
            collateralRewardAmount = repayRad / price; // [WAD]
        }
    }

    /**
     * @notice Ion's liquidation module has three types of liquidation that can
     * occur dependent on the health of the vault. This is a helper function to
     * compute the bounds at which each type of liquidation would occur,
     * provided the relevant state of the vault and protocol.
     * @dev The initial calculations of the bounds contain rounding errors.
     * Instead of re-implementing Math functions with proper rounding behaviour,
     * the rounding error is brute-forced away.
     * @param locs Local variables.
     * @return partialLiquidationBound The exchange rate which a partial
     * liquidation would occur. Adding 1 to this value would result in the vault
     * being healthy (and thus, not liquidatable). If this value is
     * `type(uint256).max`, then a partial liquidation is not possible.
     * @return fullLiquidationBound The exchange rate which a full liquidation
     * would occur. Adding 1 to this value would result in a partial
     * liquidation, if possible. Otherwise, adding 1 would result in the vault
     * being healthy. If this value is `type(uint256).max`, then a full
     * liquidation is not possible.
     * @return protocolLiquidationBound The exchange rate which a protocol liquidation
     * would occur. Adding 1 to this value would result in a full liquidation,
     * if possible. If a full liquidation is not possible but a partial one is,
     * then adding 1 would result in a partial liquidation. If a partial
     * liquidation isn't possible either, then adding 1 would result in the
     * vault being healthy. If this value is `type(uint256).max`, then a
     * protocol liquidation is not possible.
     */
    function getLiquidationTypeBounds(LiquidationTypeArgs memory locs)
        external
        pure
        returns (uint256 partialLiquidationBound, uint256 fullLiquidationBound, uint256 protocolLiquidationBound)
    {
        uint256 totalDebt = locs.normalizedDebt * locs.rate;

        if (totalDebt == 0) return (type(uint256).max, type(uint256).max, type(uint256).max);

        partialLiquidationBound =
            totalDebt.radDivDown(locs.collateralAmount * locs.liquidationThreshold).scaleDownToWad(45);

        // Calculate full liquidation bound

        locs.negativeB =
            (totalDebt.radMulDown(locs.baseDiscount) + locs.dust.radMulDown(locs.targetHealth)).scaleDownToWad(27);
        locs.bSquared = locs.negativeB.wadMulDown(locs.negativeB);
        locs.fourAc = _calculateFullFourAC(locs, totalDebt);

        uint256 collateralValue =
            (locs.negativeB + ud60x18((uint256(locs.bSquared.toInt256() - locs.fourAc))).sqrt().unwrap()) / 2;
        fullLiquidationBound = collateralValue.wadDivDown(locs.collateralAmount).rayDivDown(locs.liquidationThreshold);

        // Calculate protocol liquidation bound

        locs.negativeB = totalDebt.radMulDown(locs.baseDiscount).scaleDownToWad(27);
        locs.bSquared = locs.negativeB.wadMulDown(locs.negativeB);
        locs.fourAc =
            (4 * (totalDebt.radMulDown(totalDebt).radMulDown(locs.liquidationThreshold).scaleDownToWad(27)).toInt256());

        collateralValue =
            (locs.negativeB + ud60x18(uint256(locs.bSquared.toInt256() + locs.fourAc)).sqrt().unwrap()) / 2;
        protocolLiquidationBound =
            collateralValue.wadDivDown(locs.collateralAmount).rayDivDown(locs.liquidationThreshold);

        // See if partial liquidation bound is possible

        uint256 collateralValueFull =
            (locs.collateralAmount * locs.liquidationThreshold).wadMulDown(fullLiquidationBound).scaleDownToRay(45); // [RAY]
        uint256 healthRatio = collateralValueFull.radDivDown(totalDebt); // [RAY]

        if (healthRatio >= RAY || partialLiquidationBound == fullLiquidationBound) {
            fullLiquidationBound = partialLiquidationBound;
            partialLiquidationBound = type(uint256).max;
        } else {
            fullLiquidationBound = _findRoundedFullLiquidationBound(locs, fullLiquidationBound);

            if (fullLiquidationBound != type(uint256).max) {
                collateralValueFull = locs.collateralAmount * locs.liquidationThreshold.radMulDown(fullLiquidationBound); // [RAY]
                healthRatio = collateralValueFull.radDivDown(totalDebt);

                if (healthRatio >= RAY || partialLiquidationBound == fullLiquidationBound) {
                    fullLiquidationBound = partialLiquidationBound;
                    partialLiquidationBound = type(uint256).max;
                }
            }
        }

        if (partialLiquidationBound == protocolLiquidationBound) partialLiquidationBound = type(uint256).max;

        // See if full liquidation is possible

        uint256 collateralValueProtocol =
            (locs.collateralAmount * locs.liquidationThreshold).wadMulDown(protocolLiquidationBound).scaleDownToRay(45); // [RAY]
        healthRatio = collateralValueProtocol.radDivDown(totalDebt); // [RAY]

        if (healthRatio >= RAY || fullLiquidationBound == protocolLiquidationBound) {
            protocolLiquidationBound = fullLiquidationBound;
            fullLiquidationBound = type(uint256).max;
        } else {
            protocolLiquidationBound = _findRoundedProtocolLiquidationBound(locs, protocolLiquidationBound);
            collateralValueProtocol =
                locs.collateralAmount * locs.liquidationThreshold.radMulDown(protocolLiquidationBound); // [RAY]
            healthRatio = collateralValueProtocol.radDivDown(totalDebt);

            if (healthRatio >= RAY || fullLiquidationBound == protocolLiquidationBound) {
                protocolLiquidationBound = fullLiquidationBound;
                fullLiquidationBound = type(uint256).max;
            }
        }

        if (partialLiquidationBound == protocolLiquidationBound) partialLiquidationBound = type(uint256).max;
    }

    function _calculateFullFourAC(LiquidationTypeArgs memory locs, uint256 totalDebt) internal pure returns (int256) {
        int256 aC = (
            locs.dust.radMulDown(totalDebt).radMulDown(locs.targetHealth).rayMulDown(locs.baseDiscount).toInt256()
                + locs.dust.radMulDown(totalDebt).radMulDown(locs.liquidationThreshold).toInt256()
                - totalDebt.radMulDown(totalDebt).radMulDown(locs.liquidationThreshold).toInt256()
        );

        bool isNegative = aC < 0;

        uint256 aCUint = isNegative ? uint256(-aC) : uint256(aC);

        uint256 fourACUint = aCUint.mulDiv(4, 10 ** (27 - 18));

        return isNegative ? -int256(fourACUint) : int256(fourACUint);
    }

    /**
     * @notice Doing the complex quadratic math in solidity results in rounding
     * errors. Since this contract is not designed to executed on-chain, instead
     * of re-implementing the math functions with proper rounding behaviour, the
     * rounding error is brute-forced away.
     * @param args Arguments.
     * @param unroundedBound The unrounded bound.
     */
    function _findRoundedFullLiquidationBound(
        LiquidationTypeArgs memory args,
        uint256 unroundedBound
    )
        internal
        pure
        returns (uint256)
    {
        for (uint256 i = 0; i < 5; i++) {
            (, uint256 repayRad) = getRepayAmount(
                args.collateralAmount,
                args.normalizedDebt,
                args.rate,
                unroundedBound - 1 + i,
                args.maxDiscount,
                args.baseDiscount,
                args.targetHealth,
                args.liquidationThreshold
            );

            (, uint256 repayRad2) = getRepayAmount(
                args.collateralAmount,
                args.normalizedDebt,
                args.rate,
                unroundedBound + i,
                args.maxDiscount,
                args.baseDiscount,
                args.targetHealth,
                args.liquidationThreshold
            );

            if (!(repayRad > args.normalizedDebt * args.rate) && args.normalizedDebt * args.rate < args.dust + repayRad)
            {
                if (
                    !(repayRad2 > args.normalizedDebt * args.rate)
                        && args.normalizedDebt * args.rate >= args.dust + repayRad2
                ) {
                    return unroundedBound + i - 1;
                }
            }
        }

        return type(uint256).max;
    }

    /**
     * @notice Doing the complex quadratic math in solidity results in rounding
     * errors. Since this contract is not designed to executed on-chain, instead
     * of re-implementing the math functions with proper rounding behaviour, the
     * rounding error is brute-forced away.
     * @param args Arguments.
     * @param unroundedBound The unrounded bound.
     */
    function _findRoundedProtocolLiquidationBound(
        LiquidationTypeArgs memory args,
        uint256 unroundedBound
    )
        internal
        pure
        returns (uint256)
    {
        for (uint256 i = 0; i < 10; i++) {
            (, uint256 repayRad) = getRepayAmount(
                args.collateralAmount,
                args.normalizedDebt,
                args.rate,
                unroundedBound + i,
                args.maxDiscount,
                args.baseDiscount,
                args.targetHealth,
                args.liquidationThreshold
            );

            (, uint256 repayRad2) = getRepayAmount(
                args.collateralAmount,
                args.normalizedDebt,
                args.rate,
                unroundedBound + i + 1,
                args.maxDiscount,
                args.baseDiscount,
                args.targetHealth,
                args.liquidationThreshold
            );

            if (repayRad > args.normalizedDebt * args.rate) {
                if (!(repayRad2 > args.normalizedDebt * args.rate)) return unroundedBound + i;
            }
        }

        revert("Could not find rounded protocol liquidation bound");
    }
}
