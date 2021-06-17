#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    use casper_types::{
        account::AccountHash, runtime_args, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    const COUNT_KEY: &str = "count";
    const COUNTER_INC: &str = "counter_inc";
    const COUNTER_RESET: &str = "counter_reset";

    pub struct Context {
        pub context: TestContext,
        pub account_addr: AccountHash,
    }

    impl Context {
        pub fn new() -> Self {
            let public_key: PublicKey = SecretKey::ed25519_from_bytes([1u8; 32]).unwrap().into();
            let account_addr = AccountHash::from(&public_key);
            let context = TestContextBuilder::new()
                .with_public_key(public_key, U512::from(100_000_000_000_000u64))
                .build();
            Self {
                context,
                account_addr,
            }
        }

        pub fn deploy_v1(&mut self) {
            self.deploy_wasm("counter-v1-install.wasm");
        }

        pub fn deploy_v2(&mut self) {
            self.deploy_wasm("counter-v2-reset.wasm");
        }

        pub fn increment(&mut self) {
            self.call_counter(COUNTER_INC);
        }

        pub fn reset_counter(&mut self) {
            self.call_counter(COUNTER_RESET);
        }

        pub fn get_counter(&self) -> i32 {
            self.context
                .query(self.account_addr, &[COUNT_KEY.to_string()])
                .unwrap()
                .into_t()
                .unwrap()
        }

        fn deploy_wasm(&mut self, wasm_name: &str) {
            self.deploy_wasm_with_args(wasm_name, RuntimeArgs::new());
        }

        fn deploy_wasm_with_args(&mut self, wasm_name: &str, session_args: RuntimeArgs) {
            let session_code = Code::from(wasm_name);
            let define_session = SessionBuilder::new(session_code, session_args)
                .with_address(self.account_addr)
                .with_authorization_keys(&[self.account_addr])
                .build();
            self.context.run(define_session);
        }

        fn call_counter(&mut self, entrypoint_name: &str) {
            self.deploy_wasm_with_args(
                "counter-call.wasm",
                runtime_args! {
                    "entrypoint_name" => entrypoint_name
                },
            );
        }
    }

    #[test]
    fn should_deploy_with_counter_zero() {
        let mut context = Context::new();
        context.deploy_v1();
        assert_eq!(context.get_counter(), 0);
    }

    #[test]
    fn should_increment_in_v1() {
        let mut context = Context::new();
        context.deploy_v1();
        for expected_value in 1..=3 {
            context.increment();
            assert_eq!(context.get_counter(), expected_value);
        }
    }

    #[test]
    fn should_increment_in_v2() {
        let mut context = Context::new();
        context.deploy_v1();
        for expected_value in 1..=3 {
            context.increment();
            assert_eq!(context.get_counter(), expected_value);
        }
        context.deploy_v2();
        for expected_value in 4..=7 {
            context.increment();
            assert_eq!(context.get_counter(), expected_value);
        }
    }

    #[test]
    fn should_reset_in_v2() {
        let mut context = Context::new();
        context.deploy_v1();
        context.deploy_v2();
        for expected_value in 1..=3 {
            context.increment();
            assert_eq!(context.get_counter(), expected_value);
        }
        context.reset_counter();
        assert_eq!(context.get_counter(), 0);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
