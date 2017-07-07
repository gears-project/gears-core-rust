use validation::common::{ValidationError, ValidationErrors};
use structure::model::ModelDocument;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(model: &ModelDocument) -> ValidationErrors {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_locales_have_translation_docs(&model));
        errors.extend(Validation::all_translation_docs_have_locales(&model));

        errors
    }

    pub fn all_locales_have_translation_docs(model: &ModelDocument) -> ValidationErrors {
        debug!("all_locales_have_translation_docs");
        let mut errors = Vec::<ValidationError>::new();

        for locale in &model.doc.config.doc.locales {
            if !model.has_translation(&locale) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("Model: locale '{:?}' has no translation document",
                                                 &locale),
                                paths: vec!["/config/locales".to_owned()],
                            });
            }
        }

        errors
    }

    pub fn all_translation_docs_have_locales(model: &ModelDocument) -> Vec<ValidationError> {
        debug!("all_translation_docs_have_locales");
        let mut errors = Vec::<ValidationError>::new();

        for t in &model.doc.translations {
            if !model.has_locale(&t.doc.locale) {
                errors.push(ValidationError {
                                code: 1,
                                message: format!("Model: translation doc exists with locale '{:?}' but it is not listed as a supported locale",
                                                 &t.doc.locale),
                                paths: vec!["/config/translations".to_owned()],
                            });
            }
        }

        errors
    }
}
