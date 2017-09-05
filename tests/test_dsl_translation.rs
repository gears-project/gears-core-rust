extern crate env_logger;

extern crate gears;

use gears::dsl::command::*;
use gears::structure::translation::Translation;

#[test]
fn test_dsl_translation_interpret() {
    let _ = env_logger::init();

    let mut t = Translation::default();
    assert_eq!(t.items.len(), 0);

    assert!(t.interpret_dsl("set locale en_GB;").is_ok());

    assert!(t.interpret_dsl("add key value;").is_ok());
    assert_eq!(t.items.len(), 1);

    assert!(t.interpret_dsl("add keyA valueA;").is_ok());
    assert_eq!(t.items.len(), 2);

    assert!(t.interpret_dsl("remove keyA;").is_ok());
    assert_eq!(t.items.len(), 1);

    assert!(
        t.interpret_dsl("add keyB 'Some Random Value with Number 2';")
            .is_ok()
    );
    assert_eq!(t.items.len(), 2);

}

#[test]
fn test_dsl_translation_regenerate() {
    let _ = env_logger::init();

    let mut t1 = Translation::default();
    assert!(t1.interpret_dsl("set locale en_GB;").is_ok());
    assert!(t1.interpret_dsl(r#"add keyA 'ValueA';"#).is_ok());
    assert!(t1.interpret_dsl(r#"add keyB 'ValueB';"#).is_ok());
    assert!(t1.interpret_dsl(r#"add keyB 'Value C';"#).is_ok());
    assert!(t1.interpret_dsl(r#"add keyB 'Value C 1';"#).is_ok());

    let mut t2 = Translation::default();
    assert!(t2.consume_dsl(&t1.generate_dsl()).is_ok());

    assert_eq!(t1, t2);

}
