#!/bin/bash

CMD_ARGS="--pool ${POOL} --liquidation ${LIQUIDATION} --treasury ${TREASURY} --collateral ${COLLATERAL} --join ${JOIN} --curve-liquidator ${CLIQUIDATOR} --uniswap-liquidator ${ULIQUIDATOR} --curve-pool ${CPOOL} --uniswap-pool ${UPOOL}"

# Execute the Rust binary with command line arguments
/usr/local/bin/ion-liquidator $CMD_ARGS
