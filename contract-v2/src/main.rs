#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// This code imports necessary aspects of external crates that we will use in our contract code.
extern crate alloc;

// Importing Rust types.
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
// Importing aspects of the Casper platform.
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
// Importing specific Casper types.
use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys},
    CLType, CLValue, URef,
};

// Creating constants for the various contract entry points.
const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";
const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement";

// Constants for the keys pointing to values stored in the contract's named keys.
const CONTRACT_VERSION_KEY: &str = "version";
const CONTRACT_KEY: &str = "counter";
const COUNT_KEY: &str = "count";

// Constants for the keys pointing to values stored in the account's named keys.
const CONTRACT_PACKAGE_NAME: &str = "counter_package_name";
const CONTRACT_ACCESS_UREF: &str = "counter_access_uref";

// Entry point that increments the count value by 1.
#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, 1); // Increment the count by 1.
}

// Entry point that returns the count value.
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

// Entry point that decrements the count value by 1.
#[no_mangle]
pub extern "C" fn counter_decrement() {
    let uref: URef = runtime::get_key(COUNT_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    storage::add(uref, -1); // Decrement the count.
}

// Helper function that installs the counter contract on chain.
fn install_counter() {
    // Initialize the count to 0, locally.
    let count_start = storage::new_uref(0_i32);

    // In the named keys of the contract, add a key for the count.
    let mut counter_named_keys = NamedKeys::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, count_start.into());

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

// Helper function that upgrades the contract package to a new version.
fn upgrade_counter() {
    // In this version, we will not add any named keys.
    // The named keys from the previous version should still be available.
    // Create a new entry point list that includes counter_decrement.
    // We need to specify all entry points, including the ones from the previous version.
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
    let counter_package_hash = runtime::get_key(CONTRACT_PACKAGE_NAME)
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    // Add a new contract version to the package with the new list of entry points.
    let (stored_contract_hash, contract_version) = storage::add_contract_version(
        counter_package_hash,
        counter_entry_points,
        NamedKeys::default(),
    );

    // Here we are updating the version named key with a new value.
    // The version named key should already be part of the account.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Add the latest contract hash into the named key.
    // The key should already exist and we will have access to it in this version.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}

// Entry point that executes automatically when a caller installs the contract.
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
