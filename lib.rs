#![cfg_attr(not(feature = "std"), no_std, no_main)]
extern crate alloc;

#[ink::contract]
mod flipper {

    // use std::result::Result;
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TestError {
        MULOverflow,
        DIVOverflow,
        ADDOverflow,
        SUBOverflow,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct Flipper {
        last_timestamp: u128,
        value: u128,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                last_timestamp: Self::env().block_timestamp() as u128,
                value: u128::default(),
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                last_timestamp: Self::env().block_timestamp() as u128,
                value: u128::default(),
            }
        }

        #[ink(message)]
        pub fn update_timestamp_diffrent_operations_mul(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication =
                match 1000000000000000000000000000000u128.checked_mul(timestamp_delta) {
                    Some(result) => result,
                    None => return Err(TestError::MULOverflow),
                };

            let value = match multiplication.checked_mul(1000000000000) {
                Some(result) => result,
                None => return Err(TestError::MULOverflow),
            };

            // To don't let compiler ignore the value
            ink::env::debug_println!("value = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp_diffrent_operations_add(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication =
                match 1000000000000000000000000000000u128.checked_mul(timestamp_delta) {
                    Some(result) => result,
                    None => return Err(TestError::MULOverflow),
                };

            let value = match multiplication.checked_add(1000000000000) {
                Some(result) => result,
                None => return Err(TestError::ADDOverflow),
            };

            // To don't let compiler ignore the value
            ink::env::debug_println!("value = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp_without_store(&mut self) -> Result<(), TestError> {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication =
                match 1000000000000000000000000000000u128.checked_mul(timestamp_delta) {
                    Some(result) => result,
                    None => return Err(TestError::MULOverflow),
                };

            let value = match multiplication.checked_div(1000000000000) {
                Some(result) => result,
                None => return Err(TestError::DIVOverflow),
            };

            // To don't let compiler ignore the division
            ink::env::debug_println!("division = {}", value);
            Ok(())
        }

        #[ink(message)]
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            // 10^30 * delta
            let multiplication = 1000000000000000000000000000000u128 * (timestamp_delta);
            let division = multiplication / (1000000000000);

            let value = division;
            self.last_timestamp = current_timestamp as u128;
            self.value = value;
        }

        #[ink(message)]
        pub fn get_timestamps(&self) -> (u128, u128) {
            (self.last_timestamp, self.value)
        }
    }

    #[cfg(test)]
    mod tests {
        use std::error::Error;

        use drink::{
            runtime::MinimalRuntime,
            session::{Session, NO_ARGS},
        };

        #[drink::contract_bundle_provider]
        enum BundleProvider {}

        #[drink::test]
        fn test_update_timestamp() -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::Flipper.bundle()?;

            let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
                contract,
                "new",
                NO_ARGS,
                vec![],
                None,
            )?;

            let timestamps: (u128, u128) = session.call("get_timestamps", NO_ARGS, None)??;

            let mut last_timestamp = timestamps.0;

            for _ in 1..200 {
                let timestamps: (u128, u128) = session.call("get_timestamps", NO_ARGS, None)??;

                // Timestamp diff appears to be always 0
                println!(
                    "timestamp diff = {:?}  [{:?} - {:?}]",
                    timestamps.0 - last_timestamp,
                    timestamps.0,
                    last_timestamp
                );

                last_timestamp = timestamps.0;

                session.call("update_timestamp", NO_ARGS, None)??;
            }

            Ok(())
        }

        #[drink::test]
        fn test_update_timestamp_without_updating_state() -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::Flipper.bundle()?;

            let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
                contract,
                "new",
                NO_ARGS,
                vec![],
                None,
            )?;

            for _ in 1..200 {
                session.call("update_timestamp_without_store", NO_ARGS, None)??;
            }

            Ok(())
        }

        #[drink::test]
        fn test_update_timestamp_add_operation() -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::Flipper.bundle()?;

            let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
                contract,
                "new",
                NO_ARGS,
                vec![],
                None,
            )?;

            for _ in 1..200 {
                session.call("update_timestamp_diffrent_operations_add", NO_ARGS, None)??;
            }

            Ok(())
        }

        #[drink::test]
        fn test_update_timestamp_mul_operation() -> Result<(), Box<dyn Error>> {
            let contract = BundleProvider::Flipper.bundle()?;

            let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
                contract,
                "new",
                NO_ARGS,
                vec![],
                None,
            )?;

            for _ in 1..200 {
                session.call("update_timestamp_diffrent_operations_mul", NO_ARGS, None)??;
            }

            Ok(())
        }
    }
}
