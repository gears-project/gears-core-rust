use ::xflow::xfstruct::*;
use ::xflow::errors::*;

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

    /// Constructs a new `Validation`
    ///
    /// # Example
    /// ```
    /// use xfdocs::xflow::validation::{Validation};
    /// let validation = Validation::new();
    /// ```
    pub fn new() -> Validation {
        Validation {
            errors: Vec::<ValidationError>::new()
        }
    }

   pub fn validate(&self, xflow:&XFlowStruct) {
       // let mut x = self.all_edges_have_nodes(xflow);
       // self.errors.append(x);
   }

    pub fn all_edges_have_nodes(xflow:&XFlowStruct) -> Vec<&ValidationError> {
        let mut errors = Vec::<&ValidationError>::new();

        let mut node_ids = xflow.nodes.iter().map({|node|
            node.id
        }).collect::<Vec<i32>>();

        node_ids.sort();
        node_ids.dedup();

        // println!("Got x3 {:?}", node_ids);


//       xflow.edges.iter().map({|edge|
//
//           println!("Got edge {:?}", &edge);
//
//           if !node_ids.contains(edge[0]) {
//               errors.push(ValidationError {
//                   code:    1,
//                   message: format!("Edge {} has no connecting node {:?}", *edge, *edge[0]),
//                   paths:   vec![format!("/edges/{}", *edge)],
//               });
//           };
//
//           if !node_ids.contains(edge[1]) {
//               errors.push(ValidationError {
//                   code:    1,
//                   message: format!("Edge {} has no connecting node {:?}", *edge, *edge[1]),
//                   paths:   vec![format!("/edges/{}", *edge)],
//               });
//           };
//
//       });

       errors
   }
//
//    fn has_one_entry_node(&self, xflow:&XFlowStruct) -> Vec<ValidationError> {
//        let mut errors = Vec::<ValidationError>::new();
//
//        let res = xflow.get_nodes_by("flow", "start");
//        match res.len() {
//            0 => {
//                errors.push(ValidationError {
//                    code:    1,
//                    message: "XFlow has no entry nodes".to_string(),
//                    paths:   vec!["/nodes".to_string()],
//                });
//            },
//            1 => {},
//            _ => {
//                //
//                // XXX: Add multiple paths
//                //
//                errors.push(ValidationError {
//                    code:    1,
//                    message: "XFlow has multiple entry nodes".to_string(),
//                    paths:   vec!["/nodes".to_string()],
//                });
//            }
//        }
//
//        errors
//    }
//
//    fn has_terminal_nodes(&self, xflow:&XFlowStruct) -> Vec<ValidationError> {
//        let mut errors = Vec::<ValidationError>::new();
//
//        let res = xflow.get_nodes_by("flow", "end");
//        match res.len() {
//            0 => {
//                errors.push(ValidationError {
//                    code:    1,
//                    message: "XFlow has no terminal nodes".to_string(),
//                    paths:   vec!["/nodes".to_string()],
//                });
//            }
//            _ => {}
//        }
//
//        errors
//
//    }
//
//
//        all_edges_have_nodes(flow),
//        has_one_entry_node(flow),
//        has_terminal_nodes(flow),
//        all_node_actions_have_matching_requirements(flow),
//        expressions_reference_known_variables(flow),
//        all_return_values_exist(flow),
//        variables_are_defined_only_once(flow),
//        all_nodes_have_at_least_one_edge(flow)
}
