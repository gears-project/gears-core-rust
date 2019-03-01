use super::common::{Document, DocumentList, I18NString};

use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};
use jsonapi::model::*;

pub type TranslationDocument = Document<Translation>;
jsonapi_model!(TranslationDocument; "translation");
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
            locale: "".to_owned(),
            language: "".to_owned(),
            country: "".to_owned(),
            items: HashMap::<String, I18NString>::new(),
        }
    }
}

impl Translation {
    pub fn add(&mut self, key: &str, value: &str) -> () {
        self.items.insert(
            key.to_string().clone(),
            I18NString {
                locale: self.locale.clone(),
                key: key.to_string().clone(),
                value: value.to_string().clone(),
            },
        );
    }
    pub fn add_untranslated_from(&mut self, item: &I18NString) -> () {
        let value = format!("-untranslated-:{}", item.value);
        self.add(&item.key, &value);
    }
}

