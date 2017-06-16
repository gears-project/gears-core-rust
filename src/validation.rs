use structure::xflow::*;
use flox;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct ValidationError {
    pub code: i32,
    pub message: String,
    pub paths: Vec<String>,
}

impl ValidationError {
    /// Constructs a new `ValidationError`
    ///
    /// # Example
    /// ```
    /// use xflow::validation::{ValidationError};
    /// let err = ValidationError::new(1, "sample error".to_string(), Vec::<String>::new());
    /// println!("Validation error {}", err.message);
    /// ```
    pub fn new(code: i32, message: String, paths: Vec<String>) -> ValidationError {
        ValidationError {
            code: code,
            message: message,
            paths: paths,
        }
    }
}

#[derive(Debug)]
pub struct Validation {
    pub errors: Vec<ValidationError>,
}

impl Validation {
    pub fn validate(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_edges_have_nodes(xflow));
        errors.extend(Validation::has_one_entry_node(xflow));
        errors.extend(Validation::has_terminal_nodes(xflow));
        errors.extend(Validation::all_nodes_have_at_least_one_edge(xflow));
        errors.extend(Validation::all_node_actions_have_matching_requirements(xflow));
        errors.extend(Validation::variables_are_defined_only_once(xflow));
        errors.extend(Validation::all_return_values_exist(xflow));
        errors.extend(Validation::no_variable_redefinition(xflow));
        errors.extend(Validation::all_flox_variables_exist(xflow));

        errors
    }

    pub fn all_edges_have_nodes(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut node_ids = xflow
            .nodes
            .iter()
            .map({
                     |node| node.id
                 })
            .collect::<Vec<i32>>();

        node_ids.sort();
        node_ids.dedup();

        for edge in &xflow.edges {

            if !node_ids.contains(&edge.0) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("Edge {:?} has no connecting node {:?}",
                                                 edge,
                                                 edge.0),
                                paths: vec![format!("/edges/{:?}", edge)],
                            });
            };

            if !node_ids.contains(&edge.1) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("Edge {:?} has no connecting node {:?}",
                                                 edge,
                                                 edge.1),
                                paths: vec![format!("/edges/{:?}", edge)],
                            });
            };
        }

        errors
    }

    pub fn has_one_entry_node(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = xflow.get_nodes_by("flow", "start");
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

    pub fn has_terminal_nodes(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = xflow.get_nodes_by("flow", "end");

        if let 0 = res.len() {
            errors.push(ValidationError {
                            code: 1,
                            message: "XFlow has no terminal nodes".into(),
                            paths: vec!["/nodes".into()],
                        });
        }

        errors

    }

    pub fn all_nodes_have_at_least_one_edge(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        for node in &xflow.nodes {
            let res = xflow
                .edges
                .iter()
                .filter({
                            |edge| node.id == edge.0 || node.id == edge.1
                        })
                .collect::<Vec<&XFlowEdge>>();

            if res.is_empty() {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("XFlow node '{}' is not connected to an edge",
                                                 node.id),
                                paths: vec![format!("/nodes/{}", node.id)],
                            });
            }
        }

        errors
    }

    pub fn all_node_actions_have_matching_requirements(xflow: &XFlowStruct)
                                                       -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let reqs = xflow
            .requirements
            .iter()
            .map({
                     |req| req.xtype.clone()
                 })
            .collect::<Vec<String>>();

        for node in &xflow.nodes {

            if !reqs.contains(&node.nodetype) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!(
                        "XFlow node '{}' has an unmatched capability requirement \
                                      '{}'",
                        node.id,
                        node.nodetype
                    ),
                    paths: vec![format!("/nodes/{}", node.id)],
                });
            }
        }

        errors
    }

    pub fn variables_are_defined_only_once(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut input_vars = HashSet::<String>::new();
        for xvar in &xflow.variables.input {
            if input_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("XFlow input variable '{}' defined more than once",
                                                 xvar.name),
                                paths: vec![format!("/variables/input/{}", xvar.name)],
                            });
            } else {
                input_vars.insert(xvar.name.clone());
            }
        }

        let mut local_vars = HashSet::<String>::new();
        for xvar in &xflow.variables.local {
            if local_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("XFlow local variable '{}' defined more than once",
                                                 xvar.name),
                                paths: vec![format!("/variables/local/{}", xvar.name)],
                            });
            } else {
                local_vars.insert(xvar.name.clone());
            }
        }

        let mut output_vars = HashSet::<String>::new();
        for xvar in &xflow.variables.output {
            if output_vars.contains(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("XFlow output variable '{}' defined more than once",
                                     xvar.name),
                    paths: vec![format!("/variables/output/{}", xvar.name)],
                });
            } else {
                output_vars.insert(xvar.name.clone());
            }
        }

        errors
    }

    pub fn all_return_values_exist(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut inputs = HashMap::<&String, &XFlowVariableDefinition>::new();
        let mut locals = HashMap::<&String, &XFlowVariable>::new();

        for xvar in &xflow.variables.input {
            inputs.insert(&xvar.name, &xvar);
        }

        for xvar in &xflow.variables.local {
            locals.insert(&xvar.name, &xvar);
        }

        for xvar in &xflow.variables.output {
            if !locals.contains_key(&xvar.name) && !inputs.contains_key(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("XFlow output variable '{}' has no local or input definition",
                                     xvar.name),
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

    pub fn no_variable_redefinition(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut locals = HashMap::<&String, &XFlowVariable>::new();

        for xvar in &xflow.variables.local {
            locals.insert(&xvar.name, &xvar);
        }

        for xvar in &xflow.variables.input {
            if locals.contains_key(&xvar.name) {
                errors.push(ValidationError {
                    code: 1,
                    message: format!("XFlow input variable '{}' is redefined in local scope",
                                     xvar.name),
                    paths: vec![format!("/variables/input/{}", xvar.name)],
                });
            }
        }

        errors
    }

    pub fn all_flox_variables_exist(xflow: &XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let nodes = xflow.get_nodes_of_type(&"flox");
        let names_in_xflow = xflow.get_all_variable_names();

        for node in nodes {
            match node.parameters {
                Some(ref params) => {
                    match params.get("expression") {
                        Some(expr) => {
                            // XXX: Remove unwraps
                            for flox_var in flox::extract_variable_names(expr.as_str().unwrap())
                                    .unwrap() {
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
                        None => {
                            // XXX This should not happen
                        }
                    }
                }
                None => {
                    // XXX This should not happen
                }
            }
        }

        errors
    }


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
