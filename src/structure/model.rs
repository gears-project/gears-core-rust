use super::domain;
use super::xflow;
use super::form;

#[derive(Debug, Serialize, Clone)]
pub struct Model {
    pub domain: domain::DomainDocument,
    pub xflows: Vec<xflow::XFlow>,
    pub forms: Vec<form::Form>,
}

impl Model {}
