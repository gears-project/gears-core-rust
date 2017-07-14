extern crate env_logger;

extern crate ratel;
use ratel::parser;

extern crate xflow;
use xflow::structure::xflow::*;
use xflow::generation::xflow_to_es5;

mod common;
use common::load_doc;

// partof: #TST-artifact-generation-xflow
//
#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let xflow = load_doc::<XFlowDocument>("resource/docs/xflow/flows/branch_boolean_condition.json");
    let s_es5 = xflow_to_es5::output_es5(&xflow);
    let s_es = xflow_to_es5::output(&xflow);

    let _ = parser::parse(s_es5.to_string()).expect("Must compile");
    let _ = parser::parse(s_es.to_string()).expect("Must compile");

    //
    //XXX: A little more assurance would be nice here
    assert_ne!(s_es, "");
    // println!("JS! {}", s_es5);
    println!("JS LATEST! {}", s_es);
}
