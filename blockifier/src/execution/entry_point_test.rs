use std::collections::HashMap;
use std::rc::Rc;

use pretty_assertions::assert_eq;
use starknet_api::core::{ClassHash, ContractAddress, EntryPointSelector};
use starknet_api::hash::StarkHash;
use starknet_api::shash;
use starknet_api::state::EntryPointType;
use starknet_api::transaction::CallData;

use crate::cached_state::{CachedState, DictStateReader};
use crate::execution::contract_class::ContractClass;
use crate::execution::entry_point::{CallEntryPoint, CallExecution, CallInfo};
use crate::transaction::transaction_utils::get_contract_class;

const TEST_CONTRACT_PATH: &str = "./feature_contracts/compiled/simple_contract_compiled.json";
const WITHOUT_ARG_SELECTOR: &str =
    "0x382a967a31be13f23e23a5345f7a89b0362cc157d6fbe7564e6396a83cf4b4f";
const WITH_ARG_SELECTOR: &str = "0xe7def693d16806ca2a2f398d8de5951344663ba77f340ed7a958da731872fc";
const BITWISE_AND_SELECTOR: &str =
    "0xad451bd0dba3d8d97104e1bfc474f88605ccc7acbe1c846839a120fdf30d95";
const SQRT_SELECTOR: &str = "0x137a07fa9c479e27114b8ae1fbf252f2065cf91a0d8615272e060a7ccf37309";
const RETURN_RESULT_SELECTOR: &str =
    "0x39a1491f76903a16feed0a6433bec78de4c73194944e1118e226820ad479701";
const GET_VALUE_SELECTOR: &str =
    "0x26813d396fdb198e9ead934e4f7a592a8b88a059e45ab0eb6ee53494e8d45b0";
const TEST_LIBRARY_CALL_SELECTOR: &str =
    "0x3604cea1cdb094a73a31144f14a3e5861613c008e1e879939ebc4827d10cd50";
const TEST_CLASS_HASH: &str = "0x1";
const TEST_CONTRACT_ADDRESS: &str = "0x100";

// TODO(Noa, 30/12/22): Move it to a test_utils.rs file and use it in cached_state_test.rs (same for
// the relevant constants above)
fn create_test_contract_class() -> ContractClass {
    get_contract_class(TEST_CONTRACT_PATH)
}

fn create_test_state() -> CachedState<DictStateReader> {
    let class_hash_to_class = HashMap::from([(
        ClassHash(shash!(TEST_CLASS_HASH)),
        Rc::new(create_test_contract_class()),
    )]);
    CachedState::new(DictStateReader { class_hash_to_class, ..Default::default() })
}

fn trivial_external_entrypoint() -> CallEntryPoint {
    CallEntryPoint {
        class_hash: ClassHash(shash!(TEST_CLASS_HASH)),
        entry_point_type: EntryPointType::External,
        entry_point_selector: EntryPointSelector(shash!(0)),
        calldata: CallData(vec![]),
        storage_address: ContractAddress::try_from(shash!(TEST_CONTRACT_ADDRESS)).unwrap(),
    }
}

#[test]
fn test_call_info() {
    let state = create_test_state();
    let entry_point = CallEntryPoint {
        entry_point_selector: EntryPointSelector(shash!(WITHOUT_ARG_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    let expected_call_info = CallInfo {
        call: entry_point.clone(),
        execution: CallExecution { retdata: vec![] },
        inner_calls: vec![],
    };
    assert_eq!(entry_point.execute(state).unwrap(), expected_call_info);
}

#[test]
fn test_entry_point_without_arg() {
    let state = create_test_state();
    let entry_point = CallEntryPoint {
        entry_point_selector: EntryPointSelector(shash!(WITHOUT_ARG_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(entry_point.execute(state).unwrap().execution, CallExecution { retdata: vec![] });
}

#[test]
fn test_entry_point_with_arg() {
    let state = create_test_state();
    let calldata = CallData(vec![shash!(25)]);
    let entry_point = CallEntryPoint {
        calldata,
        entry_point_selector: EntryPointSelector(shash!(WITH_ARG_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(entry_point.execute(state).unwrap().execution, CallExecution { retdata: vec![] });
}

#[test]
fn test_entry_point_with_builtin() {
    let state = create_test_state();
    let calldata = CallData(vec![shash!(47), shash!(31)]);
    let entry_point = CallEntryPoint {
        calldata,
        entry_point_selector: EntryPointSelector(shash!(BITWISE_AND_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(entry_point.execute(state).unwrap().execution, CallExecution { retdata: vec![] });
}

#[test]
fn test_entry_point_with_hint() {
    let state = create_test_state();
    let calldata = CallData(vec![shash!(81)]);
    let entry_point = CallEntryPoint {
        calldata,
        entry_point_selector: EntryPointSelector(shash!(SQRT_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(entry_point.execute(state).unwrap().execution, CallExecution { retdata: vec![] });
}

#[test]
fn test_entry_point_with_return_value() {
    let state = create_test_state();
    let calldata = CallData(vec![shash!(23)]);
    let entry_point = CallEntryPoint {
        calldata,
        entry_point_selector: EntryPointSelector(shash!(RETURN_RESULT_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(
        entry_point.execute(state).unwrap().execution,
        CallExecution { retdata: vec![shash!(23)] }
    );
}

#[test]
fn test_entry_point_not_found_in_contract() {
    let state = create_test_state();
    let entry_point = CallEntryPoint {
        entry_point_selector: EntryPointSelector(shash!(2)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(
        format!("Entry point {:#?} not found in contract.", entry_point.entry_point_selector),
        format!("{}", entry_point.execute(state).unwrap_err())
    );
}

#[test]
fn test_entry_point_with_syscall() {
    let state = create_test_state();
    let calldata = CallData(vec![shash!(1234)]);
    let entry_point = CallEntryPoint {
        calldata,
        entry_point_selector: EntryPointSelector(shash!(GET_VALUE_SELECTOR)),
        ..trivial_external_entrypoint()
    };
    assert_eq!(
        entry_point.execute(state).unwrap().execution,
        CallExecution { retdata: vec![shash!(18)] }
    );
}

#[test]
fn test_entry_point_with_library_call() {
    let state = create_test_state();
    let calldata = CallData(vec![
        shash!(TEST_CLASS_HASH),        // Class hash.
        shash!(RETURN_RESULT_SELECTOR), // Function selector.
        shash!(1),                      // Calldata length.
        shash!(91),                     // Calldata.
    ]);
    let entry_point = CallEntryPoint {
        entry_point_selector: EntryPointSelector(shash!(TEST_LIBRARY_CALL_SELECTOR)),
        calldata,
        ..trivial_external_entrypoint()
    };
    // TODO(AlonH, 21/12/2022): Compare the whole CallInfo.
    assert_eq!(entry_point.execute(state).unwrap().execution.retdata, vec![shash!(91)]);
}