extern crate env_logger;
extern crate gears;

use gears::structure::xflow::*;
use gears::validation::xflow::*;

mod common;
use common::load_doc;

#[test]
fn test_init_validation() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    assert_eq!(xfs.body.nodes.len(), 10);
}

#[test]
fn test_validations_ok() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");

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
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/edges_without_nodes.json");

    let res_a = Validation::all_edges_have_nodes(&xfs);

    assert_eq!(res_a.len(), 3);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/edges/(1, 4)");

}

#[test]
fn test_validations_all_nodes_have_edges() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/orphan_node.json");

    let res_a = Validation::all_nodes_have_at_least_one_edge(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/2");

}

#[test]
fn test_validations_has_one_entry_node() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/multiple_entry_nodes.json");

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_one_entry_node_ii() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/no_entry_nodes.json");

    let res_a = Validation::has_one_entry_node(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_validations_has_terminal_nodes() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/no_terminal_nodes.json");

    let res_a = Validation::has_terminal_nodes(&xfs);

    assert_eq!(res_a.len(), 1);
    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes");

}

#[test]
fn test_all_node_actions_have_matching_requirements() {
    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/bad_flows/bad_capabilities.json");

    let res_a = Validation::all_node_actions_have_matching_requirements(&xfs);

    assert_eq!(res_a.len(), 2);

    assert_eq!(res_a[0].code, 1);
    assert_eq!(res_a[0].paths[0], "/nodes/1");

    assert_eq!(res_a[1].code, 1);
    assert_eq!(res_a[1].paths[0], "/nodes/3");

}

#[test]
fn test_all_good_flows_validate() {
    let _ = env_logger::try_init();
    let flows = vec![
        "10_steps.json",
        "arithmetic_addition.json",
        "arithmetic_addition_multiple_return_values.json",
        "arithmetic_addition_with_variables.json",
        "branch_boolean_and_expressions_return.json",
        "branch_boolean_condition.json",
        "branch_boolean.json",
        // "create_object.json",
        "loop_5x.json",
        "loop_infinite.json",
    ];

    for flow in flows {
        let xfs =
            load_doc::<XFlowDocument>((format!("resource/docs/xflow/flows/{}", flow)).as_str());

        let res = Validation::validate(&xfs);
        if !res.is_empty() {
            println!(
                "ERROR: Flow listed as good does not validate : {} - {:?}",
                flow,
                res
            );
            assert!(false);
        }
    }
}

#[test]
fn test_no_bad_flows_validate() {
    let flows = vec![
        "bad_capabilities.json",
        "double_variables_per_scope.json",
        "edges_without_nodes.json",
        "empty.json",
        "multiple_entry_nodes.json",
        "no_entry_nodes.json",
        "no_terminal_nodes.json",
        "orphan_node.json",
        "output_variable_type_mismatch.json",
        "redefined_local_variables.json",
        "unreferenced_variables.json",
    ];

    for flow in flows {
        let xfs =
            load_doc::<XFlowDocument>((format!("resource/docs/xflow/bad_flows/{}", flow)).as_str());
        let res = Validation::validate(&xfs);

        if res.is_empty() {
            println!("ERROR: Flow listed as bad validates OK : {}", flow);
            assert!(false);
        }
    }

}
