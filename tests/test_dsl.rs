extern crate env_logger;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::model::ModelDocument;

#[test]
fn test_parse_list() {
    let _ = env_logger::init();

    match parse_command(&"list xflow") {
        Ok(cmd) => assert_eq!(cmd, Command::List(ModelComponent::XFlow)),
        Err(_) => assert!(false),
    }

    match parse_command(&"list translation") {
        Ok(cmd) => assert_eq!(cmd, Command::List(ModelComponent::Translation)),
        Err(_) => assert!(false),
    }

    match parse_command(&"list page") {
        Ok(cmd) => assert_eq!(cmd, Command::List(ModelComponent::Page)),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parse_set() {
    let _ = env_logger::init();

    match parse_command(&"set a b") {
        Ok(cmd) => assert_eq!(cmd, Command::Set("a".to_owned(), "b".to_owned())),
        Err(_) => assert!(false),
    }

}

#[test]
fn test_parse_generate() {
    let _ = env_logger::init();

    match parse_command(&"generate xflow abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Generate(ModelComponent::XFlow, "abc".to_owned())
            )
        }
        Err(_) => assert!(false),
    }

    match parse_command(&"generate translation es_ES") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Generate(ModelComponent::Translation, "es_ES".to_owned())
            )
        }
        Err(_) => assert!(false),
    }

    match parse_command(&"generate page abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Generate(ModelComponent::Page, "abc".to_owned())
            )
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parse_destroy() {
    let _ = env_logger::init();

    match parse_command(&"destroy xflow abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Destroy(ModelComponent::XFlow, "abc".to_owned())
            )
        }
        Err(_) => assert!(false),
    }

    match parse_command(&"destroy translation es_ES") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Destroy(ModelComponent::Translation, "es_ES".to_owned())
            )
        }
        Err(_) => assert!(false),
    }

    match parse_command(&"destroy page abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                Command::Destroy(ModelComponent::Page, "abc".to_owned())
            )
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parse_domain_dsl() {
    let _ = env_logger::init();

    match parse_dsl_command(&"with domain add entity abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                DslCommand::Domain(DomainCommand::AddEntity("abc".to_owned()))
            )
        }
        Err(_) => assert!(false),
    }

    match parse_dsl_command(&"with domain remove entity abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                DslCommand::Domain(DomainCommand::RemoveEntity("abc".to_owned()))
            )
        }
        Err(_) => assert!(false),
    }

    match parse_dsl_command(&"with domain entity abc add attribute name:string") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                DslCommand::Domain(DomainCommand::AddAttribute(
                    "abc".to_owned(),
                    "name".to_string(),
                    "string".to_string(),
                ))
            )
        }
        Err(_) => assert!(false),
    }

    match parse_dsl_command(&"with domain entity abc remove attribute name") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                DslCommand::Domain(DomainCommand::RemoveAttribute(
                    "abc".to_owned(),
                    "name".to_string(),
                ))
            )
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parse_xflow_dsl() {
    let _ = env_logger::init();

    match parse_dsl_command(&"with xflow add node abc") {
        Ok(cmd) => {
            assert_eq!(
                cmd,
                DslCommand::XFlow(XFlowCommand::AddNode("abc".to_owned()))
            )
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn test_dsl_change_model() {
    let _ = env_logger::init();
    let mut model = ModelDocument::default();

    match parse_dsl_command(&"with domain add entity abc") {
        Ok(cmd) => {
            match run_command(&mut model, &cmd) {
                Ok(_) => {
                    assert_eq!(model.doc.domain.doc.entities.len(), 1);
                    assert_eq!(model.doc.domain.doc.entities[0].name, "abc".to_owned())
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }

    match parse_dsl_command(&"with config set default_locale es_ES") {
        Ok(cmd) => {
            match run_command(&mut model, &cmd) {
                Ok(_) => {
                    assert_eq!(model.doc.config.doc.default_locale, "es_ES".to_owned());
                }
                Err(_) => assert!(false),
            }
        }
        Err(_) => assert!(false),
    }


}
