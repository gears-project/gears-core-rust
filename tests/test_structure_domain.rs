extern crate env_logger;

extern crate xflow;
use xflow::structure::domain::*;

mod common;
use common::load_doc;

#[test]
fn test_load_domain() {
    let _ = env_logger::init();

    let domain = load_doc::<DomainDocument>("resource/docs/domain/good/basic.json");

    assert_eq!(std::mem::size_of_val(&domain), 232);
    assert_eq!(domain.doc.events.change.len(), 0);
    assert_eq!(domain.doc.events.update.len(), 0);
    assert_eq!(domain.doc.events.read.len(), 0);
    assert_eq!(domain.doc.events.delete.len(), 0);
    assert_eq!(domain.doc.events.all.len(), 1);

}
