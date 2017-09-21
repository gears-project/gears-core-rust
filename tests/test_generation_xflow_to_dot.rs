extern crate env_logger;

extern crate gears;

use gears::structure::xflow::*;
use gears::generation::xflow_to_dot;

mod common;
use common::load_doc;

// partof: TST-artifact-generation-xflow
//
#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let xflow = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    let res = xflow_to_dot::output(&xflow);

    assert_ne!(res, "");
    println!("DOTFILE! {}", res);
}
