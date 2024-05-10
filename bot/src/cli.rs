use clap::{command, Parser};
use ethers::types::Address;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[clap(name = "liquidator", version = "0.1.0")]
pub struct Cli {
    #[arg(id = "pool", long, short, long_help = "Set the Ion Pool address.")]
    pub ion_pool: Address,

    #[arg(
        id = "liquidation",
        long,
        short,
        long_help = "Set the Liquidation address."
    )]
    pub liquidation: Address,

    #[arg(id = "treasury", long, short, long_help = "Set the Treasury address.")]
    pub treasury: Address,

    #[arg(
        id = "collateral",
        long,
        short,
        long_help = "Set the collateral address."
    )]
    pub collateral_erc20: Address,

    #[arg(id = "join", long, short, long_help = "Set the Gem Join address.")]
    pub gem_join: Address,

    #[arg(
        id = "curve-liquidator",
        long,
        visible_alias = "cl",
        long_help = "Set the Curve Liquidator address."
    )]
    pub curve_liquidator: Address,

    #[arg(
        id = "uniswap-liquidator",
        long,
        visible_alias = "ul",
        long_help = "Set the Uniswap Liquidator address."
    )]
    pub uniswap_liquidator: Address,

    #[arg(
        id = "curve-pool",
        long,
        visible_alias = "cp",
        long_help = "Set the Curve Pool address."
    )]
    pub curve_pool: Address,

    #[arg(
        id = "uniswap-pool",
        long,
        short,
        visible_alias = "up",
        long_help = "Set the Uniswap Pool address."
    )]
    pub uniswap_pool: Address,
}
