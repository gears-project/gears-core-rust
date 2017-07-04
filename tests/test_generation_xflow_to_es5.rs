extern crate env_logger;

extern crate xflow;
use xflow::structure::xflow::*;
use xflow::generation::xflow_to_es5;

mod common;
use common::load_doc;

#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let xflow = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    // let s = xflow_to_es5::output_es5(&xflow);
    //
    //XXX: A little more assurance would be nice here
    // assert_ne!(s, "");
    // println!("JS! {}", s.into_bytes(());
}
