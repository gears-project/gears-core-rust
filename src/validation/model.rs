use validation::common::{ValidationError, ValidationErrors};
use structure::model::ModelDocument;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(model: &ModelDocument) -> ValidationErrors {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_locales_have_translation_docs(&model));
        errors.extend(Validation::all_translation_docs_have_locales(&model));
        errors.extend(Validation::all_page_xflow_references_point_to_existing_entities(&model));
        errors.extend(Validation::all_doc_collections_have_correct_unique_keys(&model));

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

    pub fn all_page_xflow_references_point_to_existing_entities(model: &ModelDocument)
                                                                -> Vec<ValidationError> {
        debug!("all_page_xflow_references_point_to_existing_entities");
        let mut errors = Vec::<ValidationError>::new();
        let xflow_ids = model.all_xflow_ids();

        for page in &model.doc.pages {
            for xflow_reference in &page.all_xflow_references() {
                if !xflow_ids.contains(xflow_reference) {
                    let message = format!("Page: Contains a reference to xflow id '{}', which does not exist in this model",
                                          xflow_reference);
                    errors.push(ValidationError {
                                    code: 1,
                                    message: message,
                                    paths: vec![format!("/pages/{id}", id = page.id)],
                                });
                }
            }
        }

        errors
    }

    pub fn all_doc_collections_have_correct_unique_keys(model: &ModelDocument)
                                                        -> Vec<ValidationError> {
        debug!("all_doc_collections_have_correct_unique_keys");
        let mut errors = Vec::<ValidationError>::new();

        let mut xflow_ids = HashSet::<&String>::new();

        for doc in &model.doc.xflows {
            if xflow_ids.contains(&doc.id) {
                let message = format!("xflow: Duplicate ID found in XFlow document '{}'", doc.id);
                errors.push(ValidationError {
                                code: 1,
                                message: message,
                                paths: vec![format!("/xflow/{id}", id = doc.id)],
                            });
            } else {
                xflow_ids.insert(&doc.id);
            }
        }

        let mut page_ids = HashSet::<&String>::new();

        for doc in &model.doc.pages {
            if page_ids.contains(&doc.id) {
                let message = format!("page: Duplicate ID found in page document '{}'", doc.id);
                errors.push(ValidationError {
                                code: 1,
                                message: message,
                                paths: vec![format!("/page/{id}", id = doc.id)],
                            });
            } else {
                page_ids.insert(&doc.id);
            }
        }

        let mut translation_ids = HashSet::<&String>::new();

        for doc in &model.doc.translations {
            if translation_ids.contains(&doc.id) {
                let message = format!("translation: Duplicate ID found in translation document '{}'",
                                      doc.id);
                errors.push(ValidationError {
                                code: 1,
                                message: message,
                                paths: vec![format!("/translation/{id}", id = doc.id)],
                            });
            } else {
                translation_ids.insert(&doc.id);
            }
        }

        errors
    }
}
