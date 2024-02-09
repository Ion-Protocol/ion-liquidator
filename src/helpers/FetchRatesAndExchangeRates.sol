// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { ReserveOracle } from "@ionprotocol/src/oracles/reserve/ReserveOracle.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";

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
    constructor(Liquidation liquidationContract, IonPool ionPool) {
        ReserveOracle reserveOracle = ReserveOracle(liquidationContract.RESERVE_ORACLE());
        IlkRateData memory ilkData =
            IlkRateData({ exchangeRate: reserveOracle.currentExchangeRate(), ilkRate: ionPool.rate(0) });

        bytes memory abiEncodedReturnData = abi.encode(ilkData);

        assembly {
            let ptr := add(abiEncodedReturnData, 0x20)
            return(ptr, sub(msize(), ptr))
        }
    }
}
