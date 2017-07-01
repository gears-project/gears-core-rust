extern crate env_logger;

extern crate xflow;
use xflow::structure::xflow::*;
use xflow::generation::xflow_to_es5;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();

    let json_string = read_json_file("resource/docs/xflow/flows/10_steps.json");
    let xflow = XFlowDocument::from_json(&json_string);
    // let s = xflow_to_es5::output_es5(&xflow);
    //
    //XXX: A little more assurance would be nice here
    // assert_ne!(s, "");
    // println!("JS! {}", s.into_bytes(());
}
