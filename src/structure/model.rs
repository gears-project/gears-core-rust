use super::domain;

#[derive(Debug, Serialize, Clone)]
pub struct Model {
    pub domain: domain::DomainDocument,
}

impl Model {}
