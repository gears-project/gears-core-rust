extern crate env_logger;

extern crate gears;

use gears::structure::domain::*;
use gears::generation::domain_to_dot;

mod common;
use common::load_doc;

// partof: TST-artifact-generation-domain
//
#[test]
fn test_load_basic_document() {
    let _ = env_logger::init();

    let domain = load_doc::<DomainDocument>("resource/docs/domain/good/basic.json");
    let res = domain_to_dot::output(&domain);

    assert_ne!(res, "");
    println!("DOTFILE! {}", res);
}
