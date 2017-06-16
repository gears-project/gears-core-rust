extern crate env_logger;

extern crate xflow;
use xflow::structure::xflow::*;

#[test]
fn test_xflow_default() {
    let _ = env_logger::init();

    let xfs = XFlowStruct::default();

    assert_eq!(xfs.name, "");
    assert_eq!(xfs.version, 1);
}

#[test]
fn test_xflow_to_json() {
    let _ = env_logger::init();

    let xfs = XFlowStruct::default();

    assert_eq!(xfs.to_json(), r#"{"id":"","version":1,"name":"","requirements":[],"variables":{"input":[],"local":[],"output":[]},"nodes":[],"edges":[],"branches":[]}"#);
}

#[test]
fn test_xflow_to_str() {
    let _ = env_logger::init();

    let xfs = XFlowStruct::default();

    assert_eq!(xfs.to_string(), r#"xflow "#);
}
