extern crate env_logger;

extern crate xflow;

use xflow::util::fs::*;
use xflow::structure::common::Translatable;
use xflow::structure::domain::*;
use xflow::structure::page::*;
use xflow::structure::translation::*;

mod common;
use common::load_doc;

#[test]
fn test_translate_model() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let model_en = model_from_fs(&"resource/projects/basic").unwrap();

    let model_en_nl = model_en.as_locale(&"nl_NL").unwrap();
    let model_en_nl_en = model_en_nl.as_locale(&"en_US").unwrap();
    let model_en_nl_en_nl = model_en_nl_en.as_locale(&"nl_NL").unwrap();

    assert_ne!(model_en, model_en_nl);
    assert_eq!(model_en_nl, model_en_nl_en_nl);
}

#[test]
fn test_translate_basic_page() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let page = load_doc::<PageDocument>("resource/docs/page/good/basic.json");
    let t = load_doc::<TranslationDocument>("resource/docs/translation/good/basic-nl_NL.json");

    let translated_page = page.translate(&t);

    assert_ne!(page, translated_page);
}

#[test]
fn test_translate_basic_domain() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let domain = load_doc::<DomainDocument>("resource/docs/domain/good/basic.json");
    let t_nl = load_doc::<TranslationDocument>("resource/docs/translation/good/basic-nl_NL.json");
    let t_en = load_doc::<TranslationDocument>("resource/docs/translation/good/basic-en_US.json");

    let domain_nl = domain.translate(&t_nl);
    let domain_en = domain_nl.translate(&t_en);

    assert_ne!(domain, domain_nl);
    assert_ne!(domain_nl, domain_en);
}

#[test]
fn test_translate_model_add_translations() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let mut model = model_from_fs(&"resource/projects/basic").unwrap();
    model.doc.config.doc.locales.push("es_ES".to_owned());
    model.pad_all_translations();

    let model_es = model.as_locale(&"es_ES").unwrap();

    assert_ne!(model, model_es);
}
