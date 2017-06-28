extern crate env_logger;

extern crate xflow;
use xflow::structure::page::*;
use xflow::generation::vue_form;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/page/good/nested.json");
    let form = PageDocument::from_json(&json_string);
    let s = vue_form::output_html(&form);
    //
    //XXX: A little more assurance would be nice here
    assert_ne!(s, "");
}

#[test]
fn test_load_basic_form_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/page/good/form.json");
    let form = PageDocument::from_json(&json_string);
    let s = vue_form::output_html(&form);
    //
    //XXX: A little more assurance would be nice here
    assert_ne!(s, "");
}
