use std::error::Error;

use drink::{
    runtime::MinimalRuntime,
    session::{Session, NO_ARGS},
};

#[drink::contract_bundle_provider]
enum BundleProvider {}

#[drink::test]
fn basic_operations() -> Result<(), Box<dyn Error>> {
    let contract = BundleProvider::Flipper.bundle()?;

    let mut session = Session::<MinimalRuntime>::new()?.deploy_bundle_and(
        contract,
        "new",
        NO_ARGS,
        vec![],
        None,
    )?;

    let get: u32 = session.call("get", NO_ARGS, None)??;
    assert_eq!(get, 0);

    session.call("set", &["2"], None)??;

    let get: u32 = session.call("get_and_print", NO_ARGS, None)??;
    assert_eq!(get, 2);

    Ok(())
}
