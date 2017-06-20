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
    Row(RowComponent),
    Header1(Header1Component),
    Header2(Header2Component),
    Header3(Header3Component),
    Column3(Column3Component),
    Column6(Column6Component),
    Column12(Column12Component),
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

pub type RowComponent = LayoutComponent;
pub type Header1Component = TextComponent;
pub type Header2Component = TextComponent;
pub type Header3Component = TextComponent;
pub type Column3Component = LayoutComponent;
pub type Column6Component = LayoutComponent;
pub type Column12Component = LayoutComponent;

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
