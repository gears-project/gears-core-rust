use validation::common::ValidationError;
use structure::page::PageDocument;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(page: &PageDocument) -> Vec<ValidationError> {
        let errors = Vec::<ValidationError>::new();

        errors
    }
}
