// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { UniswapV3Liquidator } from "../../../src/UniswapV3Liquidator.sol";
import { LiquidatorForkSharedSetup } from "../../helpers/LiquidatorForkSharedSetup.sol";

import { MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract UniswapV3LiquidatorTest is LiquidatorForkSharedSetup {
    function test_liquidate() external {
        // LTV in the tests are 100% and liquidation thresholds are 95%. So a
        // fully borrowed position will be immediately liquidatable if spot
        // price and reserve price were the same. So setting the reserve
        // price to the spot price will make the position liquidatable.

        for (uint8 i = 0; i < ionPool.ilkCount(); i++) {
            // Create the 100% LTV position
            gemJoins[i].join(address(this), 1e18);
            ionPool.depositCollateral(i, address(this), address(this), 1e18, new bytes32[](0));
            ionPool.borrow(i, address(this), address(this), 0.97e18, new bytes32[](0));

            // Move reserve price downwards to 1
            MockReserveOracle(reserveOracles[i]).setExchangeRate(1e18);

            // Now position is liquidatable since debt to collateral ratio is
            // 100% but should never be more than 95%.
        }

        // WstEth liquidation
        uniswapV3Liquidator.liquidate({
            ilkIndex: 0,
            user: address(this),
            collateralToken: _getCollaterals()[0],
            gemJoin: gemJoins[0],
            poolId: bytes32(uint256(uint160(address(WSTETH_WETH_POOL)))),
            wethIsToken0: false
        });
    }
}
