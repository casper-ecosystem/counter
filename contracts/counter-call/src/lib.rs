#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::vec::Vec;

use contract_ffi::contract_api::pointers::ContractPointer;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

#[no_mangle]
pub extern "C" fn call() {
    let counter_uref = contract_api::get_key("counter").unwrap_or_revert_with(Error::User(100));
    let pointer = if let Key::Hash(hash) = counter_uref {
        ContractPointer::Hash(hash)
    } else {
        contract_api::revert(Error::User(66))
    };

    {
        let arg = "inc";
        contract_api::call_contract::<_, ()>(pointer.clone(), &(arg,), &Vec::new())
    }

    let _result: i32 = {
        let arg = "get";
        contract_api::call_contract(pointer, &(arg,), &Vec::new())
    };
}
