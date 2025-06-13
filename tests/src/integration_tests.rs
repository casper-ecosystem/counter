#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        ExecuteRequestBuilder, LmdbWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, LOCAL_GENESIS_REQUEST,
    };
    use casper_types::{contracts::ContractHash, runtime_args};

    // Contract Wasm File Paths (Constants)
    const COUNTER_V1_WASM: &str = "counter-v1.wasm";
    const COUNTER_V2_WASM: &str = "counter-v2.wasm";
    const COUNTER_V3_WASM: &str = "counter-v3.wasm";
    const COUNTER_CALL_WASM: &str = "counter-call.wasm";

    // Contract Storage Keys (Constants)
    const CONTRACT_KEY: &str = "counter";
    const COUNT_KEY: &str = "count";
    const LAST_UPDATED_KEY: &str = "last_updated";
    const CONTRACT_VERSION_KEY: &str = "version";

    // Contract Entry Points (Constants)
    const ENTRY_POINT_COUNTER_DECREMENT: &str = "counter_decrement";
    const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
    #[cfg(test)]
    const ENTRY_POINT_COUNTER_LAST_UPDATED_AT: &str = "counter_last_updated_at";

    // Helper Functions

    /// Deploys a contract version to the LmdbWasmTestBuilder
    fn deploy_contract(builder: &mut LmdbWasmTestBuilder, wasm_code: &str) -> ContractHash {
        let request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, wasm_code, runtime_args! {})
                .build();
        builder.exec(request).expect_success().commit();
        get_contract_hash_from_account(builder, CONTRACT_KEY)
    }

    /// Retrieves the contract hash from the default account's storage by a given key
    fn get_contract_hash_from_account(
        builder: &mut LmdbWasmTestBuilder,
        key: &str,
    ) -> ContractHash {
        builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("must have account")
            .named_keys()
            .get(key)
            .expect("must have contract hash key")
            .into_hash_addr()
            .map(ContractHash::new)
            .expect("must get contract hash")
    }

    /// Retrieves the value stored under the `count` key in the given contract
    fn get_count(builder: &mut LmdbWasmTestBuilder, contract_hash: ContractHash) -> i32 {
        let count_key = *builder
            .get_contract(contract_hash)
            .expect("this contract should exist")
            .named_keys()
            .get(COUNT_KEY)
            .expect("count uref should exist in the contract named keys");

        builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<i32>()
            .expect("should be i32.")
    }

    /// Retrieves the value stored under the `last_updated_at` key in the given contract (if present)
    fn get_last_updated_at(builder: &mut LmdbWasmTestBuilder, contract_hash: ContractHash) -> u64 {
        let count_key = *builder
            .get_contract(contract_hash)
            .expect("this contract should exist")
            .named_keys()
            .get(LAST_UPDATED_KEY)
            .expect("count uref should exist in the contract named keys");

        builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u64>()
            .expect("should be u64.")
    }

    /// Retrieves the contract version stored in the default account's storage
    fn get_contract_version(builder: &mut LmdbWasmTestBuilder) -> u32 {
        let version_key = *builder
            .get_account(*DEFAULT_ACCOUNT_ADDR)
            .expect("should have account")
            .named_keys()
            .get(CONTRACT_VERSION_KEY)
            .expect("version uref should exist");

        builder
            .query(None, version_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u32>()
            .expect("should be u32.")
    }

    #[test]
    fn install_and_test_version1() {
        // Test summary:
        // - Installs and verifies counter contract v1.
        // - Verifies initial contract version and count value (0).
        // - Tests counter_inc entry point in v1 and updates count.
        // - Attempts (and expects failure) to call counter_decrement_at (not present in v1).
        // - Ensures count value remains unchanged after failed decrement attempt.

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // Deploy the counter contract (v1)
        let contract_v1_hash = deploy_contract(&mut builder, COUNTER_V1_WASM);

        // Verify the initial contract version is 1
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 1);

        // Verify the initial value of count is 0
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 0);

        // Use session code to increment the counter
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
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);

        // Call the counter_decrement entry point (not in v1)
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash.into(),
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        // Expect an error since counter_decrement_at doesn't exist in v1
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        // Ensure the count value remains 1 after failed decrement attempt
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);
    }

    #[test]
    fn install_and_test_version1_and_upgrade_to_version2() {
        // Test summary:
        // - Installs and verifies counter contract v1.
        // - Tests counter_inc entry point in v1.
        // - Attempts (and expects failure) to call counter_decrement_at (not present in v1).
        // - Upgrades the contract to v2.
        // - Verifies contract version is now 2.
        // - Tests counter_inc entry point in v2.
        // - Tests counter_decrement_at entry point in v2 (successful after upgrade).

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // Deploy the counter contract (v1)
        let contract_v1_hash = deploy_contract(&mut builder, COUNTER_V1_WASM);

        // Verify the initial contract version is 1
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 1);

        // Verify the initial value of count is 0
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 0);

        // Use session code to increment the counter
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
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);

        // Call the counter_decrement entry point (not in v1)
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v1_hash.into(),
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        // Expect an error since counter_decrement_at doesn't exist in v1
        builder
            .exec(contract_decrement_request)
            .expect_failure()
            .commit();

        // Ensure the count value remains 1 after failed decrement attempt
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);

        // Deploy the v2 contract (should upgrade from v1)
        let contract_v2_hash = deploy_contract(&mut builder, COUNTER_V2_WASM);
        assert_ne!(contract_v1_hash, contract_v2_hash);

        // Verify the contract version is now 2 (after upgrade)
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 2);

        // Use session code to increment the counter
        let contract_increment_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v2_hash.into(),
            ENTRY_POINT_COUNTER_INC,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_increment_request)
            .expect_success()
            .commit();

        // Verify the count is now 2
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 2);

        // Test counter_decrement_at entry point in v2 (should work now)
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v2_hash.into(),
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_decrement_request)
            .expect_success()
            .commit();

        // Expect the counter to be 1 now
        // This tells us the decrement functionality is available
        let count = get_count(&mut builder, contract_v1_hash);
        assert_eq!(count, 1);
    }

    #[test]
    fn install_and_test_version2() {
        // Test summary:
        // - Directly installs and verifies counter contract v2 (skipping v1).
        // - Verifies contract version is 2.
        // - Verifies initial count value is 0.
        // - Tests counter_inc entry point and updates count.
        // - Tests counter_decrement_at entry point and updates count.

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // Deploy the counter contract (v2) - no v1 installation
        let contract_v2_hash = deploy_contract(&mut builder, COUNTER_V2_WASM);

        // Verify the contract version is now 2 (directly installed)
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 2);

        // Verify the initial value of count is 0
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 0);

        // Use session code to increment the counter
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v2_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify the value of count is now 1
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 1);

        // Test decrement functionality (assumed to be present in v2)
        let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v2_hash.into(),
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_decrement_request)
            .expect_success()
            .commit();

        // Expect the counter to be 0 now
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 0);
    }

    #[test]
    fn install_and_test_version2_and_upgrade_to_version3() {
        // Test summary:
        // - Installs and verifies counter contract v2.
        // - Tests counter_inc entry point in v2.
        // - Attempts (and expects failure) to call counter_last_updated_at (not present in v2).
        // - Upgrades the contract to v3.
        // - Verifies contract version is now 3.
        // - Checks that count value remains 1 after upgrade.
        // - Verifies last_updated_at functionality in v3 (initially 0).
        // - Tests counter_inc entry point in v3 and updates last_updated_at.

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // Deploy the counter contract (v2)
        let contract_v2_hash = deploy_contract(&mut builder, COUNTER_V2_WASM);

        // Verify the initial contract version is 2
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 2);

        // Verify the initial value of count is 0
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 0);

        // Use session code to increment the counter
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v2_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify the value of count is now 1
        let count = get_count(&mut builder, contract_v2_hash);
        assert_eq!(count, 1);

        // Call the counter_last_updated_at entry point (not in v2)
        let contract_counter_last_updated_at_request =
            ExecuteRequestBuilder::contract_call_by_hash(
                *DEFAULT_ACCOUNT_ADDR,
                contract_v2_hash.into(),
                ENTRY_POINT_COUNTER_LAST_UPDATED_AT,
                runtime_args! {},
            )
            .build();

        // Expect an error since counter_last_updated_at doesn't exist in v2
        builder
            .exec(contract_counter_last_updated_at_request)
            .expect_failure()
            .commit();

        // Deploy the v3 contract (should upgrade from v2)
        let contract_v3_hash = deploy_contract(&mut builder, COUNTER_V3_WASM);
        assert_ne!(contract_v2_hash, contract_v3_hash);

        // Verify the contract version is now 3 (after upgrade)
        let version = get_contract_version(&mut builder);
        assert_eq!(version, 3);

        // Verify the count value remains 1 after upgrade
        let count = get_count(&mut builder, contract_v3_hash);
        assert_eq!(count, 1);

        // Verify last_updated_at is 0 initially in v3
        let last_updated_at = get_last_updated_at(&mut builder, contract_v3_hash);
        assert_eq!(last_updated_at, 0);

        // Call the increment entry point to increment the value stored under "count"
        let contract_increment_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v3_hash.into(),
            ENTRY_POINT_COUNTER_INC,
            runtime_args! {},
        )
        .with_block_time(10)
        .build();

        builder
            .exec(contract_increment_request)
            .expect_success()
            .commit();

        // Verify the count is now 2
        let count = get_count(&mut builder, contract_v3_hash);
        assert_eq!(count, 2);

        // Verify the last updated_at is now 10
        let last_updated_at = get_last_updated_at(&mut builder, contract_v3_hash);
        assert_eq!(last_updated_at, 10);
    }

    #[test]
    fn install_and_test_version3() {
        // Test summary:
        // - Deploys the counter v3 contract.
        // - Verifies initial state and version.
        // - Tests increment and decrement entry points.
        // - Asserts correct state updates, including last_updated_at.

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // Deploy the contract
        let contract_v3_hash = deploy_contract(&mut builder, COUNTER_V3_WASM);

        // Verify the contract version
        let contract_version = get_contract_version(&mut builder);
        assert_eq!(contract_version, 2);

        // Verify the initial value of count
        let initial_count = get_count(&mut builder, contract_v3_hash);
        assert_eq!(initial_count, 0);

        // Call the increment entry point
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! { CONTRACT_KEY => contract_v3_hash },
        )
        .with_block_time(1)
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // Verify count is now 1 and last_updated_at is 1
        let count = get_count(&mut builder, contract_v3_hash);
        assert_eq!(count, 1);
        let last_updated_at = get_last_updated_at(&mut builder, contract_v3_hash);
        assert_eq!(last_updated_at, 1);

        // Call the decrement entry point
        let contract_call_request = ExecuteRequestBuilder::contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            contract_v3_hash.into(),
            ENTRY_POINT_COUNTER_DECREMENT,
            runtime_args! {},
        )
        .with_block_time(2)
        .build();

        builder
            .exec(contract_call_request)
            .expect_success()
            .commit();

        // Verify count is back to 0 and last_updated_at is 2
        let count = get_count(&mut builder, contract_v3_hash);
        assert_eq!(count, 0);
        let last_updated_at = get_last_updated_at(&mut builder, contract_v3_hash);
        assert_eq!(last_updated_at, 2);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
