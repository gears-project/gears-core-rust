use serde_json::Value;
use std::collections::HashMap;

pub type FormDocument = super::common::Document<Form>;

#[derive(Debug, Serialize, Clone)]
pub struct Form {
    pub title: String,
    pub components: Components,
}

pub type Components = Vec<Component>;

#[derive(Debug, Serialize, Clone)]
pub struct Component {
    pub component: String,
    pub config: Value,
    pub components: Components,
}

#[derive(Debug, Serialize, Clone)]
pub struct DatatableComponent<'a> {
    pub name: String,
    pub config: DatatableConfig,
    pub components: &'a Components,
}

#[derive(Debug, Serialize, Clone)]
pub struct DatatableConfig {
    pub entity: String,
    pub attributes: Vec<String>,
    pub eventbindings: HashMap<String, String>,
}
