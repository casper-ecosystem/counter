#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use contract_ffi::{
    contract_api::{runtime, storage, Error as ApiError, TURef},
    key::Key,
    unwrap_or_revert::UnwrapOrRevert,
};

const COUNT_KEY: &str = "count";
const COUNTER_EXT: &str = "counter_ext";
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
    let turef: TURef<i32> = runtime::get_key(COUNT_KEY).unwrap().to_turef().unwrap();
    let method_name: String = runtime::get_arg(Arg::MethodName as u32)
        .unwrap_or_revert_with(ApiError::MissingArgument)
        .unwrap_or_revert_with(ApiError::InvalidArgument);
    match method_name.as_str() {
        INC_METHOD => storage::add(turef, 1),
        GET_METHOD => {
            let result = storage::read(turef)
                .unwrap_or_revert_with(ApiError::Read)
                .unwrap_or_revert_with(ApiError::ValueNotFound);
            runtime::ret(result, Vec::new());
        }
        _ => runtime::revert(Error::UnknownMethodName),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = storage::new_turef(0); //initialize counter

    //create map of references for stored contract
    let mut counter_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from(COUNT_KEY);
    counter_urefs.insert(key_name, counter_local_key.into());

    let pointer = storage::store_function_at_hash(COUNTER_EXT, counter_urefs);
    runtime::put_key(COUNTER_KEY, &pointer.into());
}
