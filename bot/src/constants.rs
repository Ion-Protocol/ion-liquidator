use ethers::{prelude::Lazy, types::Address};
use ethers_core::{types::H256, utils::keccak256};
use std::str::FromStr;

pub fn get_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{} not found", key))
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
            https_url: get_env("HTTP_RPC_URL"),
            wss_url: get_env("WS_RPC_URL"),
            private_key: get_env("PRIVATE_KEY"),
            chain_id: get_env("CHAIN_ID")
                .parse()
                .expect("CHAIN_ID is not a number"),
        }
    }
}

pub static ZERO_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str("0x0000000000000000000000000000000000000000").unwrap());
pub static WETH9: Lazy<Address> =
    Lazy::new(|| Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap());

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
