use std::collections::HashMap;

use super::common::{Document, Translatable, I18NString};
use super::domain;
use super::xflow;
use super::page;
use super::translation::TranslationDocument;

pub type ModelDocument = Document<Model>;
pub type ModelConfigDocument = Document<ModelConfig>;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Model {
    pub config: ModelConfigDocument,
    pub domain: domain::DomainDocument,
    pub xflows: Vec<xflow::XFlowDocument>,
    pub pages: Vec<page::PageDocument>,
    pub translations: Vec<TranslationDocument>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            config: ModelConfigDocument::default(),
            domain: domain::DomainDocument::default(),
            xflows: Vec::<xflow::XFlowDocument>::new(),
            pages: Vec::<page::PageDocument>::new(),
            translations: Vec::<TranslationDocument>::new(),
        }
    }
}

fn pad_translation_doc(t: &mut TranslationDocument,
                       strings_in_model: &HashMap<String, &I18NString>)
                       -> () {
    for (key, item) in strings_in_model {
        if !t.doc.items.contains_key(key) {
            let value = format!("UNTRANSLATED {:?}", item.value);
            let item = I18NString {
                locale: t.doc.locale.clone(),
                key: key.clone(),
                value: value,
            };
            debug!("Untranslated string, locale :'{:?}', value '{:?}'",
                   item.locale,
                   item.value);
            t.doc.items.insert(key.clone(), item);
        }
    }
}

impl ModelDocument {
    pub fn as_locale(&self, locale: &str) -> Result<ModelDocument, String> {

        match self.get_translation(&locale) {
            Ok(translation) => {
                let mut model = self.clone();
                model.translate_in_place(&translation);
                Ok(model)
            }
            Err(err) => Err(err.to_string()),
        }

    }

    pub fn has_translation(&self, locale: &str) -> bool {
        match self.get_translation(&locale) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_translation(&self, locale: &str) -> Result<&TranslationDocument, String> {
        let res: Vec<&TranslationDocument> = self.doc
            .translations
            .iter()
            .filter({
                        |t| t.doc.locale == locale
                    })
            .collect();

        match res.len() {
            0 => Err("Locale not available in this document".to_owned()),
            1 => Ok(&res[0]),
            _ => Err("More than one instance of this locale available in this document".to_owned()),
        }
    }

    /*
    pub fn pad_all_translations(&mut self) -> () {
        unimplemented!();
        let mut i18n_items_in_model = HashMap::<String, &I18NString>::new();
        for item in self.all_i18n_strings() {
            i18n_items_in_model.insert(item.key.clone(), item);
        }

        let locales = self.doc.config.doc.locales.clone();

        for locale in locales {
            match self.get_translation(&locale) {
                Ok(ref mut t) => {
                    pad_translation_doc(t, &i18n_items_in_model);
                }
                Err(_) => {
                    let mut t = TranslationDocument::default();
                    t.doc.locale = locale.clone();
                    // self.doc.translations.push(t);
                    // pad_translation_doc(t, &i18n_items_in_model);
                }
            };

        }
    }
    */
}

impl Translatable for ModelDocument {
    fn translate_in_place(&mut self, t: &TranslationDocument) -> () {
        for ref mut page in &mut self.doc.pages {
            page.translate_in_place(&t);
        }
        self.doc.domain.translate_in_place(&t);
    }

    fn translate(&self, t: &TranslationDocument) -> ModelDocument {
        let mut doc = self.clone();
        doc.translate_in_place(&t);
        doc
    }

    fn all_i18n_strings(&self) -> Vec<&I18NString> {
        let mut ts = Vec::<&I18NString>::new();

        for ref page in &self.doc.pages {
            ts.append(&mut page.all_i18n_strings());
        }

        ts.append(&mut self.doc.domain.all_i18n_strings());

        ts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ModelConfig {
    pub default_locale: String,
    pub active_locale: String,
    pub locales: Vec<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            default_locale: "en_US".to_owned(),
            active_locale: "en_US".to_owned(),
            locales: vec!["en_US".to_owned()],
        }
    }
}
