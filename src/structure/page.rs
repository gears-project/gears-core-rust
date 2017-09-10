use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};
use uuid::Uuid;

use super::common::{Document, DocumentList, I18NString, Translatable};
use super::translation::TranslationDocument;
use dsl::command::{GearsDsl, DslTree, DslToken, DslTokens, command_grammar};

pub type PageDocument = Document<Page>;
pub type PageDocumentList = DocumentList<Page>;

pub type Components = Vec<Component>;

impl PageDocument {
    pub fn all_xflow_references(&self) -> Vec<&Uuid> {
        collect_xflow_references(&self.doc.components)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub title: I18NString,
    pub components: Vec<Component>,
}

impl Default for Page {
    fn default() -> Page {
        Page {
            title: I18NString::default(),
            components: Vec::<Component>::new(),
        }
    }
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
    #[serde(serialize_with = "ordered_map")]
    pub eventbindings: HashMap<String, Uuid>,
}

fn ordered_map<S>(value: &HashMap<String, Uuid>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // partof: SPC-serialization-fs
    // Consistent serialization
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
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
        match *self {
            FormControlType::Text => "text".to_owned(),
            FormControlType::Radio => "radio".to_owned(),
            FormControlType::Checkbox => "checkbox".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FormControlGroupConfig {
    pub label: I18NString,
    pub placeholder: I18NString,
    pub form_control_type: FormControlType,
}


fn collect_xflow_reference(c: &Component) -> Vec<&Uuid> {
    let mut res = Vec::<&Uuid>::new();

    match *c {

        //
        // No xflow references
        //
        Component::Header1(_) => {}
        Component::Header2(_) => {}
        Component::Header3(_) => {}
        Component::Label(_) => {}
        Component::Button(_) => {}
        Component::TextInput(_) => {}

        //
        // Containers
        //
        Component::Row(ref c) => res.append(&mut collect_xflow_references(&c.components)),
        Component::Column3(ref c) => res.append(&mut collect_xflow_references(&c.components)),
        Component::Column6(ref c) => res.append(&mut collect_xflow_references(&c.components)),
        Component::Column12(ref c) => res.append(&mut collect_xflow_references(&c.components)),

        //
        // Containers
        //
        Component::Datatable(ref c) => {
            for (_, ref xflow) in &c.config.eventbindings {
                res.push(&xflow);
            }
        }

        // XXX
        _ => {}

    }
    res
}

fn collect_xflow_references(components: &Components) -> Vec<&Uuid> {
    let mut res = Vec::<&Uuid>::new();
    for component in components {
        res.append(&mut collect_xflow_reference(&component));
    }
    res
}

fn collect_i18nstring(c: &Component) -> Vec<&I18NString> {
    let mut res = Vec::<&I18NString>::new();

    match *c {
        Component::Header1(ref c) => res.push(&c.config.text),
        Component::Header2(ref c) => res.push(&c.config.text),
        Component::Header3(ref c) => res.push(&c.config.text),
        Component::Label(ref c) => res.push(&c.config.text),
        Component::Button(ref c) => res.push(&c.config.text),
        Component::TextInput(ref c) => res.push(&c.config.placeholder),
        Component::Column3(ref c) => res.append(&mut collect_i18nstrings(&c.components)),
        Component::Column6(ref c) => res.append(&mut collect_i18nstrings(&c.components)),
        Component::Column12(ref c) => res.append(&mut collect_i18nstrings(&c.components)),
        // XXX: Implement others
        _ => {}
    }
    res
}

fn collect_i18nstrings(components: &Components) -> Vec<&I18NString> {
    let mut res = Vec::<&I18NString>::new();
    for component in components {
        res.append(&mut collect_i18nstring(&component));
    }
    res
}

fn translate_component(c: &mut Component, t: &TranslationDocument) -> () {
    match *c {
        Component::Header1(ref mut c) => c.config.text.translate_self(t),
        Component::Header2(ref mut c) => c.config.text.translate_self(t),
        Component::Header3(ref mut c) => c.config.text.translate_self(t),
        Component::Label(ref mut c) => c.config.text.translate_self(t),
        Component::Button(ref mut c) => c.config.text.translate_self(t),
        Component::TextInput(ref mut c) => c.config.placeholder.translate_self(t),

        Component::Column3(ref mut c) => translate_components(&mut c.components, &t),
        Component::Column6(ref mut c) => translate_components(&mut c.components, &t),
        Component::Column12(ref mut c) => translate_components(&mut c.components, &t),
        Component::Row(ref mut c) => translate_components(&mut c.components, &t),

        Component::Datatable(_) => {}
        Component::Form(_) => {}
        Component::FormControlGroup(ref mut c) => {
            c.config.label.translate_self(t);
            c.config.placeholder.translate_self(t);
        }
    }
}

fn translate_components(components: &mut Components, t: &TranslationDocument) -> () {
    for ref mut component in components {
        translate_component(component, &t);
    }
}

impl Translatable for PageDocument {
    fn translate_in_place(&mut self, t: &TranslationDocument) -> () {
        self.doc.title.translate_self(&t);
        translate_components(&mut self.doc.components, &t);
    }

    fn translate(&self, t: &TranslationDocument) -> PageDocument {
        let mut doc = self.clone();
        doc.translate_in_place(&t);
        doc
    }

    fn all_i18n_strings(&self) -> Vec<&I18NString> {
        collect_i18nstrings(&self.doc.components)
    }
}

impl GearsDsl for Page {
    fn generate_dsl(&self) -> DslTokens {
        let mut res = DslTokens::new();
        unimplemented!();
    }

    fn consume_command(&mut self, s: &str) -> Result<(), String> {
        debug!("consume_command : received command string '{:?}'", s);
        unimplemented!();
    }

    fn consume_scope(&mut self, s: &str, tree: &Vec<DslTree>) -> Result<(), String> {
        unimplemented!();
    }
}
