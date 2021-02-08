#![no_std]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};
use types::{
    api_error::ApiError,
    URef,
    Key,
    contracts::{EntryPoint, EntryPoints},
    CLValue
};
use contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

const COUNT_KEY: &str = "count";
const COUNTER_ENTRY: &str = "counter_ext";
const COUNTER_KEY: &str = "counter";
const GET_METHOD: &str = "get";
const INC_METHOD: &str = "inc";

enum Arg {
    MethodName = 0,
}

#[repr(u16)]
enum Error {
    UnknownMethodName = 0,
}

impl Into<ApiError> for Error {
    fn into(self) -> ApiError {
        ApiError::User(self as u16)
    }
}

#[no_mangle]
pub extern "C" fn counter_ext() {
    let uref: URef = runtime::get_key(COUNT_KEY).unwrap().into_uref().unwrap();
    let method_name: String = runtime::get_named_arg(Arg::MethodName as u32)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    match method_name.as_str() {
        INC_METHOD => storage::add(uref, 1),
        GET_METHOD => {
            let result: i32 = storage::read(uref)
                .unwrap_or_revert_with(ApiError::Read)
                .unwrap_or_revert_with(ApiError::ValueNotFound);
            let typed_result = CLValue::from_t(result)
                .unwrap_or_revert();
            runtime::ret(typed_result);
        }
        _ => runtime::revert(Error::UnknownMethodName),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = storage::new_uref(0); //initialize counter

    //create map of references for stored contract
    let mut counter_named_keys: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, counter_local_key.into());

    // create entry point
    let counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::default_with_name(COUNTER_ENTRY));

    let stored_contract_hash = storage::new_contract(counter_entry_points, Some(counter_named_keys), None, None);
    runtime::put_key(COUNTER_KEY, stored_contract_hash.0.into());
}
