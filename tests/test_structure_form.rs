extern crate env_logger;

extern crate xflow;
use xflow::structure::page::*;

mod common;
use common::load_doc;

#[test]
fn test_load_basic_form_document() {
    let _ = env_logger::init();
    let doc = load_doc::<PageDocument>("resource/docs/page/good/basic.json");
    let _ = format!("{:?}", doc);
}

#[test]
fn test_load_nested_form_document() {
    let _ = env_logger::init();
    let doc = load_doc::<PageDocument>("resource/docs/page/good/nested.json");
    let _ = format!("{:?}", doc);
}
