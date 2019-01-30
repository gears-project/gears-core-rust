use std::collections::HashSet;

use crate::validation::common::ValidationError;
use crate::structure::domain::DomainDocument;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(doc: &DomainDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_references_point_to_existing_entities(&doc));

        errors
    }

    pub fn all_references_point_to_existing_entities(doc: &DomainDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        let mut entities = HashSet::<&String>::new();

        for entity in &doc.body.entities {
            entities.insert(&entity.name);
        }

        for entity in &doc.body.entities {
            for reference in &entity.references {
                if !entities.contains(&reference.name) {
                    errors.push(ValidationError {
                        code: 1,
                        message: format!("Domain : Entity '{entity}' contains a reference to non-existent entity '{reference}'", entity=entity.name, reference=&reference.name),
                        paths: vec![
                            format!("/domain/entities/{entity}/references", entity=entity.name)
                        ]
                    });
                }
            }
        }

        errors
    }
}
