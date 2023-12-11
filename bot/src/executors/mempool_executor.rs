use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, U256},
};
use std::sync::Arc;

/// An executor that sends transactions to the mempool.
pub struct MempoolExecutor<M> {
    client: Arc<M>,
}

#[derive(Debug, Clone)]
pub struct SubmitTxToMempool {
    pub tx: TypedTransaction,
    pub gas_price: Option<U256>,
}

impl<M: Middleware> MempoolExecutor<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<M> Executor<SubmitTxToMempool> for MempoolExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        match action.gas_price {
            Some(gas_price) => action.tx.set_gas_price(gas_price),
            None => &mut action.tx,
        };

        self.client.send_transaction(action.tx, None).await?;
        Ok(())
    }
}
