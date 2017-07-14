use validation::common::ValidationError;
use structure::translation::TranslationDocument;

#[derive(Debug)]
pub struct Validation {}

impl Validation {
    pub fn validate(t: &TranslationDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        errors.extend(Validation::all_items_have_correct_key(&t));

        errors
    }

    pub fn all_items_have_correct_key(t: &TranslationDocument) -> Vec<ValidationError> {
        let mut errors = Vec::<ValidationError>::new();

        for (key, item) in &t.doc.items {
            if !key.eq(&item.key) {
                let message = format!("Translation document {locale} has an item with key '{itemkey}' stored using a different key '{key}'",
                                      locale = t.doc.locale,
                                      itemkey = item.key,
                                      key = key);
                errors.push(ValidationError {
                                code: 1,
                                message: message,
                                paths: vec![format!("/domain/translations/{locale}/items",
                                                    locale = t.doc.locale)],
                            });
            }
            if !t.doc.locale.eq(&item.locale) {
                let message = format!("Translation document {locale} has an item with key '{key}' set to a different locale '{itemlocale}'",
                                      locale = t.doc.locale,
                                      itemlocale = item.locale,
                                      key = key);
                errors.push(ValidationError {
                                code: 1,
                                message: message,
                                paths: vec![format!("/domain/translations/{locale}/items",
                                                    locale = t.doc.locale)],
                            });
            }
        }

        errors
    }
}
