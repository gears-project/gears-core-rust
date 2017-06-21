extern crate env_logger;

extern crate xflow;
use xflow::structure::form::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_basic_form_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/form/good/basic.json");
    let doc = FormDocument::from_json(&json_string);

    println!("Loaded domain document : {:?}", doc);

}

#[test]
fn test_load_nested_form_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/form/good/nested.json");
    let doc = FormDocument::from_json(&json_string);

    println!("Loaded domain document : {:?}", doc);

}
