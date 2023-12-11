//! Custom print inspector, it has step level information of execution.
//! It is a great tool if some debugging is needed.

use revm::interpreter::{opcode, CallInputs, CreateInputs, Gas, InstructionResult, Interpreter};
use revm::primitives::{Address, Bytes, U256};
use revm::{inspectors::GasInspector, Database, EVMData, Inspector};

/// Custom print [Inspector], it has step level information of execution.
///
/// It is a great tool if some debugging is needed.
#[derive(Clone, Debug, Default)]
pub struct CustomPrintTracer {
    gas_inspector: GasInspector,
}

impl<DB: Database> Inspector<DB> for CustomPrintTracer {
    fn initialize_interp(&mut self, interp: &mut Interpreter<'_>, data: &mut EVMData<'_, DB>) {
        self.gas_inspector.initialize_interp(interp, data);
    }

    // get opcode by calling `interp.contract.opcode(interp.program_counter())`.
    // all other information can be obtained from interp.
    fn step(&mut self, interp: &mut Interpreter<'_>, data: &mut EVMData<'_, DB>) {
        let opcode = interp.current_opcode();
        let opcode_str = opcode::OPCODE_JUMPMAP[opcode as usize];

        let gas_remaining = self.gas_inspector.gas_remaining();

        if opcode_str.unwrap() == "RETURN" {
            println!(
            "depth:{}, PC:{}, gas:{:#x}({}), OPCODE: {:?}({:?})  refund:{:#x}({}) Stack:{:?}, Data size:{}",
            data.journaled_state.depth(),
            interp.program_counter(),
            gas_remaining,
            gas_remaining,
            opcode_str.unwrap_or("UNKNOWN"),
            opcode,
            interp.gas.refunded(),
            interp.gas.refunded(),
            interp.stack.data(),
            interp.shared_memory.len(),
        );
        }

        self.gas_inspector.step(interp, data);
    }

    fn step_end(&mut self, interp: &mut Interpreter<'_>, data: &mut EVMData<'_, DB>) {
        self.gas_inspector.step_end(interp, data);
    }

    fn call_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CallInputs,
        remaining_gas: Gas,
        ret: InstructionResult,
        out: Bytes,
    ) -> (InstructionResult, Gas, Bytes) {
        self.gas_inspector
            .call_end(data, inputs, remaining_gas, ret, out.clone());
        (ret, remaining_gas, out)
    }

    fn create_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CreateInputs,
        ret: InstructionResult,
        address: Option<Address>,
        remaining_gas: Gas,
        out: Bytes,
    ) -> (InstructionResult, Option<Address>, Gas, Bytes) {
        self.gas_inspector
            .create_end(data, inputs, ret, address, remaining_gas, out.clone());
        (ret, address, remaining_gas, out)
    }

    fn call(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        inputs: &mut CallInputs,
    ) -> (InstructionResult, Gas, Bytes) {
        println!(
            "SM CALL:   {:?}, context:{:?}, is_static:{:?}, transfer:{:?}, input_size:{:?}",
            inputs.contract, inputs.context, inputs.is_static, inputs.transfer, inputs.input,
        );
        (InstructionResult::Continue, Gas::new(0), Bytes::new())
    }

    fn create(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        inputs: &mut CreateInputs,
    ) -> (InstructionResult, Option<Address>, Gas, Bytes) {
        println!(
            "CREATE CALL: caller:{:?}, scheme:{:?}, value:{:?}, init_code:{:?}, gas:{:?}",
            inputs.caller, inputs.scheme, inputs.value, inputs.init_code, inputs.gas_limit
        );
        (InstructionResult::Continue, None, Gas::new(0), Bytes::new())
    }

    fn selfdestruct(&mut self, contract: Address, target: Address, value: U256) {
        println!(
            "SELFDESTRUCT: contract: {:?}, refund target: {:?}, value {:?}",
            contract, target, value
        );
    }
}
