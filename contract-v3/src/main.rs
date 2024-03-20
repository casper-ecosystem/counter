#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef,
};

///  Constands for contract entry points
const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";
const ENTRY_POINT_COUNTER_LAST_UPDATED_AT: &str = "counter_last_updated_at";
const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement";

/// Constants for the keys pointing to values stored in the contract's named keys.
const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "counter";
const COUNT_KEY: &str = "count";
const LAST_UPDATED_KEY: &str = "last_updated";

/// Constants for the keys pointing to values stored in the account's named keys.
const CONTRACT_PACKAGE_NAME: &str = "counter_package_name";
const CONTRACT_ACCESS_UREF: &str = "counter_access_uref";

/// Entry point that increments the count value by 1.
#[no_mangle]
pub extern "C" fn counter_inc() {
    let count_uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(count_uref, 1);

    let last_updated_uref = runtime::get_key(LAST_UPDATED_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let timestamp: u64 = runtime::get_blocktime().into();
    storage::write(last_updated_uref, timestamp);
}

/// Entry point that returns the count value.
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

/// Entry point that returns the last updated timestamp of the counter.
#[no_mangle]
pub extern "C" fn counter_last_updated_at() {
    let uref: URef = runtime::get_key(LAST_UPDATED_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let result: i32 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result); // Return the count value.
}

/// Entry point that decrements the count value by 1.
#[no_mangle]
pub extern "C" fn counter_decrement() {
    let count_uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(count_uref, -1); // Decrement the count.

    let last_updated_uref = runtime::get_key(LAST_UPDATED_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let timestamp: u64 = runtime::get_blocktime().into();
    storage::write(last_updated_uref, timestamp);
}

/// Helper function that installs the counter contract on chain.
fn install_counter() {
    // Initialize the count to 0, locally.
    let count_start = storage::new_uref(0_i32);
    let last_updated = storage::new_uref(0_u64);

    // In the named keys of the contract, add keys for `count` and `last_updated`.
    let mut counter_named_keys = NamedKeys::new();
    counter_named_keys.insert(String::from(COUNT_KEY), count_start.into());
    counter_named_keys.insert(String::from(LAST_UPDATED_KEY), last_updated.into());

    // Create the entry points for this contract.
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

    // Create a new contract package that can be upgraded.
    let (stored_contract_hash, contract_version) = storage::new_contract(
        counter_entry_points,
        Some(counter_named_keys),
        Some("counter_package_name".to_string()),
        Some("counter_access_uref".to_string()),
    );

    /* To create a locked contract instead, use new_locked_contract and throw away the contract version returned
    let (stored_contract_hash, _) =
        storage::new_locked_contract(counter_entry_points, Some(counter_named_keys), None, None); */

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}

/// Helper function that upgrades the contract package to a new version.
fn upgrade_counter() {
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

    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_LAST_UPDATED_AT,
        Vec::new(),
        CLType::U64,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // Get the counter package hash so we can upgrade the package.
    let counter_package_hash = runtime::get_key(CONTRACT_PACKAGE_NAME)
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    // Get the existing named keys and add a new one to it
    let mut named_keys = NamedKeys::default();
    let last_updated = storage::new_uref(0_u64);
    named_keys.insert(String::from(LAST_UPDATED_KEY), last_updated.into());

    // Add a new contract version to the package with the new list of entry points.
    let (stored_contract_hash, contract_version) =
        storage::add_contract_version(counter_package_hash, counter_entry_points, named_keys);

    // Here we are updating the version named key with a new value.
    // The version named key should already be part of the account.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Add the latest contract hash into the named key.
    // The key should already exist and we will have access to it in this version.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}

/// Entry point that executes automatically when a caller installs the contract.
#[no_mangle]
pub extern "C" fn call() {
    match runtime::get_key(CONTRACT_ACCESS_UREF) {
        None => {
            // The given key doesn't exist, so install the contract.
            install_counter();
            // Next, upgrade the contract.
            upgrade_counter();
        }
        Some(_contract_key) => {
            // The stored contract and key exist, so upgrade the contract.
            upgrade_counter();
        }
    }
}
