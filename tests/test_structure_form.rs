extern crate env_logger;

extern crate xflow;
use xflow::structure::form::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/form/good/basic.json");
    let doc = FormDocument::from_json(&json_string);

    println!("Loaded domain document : {:?}", doc);

}
