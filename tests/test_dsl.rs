extern crate env_logger;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::model::Model;
use gears::structure::domain::Domain;

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
fn test_dsl_domain() {
    let _ = env_logger::init();

    let mut domain = Domain::default();
    let e_count = domain.entities.len();
    domain.interpret_dsl("add entity abc;");
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);
}

#[test]
fn test_dsl_domain_multiple_commands() {
    let _ = env_logger::init();

    let mut domain = Domain::default();

    let e_count = domain.entities.len();

    domain.interpret_dsl("add entity abc; remove entity abc; add entity post;");
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);

}

#[test]
fn test_dsl_domain_generate_and_consume() {
    let _ = env_logger::init();

    let mut domain = Domain::default();

    let e_count = domain.entities.len();

    domain
        .interpret_dsl("add entity abc; remove entity abc; add entity post; add entity comment; add entity log;")
        .ok();

    let script = domain.to_text_dsl();

    let mut next_domain = Domain::default();
    next_domain.interpret_dsl(&script).ok();

    assert_eq!(domain, next_domain);
}

#[test]
fn test_dsl_model_interpret() {
    let _ = env_logger::init();

    let mut model = Model::default();
    assert_eq!(model.domain.doc.entities.len(), 0);

    let _ = model.interpret_dsl("with domain { add entity zork; };");
    assert_eq!(model.domain.doc.entities.len(), 1);

    let _ = model.interpret_dsl("with domain { remove entity zork; };");
    assert_eq!(model.domain.doc.entities.len(), 0);

}

#[test]
fn test_dsl_model_interpret_multiline() {
    let _ = env_logger::init();

    let mut model = Model::default();
    assert_eq!(model.domain.doc.entities.len(), 0);

    let _ = model.interpret_dsl(
        &r#"with domain { add entity zork; add entity bork; add entity fnord; };"#,
    );
    assert_eq!(model.domain.doc.entities.len(), 3);

    let _ = model.interpret_dsl(
        &r#"with domain { remove entity zork; remove entity bork; };"#,
    );
    assert_eq!(model.domain.doc.entities.len(), 1);

}
