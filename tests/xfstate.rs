extern crate gears;
use gears::structure::xflow::*;
use gears::runtime::xfstate::*;

#[cfg(test)]
#[test]
fn test_store_has() {
    let mut xfstate = XFState::default();
    let xvar = XFlowVariable {
        name: "number1".to_string(),
        vtype: XFlowValueType::String,
        value: XFlowValue::String("number1".to_owned()),
    };

    assert_eq!(xfstate.len(), 0);
    assert_eq!(xfstate.is_empty(), true);

    xfstate.add(&xvar);

    assert_eq!(xfstate.len(), 1);
    assert_eq!(xfstate.is_empty(), false);

    assert_eq!(xfstate.has("number1"), true);

    match xfstate.get("number1") {
        Some(res) => assert_eq!(res.name, "number1"),
        None => println!("Error, number1 not found in xfstate"),
    }

}

#[test]
fn test_store_add_and_remove() {
    let mut xfstate = XFState::default();
    let xvar = XFlowVariable {
        name: "number1".to_string(),
        vtype: XFlowValueType::String,
        value: XFlowValue::String("number1".to_owned()),
    };

    xfstate.add(&xvar);
    assert_eq!(xfstate.has("number1"), true);

    xfstate.remove("number1");

    assert_eq!(xfstate.has("number1"), false);
    assert_eq!(xfstate.is_empty(), true);
}
