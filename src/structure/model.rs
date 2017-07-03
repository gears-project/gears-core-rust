use super::common;
use super::domain;
use super::xflow;
use super::page;

pub type ModelDocument = common::Document<Model>;
pub type ModelConfigDocument = common::Document<ModelConfig>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Model {
    pub config: ModelConfigDocument,
    pub domain: domain::DomainDocument,
    pub xflows: Vec<xflow::XFlowDocument>,
    pub pages: Vec<page::PageDocument>,
}

impl Model {}

impl Default for Model {
    fn default() -> Self {
        Model {
            config: ModelConfigDocument::default(),
            domain: domain::DomainDocument::default(),
            xflows: Vec::<xflow::XFlowDocument>::new(),
            pages: Vec::<page::PageDocument>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ModelConfig {
    pub default_locale: String,
    pub active_locale: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            default_locale: "en_US".to_owned(),
            active_locale: "en_US".to_owned(),
        }
    }
}
