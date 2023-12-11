use crate::constants::{ION_TREASURY, TRANSFER_EVENT_HASH};
use crate::executors::mempool_executor::SubmitTxToMempool;
use alloy_primitives::{Address, Bytes as aBytes, B256};
use anvil::eth::backend::mem::inspector::Inspector;
use anyhow::Result;
use artemis_core::types::Strategy;
use bindings::{
    fetch_parameters::FETCHPARAMETERS_BYTECODE,
    fetch_rates_and_exchange_rates::FETCHRATESANDEXCHANGERATES_BYTECODE,
    ion_pool::{
        BorrowFilter, ConfiscateVaultFilter, DepositCollateralFilter, IonPool, RepayFilter,
        WithdrawCollateralFilter,
    },
    liquidation_helpers::LIQUIDATIONHELPERS_BYTECODE,
};
use ethers_core::{
    abi::{
        decode, encode, Bytes,
        ParamType::{Array, Tuple, Uint},
        Token,
    },
    types::{
        Eip1559TransactionRequest, Filter, Log, NameOrAddress, Sign, TransactionRequest, H160, U256,
    },
};
use ethers_providers::Middleware;
use foundry_evm::{fork::SharedBackend, revm::primitives::U256 as rU256};
use log::{error, info};
use revm::{
    db::CacheDB,
    primitives::{
        AccountInfo, Bytecode, CreateScheme, ExecutionResult, Output, ResultAndState, TransactTo,
        TxEnv,
    },
    InMemoryDB, EVM,
};
use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_trait::async_trait;

use crate::{
    constants::{
        BORROW_EVENT_HASH, CONFISCATE_VAULT_EVENT_HASH, CURVE_LIQUIDATOR,
        DEPOSIT_COLLATERAL_EVENT_HASH, ETHX_ADDRESS, ETHX_CURVE_POOL, ETHX_GEM_JOIN,
        ETHX_UNISWAP_POOL, EVENTS, ION_POOL_ADDRESS, LIQUIDATION_ADDRESS, REPAY_EVENT_HASH,
        ST_ETH_CURVE_POOL, SW_ETH_3000_UNISWAP_POOL, SW_ETH_500_UNISWAP_POOL, SW_ETH_ADDRESS,
        SW_ETH_GEM_JOIN, UNISWAP_LIQUIDATOR, WITHDRAW_COLLATERAL_EVENT_HASH, WST_ETH_ADDRESS,
        WST_ETH_CURVE_LIQUIDATOR, WST_ETH_GEM_JOIN, WST_ETH_UNISWAP_POOL,
    },
    helpers::{
        foundry_evm_helpers::setup_foundry_evm,
        revm_helpers::{create_evm_instance, deploy_contract, evm_env_setup},
    },
    types::{Action, Event, IlkRateData, PoolInfo, VaultInfo, VaultKey},
};

#[derive(Debug, Clone)]
pub struct IonLiquidatorStrategy<M> {
    provider: Arc<M>,
    ion_pool: Arc<IonPool<M>>,
    vaults: HashMap<VaultKey, VaultInfo>,
    revm: EVM<InMemoryDB>,

    collaterals: Box<[H160]>,
    gem_joins: Box<[H160]>,

    dex_pools: Vec<Vec<PoolInfo>>,

    liquidation_helper_addr: H160,

    // Below fields should only be set once during the syncing phase.
    ilk_count: u32,
    liquidation_thresholds: Box<[U256]>,
    max_discounts: Box<[U256]>,
    dusts: Box<[U256]>,
    base_discount: U256,
    target_health: U256,
}

impl<M: Middleware + 'static> IonLiquidatorStrategy<M> {
    pub fn new(provider: Arc<M>) -> Self {
        let ion_pool = Arc::new(IonPool::new(*ION_POOL_ADDRESS, provider.clone()));

        let mut revm = create_evm_instance();
        evm_env_setup(&mut revm);
        let liquidation_helper_addr =
            deploy_contract(&mut revm, LIQUIDATIONHELPERS_BYTECODE.clone(), &[])
                .expect("Failed to deploy LiquidationHelpers contract on REVM");

        let collaterals = Box::new([*WST_ETH_ADDRESS, *ETHX_ADDRESS, *SW_ETH_ADDRESS]);

        let gem_joins = Box::new([*WST_ETH_GEM_JOIN, *ETHX_GEM_JOIN, *SW_ETH_GEM_JOIN]);

        let wst_eth_pools = vec![
            PoolInfo::Curve(address_to_bytes32(&ST_ETH_CURVE_POOL).into(), true),
            PoolInfo::Uniswap(address_to_bytes32(&WST_ETH_UNISWAP_POOL).into(), false),
        ];

        let ethx_pools = vec![
            PoolInfo::Curve(address_to_bytes32(&ETHX_CURVE_POOL).into(), true),
            PoolInfo::Uniswap(address_to_bytes32(&ETHX_UNISWAP_POOL).into(), false),
        ];

        let sw_eth_pools = vec![
            PoolInfo::Uniswap(address_to_bytes32(&SW_ETH_500_UNISWAP_POOL).into(), true),
            PoolInfo::Uniswap(address_to_bytes32(&SW_ETH_3000_UNISWAP_POOL).into(), true),
        ];

        let dex_pools = vec![wst_eth_pools, ethx_pools, sw_eth_pools];

        Self {
            provider,
            ion_pool,
            vaults: HashMap::new(),
            revm,

            collaterals,
            gem_joins,

            dex_pools,

            liquidation_helper_addr,
            ilk_count: 0,

            liquidation_thresholds: Box::new([]),
            max_discounts: Box::new([]),
            dusts: Box::new([]),
            base_discount: U256::zero(),
            target_health: U256::zero(),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for IonLiquidatorStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        let ilk_count = self
            .ion_pool
            .ilk_count()
            .call()
            .await
            .expect("Calling ilk_count() failed")
            .as_u32();
        self.ilk_count = ilk_count;

        let starting_block = 0;
        let current_block = self.provider.get_block_number().await?.as_u64();

        info!(
            "Syncing vaults from block {} to {}",
            starting_block, current_block
        );

        self.sync_vaults(starting_block, current_block).await?;
        self.sync_parameters().await?;

        info!("Liquidator synced!");

        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            // It is possible for a block to be processed before an IonPool event is. If the IonPool event would have put a vault (in memory) into a liquidatable state, then the liquidatable state of that vault will be recognized when the next block is processed.
            Event::NewBlock(block_number, block_timestamp) => {
                info!("Processing new block");

                let rate_data = match self.fetch_rates_and_exchange_rates().await {
                    Ok(rate_data) => rate_data,
                    Err(e) => {
                        error!("Error fetching rates and exchange rates: {:?}", e);
                        vec![]
                    }
                };

                let current_gas_price = match self.provider.get_gas_price().await {
                    Ok(gas_price) => gas_price,
                    Err(e) => {
                        error!("Error fetching gas price: {:?}", e);
                        return vec![];
                    }
                };

                let mut actions = vec![];

                let _ = self.vaults
                    .iter()
                    .filter_map(|(key, info)| {
                        let sig: Vec<u8> = [252, 139, 29, 222].to_vec();

                        let args: Vec<u8> = encode(&[
                            Token::Uint(info.collateral),
                            Token::Uint(info.normalized_debt),
                            Token::Uint(rate_data[key.ilk_index as usize].rate),
                            Token::Uint(rate_data[key.ilk_index as usize].exchange_rate),
                            Token::Uint(self.max_discounts[key.ilk_index as usize]),
                            Token::Uint(self.base_discount),
                            Token::Uint(self.target_health),
                            Token::Uint(self.liquidation_thresholds[key.ilk_index as usize]),
                            Token::Uint(self.dusts[key.ilk_index as usize])
                        ]);

                        let tx_payload = [sig, args].concat();

                        self.revm.env.tx.transact_to =
                            TransactTo::Call(self.liquidation_helper_addr.as_fixed_bytes().into());
                        self.revm.env.tx.data = tx_payload.into();

                        let ResultAndState { result, .. } = match self.revm.transact_ref() {
                            Ok(result) => result,
                            Err(e) => {
                                error!("EVM call failed: {e:?}");
                                return None;
                            }
                        };

                        let ( repay_amount, collateral_reward ) = match result {
                            ExecutionResult::Success { output, .. } => match output {
                                Output::Call(o) => {
                                    let decoded_data = decode(&[Uint(256), Uint(256)], &o)
                                        .expect("Failed to decode return data");

                                    let decoded_data = decoded_data
                                        .iter()
                                        .map(|token| match token {
                                            Token::Uint(repay_amount) => *repay_amount,
                                            _ => panic!("Unexpected token type"),
                                        })
                                        .collect::<Vec<U256>>();

                                    (decoded_data[0], decoded_data[1])
                                }
                                Output::Create(_, _) => panic!("Unexpected tx type on getRepayAndCollateralReward()... should be a call"),
                            },
                            ExecutionResult::Revert { gas_used: _, output } => panic!("getRepayAndCollateralReward() reverted: {:?}", output),
                            _ => panic!("getRepayAndCollateralReward() tx failed"),
                        };

                        if repay_amount > 0.into() {
                            Some((key, info, repay_amount, collateral_reward))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .iter()
                    // Check liquidatable vaults for profitability
                    .for_each(|liquidatable_vault: &(&VaultKey, &VaultInfo, U256, U256)| {
                        let (key, _, _, _) = liquidatable_vault;

                        // Random address
                        let tx_sender =
                            Address::from_str("0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266")
                                .unwrap();

                        let mut foundry_evm =
                            setup_foundry_evm(self.provider.clone(), block_number, block_timestamp);
                        let fork_db = foundry_evm.db.as_mut().unwrap();

                        let ten_ether = rU256::from(10)
                            .checked_mul(rU256::from(10).pow(rU256::from(18)))
                            .unwrap();
                        let tx_sender_acc_info =
                            AccountInfo::new(ten_ether, 0, B256::default(), Bytecode::default());
                        fork_db.insert_account_info(tx_sender, tx_sender_acc_info);

                        let simulations = self.simulate_liquidation(
                            &mut foundry_evm,
                            key.ilk_index,
                            &tx_sender,
                            &key.user,
                        );
                        let optimal_simulation = simulations.iter().filter_map(|simulation| {
                            let (treasury_reward, gas_used, gas_refunded, calldata, liquidator_addr) = simulation;

                            let gas_limit = gas_used + gas_refunded;
                            let cost_of_tx = current_gas_price * gas_limit * 104 / 100; // Add a 4% buffer to the cost
                            let profit = treasury_reward - cost_of_tx;

                            if profit > 0.into() {
                                Some((profit, calldata, liquidator_addr))
                            } else {
                                None
                            }
                        }).max_by_key(|(profit, _, _)| *profit);

                        if let Some((_, calldata, liquidator_addr)) = optimal_simulation {
                             {
                                let tx = Eip1559TransactionRequest {
                                    from: Some(H160::from(TryInto::<[u8; 20]>::try_into(tx_sender.0).unwrap())),
                                    to: Some(NameOrAddress::Address(*liquidator_addr)),
                                    data: Some(calldata.clone().0.into()),
                                    gas: None,
                                    chain_id: Some(31337u64.into()),
                                    value: None,
                                    nonce: None,
                                    access_list: Default::default(),
                                    max_fee_per_gas: None,
                                    max_priority_fee_per_gas: None
                                };

                                let action = Action::Liquidate(SubmitTxToMempool {
                                    tx: tx.into(),
                                    gas_price: Some(current_gas_price),
                                });

                                actions.push(action);
                            }
                        }

                    });

                return actions;
            }
            Event::Borrow(log) => match self.process_borrow_event(log) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error processing Borrow event: {:?}", e);
                }
            },
            Event::Repay(log) => match self.process_repay_event(log) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error processing Repay event: {:?}", e);
                }
            },
            Event::DepositCollateral(log) => match self.process_deposit_collateral_event(log) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error processing DepositCollateral event: {:?}", e);
                }
            },
            Event::WithdrawCollateral(log) => match self.process_withdraw_collateral_event(log) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error processing WithdrawCollateral event: {:?}", e);
                }
            },
            Event::ConfiscateVault(log) => match self.process_confiscate_vault_event(log) {
                Ok(_) => {}
                Err(e) => {
                    panic!("Error processing ConfiscateVault event: {:?}", e);
                }
            },
            Event::Other => {}
        }

        vec![]
    }
}

impl<M: Middleware + 'static> IonLiquidatorStrategy<M> {
    async fn sync_vaults(&mut self, from_block: u64, to_block: u64) -> Result<()> {
        let filter = Filter::new()
            .address(*ION_POOL_ADDRESS)
            .from_block(from_block)
            .to_block(to_block)
            .events(EVENTS);

        let events = self.provider.get_logs(&filter).await?;

        for event in events {
            let event_sig = event.topics[0];
            if event_sig == *BORROW_EVENT_HASH {
                self.process_borrow_event(event)?;
            } else if event_sig == *REPAY_EVENT_HASH {
                self.process_repay_event(event)?;
            } else if event_sig == *DEPOSIT_COLLATERAL_EVENT_HASH {
                self.process_deposit_collateral_event(event)?;
            } else if event_sig == *WITHDRAW_COLLATERAL_EVENT_HASH {
                self.process_withdraw_collateral_event(event)?;
            } else if event_sig == *CONFISCATE_VAULT_EVENT_HASH {
                self.process_confiscate_vault_event(event)?;
            }
        }

        Ok(())
    }

    async fn sync_parameters(&mut self) -> Result<()> {
        let ilk_count = Token::Uint(self.ilk_count.into());
        let liquidation = Token::Address(*LIQUIDATION_ADDRESS);
        let ion_pool = Token::Address(self.ion_pool.address());

        let constructor_args: Bytes = encode(&[ilk_count, ion_pool, liquidation]);
        let payload = [FETCHPARAMETERS_BYTECODE.clone(), constructor_args.into()].concat();

        let tx = TransactionRequest::default().data(payload);

        let return_data = self.provider.call(&tx.into(), None).await?;

        // struct IlkParameterData {
        //     uint256 liquidationThresholds;
        //     uint256 maxDiscounts;
        //     uint256 dusts;
        // }
        // struct ParameterData {
        //     uint256 baseDiscount;
        //     uint256 targetHealth;
        // }
        let decoded_data = decode(
            &[
                Array(Box::new(Tuple(vec![Uint(256), Uint(256), Uint(256)]))),
                Tuple(vec![Uint(256), Uint(256)]),
            ],
            &return_data,
        )
        .expect("Failed to decode parameter return data");

        let mut liquidation_thresholds_vec: Vec<U256> = vec![];
        let mut max_discounts_vec: Vec<U256> = vec![];
        let mut dusts_vec: Vec<U256> = vec![];

        for token in decoded_data {
            match token {
                Token::Array(ilk_parameter_data) => {
                    for parameter_data in ilk_parameter_data {
                        match parameter_data {
                            Token::Tuple(values) => {
                                if values.len() != 3 {
                                    panic!("Unexpected number of values in IlkParameterData tuple");
                                }

                                if let [Token::Uint(liquidation_thresholds), Token::Uint(max_discounts), Token::Uint(dusts)] =
                                    &values[..]
                                {
                                    liquidation_thresholds_vec.push(*liquidation_thresholds);
                                    max_discounts_vec.push(*max_discounts);
                                    dusts_vec.push(*dusts);
                                } else {
                                    panic!("Invalid types for liquidation_thresholds and max_discounts");
                                }
                            }
                            _ => panic!("Unexpected token type"),
                        }
                    }
                }
                Token::Tuple(values) => {
                    if values.len() != 2 {
                        panic!("Unexpected number of values in ParameterData tuple");
                    }

                    if let [Token::Uint(base_discount), Token::Uint(target_health)] = &values[..] {
                        self.base_discount = *base_discount;
                        self.target_health = *target_health;
                    } else {
                        panic!("Invalid types for base_discount and target_health");
                    }
                }
                _ => panic!("Unexpected token type"),
            }
        }

        self.liquidation_thresholds = liquidation_thresholds_vec.into_boxed_slice();
        self.max_discounts = max_discounts_vec.into_boxed_slice();
        self.dusts = dusts_vec.into_boxed_slice();

        Ok(())
    }

    async fn fetch_rates_and_exchange_rates(&mut self) -> Result<Vec<IlkRateData>> {
        let ilk_count = Token::Uint(self.ilk_count.into());
        let liquidation = Token::Address(*LIQUIDATION_ADDRESS);
        let ion_pool = Token::Address(*ION_POOL_ADDRESS);

        let constructor_args: Bytes = encode(&[ilk_count, liquidation, ion_pool]);
        let payload = [
            FETCHRATESANDEXCHANGERATES_BYTECODE.clone(),
            constructor_args.into(),
        ]
        .concat();

        let tx = TransactionRequest::default().data(payload);

        let return_data = self.provider.call(&tx.into(), None).await?;

        // struct IlkRateData {
        //     uint256 exchangeRate;
        //     uint256 ilkRate;
        // }
        let decoded_data = decode(
            &[Array(Box::new(Tuple(vec![Uint(256), Uint(256)])))],
            &return_data,
        )
        .expect("Failed to decode rate/exchange rate return data");

        let rate_and_exchange_rate_data: Vec<IlkRateData> = decoded_data
            .iter()
            .flat_map(|token| match token {
                Token::Array(ilk_rate_data_vec) => ilk_rate_data_vec
                    .iter()
                    .map(|token| match token {
                        Token::Tuple(values) => {
                            if values.len() != 2 {
                                panic!("Unexpected number of values in IlkRateData tuple");
                            }

                            if let [Token::Uint(exchange_rate), Token::Uint(rate)] = &values[..] {
                                IlkRateData {
                                    rate: *rate,
                                    exchange_rate: *exchange_rate,
                                }
                            } else {
                                panic!("Invalid types for rate and exchange_rate");
                            }
                        }
                        _ => panic!("Unexpected token type"),
                    })
                    .collect::<Vec<IlkRateData>>(),
                _ => panic!("Unexpected token type"),
            })
            .collect();

        Ok(rate_and_exchange_rate_data)
    }

    fn process_borrow_event(&mut self, event: Log) -> Result<()> {
        let BorrowFilter {
            ilk_index,
            user,
            amount_of_normalized_debt,
            ..
        } = self
            .ion_pool
            .decode_event::<BorrowFilter>("Borrow", event.topics, event.data)
            .expect("Error decoding Borrow Event");

        let vault_key = VaultKey { ilk_index, user };

        if let Some(value) = self.vaults.get_mut(&vault_key) {
            value.normalized_debt += amount_of_normalized_debt;
        } else {
            panic!("Borrow event for non-existent vault");
        }

        info!(
            "Processed borrow on vault {} {} for {} debt",
            ilk_index, user, amount_of_normalized_debt
        );

        Ok(())
    }

    fn process_repay_event(&mut self, event: Log) -> Result<()> {
        let RepayFilter {
            ilk_index,
            user,
            amount_of_normalized_debt,
            ..
        } = self
            .ion_pool
            .decode_event::<RepayFilter>("Repay", event.topics, event.data)
            .expect("Error decoding Repay Event");

        let vault_key = VaultKey { ilk_index, user };

        if let Some(value) = self.vaults.get_mut(&vault_key) {
            value.normalized_debt -= amount_of_normalized_debt;
        } else {
            panic!("Repay event for non-existent vault");
        }

        info!(
            "Processed repay on vault {} {} for {} debt",
            ilk_index, user, amount_of_normalized_debt
        );

        Ok(())
    }

    fn process_deposit_collateral_event(&mut self, event: Log) -> Result<()> {
        let DepositCollateralFilter {
            ilk_index,
            user,
            amount,
            ..
        } = self
            .ion_pool
            .decode_event::<DepositCollateralFilter>("DepositCollateral", event.topics, event.data)
            .expect("Error decoding DepositCollateral Event");

        let vault_key = VaultKey { ilk_index, user };

        if let Some(value) = self.vaults.get_mut(&vault_key) {
            value.collateral += amount;
        } else {
            // Deposit collateral should always be first action on a vault
            self.vaults.insert(
                vault_key,
                VaultInfo {
                    collateral: amount,
                    normalized_debt: U256::zero(),
                },
            );
        }

        info!(
            "Processed collateral deposit on vault {} {} for {} collateral",
            ilk_index, user, amount
        );

        Ok(())
    }

    fn process_withdraw_collateral_event(&mut self, event: Log) -> Result<()> {
        let WithdrawCollateralFilter {
            ilk_index,
            user,
            amount,
            ..
        } = self
            .ion_pool
            .decode_event::<WithdrawCollateralFilter>(
                "WithdrawCollateral",
                event.topics,
                event.data,
            )
            .expect("Error decoding WithdrawCollateral Event");

        let vault_key = VaultKey { ilk_index, user };

        if let Some(value) = self.vaults.get_mut(&vault_key) {
            value.collateral -= amount;
        } else {
            panic!("WithdrawCollateral event for non-existent vault");
        }

        info!(
            "Processed collateral withdrawal on vault {} {} for {} collateral",
            ilk_index, user, amount
        );

        Ok(())
    }

    fn process_confiscate_vault_event(&mut self, event: Log) -> Result<()> {
        let ConfiscateVaultFilter {
            ilk_index,
            u: user,
            change_in_collateral,
            change_in_normalized_debt,
            ..
        } = self
            .ion_pool
            .decode_event::<ConfiscateVaultFilter>("ConfiscateVault", event.topics, event.data)
            .expect("Error decoding ConfiscateVault Event");

        let vault_key = VaultKey { ilk_index, user };

        if let Some(value) = self.vaults.get_mut(&vault_key) {
            let (sign, change_in_collateral) = change_in_collateral.into_sign_and_abs();
            match sign {
                Sign::Positive => {
                    value.collateral += change_in_collateral;
                }
                Sign::Negative => {
                    value.collateral -= change_in_collateral;
                }
            }

            let (sign, change_in_normalized_debt) = change_in_normalized_debt.into_sign_and_abs();
            match sign {
                Sign::Positive => {
                    value.normalized_debt += change_in_normalized_debt;
                }
                Sign::Negative => {
                    value.normalized_debt -= change_in_normalized_debt;
                }
            }
        } else {
            // If confiscate vault is the first action on a vault, then all
            // delta values must have been positive
            self.vaults.insert(
                vault_key,
                VaultInfo {
                    collateral: change_in_collateral.abs().into_raw(),
                    normalized_debt: change_in_normalized_debt.abs().into_raw(),
                },
            );
        }

        info!(
            "Processed vault confiscation on vault {} {} with a change in collateral of {} and a change in debt of {}",
            ilk_index, user, change_in_collateral, change_in_normalized_debt
        );

        Ok(())
    }

    fn simulate_liquidation(
        &self,
        evm: &mut EVM<CacheDB<SharedBackend>>,
        ilk_index: u8,
        tx_sender: &Address,
        user: &H160,
    ) -> Vec<(
        U256,
        u64,
        u64,
        aBytes,
        H160, /*U256, u64, u64, &H160, aBytes*/
    )> {
        // Signature of the liquidate() function
        let sig: Arc<[u8]> = Arc::new([108, 176, 68, 41]);

        let common_args: Arc<[u8]> = encode(&[
            Token::Uint(ilk_index.into()),
            Token::Address(*user),
            Token::Address(self.collaterals[ilk_index as usize]),
            Token::Address(self.gem_joins[ilk_index as usize]),
        ])
        .as_slice()
        .into();

        let mut tx = TxEnv {
            caller: *tx_sender,
            transact_to: TransactTo::Create(CreateScheme::Create),
            value: Default::default(),
            data: Default::default(),
            gas_limit: 5000000,
            gas_price: rU256::from(100)
                .checked_mul(rU256::from(10).pow(rU256::from(9)))
                .unwrap(),
            nonce: None,
            chain_id: None,
            gas_priority_fee: None,
            access_list: Default::default(),
            blob_hashes: Default::default(),
            max_fee_per_blob_gas: None,
            optimism: Default::default(),
        };

        self.dex_pools[ilk_index as usize]
            .iter()
            .map(|pool_info| match pool_info {
                PoolInfo::Curve(pool_id, weth_is_token0) => {
                    let args: Arc<[u8]> = encode(&[
                        Token::FixedBytes(pool_id.to_vec()),
                        Token::Bool(*weth_is_token0),
                    ])
                    .as_slice()
                    .into();

                    let calldata = [sig.clone(), common_args.clone(), args].concat();

                    let liquidator_addr = if ilk_index == 0 {
                        *WST_ETH_CURVE_LIQUIDATOR
                    } else {
                        *CURVE_LIQUIDATOR
                    };

                    tx.data = calldata.into();
                    tx.transact_to = TransactTo::Call(liquidator_addr.as_fixed_bytes().into());

                    (tx.clone(), liquidator_addr)
                }
                PoolInfo::Uniswap(pool_id, weth_is_token0) => {
                    let args: Arc<[u8]> = encode(&[
                        Token::FixedBytes(pool_id.to_vec()),
                        Token::Bool(*weth_is_token0),
                    ])
                    .as_slice()
                    .into();

                    let calldata = [sig.clone(), common_args.clone(), args].concat();

                    tx.data = calldata.into();
                    tx.transact_to =
                        TransactTo::Call((*UNISWAP_LIQUIDATOR).as_fixed_bytes().into());

                    (tx.clone(), *UNISWAP_LIQUIDATOR)
                }
            })
            .map(|(tx, liquidator_addr)| {
                let calldata = tx.data.clone();

                evm.env.tx = tx;

                let simulation_result = match evm.transact_ref() {
                    Ok(result) => result,
                    Err(e) => {
                        panic!("EVM call failed: {e:?}");
                    }
                };

                let (treasury_reward, gas_used, gas_refunded) = match &simulation_result.result {
                    ExecutionResult::Success {
                        gas_used,
                        gas_refunded,
                        logs,
                        ..
                    } => {
                        let treasury_reward = match logs.iter().find(|log| {
                            log.topics[0] == B256::from(*TRANSFER_EVENT_HASH.as_fixed_bytes())
                                && log.topics[1] == address_to_bytes32(&liquidator_addr)
                                && log.topics[2] == address_to_bytes32(&ION_TREASURY)
                        }) {
                            Some(treasury_transfer_log) => {
                                let decoded_data =
                                    decode(&[Uint(256)], &treasury_transfer_log.data)
                                        .expect("Failed to decode transfer log data");

                                decoded_data
                                    .into_iter()
                                    .map(|token| match token {
                                        Token::Uint(treasury_reward) => treasury_reward,
                                        _ => panic!("Unexpected token type"),
                                    })
                                    .collect::<Vec<U256>>()[0]
                            }
                            None => panic!("No profit transfer log found"),
                        };

                        (treasury_reward, *gas_used, *gas_refunded)
                    }
                    _ => (0.into(), 0u64, 0u64),
                };

                // let mut t = Inspector::default().with_steps_tracing();

                // match evm.inspect_ref(&mut t) {
                //     Ok(result) => result,
                //     Err(e) => {
                //         panic!("EVM call failed: {e:?}");
                //     }
                // };

                // println!("{}", t.tracer.unwrap_or_default().traces);

                (
                    treasury_reward,
                    gas_used,
                    gas_refunded,
                    calldata,
                    liquidator_addr,
                )
            })
            .collect::<Vec<_>>()
    }
}

fn address_to_bytes32(addr: &H160) -> [u8; 32] {
    let mut temp = [0u8; 32];
    temp[12..].copy_from_slice(addr.as_fixed_bytes());
    temp
}
