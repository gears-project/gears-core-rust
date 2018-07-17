extern crate env_logger;

extern crate gears;
use gears::structure::xflow::*;

// partof: #TST-xflow
// partof: TST-serialization

#[test]
fn test_xflow_default() {
    let _ = env_logger::try_init();

    let xfs = XFlowDocument::default();

    assert_eq!(xfs.name, "default");
    assert_eq!(xfs.version, 1);
}

#[test]
fn test_xflow_to_json() {
    let _ = env_logger::try_init();

    let xfs_a = XFlowDocument::default();
    let xfs_b = XFlowDocument::from_json(&xfs_a.to_json());
    assert_eq!(xfs_a.id, xfs_b.id);
}
