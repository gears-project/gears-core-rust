extern crate xfdocs;
use xfdocs::xflow::xfstruct::*;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_json_file(filename:&str) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
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

//
// Test code
//

fn x_get_rand() -> i32 {
    5 // rand::thread_rng().gen_range(1, 11)
}

fn create_node(id:i32) -> XFlowNode {
    XFlowNode {
        id: id,
        nodetype: "flow".to_string(),
        action: "some action".to_string(),
        label: "some name".to_string()
    }
}

fn create_nodes(amount:i32) -> Vec<XFlowNode> {
    let mut nodes = Vec::<XFlowNode>::new();

    for i in 0..amount {
        nodes.push(create_node(i))
    }

    return nodes
}

fn create_edges(amount:i32) -> Vec<XFlowEdge> {
    let left   = x_get_rand();
    let right  = x_get_rand();

    let mut edges = Vec::<XFlowEdge>::new();

    for i in 0..amount {
        edges.push([left, right + i]);
    }

    edges
}

fn create_branches(amount:i32) -> Vec<XFlowBranch> {

    let mut branches = Vec::<XFlowBranch>::new();

    for i in 0..amount {
        let left   = x_get_rand();
        let right  = x_get_rand();
        branches.push(
            XFlowBranch {
                name: "Some branch".to_string(),
                edge : [left, right + i]
            })
    }

    branches
}

fn create_xflow_struct() -> XFlowStruct {
    XFlowStruct {
        id:       "id1".to_string(),
        version:  1,
        name:     "Some name".to_string(),
        requirements: Vec::<XFlowRequirement>::new(),
        variables:    XFlowVariables {
            input:  Vec::<XFlowVariable>::new(),
            local:  Vec::<XFlowVariable>::new(),
            output: Vec::<XFlowVariableDefinition>::new(),
        },
        nodes:        create_nodes(5),
        edges:        create_edges(5),
        branches:     create_branches(5)
    }
}

#[test]
fn test_xfs() {
    let xfs = create_xflow_struct();

    assert_eq!(xfs.nodes.len(), 5);
    assert_eq!(xfs.edges.len(), 5);
    assert_eq!(xfs.branches.len(), 5);
}

#[test]
fn test_xfs_fields() {
    let xfs = create_xflow_struct();

    assert_eq!(xfs.version, 1);
    assert_eq!(xfs.id, "id1".to_string());
    assert_eq!(xfs.name, "Some name");
}

#[test]
// #[should_panic]
fn test_xfs_entry() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);

    assert_eq!(xfs.get_nodes_by("flow", "start").len(), 1);
}

#[test]
fn test_xfs_get_nodes_of_type() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);

    assert_eq!(xfs.get_nodes_of_type("flow").len(), 2);
}

#[test]
fn test_xfs_from_json() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);

    assert_eq!(xfs.name, "steps".to_string());
    assert_eq!(xfs.nodes.len(), 10);
    assert_eq!(xfs.edges.len(), 9);
    assert_eq!(xfs.branches.len(), 0);

    assert_eq!(xfs.requirements.len(), 2);

    assert_eq!(xfs.variables.input.len(), 1);
    assert_eq!(xfs.variables.local.len(), 0);
    assert_eq!(xfs.variables.output.len(), 1);

    // assert_eq!(xfs.get_entry_nodes().len(), 1);
    // assert_eq!(xfs.get_terminal_nodes().len(), 1);


}

#[test]
fn test_xfs_from_json_string() {
    let empty_flow = read_json_file("data/flows/empty.json");
    let xfs = XFlowStruct::from_json(&empty_flow);

    assert_eq!(xfs.name, "empty".to_string());
    assert_eq!(xfs.nodes.len(), 0);
    assert_eq!(xfs.edges.len(), 0);
    assert_eq!(xfs.branches.len(), 0);
}



