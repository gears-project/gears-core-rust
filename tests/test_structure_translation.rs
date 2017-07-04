extern crate env_logger;

extern crate xflow;
use xflow::structure::translation::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_basic_translation_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/translation/good/basic-nl_NL.json");
    let doc = TranslationDocument::from_json(&json_string);

    println!("Loaded translation document : {:?}", doc);
}
