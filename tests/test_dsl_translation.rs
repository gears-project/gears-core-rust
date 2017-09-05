extern crate env_logger;

extern crate gears;

use gears::dsl::command::*;
use gears::structure::translation::Translation;

#[test]
fn test_dsl_translation_interpret() {
    let _ = env_logger::init();

    let mut t = Translation::default();
    assert_eq!(t.items.len(), 0);

    assert!(t.interpret_dsl("add key value;").is_ok());
    assert_eq!(t.items.len(), 1);

    assert!(t.interpret_dsl("add keyA valueA;").is_ok());
    assert_eq!(t.items.len(), 2);

    assert!(t.interpret_dsl("remove keyA;").is_ok());
    assert_eq!(t.items.len(), 1);
}

#[test]
fn test_dsl_translation_regenerate() {
    let _ = env_logger::init();

    let mut t1 = Translation::default();
    assert!(t1.interpret_dsl(r#"add keyA 'ValueA';"#).is_ok());
    assert!(t1.interpret_dsl(r#"add keyB 'ValueB';"#).is_ok());

    let mut t2 = Translation::default();
    t2.consume_dsl(&t1.generate_dsl());

    assert_eq!(t1, t2);

}
