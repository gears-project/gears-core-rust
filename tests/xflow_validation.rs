extern crate xflow;
use xflow::xfstruct::*;
use xflow::validation::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_init_validation() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);
    assert_eq!(xfs.nodes.len(), 10);
}

#[test]
fn test_validations_ok() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::all_edges_have_nodes(&xfs);
    assert_eq!(res_a.len(), 0);

    let res_a = Validation::has_one_entry_node(&xfs);
    assert_eq!(res_a.len(), 0);

    let res_a = Validation::has_terminal_nodes(&xfs);
    assert_eq!(res_a.len(), 0);

    let res_a = Validation::all_nodes_have_at_least_one_edge(&xfs);
    assert_eq!(res_a.len(), 0);

    let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);
    assert_eq!(res_a.len(), 0);
}

#[test]
fn test_validations_edges_have_nodes() {
    let json_string = read_json_file("data/bad_flows/edges_without_nodes.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::all_edges_have_nodes(&xfs);

    assert_eq!(res_a.len(), 3);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/edges/(1, 4)");

}

#[test]
fn test_validations_all_nodes_have_edges() {
    let json_string = read_json_file("data/bad_flows/orphan_node.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::all_nodes_have_at_least_one_edge(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/2");

}

#[test]
fn test_validations_has_one_entry_node() {
    let json_string = read_json_file("data/bad_flows/multiple_entry_nodes.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_one_entry_node_ii() {
    let json_string = read_json_file("data/bad_flows/no_entry_nodes.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_terminal_nodes() {
    let json_string = read_json_file("data/bad_flows/no_terminal_nodes.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::has_terminal_nodes(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_all_node_actions_have_matching_requirements() {
    let json_string = read_json_file("data/bad_flows/bad_capabilities.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);

    assert_eq!(res_a.len(), 2);

    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/1");

    assert_eq!(res_a[1].code, 1);
    assert_eq!(res_a[1].paths[0], "/nodes/3");

}
