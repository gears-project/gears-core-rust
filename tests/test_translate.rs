extern crate env_logger;

extern crate xflow;
use xflow::util::fs::*;
use xflow::structure::common::Translatable;
use xflow::structure::page::*;
use xflow::structure::translation::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_translate_model() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let model_en = model_from_fs(&"resource/projects/basic").unwrap();

    let model_en_nl = model_en.as_locale(&"nl_NL").unwrap();
    let model_en_nl_en = model_en_nl.as_locale(&"en_US").unwrap();
    let model_en_nl_en_nl = model_en_nl.as_locale(&"nl_NL").unwrap();

    assert_ne!(model_en, model_en_nl);
    assert_eq!(model_en_nl, model_en_nl_en_nl);
}

#[test]
fn test_translate_basic_page() {
    let _ = env_logger::init();
    // partof: TST-i18n

    let p_json_string = read_json_file("resource/docs/page/good/basic.json");
    let mut page = PageDocument::from_json(&p_json_string);

    let t_json_string = read_json_file("resource/docs/translation/good/basic-nl_NL.json");
    let t = TranslationDocument::from_json(&t_json_string);

    let translated_page = page.translate(&t);

    assert_ne!(page, translated_page);
}
