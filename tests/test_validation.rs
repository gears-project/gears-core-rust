extern crate env_logger;

extern crate xflow;
use xflow::validation::common::*;
use xflow::validation::xflow::*;
use xflow::structure::xflow::*;

#[test]
fn test_validation_default() {
    let _ = env_logger::init();

    let xfs = XFlowDocument::default();
    let errors = Validation::validate(&xfs);

    assert_eq!(errors.len(), 2)
}

#[test]
fn test_validation_error_instantiate() {
    let _ = env_logger::init();

    let error = ValidationError::new(1, "Test Error".to_owned(), Vec::<String>::new());
    assert_eq!(error.code, 1);
}
