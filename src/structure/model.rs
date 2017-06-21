use super::common;
use super::domain;
use super::xflow;
use super::form;


pub type ModelDocument = common::Document<Model>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub domain: domain::DomainDocument,
    pub xflows: Vec<xflow::XFlow>,
    pub forms: Vec<form::Form>,
}

impl Model {}

impl Default for Model {
    fn default() -> Self {
        Model {
            domain: domain::DomainDocument::default(),
            xflows: Vec::<xflow::XFlow>::new(),
            forms: Vec::<form::Form>::new(),
        }
    }
}
