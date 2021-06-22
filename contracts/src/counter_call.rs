#![no_main]

extern crate alloc;

use casper_types::{runtime_args::RuntimeArgs, ContractPackageHash};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

const PACKAGE: &str = "counter_package";
const ARG_ENTRYPOINT: &str = "entrypoint_name";

#[no_mangle]
pub extern "C" fn call() {
    // Get entrypoint name.
    let entrypoint: String = runtime::get_named_arg(ARG_ENTRYPOINT);

    // Read the Counter's PackageHash.
    let counter_package_hash: ContractPackageHash = runtime::get_key(PACKAGE)
        .unwrap_or_revert()
        .into_hash()
        .unwrap_or_revert()
        .into();

    // Call Counter package.
    let _: () = runtime::call_versioned_contract(
        counter_package_hash,
        None,
        &entrypoint,
        RuntimeArgs::new(),
    );
}
