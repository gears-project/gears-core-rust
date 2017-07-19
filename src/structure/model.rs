use std::collections::{HashMap, HashSet};
use uuid::Uuid;

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

    pub fn all_xflow_ids(&self) -> HashSet<&Uuid> {
        let mut xflow_ids = HashSet::<&Uuid>::new();

        for xflow in &self.doc.xflows {
            xflow_ids.insert(&xflow.id);
        }

        xflow_ids

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

    fn all_i18n_strings_map(&self) -> HashMap<String, &I18NString> {
        let mut i18n_items_in_model = HashMap::<String, &I18NString>::new();
        for item in self.all_i18n_strings() {
            i18n_items_in_model.insert(item.key.clone(), &item);
        }
        i18n_items_in_model
    }

    pub fn has_locale(&self, locale: &str) -> bool {
        let res: Vec<&String> = self.doc
            .config
            .doc
            .locales
            .iter()
            .filter({
                        |l1| *l1 == locale
                    })
            .collect();

        match res.len() {
            0 => false,
            _ => true,
        }

    }

    pub fn add_locale(&mut self, locale: &str) -> Result<(), String> {
        if !self.has_locale(&locale) {
            self.doc.config.doc.locales.push(locale.to_owned());
            Ok(())
        } else {
            let msg = format!("add_locale : The locale '{:?}' already exists", locale);
            Err(msg)
        }
    }

    pub fn pad_all_translations(&mut self) -> () {

        let missing: Vec<&String> = self.doc
            .config
            .doc
            .locales
            .iter()
            .filter(|l| !self.has_translation(l))
            .collect();

        for locale in missing {
            info!("Adding new translation for locale '{:?}'", locale);
            let mut t = TranslationDocument::default();
            t.doc.locale = locale.clone();
            for (_, item) in self.all_i18n_strings_map() {
                let new_item = I18NString {
                    key: item.key.clone(),
                    locale: locale.clone(),
                    value: format!("-untranslated-:{}", item.value),
                };
                t.doc.items.insert(new_item.key.clone(), new_item);
            }
            self.doc.translations.push(t);
        }
    }
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
