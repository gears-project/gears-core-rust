use super::model;
use super::gxmodel;
use super::xflow;
use super::domain;
use super::translation;

use crate::structure::model::ModelDocument;
use crate::structure::gxmodel::GxModel;

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
    /// use gears::validation::common::{ValidationError};
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

    errors.extend(model::Validation::validate(model));
    errors.extend(domain::Validation::validate(&model.body.domain));

    for xflow in &model.body.xflows {
        errors.extend(xflow::Validation::validate(&xflow));
    }

    for t in &model.body.translations {
        errors.extend(translation::Validation::validate(&t));
    }

    errors
}

pub fn validate_gxmodel(model: &GxModel) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    errors.extend(gxmodel::Validation::validate(model));
    errors.extend(domain::Validation::validate(&model.domain));

    for xflow in &model.xflows {
        errors.extend(xflow::Validation::validate(&xflow));
    }

    for t in &model.translations {
        errors.extend(translation::Validation::validate(&t));
    }

    errors
}
