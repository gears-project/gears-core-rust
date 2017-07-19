extern crate env_logger;
extern crate serde_json;
extern crate uuid;
extern crate gears;

use gears::structure::page::*;
use uuid::Uuid;
use std::collections::HashMap;

mod common;
use common::load_doc;

#[test]
fn test_load_basic_page_document() {
    let _ = env_logger::init();
    let doc = load_doc::<PageDocument>("resource/docs/page/good/basic.json");
    let _ = format!("{:?}", doc);
}

#[test]
fn test_load_nested_page_document() {
    let _ = env_logger::init();
    let doc = load_doc::<PageDocument>("resource/docs/page/good/nested.json");
    let _ = format!("{:?}", doc);
}

#[test]
fn test_page_component_datatable_config_ordered_hash_deserialization() {
    let _ = env_logger::init();

    let mut dtc_a = DatatableConfig {
        entity: "Post".to_owned(),
        attributes: Vec::<String>::new(),
        eventbindings: HashMap::<String, Uuid>::new(),
    };

    let mut dtc_b = dtc_a.clone();

    assert_eq!(serde_json::to_string_pretty(&dtc_a).unwrap(),
               serde_json::to_string_pretty(&dtc_b).unwrap());

    let uuid_1 = Uuid::new_v4();
    let uuid_2 = Uuid::new_v4();
    let uuid_3 = Uuid::new_v4();
    let uuid_4 = Uuid::new_v4();

    //
    // Insertion order of items is different for dtc_a and dtc_b
    //

    dtc_a.eventbindings.insert("1".to_owned(), uuid_1.clone());
    dtc_a.eventbindings.insert("2".to_owned(), uuid_2.clone());
    dtc_a.eventbindings.insert("3".to_owned(), uuid_3.clone());
    dtc_a.eventbindings.insert("4".to_owned(), uuid_4.clone());

    dtc_b.eventbindings.insert("4".to_owned(), uuid_4.clone());
    dtc_b.eventbindings.insert("3".to_owned(), uuid_3.clone());
    dtc_b.eventbindings.insert("2".to_owned(), uuid_2.clone());
    dtc_b.eventbindings.insert("1".to_owned(), uuid_1.clone());

    //
    // Deserialization should be identical
    //

    assert_eq!(serde_json::to_string_pretty(&dtc_a).unwrap(),
               serde_json::to_string_pretty(&dtc_b).unwrap());

    //
    // Also check ne condition just in case
    //

    let uuid_5 = Uuid::new_v4();
    dtc_a.eventbindings.insert("5".to_owned(), uuid_5.clone());

    assert_ne!(serde_json::to_string_pretty(&dtc_a).unwrap(),
               serde_json::to_string_pretty(&dtc_b).unwrap());

}
