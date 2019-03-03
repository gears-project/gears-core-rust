use gluon::base::types::ArcType;

use gluon::vm::api::{ ActiveThread, Getable, Pushable, ValueRef, VmType };
use gluon::vm::{self, Variants};
use gluon::Thread;

use uuid::Uuid;
use super::common::DocumentReference;

impl VmType for DocumentReference {
    type Type = Self;

    fn make_type(vm: &Thread) -> ArcType {

        vm
            .find_type_info("gears.common.documentreference")
            .expect("Could not find type")
            .into_type()

    }
}

impl<'vm> Pushable<'vm> for DocumentReference {
    fn push(self, ctx: &mut ActiveThread<'vm>) -> vm::Result<()> {
        (record! {
            id => self.id.to_hyphenated().to_string(),
        }).push(ctx)
    }
}

impl<'vm, 'value> Getable<'vm, 'value> for DocumentReference {
    fn from_value(vm: &'vm Thread, data: Variants<'value>) -> DocumentReference {
        let data = match data.as_ref() {
            ValueRef::Data(data) => data,
            _ => panic!("Value is not a complex type"),
        };

        let s = String::from_value(vm, data.lookup_field(vm, "id").unwrap());

        DocumentReference {
            id: Uuid::parse_str(&s).unwrap()
        }
    }
}


