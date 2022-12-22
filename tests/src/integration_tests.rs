#[cfg(test)]
mod tests {
    // Outlining aspects of the Casper test support crate to include.
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    // Custom Casper types that will be used within this test.
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
    /// Install version 1 of the counter contract and check its available entry points.
    /// Only the increment entry point should be available.
    /// The decrement call should fail, because that entry point should not be in this version.
    /// Test summary:
    /// - Install the counter-v1.wasm contract.
    /// - Check the contract hash.
    /// - Check the contract version is 1.
    /// - Verify the initial value of count is 0.
    /// - Test the counter_inc entry point and increment the counter.
    /// - Verify that the count value is now 1.
    /// - Call the decrement entry point, which should fail.
    /// - Ensure the count value was not decremented and is still 1.
    fn install_version1_and_check_entry_points() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Install the contract.
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

        // Check the contract hash.
        let contract_v1_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1.
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
            .expect("count uref should exist in the contract named keys");

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

        // Verify the value of count is now 1.
        let incremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 1);

        // Call the decrement entry point, which should not be in this version.
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash,
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        // Try executing the decrement entry point and expect an error.
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        // Ensure the count value was not decremented.
        let current_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(current_count, 1);
    }

    #[test]
    /// Install version 1 of the counter contract and check its functionality.
    /// Then, upgrade the contract by installing a second Wasm for version 2.
    /// Check the functionality of the second version.
    /// Test summary:
    /// - Install the counter-v1.wasm contract.
    /// - Check the contract hash.
    /// - Check the contract version is 1.
    /// - Verify the initial value of count is 0.
    /// - Test the counter_inc entry point and increment the counter.
    /// - Verify that the count value is now 1.
    /// - Call the decrement entry point, which should fail.
    /// - Ensure the count value was not decremented and is still 1.
    /// - UPGRADE the contract by installing the counter-v2.wasm.
    /// - Assert that we have a new contract hash for the upgraded version.
    /// - Verify the new contract version is 2.
    /// - Increment the counter to check that counter_inc is still working after the upgrade. Count is now 2.
    /// - Call the decrement entry point and verify that the count is now 1.
    fn install_version1_and_upgrade_to_version2() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Install the first version of the contract.
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

        // Check the contract hash.
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
            .expect("count uref should exist in the contract named keys");

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

        // Verify the value of count is now 1.
        let incremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 1);

        // Call the decrement entry point, which should not be in version 1 before the upgrade.
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash,
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        // Try executing the decrement entry point and expect an error.
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        // Ensure the count value was not decremented.
        let current_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(current_count, 1);

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

        // Expect the counter to be 1 now.
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

    #[test]
    /// Install version 2 of the counter contract without having version 1 already on chain.
    /// Test summary:
    /// - Install the counter-v2.wasm contract.
    /// - Check the contract hash exists.
    /// - Check the contract version is 2, because the code installs and upgrades under the hood.
    /// - Start checking the entry points. Verify the initial value of count is 0.
    /// - Test the counter_inc entry point and increment the counter.
    /// - Verify that the count value is now 1.
    /// - Call the decrement entry point, which should succeed.
    /// - Verify that the count is 0.
    fn install_version2_directly_without_version1() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // Install the contract.
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

        // Check the contract hash exists.
        let contract_v2_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

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

        // Verify the initial value of count is 0.
        let contract = builder
            .get_contract(contract_v2_hash)
            .expect("this contract should exist");

        let count_key = *contract
            .named_keys()
            .get(COUNT_KEY)
            .expect("count uref should exist in the contract named keys");

        let count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(count, 0);

        // Check that the increment entry point is working.
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v2_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify the value of count is now 1.
        let incremented_count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.");

        assert_eq!(incremented_count, 1);

        // Test the decrement entry point to decrement the value stored under "count".
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

        assert_eq!(decremented_count, 0);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
