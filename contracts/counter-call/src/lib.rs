#![no_std]

extern crate alloc;

use types::{
    ApiError,
    Key,
    ContractHash,
    runtime_args::RuntimeArgs
};

use contract::{
    contract_api::runtime,
    unwrap_or_revert::UnwrapOrRevert,

};

const COUNTER_KEY: &str = "counter";
const GET_METHOD: &str = "get";
const INC_METHOD: &str = "inc";
const ENTRY_POINT: &str = "call";

#[no_mangle]
pub extern "C" fn call() {
    let counter_uref = runtime::get_key(COUNTER_KEY).unwrap_or_revert_with(ApiError::GetKey);
    let contract_hash = if let Key::Hash(hash) = counter_uref {
        ContractHash::new(hash)
    } else {
        runtime::revert(ApiError::User(66))
    };

    {        
        let mut args = RuntimeArgs::new();
        args.insert("INC_METHOD", INC_METHOD).unwrap_or_revert_with(ApiError::InvalidArgument);
        runtime::call_contract(contract_hash, ENTRY_POINT, args)
    }

    let _result: i32 = {
        let mut args = RuntimeArgs::new();
        args.insert("GET_METHOD", GET_METHOD).unwrap_or_revert_with(ApiError::InvalidArgument);
        runtime::call_contract(contract_hash, ENTRY_POINT, args,)
    };
}
