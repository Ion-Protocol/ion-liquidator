use anyhow::Result;
use ethers_core::{
    abi::{encode, Token},
    types::{Address, Bytes, Log},
};
use log::error;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{CreateScheme, ExecutionResult, Output, TransactTo},
    InMemoryDB, EVM,
};

pub fn create_evm_instance() -> EVM<InMemoryDB> {
    let db = CacheDB::new(EmptyDB::default());
    let mut evm = EVM::new();
    evm.database(db);
    evm
}

pub fn evm_env_setup(evm: &mut EVM<InMemoryDB>) {
    evm.env.cfg.limit_contract_code_size = Some(0x100000);
    evm.env.cfg.disable_block_gas_limit = true;
    evm.env.cfg.disable_base_fee = true;
}

pub fn deploy_contract(
    evm: &mut EVM<InMemoryDB>,
    bytecode: Bytes,
    constructor_args: &[Token],
) -> Result<Address> {
    let constructor_args_encoded = encode(constructor_args);
    let payload = [bytecode, constructor_args_encoded.into()].concat();

    evm.env.tx.transact_to = TransactTo::Create(CreateScheme::Create);
    evm.env.tx.data = payload.into();

    let result = evm
        .transact_commit()
        .expect("Failed to deploy contract on REVM");

    match result {
        ExecutionResult::Success { output, .. } => match output {
            Output::Create(_, address) => {
                let mut fixed_bytes = [0u8; 20];
                fixed_bytes.copy_from_slice(address.unwrap().as_slice());
                return Ok(Address::from(fixed_bytes));
            }
            Output::Call(o) => {
                error!("Contract deployment returned unexpected output: {:?}", o);
            }
        },
        ExecutionResult::Halt { reason, .. } => {
            error!("Contract deployment halted: {:?}", reason);
        }

        ExecutionResult::Revert { output, .. } => {
            error!("Contract deployment reverted: {:?}", output);
        }
    }

    Ok(Address::zero())
}

#[derive(Debug, Clone)]
pub struct TxResult {
    pub output: Bytes,
    pub logs: Option<Vec<Log>>,
    pub gas_used: u64,
    pub gas_refunded: u64,
}
