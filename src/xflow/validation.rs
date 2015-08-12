use ::xflow::xfstruct::*;
// use ::xflow::errors::*;

pub struct ValidationError {
    pub code:    i32,
    pub message: String,
    pub paths:   Vec<String>,
}

impl ValidationError {
    /// Constructs a new `ValidationError`
    ///
    /// # Example
    /// ```
    /// use xfdocs::xflow::validation::{ValidationError};
    /// let err = ValidationError::new(1, "sample error".to_string(), Vec::<String>::new());
    /// println!("Validation error {}", err.message);
    /// ```
    pub fn new(code:i32, message:String, paths:Vec<String>) -> ValidationError {
        ValidationError {
            code:    code,
            message: message,
            paths:   paths
        }
    }
}

pub struct Validation {
    pub errors: Vec<ValidationError>,
}

impl Validation {

    pub fn new() -> Validation {
        Validation {
            errors: Vec::<ValidationError>::new()
        }
    }

   pub fn validate(&self, xflow:&XFlowStruct) {
       Validation::all_edges_have_nodes(xflow);
       Validation::has_one_entry_node(xflow);
       Validation::has_terminal_nodes(xflow);
   }

    pub fn all_edges_have_nodes(xflow:&XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut node_ids = xflow.nodes.iter().map({|node|
            node.id.clone()
        }).collect::<Vec<i32>>();

        node_ids.sort();
        node_ids.dedup();

        for edge in xflow.edges.iter() {

            if !node_ids.contains(&edge.0) {
                errors.push(ValidationError {
                    code:    1,
                    message: format!("Edge {:?} has no connecting node {:?}", edge, edge.0),
                    paths:   vec![format!("/edges/{:?}", edge)],
                });
            };

            if !node_ids.contains(&edge.1) {
                errors.push(ValidationError {
                    code:    1,
                    message: format!("Edge {:?} has no connecting node {:?}", edge, edge.1),
                    paths:   vec![format!("/edges/{:?}", edge)],
                });
            };
        }

       errors
   }

    pub fn has_one_entry_node(xflow:&XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = xflow.get_nodes_by("flow", "start");
        match res.len() {
            0 => {
                errors.push(ValidationError {
                    code:    1,
                    message: format!("XFlow has no entry nodes"),
                    paths:   vec![format!("/nodes")],
                });
            },
            1 => {},
            _ => {
                //
                // XXX: Add multiple paths
                //
                errors.push(ValidationError {
                    code:    1,
                    message: format!("XFlow has multiple entry nodes"),
                    paths:   vec!["/nodes".to_string()],
                });
            }
        }

        errors
    }

    pub fn has_terminal_nodes(xflow:&XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let res = xflow.get_nodes_by("flow", "end");
        match res.len() {
            0 => {
                errors.push(ValidationError {
                    code:    1,
                    message: format!("XFlow has no terminal nodes"),
                    paths:   vec![format!("/nodes")],
                });
            }
            _ => {}
        }

        errors

    }

    pub fn all_nodes_have_at_least_one_edge(xflow:&XFlowStruct) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors
    }

//     X  all_edges_have_nodes(flow),
//     X  has_one_entry_node(flow),
//     X  has_terminal_nodes(flow),
//        all_node_actions_have_matching_requirements(flow),
//        expressions_reference_known_variables(flow),
//        all_return_values_exist(flow),
//        variables_are_defined_only_once(flow),
//        all_nodes_have_at_least_one_edge(flow)
}
