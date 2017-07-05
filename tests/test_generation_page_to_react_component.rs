extern crate env_logger;

extern crate xflow;
use xflow::structure::page::*;
use xflow::generation::page_to_react_component;

mod common;
use common::load_doc;

#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let form = load_doc::<PageDocument>("resource/docs/page/good/nested.json");
    let s = page_to_react_component::output_html(&form);
    //
    //XXX: A little more assurance would be nice here
    assert_ne!(s, "");
}

#[test]
fn test_load_basic_form_document() {
    let _ = env_logger::init();

    let form = load_doc::<PageDocument>("resource/docs/page/good/form.json");
    let s = page_to_react_component::output_html(&form);
    //
    //XXX: A little more assurance would be nice here
    assert_ne!(s, "");
}
