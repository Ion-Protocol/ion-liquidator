use crate::{
    constants::{
        Env, BORROW_EVENT_HASH, CONFISCATE_VAULT_EVENT_HASH, DEPOSIT_COLLATERAL_EVENT_HASH, EVENTS,
        REPAY_EVENT_HASH, TRANSFER_EVENT_HASH, WITHDRAW_COLLATERAL_EVENT_HASH,
    },
    executors::mempool_executor::SubmitTxToMempool,
    helpers::{
        foundry_evm_helpers::setup_foundry_evm,
        revm_helpers::{create_evm_instance, deploy_contract, evm_env_setup},
    },
    types::{Action, Event, IlkRateData, PoolInfo, VaultInfo, VaultKey},
};
use alloy_primitives::{Address, Bytes as aBytes, B256};
#[allow(unused_imports)]
use anvil::eth::backend::mem::inspector::Inspector;
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
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
        ParamType::{Tuple, Uint},
        Token::{Address as TAddress, Bool as TBool, Tuple as TTuple, Uint as TUint},
    },
    types::{
        Address as EAddress, Eip1559TransactionRequest, Filter, Log, NameOrAddress, Sign,
        TransactionRequest, H160, U256,
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
use std::{collections::HashMap, str::FromStr, sync::Arc, sync::Mutex};
#[derive(Debug)]
pub struct IonLiquidatorStrategy<M> {
    provider: Arc<M>,
    env: Env,
    ion_pool: Arc<IonPool<M>>,
    vaults: HashMap<VaultKey, VaultInfo>,
    revm: Mutex<EVM<InMemoryDB>>,

    liquidation: H160,
    treasury: H160,
    collateral: H160,
    gem_join: H160,

    curve_liquidator: H160,
    uniswap_liquidator: H160,

    dex_pools: Vec<PoolInfo>,

    liquidation_helper_addr: H160,

    // Below fields should only be set once during the syncing phase.
    liquidation_threshold: U256,
    max_discount: U256,
    dust: U256,
    base_discount: U256,
    target_health: U256,
}

impl<M: Middleware + 'static> IonLiquidatorStrategy<M> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider: Arc<M>,
        env: Env,
        ion_pool_addr: EAddress,
        liquidation: EAddress,
        treasury: EAddress,
        collateral_erc20: EAddress,
        gem_join: EAddress,
        curve_liquidator: EAddress,
        uniswap_liquidator: EAddress,
        curve_pool: EAddress,
        uniswap_pool: EAddress,
    ) -> Self {
        let ion_pool = Arc::new(IonPool::new(ion_pool_addr, provider.clone()));

        let mut revm = create_evm_instance();
        evm_env_setup(&mut revm);
        let liquidation_helper_addr =
            deploy_contract(&mut revm, LIQUIDATIONHELPERS_BYTECODE.clone(), &[])
                .expect("Failed to deploy LiquidationHelpers contract on REVM");

        let revm = Mutex::new(revm);

        let collateral = collateral_erc20;

        let dex_pools = vec![
            PoolInfo::Curve(curve_pool, false),
            PoolInfo::Uniswap(uniswap_pool, true),
        ];

        Self {
            provider,
            env,
            ion_pool,
            vaults: HashMap::new(),
            revm,

            liquidation,
            treasury,
            collateral,
            gem_join,

            curve_liquidator,
            uniswap_liquidator,

            dex_pools,

            liquidation_helper_addr,

            liquidation_threshold: U256::zero(),
            max_discount: U256::zero(),
            dust: U256::zero(),
            base_discount: U256::zero(),
            target_health: U256::zero(),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for IonLiquidatorStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        let starting_block = 0;
        let current_block = self.provider.get_block_number().await?.as_u64();

        info!(
            "Syncing vaults from block {} to {}",
            starting_block, current_block
        );

        self.sync_vaults(starting_block, current_block).await?;
        self.sync_parameters().await?;

        info!("Liquidator synced!");

        info!("Liquidation threshold: {}", self.liquidation_threshold);
        info!("Max discount: {}", self.max_discount);
        info!("Dust: {}", self.dust);
        info!("Base discount: {}", self.base_discount);
        info!("Target health: {}", self.target_health);

        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            // It is possible for a block to be processed before an IonPool
            // event is. If the IonPool event would have put a vault (in memory)
            // into a liquidatable state, then the liquidatable state of that
            // vault will be recognized when the next block is processed.
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

                let mut revm = self.revm.lock().expect("Failed to lock REVM");

                revm.env.tx.transact_to =
                    TransactTo::Call(self.liquidation_helper_addr.as_fixed_bytes().into());

                let liquidatable_vaults = self.vaults
                    .iter()
                    .filter_map(|(key, info)| {
                        let sig: Vec<u8> = [252, 139, 29, 222].to_vec();

                        let args: Vec<u8> = encode(&[
                            TUint(info.collateral),
                            TUint(info.normalized_debt),
                            TUint(rate_data[key.ilk_index as usize].rate),
                            TUint(rate_data[key.ilk_index as usize].exchange_rate),
                            TUint(self.max_discount),
                            TUint(self.base_discount),
                            TUint(self.target_health),
                            TUint(self.liquidation_threshold),
                            TUint(self.dust)
                        ]);

                        let tx_payload = [sig, args].concat();

                        revm.env.tx.data = tx_payload.into();

                        let ResultAndState { result, .. } = match revm.transact_ref() {
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
                                            TUint(repay_amount) => *repay_amount,
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
                    });

                let mut liquidatable_count = 0;

                // Check liquidatable vaults for profitability
                liquidatable_vaults.for_each(
                    |liquidatable_vault: (&VaultKey, &VaultInfo, U256, U256)| {
                        liquidatable_count += 1;

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

                        let simulations =
                            self.simulate_liquidation(&mut foundry_evm, &tx_sender, &key.user);

                        let optimal_simulation = simulations
                            .iter()
                            .filter_map(|simulation| {
                                let (
                                    treasury_reward,
                                    gas_used,
                                    gas_refunded,
                                    calldata,
                                    liquidator_addr,
                                ) = simulation;

                                let gas_limit = gas_used + gas_refunded;
                                let cost_of_tx =
                                    current_gas_price * gas_limit * U256::from(102) / 100; // Add a 2% buffer to the cost

                                if *treasury_reward > cost_of_tx {
                                    let profit = treasury_reward - cost_of_tx;
                                    Some((profit, calldata, liquidator_addr))
                                } else {
                                    None
                                }
                            })
                            .max_by_key(|(profit, _, _)| *profit);

                        if let Some((_, calldata, liquidator_addr)) = optimal_simulation {
                            {
                                let tx = Eip1559TransactionRequest {
                                    from: Some(H160::from(Into::<[u8; 20]>::into(tx_sender.0))),
                                    to: Some(NameOrAddress::Address(*liquidator_addr)),
                                    data: Some(calldata.clone().0.into()),
                                    gas: None,
                                    chain_id: Some(self.env.chain_id.into()),
                                    value: None,
                                    nonce: None,
                                    access_list: Default::default(),
                                    max_fee_per_gas: None,
                                    max_priority_fee_per_gas: None,
                                };

                                let action = Action::Liquidate(SubmitTxToMempool {
                                    tx: tx.into(),
                                    gas_price: Some(current_gas_price),
                                });

                                actions.push(action);
                            }
                        }
                    },
                );

                info!("Found {} liquidatable vaults", liquidatable_count);

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
            .address(self.ion_pool.address())
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
        let liquidation = TAddress(self.liquidation);
        let ion_pool = TAddress(self.ion_pool.address());

        let constructor_args: Bytes = encode(&[ion_pool, liquidation]);
        let payload = [FETCHPARAMETERS_BYTECODE.clone(), constructor_args.into()].concat();

        let tx = TransactionRequest::default().data(payload);

        let return_data = self.provider.call(&tx.into(), None).await?;

        // struct ParameterData {
        //     uint256 liquidationThreshold;
        //     uint256 maxDiscount;
        //     uint256 dust;
        //     uint256 baseDiscount;
        //     uint256 targetHealth;
        // }
        let decoded_data = decode(
            &[Tuple(vec![
                Uint(256),
                Uint(256),
                Uint(256),
                Uint(256),
                Uint(256),
            ])],
            &return_data,
        )
        .expect("Failed to decode parameter return data");

        for token in decoded_data {
            match token {
                TTuple(values) => {
                    if values.len() != 5 {
                        panic!("Unexpected number of values in ParameterData tuple");
                    }

                    if let [TUint(liquidation_threshold), TUint(max_discount), TUint(dust), TUint(base_discount), TUint(target_health)] =
                        values[..]
                    {
                        self.liquidation_threshold = liquidation_threshold;
                        self.max_discount = max_discount;
                        self.dust = dust;
                        self.base_discount = base_discount;
                        self.target_health = target_health;
                    } else {
                        panic!("Invalid types for base_discount and target_health");
                    }
                }
                _ => panic!("Unexpected token type"),
            }
        }

        Ok(())
    }

    async fn fetch_rates_and_exchange_rates(&mut self) -> Result<Vec<IlkRateData>> {
        let liquidation = TAddress(self.liquidation);
        let ion_pool = TAddress(self.ion_pool.address());

        let constructor_args: Bytes = encode(&[liquidation, ion_pool]);
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
        let decoded_data = decode(&[Tuple(vec![Uint(256), Uint(256)])], &return_data)
            .expect("Failed to decode rate/exchange rate return data");

        let rate_and_exchange_rate_data = decoded_data
            .into_iter()
            .map(|token| match token {
                TTuple(values) => {
                    if values.len() != 2 {
                        panic!("Unexpected number of values in IlkRateData tuple");
                    }

                    if let [TUint(exchange_rate), TUint(rate)] = values[..] {
                        IlkRateData {
                            rate,
                            exchange_rate,
                        }
                    } else {
                        panic!("Invalid types for rate and exchange_rate");
                    }
                }
                _ => panic!("Unexpected token type"),
            })
            .collect::<Vec<IlkRateData>>();

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
        tx_sender: &Address,
        user: &H160,
    ) -> Vec<(
        U256,
        u64,
        u64,
        aBytes,
        H160, /*U256, u64, u64, &H160, aBytes*/
    )> {
        info!("Simulating liquidation for user: {}", user);
        // Signature of the liquidate() function
        let sig: Arc<[u8]> = Arc::new([254, 112, 122, 46]);

        let common_args: Arc<[u8]> = encode(&[
            TAddress(*user),
            TAddress(self.collateral),
            TAddress(self.gem_join),
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

        self.dex_pools
            .iter()
            .map(|pool_info| match pool_info {
                PoolInfo::Curve(pool_addr, weth_is_token0) => {
                    let args: Arc<[u8]> = encode(&[TAddress(*pool_addr), TBool(*weth_is_token0)])
                        .as_slice()
                        .into();

                    let calldata = [sig.clone(), common_args.clone(), args].concat();

                    let liquidator_addr = self.curve_liquidator;

                    tx.data = calldata.into();
                    tx.transact_to = TransactTo::Call(liquidator_addr.as_fixed_bytes().into());

                    (tx.clone(), liquidator_addr)
                }
                PoolInfo::Uniswap(pool_addr, weth_is_token0) => {
                    let args: Arc<[u8]> = encode(&[TAddress(*pool_addr), TBool(*weth_is_token0)])
                        .as_slice()
                        .into();

                    let calldata = [sig.clone(), common_args.clone(), args].concat();

                    let liquidator_addr = self.uniswap_liquidator;

                    tx.data = calldata.into();
                    tx.transact_to = TransactTo::Call((liquidator_addr).as_fixed_bytes().into());

                    (tx.clone(), liquidator_addr)
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
                                && log.topics[2] == address_to_bytes32(&self.treasury)
                        }) {
                            Some(treasury_transfer_log) => {
                                let decoded_data =
                                    decode(&[Uint(256)], &treasury_transfer_log.data)
                                        .expect("Failed to decode transfer log data");

                                decoded_data
                                    .into_iter()
                                    .map(|token| match token {
                                        TUint(treasury_reward) => treasury_reward,
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
