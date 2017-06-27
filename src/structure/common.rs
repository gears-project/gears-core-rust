use serde;
use serde_json;
use serde_yaml;

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
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the Document
    ///
    pub fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    pub fn from_json(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    /// Return a YAML representation of the Document
    ///
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
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
