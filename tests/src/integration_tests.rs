#[cfg(test)]
mod tests {

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    const COUNTER_V1_WASM: &str = "counter-v1.wasm"; // The first version of the contract
    const COUNTER_V2_WASM: &str = "counter-v2.wasm"; // The second version of the contract
    const COUNTER_CALL_WASM: &str = "counter-call.wasm"; // Session code that calls the contract

    const CONTRACT_KEY: &str = "counter"; // Named key referencing this contract
    const COUNT_KEY: &str = "count"; // Named key referencing the value to increment/decrement
    const CONTRACT_VERSION_KEY: &str = "version"; // Key maintaining the version of a contract package

    const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement"; // Entry point to decrement the count value
    const ENTRY_POINT_COUNTER_INC: &str = "counter_inc"; // Entry point to increment the count value

    #[test]
    fn should_install_and_upgrade() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        let contract_v1_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_V1_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_v1_installation_request)
            .expect_success()
            .commit();

        let contract_v1_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1. We'll check this when we upgrade later.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        let version = builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.");

        assert_eq!(version, 1);

        // Verify the initial value of count is 0.
        let contract = builder
            .get_contract(contract_v1_hash)
            .expect("this contract should exist");

        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("count uref should exis in the contract named keys");

        let count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(count, 0);

        // Use session code to increment the counter.
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v1_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify the value of count is now 1
        let incremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 1);

        ////////////////////////////////////////////////////////////////
        // Upgrade the contract.
        ////////////////////////////////////////////////////////////////
        let contract_v2_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_V2_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_v2_installation_request)
            .expect_success()
            .commit();

        let contract_v2_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Assert that we have a new contract hash for the upgraded version.
        assert_ne!(contract_v1_hash, contract_v2_hash);

        // Verify the contract version is now 2.
        let account = builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account");

        let version_key = *account
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        let version = builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.");

        assert_eq!(version, 2);

        // Call the increment entry point to increment the value stored under "count".
        let contract_increment_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v2_hash,
            ENTRY_POINT_COUNTER_INC, 
            runtime_args! {},
        )
        .build();

        builder
        .exec(contract_increment_request)
        .expect_success()
        .commit();

        // Call the decrement entry point to decrement the value stored under "count".
        let contract_call_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v2_hash,
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_call_request)
            .expect_success()
            .commit();

        // Expect the counter to be 0 now.
        // This tells us the contract was successfully upgraded and the decrement entry point can be called.
        let decremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(decremented_count, 1);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
