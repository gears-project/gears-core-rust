extern crate env_logger;
extern crate uuid;

extern crate gears;
use gears::structure::xflow::*;

mod common;
use common::load_doc;

fn create_node(id: i32) -> XFlowNode {
    XFlowNode {
        id: id,
        nodetype: XFlowNodeType::Flow,
        action: "some action".to_string(),
        label: "some name".to_string(),
        parameters: XFlowNodeParameters::Flow(FlowParameters::default()),
    }
}

fn create_nodes(amount: i32) -> Vec<XFlowNode> {
    let mut nodes = Vec::<XFlowNode>::new();

    for i in 0..amount {
        nodes.push(create_node(i))
    }

    nodes
}

fn create_edges(amount: i32) -> Vec<XFlowEdge> {
    let left = 5;
    let right = 5;

    let mut edges = Vec::<XFlowEdge>::new();

    for i in 0..amount {
        edges.push((left, right + i));
    }

    edges
}

fn create_branches(amount: i32) -> Vec<XFlowBranch> {

    let mut branches = Vec::<XFlowBranch>::new();

    for i in 0..amount {
        let left = 5;
        let right = 5;
        branches.push(XFlowBranch {
                          xvar: XFlowVariable {
                              name: "MatchValue".to_string(),
                              vtype: XFlowValueType::String,
                              value: XFlowValue::String("Some branch".to_owned()),
                          },
                          edge: (left, right + i),
                      })
    }

    branches
}

fn create_xflow_struct() -> XFlowDocument {
    XFlowDocument {
        id: uuid::Uuid::new_v4(),
        name: "Some name".to_string(),
        version: 1,
        doctype: "Some doctype".to_string(),
        doctype_version: 1,
        doc: XFlow {
            requirements: Vec::<XFlowRequirement>::new(),
            variables: XFlowVariables {
                input: Vec::<XFlowVariableDefinition>::new(),
                local: Vec::<XFlowVariable>::new(),
                output: Vec::<XFlowVariableDefinition>::new(),
            },
            nodes: create_nodes(5),
            edges: create_edges(5),
            branches: create_branches(5),
        },
    }
}

#[test]
fn test_xfs() {
    let _ = env_logger::init();

    let xfs = create_xflow_struct();

    assert_eq!(xfs.doc.nodes.len(), 5);
    assert_eq!(xfs.doc.edges.len(), 5);
    assert_eq!(xfs.doc.branches.len(), 5);
}

#[test]
fn test_xfs_fields() {
    let _ = env_logger::init();

    let xfs = create_xflow_struct();

    assert_eq!(xfs.version, 1);
    assert_eq!(xfs.name, "Some name");
}

#[test]
// #[should_panic]
fn test_xfs_entry() {
    let _ = env_logger::init();
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    assert_eq!(xfs.doc.get_nodes_by(&XFlowNodeType::Flow, "start").len(), 1);
}

#[test]
fn test_xfs_doc_get_nodes_of_type() {
    let _ = env_logger::init();
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    assert_eq!(xfs.doc.get_nodes_of_type(&XFlowNodeType::Flow).len(), 2);
}

#[test]
// #TST-serialization-json
fn test_xfs_from_json() {
    let _ = env_logger::init();
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    assert_eq!(xfs.name, "steps".to_string());
    assert_eq!(xfs.doc.nodes.len(), 10);
    assert_eq!(xfs.doc.edges.len(), 9);
    assert_eq!(xfs.doc.branches.len(), 0);

    assert_eq!(xfs.doc.requirements.len(), 2);

    assert_eq!(xfs.doc.variables.input.len(), 1);
    assert_eq!(xfs.doc.variables.local.len(), 0);
    assert_eq!(xfs.doc.variables.output.len(), 1);

    assert!(xfs.doc.get_entry_node().is_ok());
    assert!(xfs.doc.get_terminal_nodes().is_ok());

    match xfs.doc.get_entry_node() {
        Ok(res) => assert_eq!(res.id, 1),
        Err(err) => println!("Error: {:?}", err),
    }

    match xfs.doc.get_terminal_nodes() {
        Ok(res) => assert_eq!(res.len(), 1),
        Err(err) => println!("Error: {:?}", err),
    }

    match xfs.doc.get_entry_node() {
        Ok(res) => {

            let in_edges = xfs.doc.get_in_edges(res);
            let out_edges = xfs.doc.get_out_edges(res);

            assert_eq!(in_edges.len(), 0);
            assert_eq!(out_edges.len(), 1);

            assert_eq!(xfs.doc.get_branches_for(out_edges[0]).len(), 0);

        }
        Err(err) => println!("Error: {:?}", err),
    }
}

#[test]
fn test_xfs_from_json_string() {
    let _ = env_logger::init();
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/empty.json");

    assert_eq!(xfs.name, "empty".to_string());
    assert_eq!(xfs.doc.nodes.len(), 0);
    assert_eq!(xfs.doc.edges.len(), 0);
    assert_eq!(xfs.doc.branches.len(), 0);
}

#[test]
fn test_mem_profile() {
    let _ = env_logger::init();
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");

    assert_eq!(std::mem::size_of_val(&xfs), 248);
}
