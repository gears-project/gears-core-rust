use std::collections::HashMap;

use super::common::{Document, I18NString};

pub type PageDocument = Document<Page>;
pub type Components = Vec<Component>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub title: I18NString,
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
    Form(FormComponent),
    TextInput(TextInputComponent),
    Label(LabelComponent),
    Button(ButtonComponent),
    FormControlGroup(FormControlGroupComponent),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ContainerComponent<T> {
    pub config: T,
    pub components: Components,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ConfigOnlyComponent<T> {
    pub config: T,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FormControlGroupComponent {
    pub config: FormControlGroupConfig,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LayoutComponent {
    pub components: Components,
}

pub type RowComponent = LayoutComponent;
pub type Header1Component = ConfigOnlyComponent<TextConfig>;
pub type Header2Component = ConfigOnlyComponent<TextConfig>;
pub type Header3Component = ConfigOnlyComponent<TextConfig>;
pub type Column3Component = LayoutComponent;
pub type Column6Component = LayoutComponent;
pub type Column12Component = LayoutComponent;
pub type DatatableComponent = ConfigOnlyComponent<DatatableConfig>;
pub type FormComponent = ContainerComponent<FormConfig>;
pub type TextInputComponent = ConfigOnlyComponent<TextInputConfig>;
pub type LabelComponent = ConfigOnlyComponent<LabelConfig>;
pub type ButtonComponent = ConfigOnlyComponent<ButtonConfig>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextConfig {
    pub text: I18NString,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatatableConfig {
    pub entity: String,
    pub attributes: Vec<String>,
    pub eventbindings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FormConfig {
    pub entity: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TextInputConfig {
    pub placeholder: I18NString,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LabelConfig {
    pub text: I18NString,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ButtonConfig {
    pub text: I18NString,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FormControlType {
    Text,
    Radio,
    Checkbox,
}

impl FormControlType {
    pub fn to_text(&self) -> String {
        match self {
            &FormControlType::Text => "text".to_owned(),
            &FormControlType::Radio => "radio".to_owned(),
            &FormControlType::Checkbox => "checkbox".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FormControlGroupConfig {
    pub label: I18NString,
    pub placeholder: I18NString,
    pub form_control_type: FormControlType,
}
