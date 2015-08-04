extern crate xfdocs;
use xfdocs::xflow::xfstruct::{XFlowStruct};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file_mf() -> String {
    // Create a path to the desired file
    let path = Path::new("data/flows/10_steps.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            Error::description(&why)
            ),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_)    => print!("{} contains:\n{}", display, s),
    };

    s


}

#[test]
fn test_xfs() {
    let xfs = XFlowStruct::new();

    assert_eq!(xfs.nodes.len(), 5);
    assert_eq!(xfs.edges.len(), 5);
    assert_eq!(xfs.branches.len(), 5);
}

#[test]
fn test_xfs_fields() {
    let xfs = XFlowStruct::new();

    assert_eq!(xfs.version, 1);
    assert_eq!(xfs.id, "id1".to_string());
    assert_eq!(xfs.name, "Some name");
}

#[test]
#[should_panic]
fn test_xfs_entry() {
    let xfs = XFlowStruct::new();

    assert_eq!(xfs.get_entry_nodes(), 5);
}

#[test]
fn test_xfs_from_json() {
    let json_string = read_file_mf();
    let xfs = XFlowStruct::from_json(json_string);

    assert_eq!(xfs.name, "steps".to_string());
    assert_eq!(xfs.nodes.len(), 10);
    assert_eq!(xfs.edges.len(), 9);
    assert_eq!(xfs.branches.len(), 0);
}



