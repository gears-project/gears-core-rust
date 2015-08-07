use ::xflow::xfstruct::*;

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

    pub fn validate(&mut self, xflow:&XFlowStruct) {
        let mut x = self.all_edges_have_nodes(xflow);
        // self.errors.append(x);

    }

    fn all_edges_have_nodes(&self, xflow:&XFlowStruct) -> Vec<&ValidationError> {
        let mut errors = Vec::<&ValidationError>::new();

        errors

    }
}
