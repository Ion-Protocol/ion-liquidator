// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LibString } from "solady/src/utils/LibString.sol";

import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IonPool } from "@ionprotocol/src/IonPool.sol";

struct ParameterData {
    uint256 liquidationThreshold;
    uint256 maxDiscount;
    uint256 dust;
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

    constructor(IonPool ionPool, Liquidation liquidationContract) {
        uint256 liquidationThreshold = liquidationContract.LIQUIDATION_THRESHOLD();
        uint256 maxDiscount = liquidationContract.MAX_DISCOUNT();
        uint256 dust = ionPool.dust(0);
        uint256 baseDiscount = liquidationContract.BASE_DISCOUNT();
        uint256 targetHealth = liquidationContract.TARGET_HEALTH();

        ParameterData memory parameterData = ParameterData({
            liquidationThreshold: liquidationThreshold,
            maxDiscount: maxDiscount,
            dust: dust,
            baseDiscount: baseDiscount,
            targetHealth: targetHealth
        });

        bytes memory abiEncodedReturnData = abi.encode(parameterData);

        assembly {
            let ptr := add(abiEncodedReturnData, 0x20)
            return(ptr, sub(msize(), ptr))
        }
    }
}
