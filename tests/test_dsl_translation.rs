extern crate env_logger;

extern crate gears;

use gears::dsl::command::*;
use gears::structure::translation::Translation;

#[test]
fn test_dsl_translation_interpret() {
    let _ = env_logger::init();

    let mut t = Translation::default();
    assert_eq!(t.items.len(), 0);

    t.interpret_dsl("add key value;").is_ok();
    assert_eq!(t.items.len(), 1);

    t.interpret_dsl("add keyA valueA;").is_ok();
    assert_eq!(t.items.len(), 2);

    t.interpret_dsl("remove keyA;").is_ok();
    assert_eq!(t.items.len(), 1);
}
