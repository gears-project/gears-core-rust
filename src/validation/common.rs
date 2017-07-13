use super::model;
use super::xflow;
use super::domain;
use super::translation;

use structure::model::ModelDocument;

#[derive(Debug)]
pub struct ValidationError {
    pub code: i32,
    pub message: String,
    pub paths: Vec<String>,
}

pub type ValidationErrors = Vec<ValidationError>;

impl ValidationError {
    /// Constructs a new `ValidationError`
    ///
    /// # Example
    /// ```
    /// use xflow::validation::common::{ValidationError};
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

pub fn validate_model(model: &ModelDocument) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    errors.extend(model::Validation::validate(&model));
    errors.extend(domain::Validation::validate(&model.doc.domain));

    for xflow in &model.doc.xflows {
        errors.extend(xflow::Validation::validate(&xflow));
    }

    for t in &model.doc.translations {
        errors.extend(translation::Validation::validate(&t));
    }

    errors
}
