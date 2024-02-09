// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { BaseScript } from "@ionprotocol/script/Base.s.sol";
import { WETH_ADDRESS, WSTETH_ADDRESS } from "@ionprotocol/src/Constants.sol";

import { ISwapRouter } from "@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol";

import { ICurvePool } from "../src/interfaces/ICurvePool.sol";

contract TestSwapScript is BaseScript {
    ISwapRouter internal constant UNISWAP_V3_ROUTER = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);

    function run() public broadcast {
        (bool success,) = address(WSTETH_ADDRESS).call{ value: 2000e18 }("");
        require(success, "WstEth mint failed for dump");
        ISwapRouter.ExactInputSingleParams memory dumpWethWstEthParams = ISwapRouter.ExactInputSingleParams({
            tokenIn: address(WSTETH_ADDRESS),
            tokenOut: address(WETH_ADDRESS),
            fee: 100,
            recipient: address(this),
            deadline: block.timestamp + 1000000,
            amountIn: WSTETH_ADDRESS.balanceOf(broadcaster),
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        WSTETH_ADDRESS.approve(address(UNISWAP_V3_ROUTER), type(uint256).max);
        UNISWAP_V3_ROUTER.exactInputSingle(dumpWethWstEthParams);

        ICurvePool weethCurvePool = ICurvePool(0x13947303F63b363876868D070F14dc865C36463b);

        WETH_ADDRESS.approve(address(weethCurvePool), type(uint256).max);
        WETH_ADDRESS.deposit{ value: 400e18 }();
        weethCurvePool.exchange(1, 0, 400e18, 0);
    }
}
