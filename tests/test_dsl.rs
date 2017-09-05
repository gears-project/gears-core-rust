extern crate env_logger;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::model::Model;

#[test]
fn test_dsl_tokens_to_tree() {
    let _ = env_logger::init();

    let tokens = vec![
        DslToken::With("domain".to_owned()),
        DslToken::BlockOpen,

        DslToken::With("entity post".to_owned()),
        DslToken::BlockOpen,

        DslToken::With("attibute name".to_owned()),
        DslToken::BlockOpen,

        DslToken::Comment("This adds a default value of 'zork'".to_owned()),
        DslToken::Command("add default 'zork'".to_owned()),

        DslToken::BlockClose,
        DslToken::BlockClose,
        DslToken::BlockClose,
    ];

    match tokens_as_tree(&tokens) {
        Ok(tree) => {
            assert_eq!(tree.len(), 1);
            match tree[0] {
                DslTree::Scope(ref e, ref v) => assert_eq!(e, "domain"),
                _ => assert!(false),
            }
        }
        Err(_) => {
            assert!(false);
        }
    }
}

#[test]
fn test_dsl_model_interpret() {
    let _ = env_logger::init();

    let mut model = Model::default();
    assert_eq!(model.domain.doc.entities.len(), 0);

    assert!(
        model
            .interpret_dsl("with domain { add entity zork; };")
            .is_ok()
    );
    assert_eq!(model.domain.doc.entities.len(), 1);

    assert!(
        model
            .interpret_dsl("with domain { remove entity zork; };")
            .is_ok()
    );
    assert_eq!(model.domain.doc.entities.len(), 0);

}

#[test]
fn test_dsl_model_interpret_translations() {
    let _ = env_logger::init();

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
            .interpret_dsl(
                "with translations { add nlNL; with nlNL { add bread brood; }; };",
            )
            .is_ok()
    );

    assert!(
        model
            .interpret_dsl(
                "with translations { with nlNL { add breadA broodA; add breadB broodB; }; };",
            )
            .is_ok()
    );

}

#[test]
fn test_dsl_model_interpret_xflows() {
    let _ = env_logger::init();

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
    let _ = env_logger::init();

    let mut model = Model::default();
    assert_eq!(model.pages.len(), 0);

    assert!(model.interpret_dsl("with pages { add pageone; };").is_ok());
    assert_eq!(model.pages.len(), 1);

    assert!(model.interpret_dsl("with pages { add pagetwo; };").is_ok());
    assert_eq!(model.pages.len(), 2);
}

#[test]
fn test_dsl_model_interpret_multiline() {
    let _ = env_logger::init();

    let mut model = Model::default();
    assert_eq!(model.domain.doc.entities.len(), 0);

    assert!(
        model
            .interpret_dsl(
                &r#"with domain { add entity zork; add entity bork; add entity fnord; };"#,
            )
            .is_ok()
    );

    assert_eq!(model.domain.doc.entities.len(), 3);

    assert!(
        model
            .interpret_dsl(
                &r#"with domain { remove entity zork; remove entity bork; };"#,
            )
            .is_ok()
    );
    assert_eq!(model.domain.doc.entities.len(), 1);

}
