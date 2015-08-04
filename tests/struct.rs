extern crate xfdocs;
use xfdocs::xflow::xfstruct::{XFlowStruct};

#[test]
fn test_xfs() {
    let xfs = XFlowStruct::new();
    println!("Hello, xflow {:?}", xfs.to_string());
    assert_eq!(xfs.nodes.len(), 5);
    assert_eq!(xfs.edges.len(), 5);
    assert_eq!(xfs.branches.len(), 5);
}

