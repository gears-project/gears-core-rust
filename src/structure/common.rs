use serde;
use serde_json;
use serde_yaml;

use structure::translation::TranslationDocument;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Document<T> {
    pub id: String,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub doc: T,
}

impl<T> Document<T>
    where T: serde::Serialize + serde::de::DeserializeOwned + Eq
{
    /// Return a string representation of the Document
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
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
            id: "".to_owned(),
            name: "".to_owned(),
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
