#[cfg(test)]
mod tests {

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs};

    const COUNTER_DEFINE_WASM: &str = "counter-define.wasm"; // The main example contract
    const COUNTER_CALL_WASM: &str = "counter-call.wasm"; // The session code that calls the contract

    const CONTRACT_KEY: &str = "counter"; // Named key referencing this contract
    const COUNT_KEY: &str = "count"; // Named key referencing the count value
    const CONTRACT_VERSION_KEY: &str = "version"; // Automatically incremented version in a contract package

    #[test]
    fn should_be_able_to_install_and_increment() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        let contract_installation_request = ExecuteRequestBuilder::standard( 
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_DEFINE_WASM,
            runtime_args! {},
        )
        .build();

        builder
            .exec(contract_installation_request)
            .expect_success()
            .commit();

        let contract_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // Verify the first contract version is 1. We'll check this when we upgrade later

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

        // Verify the initial value of count is 0

        let contract = builder
            .get_contract(contract_hash)
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

        // Use session code to increment the counter

        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_hash
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
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
