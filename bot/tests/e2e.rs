use anvil::{spawn, NodeConfig};

#[cfg(test)]
async fn test_ion_liquidator() {
    let config = NodeConfig::default();
    let (_api, handle) = spawn(config).await;
}
