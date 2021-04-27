#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, SessionBuilder, TestContextBuilder};
    use casper_types::{
        account::AccountHash, runtime_args, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    const COUNT_KEY: &str = "count";
    const COUNTER_INC: &str = "counter_inc";
    const COUNTER_KEY: &str = "counter";

    #[test]
    fn should_deploy_with_counter_zero() {
        let public_key: PublicKey = SecretKey::ed25519([1u8; 32]).into();
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from("counter-define.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(session);

        let counter_value: i32 = context
            .query(
                account_addr,
                &[COUNTER_KEY.to_string(), COUNT_KEY.to_string()],
            )
            .unwrap()
            .into_t()
            .unwrap();
        assert_eq!(counter_value, 0);
    }

    #[test]
    fn should_increment_with_direct_call() {
        let public_key: PublicKey = SecretKey::ed25519([1u8; 32]).into();
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from("counter-define.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(session);

        for expected_value in 1..3 {
            // Increment value using an EntryPoint.
            let session_code = Code::NamedKey(COUNTER_KEY.to_string(), COUNTER_INC.to_string());
            let session_args = runtime_args! {};
            let session = SessionBuilder::new(session_code, session_args)
                .with_address(account_addr)
                .with_authorization_keys(&[account_addr])
                .build();
            context.run(session);

            let counter_value: i32 = context
                .query(
                    account_addr,
                    &[COUNTER_KEY.to_string(), COUNT_KEY.to_string()],
                )
                .unwrap()
                .into_t()
                .unwrap();
            assert_eq!(counter_value, expected_value);
        }
    }

    #[test]
    fn should_increment_with_counter_call_contract() {
        let public_key: PublicKey = SecretKey::ed25519([1u8; 32]).into();
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from("counter-define.wasm");
        let session_args = runtime_args! {};
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(session);

        for expected_value in 1..3 {
            let session_code = Code::from("counter-call.wasm");
            let session_args = runtime_args! {};
            let session = SessionBuilder::new(session_code, session_args)
                .with_address(account_addr)
                .with_authorization_keys(&[account_addr])
                .build();
            context.run(session);

            let counter_value: i32 = context
                .query(
                    account_addr,
                    &[COUNTER_KEY.to_string(), COUNT_KEY.to_string()],
                )
                .unwrap()
                .into_t()
                .unwrap();
            assert_eq!(counter_value, expected_value);
        }
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
