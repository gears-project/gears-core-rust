extern crate env_logger;

extern crate gears;

use gears::dsl::command::*;
use gears::structure::model::Model;

#[test]
fn test_dsl_model_interpret() {
    let _ = env_logger::try_init();

    let mut model = Model::default();
    assert_eq!(model.domain.body.entities.len(), 0);

    assert!(
        model
            .interpret_dsl("with domain { add entity zork; };")
            .is_ok()
    );
    assert_eq!(model.domain.body.entities.len(), 1);

    assert!(
        model
            .interpret_dsl("with domain { remove entity zork; };")
            .is_ok()
    );
    assert_eq!(model.domain.body.entities.len(), 0);

}

#[test]
fn test_dsl_model_interpret_translations() {
    let _ = env_logger::try_init();

    let mut model = Model::default();
    assert_eq!(model.translations.len(), 0);

    assert!(
        model
            .interpret_dsl("with translations { add enGB; };")
            .is_ok()
    );
    assert_eq!(model.translations.len(), 1);

    assert!(
        model
            .interpret_dsl("with translations { add esES; };")
            .is_ok()
    );
    assert_eq!(model.translations.len(), 2);

    assert!(
        model
            .interpret_dsl("with translations { add es_ES; };")
            .is_ok()
    );
    assert_eq!(model.translations.len(), 3);

    assert!(
        model
            .interpret_dsl(
                "with translations { add nl_NL; with nl_NL { set locale nl_NL; add bread brood; }; };",
            )
            .is_ok()
    );

    assert!(
        model
            .interpret_dsl(
                "with translations { with nl_NL { add breadA broodA; add breadB broodB; }; };",
            )
            .is_ok()
    );

    assert!(
        model
            .interpret_dsl(
                r#"with translations { with nl_NL { add good_bread 'goed brood'; }; };"#,
            )
            .is_ok()
    );

}

#[test]
fn test_dsl_model_interpret_xflows() {
    let _ = env_logger::try_init();

    let mut model = Model::default();
    assert_eq!(model.xflows.len(), 0);

    assert!(model.interpret_dsl("with xflows { add entry; };").is_ok());
    assert_eq!(model.xflows.len(), 1);

    assert!(
        model
            .interpret_dsl("with xflows { add validation; };")
            .is_ok()
    );
    assert_eq!(model.xflows.len(), 2);
}

#[test]
fn test_dsl_model_interpret_pages() {
    let _ = env_logger::try_init();

    let mut model = Model::default();
    assert_eq!(model.pages.len(), 0);

    assert!(model.interpret_dsl("with pages { add pageone; };").is_ok());
    assert_eq!(model.pages.len(), 1);

    assert!(model.interpret_dsl("with pages { add pagetwo; };").is_ok());
    assert_eq!(model.pages.len(), 2);
}

#[test]
fn test_dsl_model_interpret_multiline() {
    let _ = env_logger::try_init();

    let mut model = Model::default();
    assert_eq!(model.domain.body.entities.len(), 0);

    assert!(
        model
            .interpret_dsl(
                &r#"with domain { add entity zork; add entity bork; add entity fnord; };"#,
            )
            .is_ok()
    );

    assert_eq!(model.domain.body.entities.len(), 3);

    assert!(
        model
            .interpret_dsl(
                &r#"with domain { remove entity zork; remove entity bork; };"#,
            )
            .is_ok()
    );
    assert_eq!(model.domain.body.entities.len(), 1);

}
