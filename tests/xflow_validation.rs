extern crate env_logger;
extern crate xflow;

use xflow::structure::xflow::*;
use xflow::validation::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_init_validation() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowDocument::from_json(&json_string);
    assert_eq!(xfs.doc.nodes.len(), 10);
}

#[test]
fn test_validations_ok() {
    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowDocument::from_json(&json_string);

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
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::all_edges_have_nodes(&xfs);

    assert_eq!(res_a.len(), 3);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/edges/(1, 4)");

}

#[test]
fn test_validations_all_nodes_have_edges() {
    let json_string = read_json_file("data/bad_flows/orphan_node.json");
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::all_nodes_have_at_least_one_edge(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/2");

}

#[test]
fn test_validations_has_one_entry_node() {
    let json_string = read_json_file("data/bad_flows/multiple_entry_nodes.json");
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_one_entry_node_ii() {
    let json_string = read_json_file("data/bad_flows/no_entry_nodes.json");
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_terminal_nodes() {
    let json_string = read_json_file("data/bad_flows/no_terminal_nodes.json");
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::has_terminal_nodes(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_all_node_actions_have_matching_requirements() {
    let json_string = read_json_file("data/bad_flows/bad_capabilities.json");
    let xfs = XFlowDocument::from_json(&json_string);

    let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);

    assert_eq!(res_a.len(), 2);

    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/1");

    assert_eq!(res_a[1].code, 1);
    assert_eq!(res_a[1].paths[0], "/nodes/3");

}

#[test]
fn test_all_good_flows_validate() {
    let _ = env_logger::init();
    let flows = vec!["10_steps.json",
                     "arithmetic_addition.json",
                     "arithmetic_addition_multiple_return_values.json",
                     "arithmetic_addition_with_variables.json",
                     "branch_boolean_and_expressions_return.json",
                     "branch_boolean_condition.json",
                     "branch_boolean.json",
                     "create_object.json",
                     "loop_5x.json",
                     "loop_infinite.json"];

    for flow in flows {
        let json_string = read_json_file((format!("data/flows/{}", flow)).as_str());
        let xfs = XFlowDocument::from_json(&json_string);

        let res = Validation::validate(&xfs);
        if !res.is_empty() {
            println!("ERROR: Flow listed as good does not validate : {} - {:?}",
                     flow,
                     res);
            assert!(false);
        }
    }
}

#[test]
fn test_no_bad_flows_validate() {
    let flows = vec!["bad_capabilities.json",
                     "double_variables_per_scope.json",
                     "edges_without_nodes.json",
                     "empty.json",
                     "multiple_entry_nodes.json",
                     "no_entry_nodes.json",
                     "no_terminal_nodes.json",
                     "orphan_node.json",
                     "output_variable_type_mismatch.json",
                     "redefined_local_variables.json",
                     "unreferenced_variables.json"];

    for flow in flows {
        let json_string = read_json_file((format!("data/bad_flows/{}", flow)).as_str());
        let xfs = XFlowDocument::from_json(&json_string);

        let res = Validation::validate(&xfs);

        if res.is_empty() {
            println!("ERROR: Flow listed as bad validates OK : {}", flow);
            assert!(false);
        }
    }

}
