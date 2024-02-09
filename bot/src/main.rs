use anyhow::Result;
use artemis_core::{
    collectors::log_collector::LogCollector,
    engine::Engine,
    types::{CollectorMap, ExecutorMap},
};
use ethers::{
    middleware::MiddlewareBuilder,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::Filter,
};
use ethers_core::types::Log;
use ion_liquidator::{
    collectors::block_collector::{BlockCollector, NewBlock},
    constants::{
        Env, BORROW_EVENT_HASH, CONFISCATE_VAULT_EVENT_HASH, DEPOSIT_COLLATERAL_EVENT_HASH, EVENTS,
        ION_POOL_ADDRESS, REPAY_EVENT_HASH, WITHDRAW_COLLATERAL_EVENT_HASH,
    },
    executors::mempool_executor::MempoolExecutor,
    strategy::IonLiquidatorStrategy,
    types::{Action, Event},
    utils::setup_logger,
};
use log::info;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    setup_logger()?;

    let env = Env::new();
    let http = Provider::try_from(&env.https_url).unwrap();
    let ws = Ws::connect(&env.wss_url).await.unwrap();
    let wallet: LocalWallet = env
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(env.chain_id);
    let address = wallet.address();

    let provider = Arc::new(Provider::new(ws));
    let fb_provider = Arc::new(http.nonce_manager(address).with_signer(wallet));

    let mut engine: Engine<Event, Action> = Engine::new();

    let log_filter = Filter::new().address(*ION_POOL_ADDRESS).events(EVENTS);

    let log_collector = Box::new(LogCollector::new(provider.clone(), log_filter));
    let log_collector = CollectorMap::new(log_collector, |log: Log| match log.topics[0] {
        signature if signature == *BORROW_EVENT_HASH => Event::Borrow(log),
        signature if signature == *REPAY_EVENT_HASH => Event::Repay(log),
        signature if signature == *DEPOSIT_COLLATERAL_EVENT_HASH => Event::DepositCollateral(log),
        signature if signature == *WITHDRAW_COLLATERAL_EVENT_HASH => Event::WithdrawCollateral(log),
        signature if signature == *CONFISCATE_VAULT_EVENT_HASH => Event::ConfiscateVault(log),
        _ => Event::Other,
    });

    let block_collector = Box::new(BlockCollector::new(provider.clone()));
    let block_collector = CollectorMap::new(block_collector, |block: NewBlock| {
        Event::NewBlock(block.number.as_u64(), block.timestamp.as_u64())
    });

    let strategy = IonLiquidatorStrategy::new(provider.clone());

    let mempool_executor = Box::new(MempoolExecutor::new(fb_provider.clone()));
    let mempool_executor = ExecutorMap::new(mempool_executor, |action: Action| match action {
        Action::Liquidate(tx_data) => Some(tx_data),
    });

    engine.add_collector(Box::new(log_collector));
    engine.add_collector(Box::new(block_collector));

    engine.add_strategy(Box::new(strategy));

    engine.add_executor(Box::new(mempool_executor));

    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
