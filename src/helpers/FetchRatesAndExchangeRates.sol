// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LibString } from "solady/src/utils/LibString.sol";

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { ReserveOracle } from "@ionprotocol/src/oracles/reserve/ReserveOracle.sol";

struct IlkRateData {
    uint256 exchangeRate;
    uint256 ilkRate;
}

/**
 * @dev This contract is not meant for production use. It is meant to be used by
 * an offchain program that deploys it through `eth_call` and parses the return
 * data. This purely for data fetching purposes.
 */
contract FetchRatesAndExchangeRates {
    using LibString for *;

    constructor(uint8 amountIlks, address liquidationContract, IonPool ionPool) {
        // We want to accrue interest before getting `rate` values. However, we
        // don't want this to revert if interest accrual is paused.
        try ionPool.accrueInterest() { } catch { }

        IlkRateData[] memory ilkData = new IlkRateData[](amountIlks);

        for (uint8 i = 0; i < amountIlks; i++) {
            string memory signature = string("RESERVE_ORACLE_").concat(i.toString()).concat("()");

            (bool success, bytes memory data) = liquidationContract.staticcall(abi.encodeWithSignature(signature));
            require(success, "FetchRatesAndExchangeRates: failed to fetch reserve oracle");

            (ReserveOracle reserveOracle) = abi.decode(data, (ReserveOracle));
            ilkData[i] = IlkRateData({ exchangeRate: reserveOracle.currentExchangeRate(), ilkRate: ionPool.rate(i) });
        }

        bytes memory abiEncodedReturnData = abi.encode(ilkData);

        assembly {
            let ptr := add(abiEncodedReturnData, 0x20)
            return(ptr, sub(msize(), ptr))
        }
    }
}
