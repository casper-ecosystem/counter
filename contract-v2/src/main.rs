#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{vec::Vec};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef,
};

const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";
const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement";

const COUNTER_PACKAGE_NAME: &str = "counter_package_name";
const CONTRACT_VERSION_KEY: &str = "version";
const COUNT_KEY: &str = "count";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, 1); // Increment the count by 1.
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
    runtime::ret(typed_result); // Return the count value.
}

#[no_mangle]
pub extern "C" fn counter_decrement() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, -1); // Decrement the count.
}

#[no_mangle]
pub extern "C" fn call() {
    // In this version, we will not add any new named keys so we can comment this out.
    // let count_start = storage::new_uref(0_i32);
    // let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    // let key_name = String::from(COUNT_KEY);
    // counter_named_keys.insert(key_name, count_start.into());

    // Create a new entry point list for this contract.
    let mut counter_entry_points = EntryPoints::new();

    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_INC,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Create an entry point to decrement the counter by 1.
    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_DECREMENT,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Get the counter package hash so we can upgrade the package.
    let counter_package_hash = runtime::get_key(COUNTER_PACKAGE_NAME)
    .unwrap_or_revert()
    .into_hash()
    .unwrap()
    .into();

    // Add a new contract version to the package.
    // This contract will have a new entry point; but, it will not add named keys.
    let (stored_contract_hash, contract_version) = storage::add_contract_version(
        counter_package_hash,
        counter_entry_points,   // New list of entry points
        NamedKeys::default());  // Default named keys

    // Here we are updating the version named key with a new value. 
    // The version named key should already be part of the account.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // We don't need to re-create a named key for the contract hash in this version.
    // The key should already exist and we will have access to it in this version.
    // runtime::put_key("counter", stored_contract_hash.into());
}
