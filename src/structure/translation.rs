
use super::common::{Document, I18NString};

use std::collections::HashMap;

pub type TranslationDocument = Document<Translation>;
pub type TranslationMap = HashMap<String, I18NString>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Translation {
    pub locale: String,
    pub language: String,
    pub country: String,
    pub items: TranslationMap,
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
