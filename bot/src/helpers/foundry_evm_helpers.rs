use alloy_primitives::U256;
use ethers::providers::Middleware;
use foundry_evm::{
    fork::{BlockchainDb, BlockchainDbMeta, SharedBackend},
    revm::{db::CacheDB, EVM},
};
use std::{collections::BTreeSet, sync::Arc};

pub fn setup_foundry_evm<M: Middleware + 'static>(
    provider: Arc<M>,
    block_number: u64,
    block_timestamp: u64,
) -> EVM<CacheDB<SharedBackend>> {
    let shared_backend = SharedBackend::spawn_backend_thread(
        provider.clone(),
        BlockchainDb::new(
            BlockchainDbMeta {
                cfg_env: Default::default(),
                block_env: Default::default(),
                hosts: BTreeSet::from(["".to_string()]),
            },
            None,
        ),
        Some(block_number.into()),
    );
    let db = CacheDB::new(shared_backend);

    let mut evm = EVM::new();
    evm.env.block.timestamp = U256::from(block_timestamp);
    evm.database(db);

    evm
}
