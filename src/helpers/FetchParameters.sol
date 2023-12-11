// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LibString } from "solady/src/utils/LibString.sol";

import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IonPool } from "@ionprotocol/src/IonPool.sol";

struct IlkParameterData {
    uint256 liquidationThresholds;
    uint256 maxDiscounts;
    uint256 dusts;
}

struct ParameterData {
    uint256 baseDiscount;
    uint256 targetHealth;
}

/**
 * @dev This contract is not meant for production use. It is meant to be used by
 * an offchain program that deploys it through `eth_call` and parses the return
 * data. This purely for data fetching purposes.
 */
contract FetchParameters {
    using LibString for *;

    constructor(uint8 ilkAmounts, IonPool ionPool, Liquidation liquidationContract) {
        IlkParameterData[] memory ilkParameterData = new IlkParameterData[](ilkAmounts);

        for (uint8 i = 0; i < ilkAmounts; i++) {
            string memory liquidationThresholdSignature =
                string("LIQUIDATION_THRESHOLD_").concat(i.toString()).concat("()");
            (bool success, bytes memory liquidationThresholdData) =
                address(liquidationContract).staticcall(abi.encodeWithSignature(liquidationThresholdSignature));
            require(success, "FetchParameters: failed to fetch liquidation threshold");

            uint256 liquidationThreshold = abi.decode(liquidationThresholdData, (uint256));

            string memory maxDiscountSignature = string("MAX_DISCOUNT_").concat(i.toString()).concat("()");
            (bool success2, bytes memory maxDiscountData) =
                address(liquidationContract).staticcall(abi.encodeWithSignature(maxDiscountSignature));
            require(success2, "FetchParameters: failed to fetch max discount");

            uint256 maxDiscount = abi.decode(maxDiscountData, (uint256));

            ilkParameterData[i].liquidationThresholds = liquidationThreshold;
            ilkParameterData[i].maxDiscounts = maxDiscount;
            ilkParameterData[i].dusts = ionPool.dust(i);
        }

        uint256 baseDiscount = liquidationContract.BASE_DISCOUNT();
        uint256 targetHealth = liquidationContract.TARGET_HEALTH();

        ParameterData memory parameterData = ParameterData({ baseDiscount: baseDiscount, targetHealth: targetHealth });

        bytes memory abiEncodedReturnData = abi.encode(ilkParameterData, parameterData);

        assembly {
            let ptr := add(abiEncodedReturnData, 0x20)
            return(ptr, sub(msize(), ptr))
        }
    }
}
