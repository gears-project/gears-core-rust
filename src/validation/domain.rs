use validation::common::ValidationError;
use structure::domain::DomainDocument;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(domain: &DomainDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors
    }
}
