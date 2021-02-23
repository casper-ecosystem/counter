#![no_main]
#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Key, URef,
};

const COUNT_KEY: &str = "count";
const COUNTER_INC: &str = "counter_inc";
const COUNTER_GET: &str = "counter_get";
const COUNTER_KEY: &str = "counter";

#[no_mangle]
pub extern "C" fn counter_inc() {
    let uref: URef = runtime::get_key(COUNT_KEY).unwrap().into_uref().unwrap();
    storage::add(uref, 1);
}

#[no_mangle]
pub extern "C" fn counter_get() {
    let uref: URef = runtime::get_key(COUNT_KEY).unwrap().into_uref().unwrap();
    let result: i32 = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = storage::new_uref(0); //initialize counter

    //create map of references for stored contract
    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, counter_local_key.into());

    // create entry point
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

    let stored_contract_hash =
        storage::new_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key(COUNTER_KEY, stored_contract_hash.0.into());
}
