use structure::translation::TranslationDocument;
use structure::domain::DomainDocument;

pub fn translate_domain_document(translation: &TranslationDocument,
                                 doc: &DomainDocument)
                                 -> DomainDocument {
    let mut doc = doc.clone();

    for entity in &mut doc.doc.entities {
        for attribute in &mut entity.attributes {
            for validation in &mut attribute.validations {
                validation.message.translate_self(&translation);
            }
        }
    }

    doc
}
