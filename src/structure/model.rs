use super::common;
use super::domain;
use super::xflow;
use super::form;

pub type ModelDocument = common::Document<Model>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Model {
    pub domain: domain::DomainDocument,
    pub xflows: Vec<xflow::XFlowDocument>,
    pub forms: Vec<form::FormDocument>,
}

impl Model {}

impl Default for Model {
    fn default() -> Self {
        Model {
            domain: domain::DomainDocument::default(),
            xflows: Vec::<xflow::XFlowDocument>::new(),
            forms: Vec::<form::FormDocument>::new(),
        }
    }
}
