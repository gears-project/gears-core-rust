use serde;
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document<T> {
    pub id: String,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub doc: T,
}

impl<T> Document<T>
    where T: serde::Serialize + serde::de::DeserializeOwned
{
    /// Return a string representation of the Document
    ///
    pub fn to_string(&self) -> String {
        format!("document {}", self.id)
    }

    /// Return a JSON representation of the Document
    ///
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    pub fn from_json(json_string: &str) -> Self {
        serde_json::from_str(json_string).unwrap()
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
