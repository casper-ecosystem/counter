#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
        DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
        DEFAULT_PAYMENT,
    };
    use casper_execution_engine::core::engine_state::{
        run_genesis_request::RunGenesisRequest, GenesisAccount,
    };
    use casper_types::{
        account::AccountHash, runtime_args, Key, Motes, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    /// The name of the compiled counter-define smart contract file.
    const COUNTER_DEFINE_WASM: &str = "counter-define.wasm";
    /// The name of the compiled counter-call smart contract file.
    const COUNTER_CALL_WASM: &str = "counter-call.wasm";
    /// The name of the named-key under which the actual counter is stored.
    const COUNT_KEY: &str = "count";
    /// The name of the runtime arg of the stored contract used to increment the actual counter.
    const COUNTER_INC: &str = "counter_inc";
    /// The name of the named key under which the stored contract is referenced.
    const COUNTER_KEY: &str = "counter";

    struct TestFixture {
        test_builder: InMemoryWasmTestBuilder,
        account_address: AccountHash,
    }

    impl TestFixture {
        /// Initialize the test fixture by setting up a genesis account, running the genesis request
        /// and installing the counter smart contract.
        fn deploy() -> Self {
            // Create an asymmetric keypair, and derive the account address of this.
            let secret_key = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap();
            let public_key = PublicKey::from(&secret_key);
            let account_address = AccountHash::from(&public_key);

            // Make this account a genesis account (one which exists at network startup) and a
            // genesis request for the execution engine.
            let account = GenesisAccount::account(
                public_key,
                Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
                None,
            );

            let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
            genesis_config.ee_config_mut().push_account(account);

            let run_genesis_request = RunGenesisRequest::new(
                *DEFAULT_GENESIS_CONFIG_HASH,
                genesis_config.protocol_version(),
                genesis_config.take_ee_config(),
            );

            // Create a new WasmTestBuilder (a wrapper for the execution engine) and execute the
            // genesis request to initialize global state.
            let mut test_builder = InMemoryWasmTestBuilder::default();
            test_builder.run_genesis(&run_genesis_request).commit();

            // Create a deploy which will "install" the smart contract by executing the
            // counter-define contract (which takes no args).
            //
            // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.
            // Paths relative to the current working dir (e.g. 'wasm/contract.wasm') can also be
            // used, as can absolute paths.
            let installer_session_code = COUNTER_DEFINE_WASM;
            let installer_session_args = runtime_args! {};
            let installer_payment_args = runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            };
            let deploy_item = DeployItemBuilder::new()
                .with_empty_payment_bytes(installer_payment_args)
                .with_session_code(installer_session_code, installer_session_args)
                .with_authorization_keys(&[account_address])
                .with_address(account_address)
                .build();

            // Create an execute request from this deploy.
            let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

            // Execute the installer contract.
            test_builder.exec(execute_request).commit().expect_success();
            let test_context = Self {
                test_builder,
                account_address,
            };

            // Assert the contract has installed as expected by querying global state for the
            // newly-initialized count of 0.
            let initial_count = test_context.get_counter();
            assert_eq!(initial_count, 0);

            test_context
        }

        /// Query latest global state under the account using a path of "counter/count".  This
        /// should yield the value of the data stored there.
        fn get_counter(&self) -> i32 {
            self.test_builder
                .query(
                    None,
                    Key::Account(self.account_address),
                    &[COUNTER_KEY.to_string(), COUNT_KEY.to_string()],
                )
                .expect("should be stored value.")
                .as_cl_value()
                .expect("should be cl value.")
                .clone()
                .into_t::<i32>()
                .expect("should be i32.")
        }

        /// Increment the counter.
        ///
        /// If `use_stored_session` is `true`, the deploy used here is constructed to call the
        /// previously-stored contract, i.e. no Wasm is included in the deploy itself.  If it's
        /// `false`, then the counter-call contract is used as the session code for the deploy.
        fn increment_counter(&mut self, use_stored_session: bool) {
            let session_args = runtime_args! {};
            let payment_args = runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            };
            let deploy_item_builder = DeployItemBuilder::new()
                .with_empty_payment_bytes(payment_args)
                .with_authorization_keys(&[self.account_address])
                .with_address(self.account_address);
            let deploy_item = if use_stored_session {
                deploy_item_builder.with_stored_session_named_key(
                    COUNTER_KEY,
                    COUNTER_INC,
                    session_args,
                )
            } else {
                deploy_item_builder.with_session_code(COUNTER_CALL_WASM, session_args)
            }
            .build();

            let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

            self.test_builder
                .exec(execute_request)
                .commit()
                .expect_success();
        }
    }

    #[test]
    fn should_deploy_with_counter_zero() {
        assert_eq!(TestFixture::deploy().get_counter(), 0);
    }

    #[test]
    fn should_increment_with_direct_call() {
        let mut fixture = TestFixture::deploy();
        let use_stored_session = true;
        for expected_value in 1..=3 {
            fixture.increment_counter(use_stored_session);
            assert_eq!(fixture.get_counter(), expected_value);
        }
    }

    #[test]
    fn should_increment_with_counter_call_contract() {
        let mut fixture = TestFixture::deploy();
        let use_stored_session = false;
        for expected_value in 1..=3 {
            fixture.increment_counter(use_stored_session);
            assert_eq!(fixture.get_counter(), expected_value);
        }
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
