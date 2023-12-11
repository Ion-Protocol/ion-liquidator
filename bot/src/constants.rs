use ethers::{prelude::Lazy, types::Address};
use ethers_core::{types::H256, utils::keccak256};
use std::str::FromStr;

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

#[derive(Debug, Clone, Default)]
pub struct Env {
    pub https_url: String,
    pub wss_url: String,
    pub private_key: String,
    pub chain_id: u64,
}

impl Env {
    pub fn new() -> Self {
        Env {
            https_url: get_env("HTTPS_URL"),
            wss_url: get_env("WSS_URL"),
            private_key: get_env("PRIVATE_KEY"),
            chain_id: get_env("CHAIN_ID").parse().unwrap(),
        }
    }
}

pub static ZERO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x0000000000000000000000000000000000000000").unwrap());
pub static WETH9: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap());

// TODO:
pub static ION_POOL_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x484242986F57dFcA98EeC2C78427931C63F1C4ce").unwrap());
pub static LIQUIDATION_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x4aAAf621a58473779099e210c606019aD3EAe536").unwrap());
pub static ION_TREASURY: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x0000000000000000000000000000000000000001").unwrap());

// IonPool Events
pub static BORROW_EVENT_SIGNATURE: &str = "Borrow(uint8,address,address,uint256,uint256,uint256)";
pub static REPAY_EVENT_SIGNATURE: &str = "Repay(uint8,address,uint256,uint256,uint256)";
pub static DEPOSIT_COLLATERAL_EVENT_SIGNATURE: &str =
    "DepositCollateral(uint8,address,address,uint256)";
pub static WITHDRAW_COLLATERAL_EVENT_SIGNATURE: &str =
    "WithdrawCollateral(uint8,address,address,uint256)";
pub static CONFISCATE_VAULT_EVENT_SIGNATURE: &str =
    "ConfiscateVault(uint8,address,address,address,int256,int256)";

pub static BORROW_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(BORROW_EVENT_SIGNATURE)));
pub static REPAY_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(REPAY_EVENT_SIGNATURE)));
pub static DEPOSIT_COLLATERAL_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(DEPOSIT_COLLATERAL_EVENT_SIGNATURE)));
pub static WITHDRAW_COLLATERAL_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(WITHDRAW_COLLATERAL_EVENT_SIGNATURE)));
pub static CONFISCATE_VAULT_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(CONFISCATE_VAULT_EVENT_SIGNATURE)));

// ERC20 Events
pub static TRANSFER_EVENT_SIGNATURE: &str = "Transfer(address,address,uint256)";

pub static TRANSFER_EVENT_HASH: Lazy<H256> =
    Lazy::new(|| H256::from(keccak256(TRANSFER_EVENT_SIGNATURE)));

pub static EVENTS: &[&str] = &[
    BORROW_EVENT_SIGNATURE,
    REPAY_EVENT_SIGNATURE,
    DEPOSIT_COLLATERAL_EVENT_SIGNATURE,
    WITHDRAW_COLLATERAL_EVENT_SIGNATURE,
    CONFISCATE_VAULT_EVENT_SIGNATURE,
];

// Collateral addresses
pub static WST_ETH_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0").unwrap());
pub static ETHX_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xA35b1B31Ce002FBF2058D22F30f95D405200A15b").unwrap());
pub static SW_ETH_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xf951E335afb289353dc249e82926178EaC7DEd78").unwrap());

pub static WST_ETH_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x4278C5d322aB92F1D876Dd7Bd9b44d1748b88af2").unwrap());
pub static ETHX_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x0D92d35D311E54aB8EEA0394d7E773Fc5144491a").unwrap());
pub static SW_ETH_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x24EcC5E6EaA700368B8FAC259d3fBD045f695A08").unwrap());

// Liquidator addresses
pub static WST_ETH_CURVE_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x40d32E9Eff27B70f67C10133F64ea406207b1c22").unwrap());
pub static CURVE_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xeBB5613d3795F753aA2ac122e65E45B8aF0a5517").unwrap());
pub static UNISWAP_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xD4fF44cBCbE1C26aD56A2638cFB84e5CdF7397bC").unwrap());

// DEX pools
pub static ST_ETH_CURVE_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xdc24316b9ae028f1497c275eb9192a3ea0f67022").unwrap());
pub static WST_ETH_UNISWAP_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x109830a1AAaD605BbF02a9dFA7B0B92EC2FB7dAa").unwrap());
pub static WST_ETH_BALANCER_POOL: Lazy<H256> = Lazy::new(|| {
    H256::from_str("0x93d199263632a4ef4bb438f1feb99e57b4b5f0bd0000000000000000000005c2").unwrap()
});

pub static ETHX_CURVE_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x59ab5a5b5d617e478a2479b0cad80da7e2831492").unwrap());
pub static ETHX_UNISWAP_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x1b9669b12959Ad51B01FaBcF01EaBDFADB82f578").unwrap());
pub static ETHX_BALANCER_POOL: Lazy<H256> = Lazy::new(|| {
    H256::from_str("0x37b18b10ce5635a84834b26095a0ae5639dcb7520000000000000000000005cb").unwrap()
});

pub static SW_ETH_500_UNISWAP_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x30ea22c879628514f1494d4bbfef79d21a6b49a2").unwrap());
pub static SW_ETH_3000_UNISWAP_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x30ea22c879628514f1494d4bbfef79d21a6b49a2").unwrap());
pub static SW_ETH_BALANCER_POOL: Lazy<H256> = Lazy::new(|| {
    H256::from_str("0xe7e2c68d3b13d905bbb636709cf4dfd21076b9d20000000000000000000005ca").unwrap()
});
