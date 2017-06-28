extern crate env_logger;

extern crate xflow;
use xflow::util::fs::*;
use xflow::structure::model::*;

#[test]
fn test_load_model() {
    let _ = env_logger::init();

    let model_a = model_from_fs(&"resource/projects/basic").unwrap();
    let json_a = model_a.to_json();
    let model_b = ModelDocument::from_json(&json_a);
    let yaml_a = model_b.to_yaml();
    let model_c = ModelDocument::from_yaml(&yaml_a);

    assert_eq!(model_a.id, model_c.id);
    assert_eq!(model_a.doc, model_c.doc);
    assert_eq!(model_a.doc.xflows.len(), model_c.doc.xflows.len());
    assert_eq!(model_a.doc.pages[0].to_json(),
               model_c.doc.pages[0].to_json());
    assert_eq!(model_a.doc.xflows[0].to_json(),
               model_c.doc.xflows[0].to_json());
}
