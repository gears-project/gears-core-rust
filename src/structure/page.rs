use std::collections::HashMap;

use super::common::Document;

pub type PageDocument = Document<Page>;
pub type Components = Vec<Component>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub title: String,
    pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LayoutComponent {
    pub components: Components,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextComponentConfig {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextComponent {
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
    pub config: TextComponentConfig,
}

pub type RowComponent = LayoutComponent;
pub type Header1Component = TextComponent;
pub type Header2Component = TextComponent;
pub type Header3Component = TextComponent;
pub type Column3Component = LayoutComponent;
pub type Column6Component = LayoutComponent;
pub type Column12Component = LayoutComponent;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatatableComponent {
    pub config: DatatableComponentConfig,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatatableComponentConfig {
    pub entity: String,
    pub attributes: Vec<String>,
    pub eventbindings: HashMap<String, String>,
}
