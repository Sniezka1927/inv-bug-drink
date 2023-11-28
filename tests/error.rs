use std::error::Error;

use drink::{
    runtime::MinimalRuntime,
    session::{Session, NO_ARGS},
};

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test]
fn error_handling() -> Result<(), Box<dyn Error>> {
    let contract = BundleProvider::Flipper.bundle()?;

    let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
        contract,
        "new",
        NO_ARGS,
        vec![],
        None,
    )?;

    let get = session.call("err_return", NO_ARGS, None)??;
    // .unwrap_err();
    // println!("Get = {:?}", get);

    Ok(())
}
