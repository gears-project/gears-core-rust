use serde;
use serde_json;
use serde_yaml;
use uuid::Uuid;

use crate::structure::translation::TranslationDocument;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document<T> {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub body: T,
}

pub type DocumentList<T> = Vec<Document<T>>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DocumentReference {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelLoadError {
    UnParseable(String),
    BadStructure(String),
    InputError(String),
}

pub trait DocumentNature {
    type Doc;

    fn new_from_header(header: &DocumentHeader) -> Self::Doc;
    fn get_header(&self) -> DocumentHeader;
    fn set_header(&mut self, header: &DocumentHeader) -> ();

    /// Return a string representation of the Document
    ///
    fn to_string(&self) -> String;

    /// Return a summary of the Document
    ///
    fn summary(&self) -> String;

    /// Return an indented JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json(&self) -> String;

    /// Return a compact JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json_compact(&self) -> String;

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn from_json(s: &str) -> Result<Self::Doc, ModelLoadError>;

    /// Update a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn update_from_json(&mut self, s: &str) -> Result<&Self::Doc, String>;

    /// Return a YAML representation of the Document
    ///
    /// partof: #SPC-serialization-yaml
    fn to_yaml(&self) -> String;

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    fn from_yaml(s: &str) -> Result<Self::Doc, ModelLoadError>;

    /// Update a Document from a YAML string
    ///
    /// partof: SPC-serialization-yaml
    fn update_from_yaml(&mut self, s: &str) -> Result<&Self::Doc, String>;
}

pub trait DocumentFileSystemLoadable {
    type Doc;

    fn load_from_filesystem(path: &str) -> Result<Self::Doc, ModelLoadError>;
    fn write_to_filesystem(&self, path: &str) -> Result<(), ModelLoadError>;
}

impl<T> DocumentNature for Document<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Eq + Default,
{
    type Doc = Document<T>;

    fn new_from_header(header: &DocumentHeader) -> Self {
        Self {
            id: header.id.clone(),
            name: header.name.clone(),
            doctype: header.doctype.clone(),
            doctype_version: header.doctype_version.clone(),
            version: header.version.clone(),
            body: <T>::default(),
        }
    }

    fn get_header(&self) -> DocumentHeader {
        DocumentHeader {
            id: self.id.clone(),
            name: self.name.clone(),
            doctype: self.doctype.clone(),
            doctype_version: self.doctype_version.clone(),
            version: self.version.clone(),
        }
    }

    fn set_header(&mut self, header: &DocumentHeader) -> () {
        self.id = header.id.clone();
        self.name = header.name.clone();
        self.doctype = header.doctype.clone();
        self.doctype_version = header.doctype_version.clone();
        self.version = header.version.clone();
    }

    /// Return a string representation of the Document
    ///
    fn to_string(&self) -> String {
        self.summary()
    }

    /// Return a summary of the Document
    ///
    fn summary(&self) -> String {
        format!("Doc {:?} - {:?} - {:?}", self.doctype, self.id, self.name)
    }

    /// Return an indented JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn from_json(s: &str) -> Result<Self::Doc, ModelLoadError> {
        match serde_json::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn update_from_json(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_json::from_str(s).unwrap();
        *self = serde_json::from_value(value).unwrap();
        Ok(self)
    }

    /// Return a YAML representation of the Document
    ///
    /// partof: #SPC-serialization-yaml
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    fn from_yaml(s: &str) -> Result<Self, ModelLoadError> {
        match serde_yaml::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a YAML string
    ///
    /// partof: SPC-serialization-yaml
    fn update_from_yaml(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_yaml::from_str(s).unwrap();
        *self = serde_yaml::from_value(value).unwrap();
        Ok(self)
    }

}

impl<T> Default for Document<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "default".to_owned(),
            doctype: "".to_owned(),
            doctype_version: 1,
            version: 1,
            body: <T>::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[derive(Getable, Pushable, VmType)]
#[gluon(vm_type = "gears.i18nstring")]
pub struct I18NString {
    pub locale: String,
    pub key: String,
    pub value: String,
}

impl I18NString {
    pub fn new(s: String) -> I18NString {
        I18NString {
            locale: "".to_owned(),
            key: "".to_owned(),
            value: s,
        }
    }
}

impl Default for I18NString {
    fn default() -> I18NString {
        I18NString {
            locale: "".to_owned(),
            key: "".to_owned(),
            value: "".to_owned(),
        }
    }
}


impl I18NString {
    pub fn translate(&self, t: &TranslationDocument) -> Self {
        I18NString {
            locale: t.body.locale.clone(),
            key: self.key.clone(),
            value: t.body
                .items
                .get(&self.key)
                .unwrap_or(&I18NString::new("".to_owned()))
                .value
                .clone(),
        }
    }

    pub fn translate_self(&mut self, t: &TranslationDocument) -> () {
        match t.body.items.get(&self.key) {
            Some(item) => {
                self.locale = item.locale.clone();
                self.value = item.value.clone();
            }
            None => {
                warn!(
                    "No translation found for '{:?}' in locale '{:?}'",
                    self.key,
                    t.body.locale
                );
                self.locale = t.body.locale.clone();
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
    pub fn from_json(s: &str) -> Result<Self, ModelLoadError> {
        match serde_json::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Return a YAML representation of the DocumentHeader
    ///
    /// partof: SPC-serialization-yaml
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a DocumentHeader from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    pub fn from_yaml(s: &str) -> Result<Self, ModelLoadError> {
        match serde_yaml::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }
}
