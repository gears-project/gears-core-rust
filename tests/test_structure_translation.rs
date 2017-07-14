extern crate env_logger;

extern crate xflow;
use xflow::structure::translation::*;

mod common;
use common::load_doc;

#[test]
fn test_load_basic_translation_document() {
    let _ = env_logger::init();
    let doc = load_doc::<TranslationDocument>("resource/docs/translation/good/nl_NL.json");
    let _ = format!("{:?}", doc);
}
