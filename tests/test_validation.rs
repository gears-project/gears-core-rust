extern crate env_logger;

extern crate xflow;
use xflow::structure::xflow::*;
use xflow::validation::common::*;
use xflow::validation;

use xflow::util::fs::*;

#[test]
fn test_validation_default() {
    let _ = env_logger::init();

    let xfs = XFlowDocument::default();
    let errors = validation::xflow::Validation::validate(&xfs);

    assert_eq!(errors.len(), 2)
}

#[test]
fn test_validation_error_instantiate() {
    let _ = env_logger::init();

    let error = ValidationError::new(1, "Test Error".to_owned(), Vec::<String>::new());
    assert_eq!(error.code, 1);
}

#[test]
fn test_basic_model_validation() {
    let _ = env_logger::init();

    let model = model_from_fs(&"resource/projects/basic").unwrap();
    let validation_errors = validation::model::Validation::validate(&model);

    // XXX
    assert_eq!(validation_errors.len(), 18);
}
