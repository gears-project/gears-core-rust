use std::collections::HashMap;

use super::common::Document;

pub type FormDocument = Document<Form>;
pub type Components = Vec<Component>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Form {
    pub title: String,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "component")]
pub enum Component {
    Row(Row),
    Header1(Header1),
    Header2(Header2),
    Header3(Header3),
    Column3(Column3),
    Column6(Column6),
    Column12(Column12),
    Datatable(DatatableComponent),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutComponent {
    pub components: Components,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextComponentConfig {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextComponent {
    pub config: TextComponentConfig,
}

pub type Row = LayoutComponent;
pub type Header1 = TextComponent;
pub type Header2 = TextComponent;
pub type Header3 = TextComponent;
pub type Column3 = LayoutComponent;
pub type Column6 = LayoutComponent;
pub type Column12 = LayoutComponent;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatatableComponent {
    pub config: DatatableComponentConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatatableComponentConfig {
    pub entity: String,
    pub attributes: Vec<String>,
    pub eventbindings: HashMap<String, String>,
}
