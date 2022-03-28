#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Key, URef,
};

const CONTRACT_VERSION_KEY: &str = "contract_version";

const COUNT_KEY: &str = "count";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";
const COUNTER_KEY: &str = "counter";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn counter_get() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let result: i32 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn call() {

    // Create a contract package for this contract and save its hash value.
    let (contract_package_hash, _): (ContractPackageHash, URef) =
    storage::create_contract_package_at_hash();
 
    // Initialize counter to 0.
    let counter_local_key = storage::new_uref(0_i32);

    // Create initial named keys of the contract.
    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, counter_local_key.into());

    // Create entry points to get the counter value and to increment the counter by 1.
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_INC,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    counter_entry_points.add_entry_point(EntryPoint::new(
        COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create a contract that can be versioned
    let (stored_contract_hash, contract_version) =
        storage::add_contract_version(contract_package_hash, counter_entry_points, counter_named_keys);
    let version_uref = storage::new_uref(contract_version);

    // This is the code for a locked contract, kept here for comparison
    // let (stored_contract_hash, _) =
    //    storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None);

    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());
    runtime::put_key(COUNTER_KEY, stored_contract_hash.into());
}
