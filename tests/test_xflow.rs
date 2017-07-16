extern crate env_logger;

extern crate gears;
use gears::structure::xflow::*;

#[test]
fn test_xflow_default() {
    let _ = env_logger::init();

    let xfs = XFlowDocument::default();

    assert_eq!(xfs.name, "");
    assert_eq!(xfs.version, 1);
}

#[test]
fn test_xflow_to_json() {
    let _ = env_logger::init();

    let xfs_a = XFlowDocument::default();
    let xfs_b = XFlowDocument::from_json(&xfs_a.to_json());
    assert_eq!(xfs_a.id, xfs_b.id);
}

#[test]
fn test_xflow_to_str() {
    let _ = env_logger::init();

    let xfs = XFlowDocument::default();

    assert_eq!(xfs.to_string(), r#"document "#);
}
