use serde;
use serde_json;
use serde_yaml;
use uuid::Uuid;

use structure::translation::TranslationDocument;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document<T> {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub doc: T,
}

impl<T> Document<T>
    where T: serde::Serialize + serde::de::DeserializeOwned + Eq + Default + Queryable
{
    pub fn new_from_header(header: &DocumentHeader) -> Self {
        Self {
            id: header.id.clone(),
            name: header.name.clone(),
            doctype: header.doctype.clone(),
            doctype_version: header.doctype_version.clone(),
            version: header.version.clone(),
            doc: <T>::default(),
        }
    }

    pub fn get_header(&self) -> DocumentHeader {
        DocumentHeader {
            id: self.id.clone(),
            name: self.name.clone(),
            doctype: self.doctype.clone(),
            doctype_version: self.doctype_version.clone(),
            version: self.version.clone(),
        }
    }

    pub fn set_header(&mut self, header: &DocumentHeader) -> () {
        self.id = header.id.clone();
        self.name = header.name.clone();
        self.doctype = header.doctype.clone();
        self.doctype_version = header.doctype_version.clone();
        self.version = header.version.clone();
    }

    /// Return a string representation of the Document
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
    }

    /// Return a summary of the Document
    ///
    pub fn summary(&self) -> String {
        format!("Doc {:?} - {:?} - {:?}", self.doctype, self.id, self.name)
    }

    /// Return an indented JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    pub fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    /// Return a YAML representation of the Document
    ///
    /// partof: #SPC-serialization-yaml
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    pub fn from_yaml(s: &str) -> Self {
        serde_yaml::from_str(s).unwrap()
    }
}

impl<T> Default for Document<T>
    where T: Default
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "default".to_owned(),
            doctype: "".to_owned(),
            doctype_version: 1,
            version: 1,
            doc: <T>::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct I18NString {
    pub locale: String,
    pub key: String,
    pub value: String,
}

impl I18NString {
    pub fn new(s: String) -> I18NString {
        I18NString {
            locale: "en_US".to_owned(),
            key: "".to_owned(),
            value: s,
        }
    }
}

impl Default for I18NString {
    fn default() -> I18NString {
        I18NString {
            locale: "en_US".to_owned(),
            key: "-no-key-".to_owned(),
            value: "-no-value-".to_owned(),
        }
    }
}


impl I18NString {
    pub fn translate(&self, t: &TranslationDocument) -> Self {
        I18NString {
            locale: t.doc.locale.clone(),
            key: self.key.clone(),
            value: t.doc
                .items
                .get(&self.key)
                .unwrap_or(&I18NString::new("".to_owned()))
                .value
                .clone(),
        }
    }

    pub fn translate_self(&mut self, t: &TranslationDocument) -> () {
        match t.doc.items.get(&self.key) {
            Some(item) => {
                self.locale = item.locale.clone();
                self.value = item.value.clone();
            }
            None => {
                warn!("No translation found for '{:?}' in locale '{:?}'",
                      self.key,
                      t.doc.locale);
                self.locale = t.doc.locale.clone();
                self.value = "-no-value-".to_owned();
            }
        };

        debug!("Translated value is '{:?}'", self.value);
    }
}

pub trait Translatable {
    fn translate_in_place(&mut self, t: &TranslationDocument) -> ();
    fn translate(&self, t: &TranslationDocument) -> Self;
    fn all_i18n_strings(&self) -> Vec<&I18NString>;
}

//
// This struct only exists to make the top-level Model object serializable into a project's
// model.json
//

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct DocumentHeader {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
}

impl DocumentHeader {
    /// Return a string representation of the DocumentHeader
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
    }

    /// Return an indented JSON representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-json
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-json
    pub fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a DocumentHeader from a JSON string
    ///
    /// partof: SPC-serialization-json
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    /// Return a YAML representation of the DocumentHeader
    ///
    /// partof: #SPC-serialization-yaml
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a DocumentHeader from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    pub fn from_yaml(s: &str) -> Self {
        serde_yaml::from_str(s).unwrap()
    }
}

pub enum QueryPart {
    Item(String),
    ListIndex(String, i32),
    List(String),
}

pub type QueryPath = Vec<QueryPart>;

// vec![Item("domain"), ListItem("

pub trait Queryable {
    // fn tree(&self) -> String;
    // fn complete(&self) -> (String, QueryPart);
}
