use super::common::{Document, Translatable};
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

impl ModelDocument {
    pub fn as_locale(&self, locale: &str) -> Result<ModelDocument, String> {
        let translation = &self.doc.translations[0];

        let res: Vec<&TranslationDocument> = self.doc
            .translations
            .iter()
            .filter({
                        |t| t.doc.locale == locale
                    })
            .collect();

        let x = match res.len() {
            0 => Err("Locale not available in this document"),
            1 => Ok(res[0]),
            _ => Err("More than one instance of this locale available in this document"),
        };

        match x {
            Ok(translation) => {
                let mut model = self.clone();
                model.translate(&translation);
                Ok(model)
            }
            Err(err) => Err(err.to_string()),
        }

    }
}

impl Translatable for ModelDocument {
    fn translate_in_place(&mut self, t: &TranslationDocument) -> () {
        for ref mut page in &mut self.doc.pages {
            page.translate_in_place(&t);
        }
    }

    fn translate(&self, t: &TranslationDocument) -> ModelDocument {
        let mut doc = self.clone();
        doc.translate_in_place(&t);
        doc
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
