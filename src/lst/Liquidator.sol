// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { IonPool } from "@ionprotocol/src/IonPool.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";
import { GemJoin } from "@ionprotocol/src/join/GemJoin.sol";

import { IVault, IERC20 as IERC20Balancer } from "@balancer-labs/v2-interfaces/contracts/vault/IVault.sol";
import { IFlashLoanRecipient } from "@balancer-labs/v2-interfaces/contracts/vault/IFlashLoanRecipient.sol";

import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

IVault constant VAULT = IVault(0xBA12222222228d8Ba445958a75a0704d566BF2C8);

abstract contract Liquidator is IFlashLoanRecipient {
    using SafeERC20 for IWETH9;

    IonPool internal immutable POOL;
    Liquidation internal immutable LIQUIDATION;
    IWETH9 internal immutable WETH;
    address internal immutable TREASURY;

    constructor(IonPool _ionPool, Liquidation _liquidation, IWETH9 _weth, address _treasury) {
        POOL = _ionPool;
        LIQUIDATION = _liquidation;
        WETH = _weth;
        TREASURY = _treasury;

        WETH.approve(address(LIQUIDATION), type(uint256).max);
    }

    /**
     * @dev It is possible for a liquidation tx to be frontrun by a user
     * repaying debt or depositing more collateral. If the vault is no longer
     * liquidatable, then the liquidation will fail at the `Liquidation`
     * contract layer. If the vault is safer but still liquidatable, then the tx
     * will only succeed if the transaction is profitable (not accounting for
     * gas).
     * @param ilkIndex of the vault to be liquidated
     * @param user owner of the vault.
     * @param collateralToken `collateralReward` amount to be received
     * @param poolId For Balancer, a bytes32 field is necessary to
     * identify the pool. However, for Curve and Uniswap, simply packing an
     * address into the bytes32 will suffice.
     */
    function liquidate(
        uint8 ilkIndex,
        address user,
        IERC20 collateralToken,
        GemJoin gemJoin,
        bytes32 poolId,
        bool wethIsToken0
    )
        external
    {
        IERC20Balancer[] memory addresses = new IERC20Balancer[](1);
        addresses[0] = IERC20Balancer(address(WETH));

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = LIQUIDATION.getRepayAmt(ilkIndex, user);

        VAULT.flashLoan(
            IFlashLoanRecipient(address(this)),
            addresses,
            amounts,
            abi.encode(ilkIndex, user, collateralToken, gemJoin, poolId, wethIsToken0)
        );
    }

    function receiveFlashLoan(
        IERC20Balancer[] memory,
        uint256[] memory amounts,
        uint256[] memory,
        bytes memory userData
    )
        external
        override
    {
        uint256 repayAmount = amounts[0];

        (uint8 ilkIndex, address user, IERC20 collateralToken, GemJoin gemJoin, bytes32 poolId, bool wethIsToken0) =
            abi.decode(userData, (uint8, address, IERC20, GemJoin, bytes32, bool));

        uint256 collateralBefore = POOL.gem(ilkIndex, address(this));
        LIQUIDATION.liquidate(ilkIndex, user, address(this));
        uint256 collateralReward = POOL.gem(ilkIndex, address(this)) - collateralBefore;
        gemJoin.exit(address(this), collateralReward);

        // We skip slippage control here depending on lack of profitability to
        // cause a revert and using flashbots to prevent frontrunning.
        uint256 wethOutput = _executeSwap(poolId, collateralToken, collateralReward, wethIsToken0);

        // Underflow revert desired as it ensures profitability.
        uint256 profit = wethOutput - repayAmount;

        WETH.safeTransfer(address(VAULT), repayAmount);
        WETH.safeTransfer(address(TREASURY), profit);
    }

    /**
     *
     * @param poolId For Balancer, a bytes32 field is necessary to
     * identify the pool. However, for Curve and Uniswap, simply packing an
     * address into the bytes32 will suffice.
     */
    function _executeSwap(
        bytes32 poolId,
        IERC20 collateralToken,
        uint256 amountCollateralToSwap,
        bool wethIsToken0
    )
        internal
        virtual
        returns (uint256 wethOutput);
}
