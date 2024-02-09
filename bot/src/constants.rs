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
    Lazy::new(|| Address::from_str("0x00000000005a1De4c0eb34609e211AD8831707E0").unwrap());
pub static LIQUIDATION_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x00000000002Ddfa58A917ee47c5BbB909A2227C4").unwrap());
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
pub static WEETH_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xCd5fE23C85820F7B72D0926FC9b05b43E359b7ee").unwrap());

pub static WEETH_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x1f0CEf277C05CBd9f96dd91e6a9D9C2422b00E55").unwrap());

// Liquidator addresses
pub static WEETH_CURVE_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x14AB8eC88894568fad716D9cd81bb4BdAF343fE0").unwrap());
pub static WEETH_UNISWAP_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x58FE37a48c2EFc2e66e7C8AaC8e91789835F8282").unwrap());

// DEX pools
pub static WEETH_WETH_UNISWAP_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x7A415B19932c0105c82FDB6b720bb01B0CC2CAe3").unwrap());
pub static WEETH_WETH_CURVE_POOL: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x13947303F63b363876868D070F14dc865C36463b").unwrap());
