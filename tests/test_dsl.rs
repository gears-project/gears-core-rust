extern crate env_logger;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::domain::Domain;

#[test]
fn test_dsl_domain() {
    let _ = env_logger::init();

    let mut domain = Domain::default();
    let e_count = domain.entities.len();
    domain.interpret_dsl(&"add entity abc;");
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);
}

#[test]
fn test_dsl_domain_multiple_commands() {
    let _ = env_logger::init();

    let mut domain = Domain::default();

    let e_count = domain.entities.len();

    domain.interpret_dsl(&"add entity abc; remove entity abc; add entity post;");
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);

}

#[test]
fn test_dsl_domain_generate_and_consume() {
    let _ = env_logger::init();

    let mut domain = Domain::default();

    let e_count = domain.entities.len();

    domain.interpret_dsl(&"add entity abc; remove entity abc; add entity post;");
    let script = domain.to_text_dsl();

    let mut next_domain = Domain::default();
    next_domain.interpret_dsl(&script);

    assert_eq!(domain, next_domain);

}
