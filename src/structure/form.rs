use serde_json::Value;
// use std::collections::HashMap;

use super::common::Document;

pub type FormDocument = Document<Form>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Form {
    pub title: String,
    pub components: Components,
}

pub type Components = Vec<Component>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
    pub component: String,
    pub config: Option<Value>,
    pub components: Option<Components>,
}

/*
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatatableComponent<'a> {
    pub name: String,
    pub config: DatatableConfig,
    pub components: &'a Components,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatatableConfig {
    pub entity: String,
    pub attributes: Vec<String>,
    pub eventbindings: HashMap<String, String>,
}
*/
