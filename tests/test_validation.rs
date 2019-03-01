extern crate env_logger;

extern crate gears;
use gears::structure::common::DocumentFileSystemLoadable;
use gears::structure::model::ModelDocument;
use gears::structure::xflow::*;
use gears::validation::common::*;
use gears::validation;

use gears::util::fs::*;

#[test]
fn test_validation_default() {
    let _ = env_logger::try_init();

    let xfs = XFlowDocument::default();
    let errors = validation::xflow::Validation::validate(&xfs);

    assert_eq!(errors.len(), 0)
}

#[test]
fn test_validation_error_instantiate() {
    let _ = env_logger::try_init();

    let error = ValidationError::new(1, "Test Error".to_owned(), Vec::<String>::new());
    assert_eq!(error.code, 1);
}

#[test]
fn test_basic_model_validation() {
    let _ = env_logger::try_init();

    let model = ModelDocument::load_from_filesystem(&"resource/projects/basic").unwrap();
    let validation_errors = validation::model::Validation::validate(&model);

    assert_eq!(validation_errors.len(), 0);
}
