extern crate xflow;
use xflow::*;

// fn read_json_file(filename:&str) -> String {
//     helper::read_json_file(filename)
// }

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
        Ok(_)    => {}, // print!("{} contains:\n{}", display, s),
    };

    s
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
    let left   = 5;
    let right  = 5;

    let mut edges = Vec::<XFlowEdge>::new();

    for i in 0..amount {
        edges.push((left, right + i));
    }

    edges
}

fn create_branches(amount:i32) -> Vec<XFlowBranch> {

    let mut branches = Vec::<XFlowBranch>::new();

    for i in 0..amount {
        let left   = 5;
        let right  = 5;
        branches.push(
            XFlowBranch {
                name: "Some branch".to_string(),
                edge : (left, right + i)
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

    assert!(xfs.get_entry_node().is_ok());
    assert!(xfs.get_terminal_nodes().is_ok());

    match xfs.get_entry_node() {
        Ok(res)  => assert_eq!(res.id, 1),
        Err(err) => println!("Error: {:?}", err),
    }

    match xfs.get_terminal_nodes() {
        Ok(res)  => assert_eq!(res.len(), 1),
        Err(err) => println!("Error: {:?}", err),
    }

    match xfs.get_entry_node() {
        Ok(res)  => {

            let in_edges  = xfs.get_in_edges(res);
            let out_edges = xfs.get_out_edges(res);

            assert_eq!(in_edges.len(), 0);
            assert_eq!(out_edges.len(), 1);

            assert_eq!(xfs.get_branches_for(out_edges[0]).len(), 0);

        },
        Err(err) => println!("Error: {:?}", err),
    }
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

#[test]
fn test_mem_profile() {
    use std;
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);

    assert_eq!(std::mem::size_of_val(&xfs), 224);

    // println!("size of `10 steps flow` in bytes: {}", std::mem::size_of_val(&xfs));
}
