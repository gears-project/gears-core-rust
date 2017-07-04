extern crate env_logger;

extern crate xflow;
use xflow::util::fs::*;
use xflow::structure::model::*;
use xflow::structure::common::Translatable;

#[test]
fn test_translate_model() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let model_en = model_from_fs(&"resource/projects/basic").unwrap();

    /*
    let ref translation = model_en.doc.translations[0].clone();
    */

    let model_en_nl = model_en.clone();
    model_en_nl.as_locale(&"nl_NL").unwrap();

    let model_en_nl_en = model_en_nl.clone();
    model_en_nl_en.as_locale(&"en_US").unwrap();

    // assert_ne!(model_en, model_en_nl);
    assert_eq!(model_en, model_en_nl_en);
}
