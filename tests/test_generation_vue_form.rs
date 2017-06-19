extern crate env_logger;

extern crate xflow;
use xflow::structure::form::*;
use xflow::generation::vue_form::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_basic_form_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/form/good/basic.json");
    let form = FormDocument::from_json(&json_string);
    let s = VueForm::output_html(&form);
    assert_eq!(s, "");
}
