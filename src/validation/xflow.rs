use structure::xflow::*;
use flox;
use std::collections::{HashSet, HashMap};

use validation::common::ValidationError;

#[derive(Debug)]
pub struct Validation {
    pub errors: Vec<ValidationError>,
}

impl Validation {
    pub fn validate(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_edges_have_nodes(doc));
        errors.extend(Validation::has_one_entry_node(doc));
        errors.extend(Validation::has_terminal_nodes(doc));
        errors.extend(Validation::all_nodes_have_at_least_one_edge(doc));
        errors.extend(Validation::all_node_actions_have_matching_requirements(doc));
        errors.extend(Validation::variables_are_defined_only_once(doc));
        errors.extend(Validation::all_return_values_exist(doc));
        errors.extend(Validation::no_variable_redefinition(doc));
        errors.extend(Validation::all_flox_variables_exist(doc));

        errors
    }

    pub fn all_edges_have_nodes(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut node_ids = doc.body
            .nodes
            .iter()
            .map({
                |node| node.id
            })
            .collect::<Vec<i32>>();

        node_ids.sort();
        node_ids.dedup();

        for edge in &doc.body.edges {

            if !node_ids.contains(&edge.0) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("Edge {:?} has no connecting node {:?}", edge, edge.0),
                    paths: vec![format!("/edges/{:?}", edge)],
                });
            };

            if !node_ids.contains(&edge.1) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("Edge {:?} has no connecting node {:?}", edge, edge.1),
                    paths: vec![format!("/edges/{:?}", edge)],
                });
            };
        }

        errors
    }

    pub fn has_one_entry_node(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = doc.body.get_nodes_by(&XFlowNodeType::Flow, "start");
        match res.len() {
            0 => {
                errors.push(ValidationError {
                    code: 1,
                    message: "XFlow has no entry nodes".into(),
                    paths: vec!["/nodes".into()],
                });
            }
            1 => {}
            _ => {
                // XXX: Add multiple paths
                //
                errors.push(ValidationError {
                    code: 1,
                    message: "XFlow has multiple entry nodes".into(),
                    paths: vec!["/nodes".to_owned()],
                });
            }
        }

        errors
    }

    pub fn has_terminal_nodes(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = doc.body.get_nodes_by(&XFlowNodeType::Flow, "end");

        if let 0 = res.len() {
            errors.push(ValidationError {
                code: 1,
                message: "XFlow has no terminal nodes".into(),
                paths: vec!["/nodes".into()],
            });
        }

        errors
    }

    pub fn all_nodes_have_at_least_one_edge(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        for node in &doc.body.nodes {
            let res = doc.body
                .edges
                .iter()
                .filter({
                    |edge| node.id == edge.0 || node.id == edge.1
                })
                .collect::<Vec<&XFlowEdge>>();

            if res.is_empty() {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("XFlow node '{}' is not connected to an edge", node.id),
                    paths: vec![format!("/nodes/{}", node.id)],
                });
            }
        }

        errors
    }

    pub fn all_node_actions_have_matching_requirements(
        doc: &XFlowDocument,
    ) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let reqs = doc.body
            .requirements
            .iter()
            .map({
                |req| req.xtype.clone()
            })
            .collect::<Vec<XFlowNodeType>>();

        for node in &doc.body.nodes {

            if !reqs.contains(&node.nodetype) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow node '{}' has an unmatched capability requirement \
                                      '{:?}'",
                        node.id,
                        node.nodetype
                    ),
                    paths: vec![format!("/nodes/{}", node.id)],
                });
            }
        }

        errors
    }

    pub fn variables_are_defined_only_once(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut input_vars = HashSet::<String>::new();
        for xvar in &doc.body.variables.input {
            if input_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow input variable '{}' defined more than once",
                        xvar.name
                    ),
                    paths: vec![format!("/variables/input/{}", xvar.name)],
                });
            } else {
                input_vars.insert(xvar.name.clone());
            }
        }

        let mut local_vars = HashSet::<String>::new();
        for xvar in &doc.body.variables.local {
            if local_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow local variable '{}' defined more than once",
                        xvar.name
                    ),
                    paths: vec![format!("/variables/local/{}", xvar.name)],
                });
            } else {
                local_vars.insert(xvar.name.clone());
            }
        }

        let mut output_vars = HashSet::<String>::new();
        for xvar in &doc.body.variables.output {
            if output_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow output variable '{}' defined more than once",
                        xvar.name
                    ),
                    paths: vec![format!("/variables/output/{}", xvar.name)],
                });
            } else {
                output_vars.insert(xvar.name.clone());
            }
        }

        errors
    }

    pub fn all_return_values_exist(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut inputs = HashMap::<&String, &XFlowVariableDefinition>::new();
        let mut locals = HashMap::<&String, &XFlowVariable>::new();

        for xvar in &doc.body.variables.input {
            inputs.insert(&xvar.name, xvar);
        }

        for xvar in &doc.body.variables.local {
            locals.insert(&xvar.name, xvar);
        }

        for xvar in &doc.body.variables.output {
            if !locals.contains_key(&xvar.name) && !inputs.contains_key(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow output variable '{}' has no local or input definition",
                        xvar.name
                    ),
                    paths: vec![format!("/variables/output/{}", xvar.name)],
                });
            }
            if let Some(local_xvar) = locals.get(&xvar.name) {
                if local_xvar.vtype != xvar.vtype {
                    errors.push(ValidationError {
                        code: 1,
                        message: format!(
                            "XFlow output variable '{}' vtype is incompatible with \
                                          its local definition",
                            xvar.name
                        ),
                        paths: vec![format!("/variables/output/{}", xvar.name)],
                    });
                }
            }
            if let Some(input_xvar) = inputs.get(&xvar.name) {
                if input_xvar.vtype != xvar.vtype {
                    errors.push(ValidationError {
                        code: 1,
                        message: format!(
                            "XFlow output variable '{}' vtype is incompatible with \
                                          its input definition",
                            xvar.name
                        ),
                        paths: vec![format!("/variables/output/{}", xvar.name)],
                    });
                }
            }
        }


        errors
    }

    pub fn no_variable_redefinition(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut locals = HashMap::<&String, &XFlowVariable>::new();

        for xvar in &doc.body.variables.local {
            locals.insert(&xvar.name, xvar);
        }

        for xvar in &doc.body.variables.input {
            if locals.contains_key(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow input variable '{}' is redefined in local scope",
                        xvar.name
                    ),
                    paths: vec![format!("/variables/input/{}", xvar.name)],
                });
            }
        }

        errors
    }

    pub fn all_flox_variables_exist(doc: &XFlowDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let nodes = doc.body.get_nodes_of_type(&XFlowNodeType::Flow);
        let names_in_xflow = doc.body.get_all_variable_names();

        for node in nodes {
            if let XFlowNodeParameters::Flox(ref flox_params) = node.parameters {
                for flox_var in flox::extract_variable_names(&flox_params.expression).unwrap() {
                    if !names_in_xflow.contains(flox_var) {
                        errors.push(ValidationError {
                            code: 1,
                            message: format!(
                                "Flox expression references variable \
                                                          '{}' which is not defined in this flow",
                                flox_var
                            ),
                            paths: vec![format!("/nodes/{}", node.id)],
                        });
                    }
                }
            }
        }

        errors
    c


    //     X  all_edges_have_nodes(flow),
    //     X  has_one_entry_node(flow),
    //     X  has_terminal_nodes(flow),
    //     X  all_node_actions_have_matching_requirements(flow),
    //     X  all_return_values_exist(flow),
    //     X  variables_are_defined_only_once(flow),
    //     X  all_nodes_have_at_least_one_edge(flow)
    //     X  no_variable_redefinition(flow)
    //     X  all_flox_variables_exist
}

impl Default for Validation {
    fn default() -> Self {
        Validation { errors: Vec::<ValidationError>::new() }
    }
}
