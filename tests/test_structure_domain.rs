extern crate env_logger;

extern crate xflow;
use xflow::structure::domain::*;

mod helper;
use helper::read_json_file;

#[test]
fn test_load_domain() {
    let _ = env_logger::init();

    use std;
    let json_string = read_json_file("resource/docs/domain/good/basic.json");
    let domain = DomainDocument::from_json(&json_string);

    println!("Loaded domain document : {:?}", domain);

    assert_eq!(std::mem::size_of_val(&domain), 232);
    assert_eq!(domain.doc.events.change.len(), 0);
    assert_eq!(domain.doc.events.update.len(), 0);
    assert_eq!(domain.doc.events.read.len(), 0);
    assert_eq!(domain.doc.events.delete.len(), 0);
    assert_eq!(domain.doc.events.all.len(), 1);

}
