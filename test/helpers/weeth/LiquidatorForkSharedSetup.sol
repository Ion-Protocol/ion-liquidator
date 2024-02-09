// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { LiquidationHelpers } from "../../../src/helpers/LiquidationHelpers.sol";
import { WeEthCurveLiquidator } from "../../../src/WeEthCurveLiquidator.sol";
import { WeEthUniswapLiquidator } from "../../../src/WeEthUniswapLiquidator.sol";

import { MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";
import { WeEthIonPoolSharedSetup } from "@ionprotocol/test/helpers/weeth/WeEthIonPoolSharedSetup.sol";
import {
    WEETH_ADDRESS,
    EETH_ADDRESS,
    WSTETH_ADDRESS,
    REDSTONE_WEETH_ETH_PRICE_FEED,
    ETH_PER_STETH_CHAINLINK,
    REDSTONE_DECIMALS
} from "@ionprotocol/src/Constants.sol";
import { WadRayMath } from "@ionprotocol/src/libraries/math/WadRayMath.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { EtherFiLibrary } from "@ionprotocol/src/libraries/EtherFiLibrary.sol";
import { IWeEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeCast } from "@openzeppelin/contracts/utils/math/SafeCast.sol";

abstract contract LiquidatorForkSharedSetup is WeEthIonPoolSharedSetup {
    using WadRayMath for uint256;
    using SafeCast for int256;
    using EtherFiLibrary for IWeEth;

    address[] reserveOracles;
    uint256[] maxDiscounts;
    uint256[] liquidationThresholds;

    address internal immutable PROTOCOL = vm.addr(102_103);
    uint256 internal constant TARGET_HEALTH = 1.25e27;
    uint256 internal constant LIQUIDATION_THRESHOLD = 0.95e27;
    uint256 internal constant BASE_DISCOUNT = 0;

    Liquidation liquidation;
    LiquidationHelpers liquidationHelpers;

    WeEthCurveLiquidator weEthCurveLiquidator;
    WeEthUniswapLiquidator weEthUniswapLiquidator;

    function setUp() public virtual override {
        vm.createSelectFork(vm.envString("RPC_URL"), 19_017_314);

        super.setUp();

        liquidationHelpers = new LiquidationHelpers();

        uint256 weEthWstEthExchangeRate = WEETH_ADDRESS.getRate().wadMulDown(WSTETH_ADDRESS.tokensPerStEth());
        address weEthWstEthReserveOracle = address(new MockReserveOracle(weEthWstEthExchangeRate));

        reserveOracles = new address[](1);
        reserveOracles[0] = weEthWstEthReserveOracle;

        maxDiscounts = new uint256[](1);
        maxDiscounts[0] = 0.2e27;

        liquidationThresholds = new uint256[](1);
        liquidationThresholds[0] = LIQUIDATION_THRESHOLD;

        liquidation = new Liquidation(
            address(ionPool),
            PROTOCOL,
            reserveOracles,
            liquidationThresholds,
            TARGET_HEALTH,
            BASE_DISCOUNT,
            maxDiscounts
        );
        ionPool.grantRole(ionPool.LIQUIDATOR_ROLE(), address(liquidation));

        weEthCurveLiquidator = new WeEthCurveLiquidator(liquidation, address(this));
        weEthUniswapLiquidator = new WeEthUniswapLiquidator(liquidation, address(this));

        spotOracles[0].setPrice(_getSpot() / 1e9);

        vm.deal(lender1, INITIAL_LENDER_UNDERLYING_BALANCE);
        vm.deal(lender2, INITIAL_LENDER_UNDERLYING_BALANCE);

        vm.startPrank(lender1);
        (bool success,) = address(WSTETH_ADDRESS).call{ value: INITIAL_LENDER_UNDERLYING_BALANCE }("");
        require(success, "WstEth mint failed");
        WSTETH_ADDRESS.approve(address(ionPool), type(uint256).max);
        ionPool.supply(lender1, WSTETH_ADDRESS.balanceOf(lender1), emptyProof);
        vm.stopPrank();

        vm.startPrank(lender2);
        (success,) = address(WSTETH_ADDRESS).call{ value: INITIAL_LENDER_UNDERLYING_BALANCE }("");
        require(success, "WstEth mint failed");
        WSTETH_ADDRESS.approve(address(ionPool), type(uint256).max);
        ionPool.supply(lender1, WSTETH_ADDRESS.balanceOf(lender2), emptyProof);
        vm.stopPrank();

        EETH_ADDRESS.approve(address(WEETH_ADDRESS), type(uint256).max);
        WEETH_ADDRESS.depositForLrt(20e18);

        IERC20[] memory _collaterals = _getCollaterals();
        for (uint256 i = 0; i < _collaterals.length; i++) {
            _collaterals[i].approve(address(gemJoins[i]), type(uint256).max);
        }
    }

    function _getSpot() internal view override returns (uint256) {
        (, int256 ethPerWeEth,,,) = REDSTONE_WEETH_ETH_PRICE_FEED.latestRoundData();
        (, int256 ethPerStEth,,,) = ETH_PER_STETH_CHAINLINK.latestRoundData();

        uint256 stEthPerWeEth =
            ethPerWeEth.toUint256().scaleUpToWad(REDSTONE_DECIMALS).wadDivDown(ethPerStEth.toUint256()); // [wad]

        uint256 wstEthPerStEth = WSTETH_ADDRESS.tokensPerStEth(); // [wad]
        uint256 weEthWstEthSpot = stEthPerWeEth.wadMulDown(wstEthPerStEth);
        return weEthWstEthSpot * 1e9;
    }
}
