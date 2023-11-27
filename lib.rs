#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {

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
        pub fn update_timestamp(&mut self) {
            let current_timestamp = self.env().block_timestamp().clone() as u128;
            let timestamp_delta = (current_timestamp - self.last_timestamp) as u128;

            let multiplication =1000000000000000000000000000000u128.checked_mul(timestamp_delta).unwrap_or_else(|| panic!("Multiplication overflow"));
            let division = multiplication.checked_div(1000000000000).unwrap_or_else(|| panic!("Division overflow"));

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
        fn bugged_value() -> Result<(), Box<dyn Error>> {
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

            for _ in 1..1000 {
                let timestamps: (u128, u128) = session.call("get_timestamps", NO_ARGS, None)??;

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
    }
}
