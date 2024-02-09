use crate::executors::mempool_executor::SubmitTxToMempool;
use ethers_core::types::{Log, H160, U256};

#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(u64, u64),
    Borrow(Log),
    Repay(Log),
    DepositCollateral(Log),
    WithdrawCollateral(Log),
    ConfiscateVault(Log),
    // No-op
    Other,
}

#[derive(Debug, Clone)]
pub enum Action {
    Liquidate(SubmitTxToMempool),
}

#[derive(Debug, Clone)]
pub enum PoolInfo {
    Uniswap(H160, bool),
    Curve(H160, bool),
}

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct VaultKey {
    pub ilk_index: u8,
    pub user: H160,
}

#[derive(Hash, Debug, Clone)]
pub struct VaultInfo {
    pub collateral: U256,
    pub normalized_debt: U256,
}

#[derive(Debug)]
pub struct IlkRateData {
    pub rate: U256,
    pub exchange_rate: U256,
}
