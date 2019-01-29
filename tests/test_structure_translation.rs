extern crate env_logger;

extern crate gears;
use gears::structure::translation::*;
use gears::structure::common::I18NString;

mod common;
use crate::common::load_doc;

#[test]
fn test_load_basic_translation_document() {
    let _ = env_logger::try_init();
    let doc = load_doc::<TranslationDocument>("resource/docs/translation/good/nl_NL.json");
    let _ = format!("{:?}", doc);
}

#[test]
fn test_translation_ordered_hash_deserialization() {
    let _ = env_logger::try_init();
    let mut t_a = load_doc::<TranslationDocument>("resource/docs/translation/good/nl_NL.json");
    let mut t_b = load_doc::<TranslationDocument>("resource/docs/translation/good/nl_NL.json");

    let item_1 = I18NString {
        locale: "nl_NL".to_owned(),
        key: "key1".to_owned(),
        value: "value1".to_owned(),
    };

    let item_2 = I18NString {
        locale: "nl_NL".to_owned(),
        key: "key2".to_owned(),
        value: "value2".to_owned(),
    };

    let item_3 = I18NString {
        locale: "nl_NL".to_owned(),
        key: "key3".to_owned(),
        value: "value3".to_owned(),
    };

    //
    // Insert order differs between t_a and t_b
    //

    t_a.body.items.insert(item_1.key.clone(), item_1.clone());
    t_a.body.items.insert(item_2.key.clone(), item_2.clone());
    t_a.body.items.insert(item_3.key.clone(), item_3.clone());

    t_b.body.items.insert(item_3.key.clone(), item_3.clone());
    t_b.body.items.insert(item_2.key.clone(), item_2.clone());
    t_b.body.items.insert(item_1.key.clone(), item_1.clone());

    //
    // Deserialization should be the same
    //

    assert_eq!(t_a.to_json(), t_b.to_json());
}
