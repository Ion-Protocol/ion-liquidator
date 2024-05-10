// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import { UniswapV3Liquidator } from "../../../src/lst/UniswapV3Liquidator.sol";
import { CurveLiquidator } from "../../../src/lst/CurveLiquidator.sol";
import { WstEthCurveLiquidator } from "../../../src/lst/WstEthCurveLiquidator.sol";
import { LiquidationHelpers } from "../../../src/helpers/LiquidationHelpers.sol";

import { WAD, RAY, WadRayMath } from "@ionprotocol/src/libraries/math/WadRayMath.sol";
import { IWETH9 } from "@ionprotocol/src/interfaces/IWETH9.sol";
import { IWstEth, IStaderStakePoolsManager, ISwEth } from "@ionprotocol/src/interfaces/ProviderInterfaces.sol";
import { IonPoolSharedSetup, MockReserveOracle } from "@ionprotocol/test/helpers/IonPoolSharedSetup.sol";
import { Liquidation } from "@ionprotocol/src/Liquidation.sol";
import { LidoLibrary } from "@ionprotocol/src/libraries/LidoLibrary.sol";
import { StaderLibrary } from "@ionprotocol/src/libraries/StaderLibrary.sol";
import { SwellLibrary } from "@ionprotocol/src/libraries/SwellLibrary.sol";

import { AggregatorV2V3Interface } from "@chainlink/contracts/src/v0.8/interfaces/AggregatorV2V3Interface.sol";

import { IRateProvider } from "@balancer-labs/v2-interfaces/contracts/pool-utils/IRateProvider.sol";

import { IUniswapV3Pool } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol";
import { IUniswapV3Factory } from "@uniswap/v3-core/contracts/interfaces/IUniswapV3Factory.sol";

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

abstract contract LiquidatorForkSharedSetup is IonPoolSharedSetup {
    using WadRayMath for *;
    using LidoLibrary for IWstEth;
    using StaderLibrary for IStaderStakePoolsManager;
    using SwellLibrary for ISwEth;

    uint256 constant INITIAL_THIS_UNDERLYING_BALANCE = 20e18;

    IWETH9 internal immutable WETH = IWETH9(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);

    IERC20 constant STETH = IERC20(0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84);
    IWstEth constant WSTETH = IWstEth(0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0);
    IStaderStakePoolsManager constant STADER = IStaderStakePoolsManager(0xcf5EA1b38380f6aF39068375516Daf40Ed70D299);
    ISwEth constant SWELL = ISwEth(0xf951E335afb289353dc249e82926178EaC7DEd78);

    AggregatorV2V3Interface constant STETH_ETH_CHAINLINK =
        AggregatorV2V3Interface(0x86392dC19c0b719886221c78AB11eb8Cf5c52812);
    IRateProvider constant STADER_POOL = IRateProvider(0x37b18B10ce5635a84834b26095A0AE5639dCB752);
    IUniswapV3Pool constant SWETH_ETH_POOL = IUniswapV3Pool(0x30eA22C879628514f1494d4BBFEF79D21A6B49A2);

    address constant ETHX = 0xA35b1B31Ce002FBF2058D22F30f95D405200A15b;

    IUniswapV3Factory internal constant FACTORY = IUniswapV3Factory(0x1F98431c8aD98523631AE4a59f267346ea31F984);
    IWETH9 constant weth = IWETH9(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);

    IUniswapV3Pool constant WSTETH_WETH_POOL = IUniswapV3Pool(0x109830a1AAaD605BbF02a9dFA7B0B92EC2FB7dAa);

    address internal immutable PROTOCOL = vm.addr(102_103);

    uint256 internal constant TARGET_HEALTH = 1.25e27;
    uint256 internal constant LIQUIDATION_THRESHOLD = 0.95e27;
    uint256 internal constant BASE_DISCOUNT = 0;

    UniswapV3Liquidator uniswapV3Liquidator;
    CurveLiquidator curveLiquidator;
    WstEthCurveLiquidator wstEthCurveLiquidator;

    Liquidation liquidation;
    LiquidationHelpers liquidationHelpers;

    address[] reserveOracles;
    uint256[] maxDiscounts;
    uint256[] liquidationThresholds;

    function setUp() public virtual override {
        vm.createSelectFork(vm.envString("RPC_URL"), 19_017_314);

        super.setUp();
        // Launch Liquidation contract

        uint256 wstEthExchangeRate = WSTETH.stEthPerToken();
        address wstEthReserveOracle = address(new MockReserveOracle(wstEthExchangeRate));

        uint256 ethXExchangeRate = STADER.getExchangeRate();
        address ethXReserveOracle = address(new MockReserveOracle(ethXExchangeRate));

        uint256 swellExchangeRate = SWELL.getRate();
        address swellReserveOracle = address(new MockReserveOracle(swellExchangeRate));

        reserveOracles = new address[](ionPool.ilkCount());
        reserveOracles[0] = wstEthReserveOracle;
        reserveOracles[1] = ethXReserveOracle;
        reserveOracles[2] = swellReserveOracle;
        liquidationThresholds = new uint256[](ionPool.ilkCount());

        for (uint256 i = 0; i < liquidationThresholds.length; i++) {
            liquidationThresholds[i] = LIQUIDATION_THRESHOLD;
        }

        maxDiscounts = new uint256[](ionPool.ilkCount());
        for (uint256 i = 0; i < liquidationThresholds.length; i++) {
            maxDiscounts[i] = 0.2e27;
        }

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

        liquidationHelpers = new LiquidationHelpers();

        // Launch Liquidator contracts

        uniswapV3Liquidator = new UniswapV3Liquidator(ionPool, liquidation, WETH, PROTOCOL);
        curveLiquidator = new CurveLiquidator(ionPool, liquidation, WETH, PROTOCOL);
        wstEthCurveLiquidator = new WstEthCurveLiquidator(ionPool, liquidation, WETH, WSTETH, STETH, PROTOCOL);

        (, int256 stEthSpot,,,) = STETH_ETH_CHAINLINK.latestRoundData();
        uint256 wstEthInEthSpot = WSTETH.getStETHByWstETH(uint256(stEthSpot));
        spotOracles[0].setPrice(wstEthInEthSpot);

        uint256 rate = STADER_POOL.getRate();
        spotOracles[1].setPrice(rate);

        (uint160 sqrtPriceX96,,,,,,) = SWETH_ETH_POOL.slot0();
        uint256 oneEthToSwethSpotPrice = uint256(sqrtPriceX96) * sqrtPriceX96 * WAD / (1 << 192); // Spot price OK for
            // testing
        uint256 oneSwethToEthSpotPrice = WAD * WAD / oneEthToSwethSpotPrice;
        spotOracles[2].setPrice(oneSwethToEthSpotPrice);

        vm.deal(lender1, INITIAL_LENDER_UNDERLYING_BALANCE);
        vm.deal(lender2, INITIAL_LENDER_UNDERLYING_BALANCE);

        vm.startPrank(lender1);
        weth.deposit{ value: INITIAL_LENDER_UNDERLYING_BALANCE }();
        weth.approve(address(ionPool), type(uint256).max);
        ionPool.supply(lender1, INITIAL_LENDER_UNDERLYING_BALANCE, emptyProof);
        vm.stopPrank();

        vm.startPrank(lender2);
        weth.deposit{ value: INITIAL_LENDER_UNDERLYING_BALANCE }();
        weth.approve(address(ionPool), type(uint256).max);
        ionPool.supply(lender2, INITIAL_LENDER_UNDERLYING_BALANCE, emptyProof);
        vm.stopPrank();

        vm.deal(address(this), INITIAL_THIS_UNDERLYING_BALANCE * ionPool.ilkCount());

        WSTETH.depositForLst(INITIAL_THIS_UNDERLYING_BALANCE);
        STADER.depositForLst(INITIAL_THIS_UNDERLYING_BALANCE);
        SWELL.depositForLst(INITIAL_THIS_UNDERLYING_BALANCE);

        IERC20[] memory _collaterals = _getCollaterals();
        for (uint256 i = 0; i < _collaterals.length; i++) {
            _collaterals[i].approve(address(gemJoins[i]), type(uint256).max);
            spotOracles[i].setPrice(1e27);
        }
    }

    function _getUnderlying() internal pure override returns (address) {
        return address(weth);
    }

    function _getCollaterals() internal pure override returns (IERC20[] memory _collaterals) {
        _collaterals = new IERC20[](3);

        _collaterals[0] = IERC20(address(WSTETH));
        _collaterals[1] = IERC20(address(ETHX));
        _collaterals[2] = IERC20(address(SWELL));
    }

    function _getDepositContracts() internal pure override returns (address[] memory depositContract) {
        depositContract = new address[](3);
        depositContract[0] = address(WSTETH);
        depositContract[1] = address(STADER);
        depositContract[2] = address(SWELL);
    }

    // /**
    //  * @notice Internal helper function for calculating the repay amount.
    //  * @param debtValue [rad] totalDebt
    //  * @param collateralValue [rad] collateral * exchangeRate * liquidationThreshold
    //  * @param liquidationThreshold [ray]
    //  * @param discount [ray]
    //  * @return repay [rad]
    //  */
    function _getRepayAmt(uint8 ilkIndex, address user) internal view returns (uint256 repay) {
        uint256 exchangeRate = MockReserveOracle(reserveOracles[ilkIndex]).currentExchangeRate().scaleUpToRay(18);
        (uint256 collateralAmount, uint256 normalizedDebt) = ionPool.vault(ilkIndex, user);
        uint256 collateralValue = (collateralAmount * exchangeRate).rayMulDown(LIQUIDATION_THRESHOLD);

        uint256 rate = ionPool.rate(ilkIndex);
        uint256 healthRatio = collateralValue.rayDivDown(normalizedDebt * rate); // round down in protocol favor

        if (healthRatio >= RAY) revert("Healthy Vault");

        uint256 discount = BASE_DISCOUNT + (RAY - healthRatio); // [ray] + ([ray] - [ray])
        discount = discount <= maxDiscounts[ilkIndex] ? discount : maxDiscounts[ilkIndex];

        uint256 debtValue = normalizedDebt * rate;
        uint256 liquidationThreshold = liquidationThresholds[ilkIndex];

        uint256 repayNum = debtValue.rayMulUp(TARGET_HEALTH) - collateralValue; // [rad] - [rad] = [rad]
        uint256 repayDen = TARGET_HEALTH - liquidationThreshold.rayDivUp(RAY - discount); // [ray]
        uint256 repayAmount = repayNum.rayDivUp(repayDen); // [rad] * RAY / [ray] = [rad]

        repay = repayAmount / RAY;
        if (repayAmount % RAY != 0) ++repay;
    }
}
