extern crate env_logger;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::domain::Domain;

fn run_domain_command(domain: &mut Domain, cmdline: &str) -> () {
    match command_grammar::expression(cmdline) {
        Ok(dsl_items) => {
            match domain.consume_dsl(&dsl_items) {
                Ok(_) => {}
                Err(err) => {
                    println!("run_domain_command : error with commands : {:?}", err);
                    assert!(false);
                }
            }
        }
        Err(err) => {
            println!("test_dsl_domain : error : {:?}", err);
            assert!(false);
        }
    }
}

#[test]
fn test_dsl_domain() {
    let _ = env_logger::init();

    let mut domain = Domain::default();
    let e_count = domain.entities.len();
    run_domain_command(&mut domain, &"add entity abc;");
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);
}

#[test]
fn test_dsl_domain_multiple_commands() {
    let _ = env_logger::init();

    let mut domain = Domain::default();

    let e_count = domain.entities.len();

    run_domain_command(
        &mut domain,
        &"add entity abc; remove entity abc; add entity post;",
    );
    assert_eq!(domain.entities.len(), e_count + 1);

    let dsl = domain.generate_dsl();
    assert_eq!(dsl.len(), 1);

}
