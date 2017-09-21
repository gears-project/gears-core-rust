extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate gears;
use gears::dsl::command::*;
use gears::structure::common::{Document, DocumentList};

// partof: #TST-dsl-consistency

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

use gears::dsl::command::{GearsDsl, DslTree, DslTokens};

#[derive(Debug, Serialize, Deserialize)]
struct Zork {
    pub grue: bool,
}

impl Default for Zork {
    fn default() -> Zork {
        Zork { grue: false }
    }
}

impl GearsDsl for Zork {
    fn generate_dsl(&self) -> DslTokens {
        unimplemented!();
    }

    fn consume_command(&mut self, s: &str) -> Result<(), String> {
        unimplemented!();
    }

    fn consume_scope(&mut self, s: &str, tree: &Vec<DslTree>) -> Result<(), String> {
        unimplemented!();
    }
}

type ZorkList = DocumentList<Zork>;

// partof: #TST-dsl-consistency-common_document_list
//
#[test]
fn test_dsl_document_list() {
    let _ = env_logger::init();


    let mut list = ZorkList::new();

    assert!(list.interpret_dsl("add itemone;").is_ok());
    assert_eq!(list.len(), 1);

    assert!(list.interpret_dsl("add itemtwo;").is_ok());
    assert_eq!(list.len(), 2);

    assert!(list.interpret_dsl("remove itemtwo;").is_ok());
    assert_eq!(list.len(), 1);

}

#[test]
fn test_dsl_document_list_multiline() {
    let _ = env_logger::init();


    let mut list = ZorkList::new();

    assert!(
        list.interpret_dsl(
            r#"
    add itemone;
    add itemtwo;
    add itemthree;
    "#,
        ).is_ok()
    );
    assert_eq!(list.len(), 3);

    assert!(
        list.interpret_dsl(
            r#"
    remove itemone;
    remove itemtwo;
    remove itemthree;
    add newitemone;
    "#,
        ).is_ok()
    );
    assert_eq!(list.len(), 1);

}
