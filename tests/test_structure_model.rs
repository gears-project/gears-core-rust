extern crate env_logger;

extern crate gears;
use gears::util::fs::*;
use gears::util::naming::{label_to_uuid};
use gears::structure::model::*;
use gears::structure::common::DocumentNature;
use gears::structure::common::DocumentFileSystemLoadable;
use gears::structure::model::ModelDocument;

#[test]
fn test_load_model() {
    let _ = env_logger::try_init();
    // TST-serialization
    // #TST-serialization-yaml
    // partof: TST-serialization-fs
    // partof: TST-serialization-json

    let model_a = ModelDocument::load_from_filesystem(&"resource/projects/basic").unwrap();
    let json_a = model_a.to_json();
    let model_b = match ModelDocument::from_json(&json_a) {
        Ok(res) => res,
        Err(_) => {
            assert!(false);
            return ()
        }
    };
    let yaml_a = model_b.to_yaml();
    let model_c = match ModelDocument::from_yaml(&yaml_a) {
        Ok(res) => res,
        Err(_) => {
            assert!(false);
            return ()
        }
    };

    assert_eq!(model_a.id, model_c.id);
    assert_eq!(model_a.body, model_c.body);
    assert_eq!(model_a.body.xflows.len(), model_c.body.xflows.len());
    assert_eq!(
        model_a.body.pages[0].to_json(),
        model_c.body.pages[0].to_json()
    );
    assert_eq!(
        model_a.body.xflows[0].to_json(),
        model_c.body.xflows[0].to_json()
    );
}

#[test]
fn test_model_collection_helpers() {
    let _ = env_logger::try_init();

    let model = ModelDocument::load_from_filesystem(&"resource/projects/basic").unwrap();

    assert!(model.has_xflow(&label_to_uuid("e4f0518a-fd0d-403e-9c20-79041c1c14ae").unwrap()));
    assert!(!model.has_page(&label_to_uuid("e4f0518a-fd0d-403e-9c20-79041c1c14ae").unwrap()));
}


