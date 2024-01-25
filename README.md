# Ion Liquidator

This project is a proof-of-concept liquidator bot for [Ion Protocol](https://github.com/Ion-Protocol/ion-protocol) built using Paradigm's [Artemis](https://github.com/paradigmxyz/artemis), a Rust MEV framework.

It is highly unlikely that this bot is profitable with room for significant optimizations in both the smart contracts and the bot strategy. It is recommended to use this repository as a starting point to build out a more optimal strategy.

## Testing on mainnet fork

Setup a mainnet fork
```
anvil --fork-url $RPC --chain-id 31337
```

Alternviately, setup with debug tracing
```
anvil --fork-url $ARCHIVE_RPC --chain-id 31337 --steps-tracing
```

Create a .env file. An http endpoint is required for submitting the transaction and a websocket url is required for maintaining a connection with the node.
```
# ion-liquidator/bot/.env

HTTPS_URL=http://localhost:8545
WSS_URL=ws://localhost:8545
CHAIN_ID=31337
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 # This private key belongs to the anvil default signer
```

In `/deployment-config/01_DeployLiquidators.json` fill `ionPool`, `liquidation`, and `treasury` (where to send bot profits) values. 
```
{
  "ionPool": [ion_pool_adddres_here],
  "liquidation": [liquidation_address_here],
  "weth": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
  "treasury": [treasury_address_here],
  "wstEth": "0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0",
  "stEth": "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84"
}
```

Deploy the liquidators. 

```
forge script script/01_DeployLiquidators.s.sol --rpc-url http://localhost:8545 --broadcast --slow
```

Fill out following values in `constants.rs`

```
# ion-liquidator/bot/src/constants.rs

...

pub static ION_POOL_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str([ion_pool_address_here]).unwrap());
pub static LIQUIDATION_ADDRESS: Lazy<Address> =
    Lazy::new(|| Address::from_str([liquidation_address_here]).unwrap());
pub static ION_TREASURY: Lazy<Address> =
    Lazy::new(|| Address::from_str([treasury_address_here]).unwrap());

...

pub static WST_ETH_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str([wst_eth_gem_join_address_here]).unwrap());
pub static ETHX_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str([ethx_gem_join_address_here]).unwrap());
pub static SW_ETH_GEM_JOIN: Lazy<Address> =
    Lazy::new(|| Address::from_str([sw_eth_gem_join_address_here]).unwrap());

// Liquidator addresses
pub static WST_ETH_CURVE_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str([wst_eth_curve_liquidator_address_here]).unwrap());
pub static CURVE_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str([ethx_curve_liquidator_address_here]).unwrap());
pub static UNISWAP_LIQUIDATOR: Lazy<Address> =
    Lazy::new(|| Address::from_str([sw_eth_curve_liquidator_address_here]).unwrap());

...

```

Run the bot

```
# ion-liquidator/bot/

cargo run --release
```


## Deploying your own instance of the protocol

It may be desirable to test a liquidator bot before an instance of Ion Protocol has been launched. In which case, the protocol can be launched onto the Mainnet fork with different initialization parameters. 

```
git clone --recursive git@github.com:Ion-Protocol/ion-protocol.git
```

Install dependencies
```
bun install
```

Inside `/deployment-config`, deployment parameters for all protocol contracts can be set.

Create `.env`.
```
CHAIN_ID=1
MAINNET_ARCHIVE_RPC_URL=https://mainnet.infura.io/v3/your-api-key
RPC_URL=http://localhost:8545
MAINNET_ETHERSCAN_URL=https://api.etherscan.io/api
```

Make sure there is an anvil node running and run `./node.sh`. This will deploy all the necessary protocol contracts as configured through `/deployment-config`.

```
# ion-protocol/

./node.sh
```

## Generate Rust Bindings for the Contracts

`forge bind --bindings-path ./bot/bindings --crate-name bindings --overwrite`
