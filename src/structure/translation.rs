use super::common::{Document, DocumentList, I18NString};

use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};

pub type TranslationDocument = Document<Translation>;
pub type TranslationDocumentList = DocumentList<Translation>;

pub type TranslationMap = HashMap<String, I18NString>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Translation {
    pub locale: String,
    pub language: String,
    pub country: String,
    #[serde(serialize_with = "ordered_map")]
    pub items: TranslationMap,
}

fn ordered_map<S>(value: &HashMap<String, I18NString>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // partof: SPC-serialization-fs
    // Consistent serialization
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl Default for Translation {
    fn default() -> Self {
        Translation {
            locale: "en_US".to_owned(),
            language: "en".to_owned(),
            country: "US".to_owned(),
            items: HashMap::<String, I18NString>::new(),
        }
    }
}
