extern crate env_logger;

extern crate xflow;
use xflow::validation::*;
use xflow::xfstruct::*;

#[test]
fn test_validation_default() {
    let _ = env_logger::init();

    let xfs = XFlowStruct::default();
    let validation = Validation::default();

    validation.validate(&xfs);

    assert_eq!(validation.errors.len(), 0)
}

#[test]
fn test_validation_error_instantiate() {
    let _ = env_logger::init();

    let error = ValidationError::new(1, "Test Error".to_owned(), Vec::<String>::new());
    assert_eq!(error.code, 1);
}
