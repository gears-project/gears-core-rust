extern crate env_logger;

extern crate xflow;

use xflow::xfstruct::*;
use xflow::xfstate::*;
use xflow::xfrunner::*;
use xflow::dispatcher::*;
use xflow::actiondispatch;

mod helper;
use helper::read_json_file;

fn build_dispatcher<'a>() -> Dispatcher<'a> {
    let mut dispatcher = Dispatcher::default();
    let flow_receiver = actiondispatch::flow::Flow::default();
    let flox_receiver = actiondispatch::flox::Flox::default();
    dispatcher.register_receiver("flow", flow_receiver);
    dispatcher.register_receiver("flox", flox_receiver);
    dispatcher
}

#[test]
fn test_run_10_steps() {
    let _ = env_logger::init();

    let json_string = read_json_file("data/flows/10_steps.json");
    let xfs = XFlowStruct::from_json(&json_string);
    assert_eq!(xfs.nodes.len(), 10);

    let dispatcher = build_dispatcher();

    let state = XFState::default();

    match XFlowRunner::new_with_input(&xfs, &dispatcher, &state) {
        Ok(mut xfrunner) => {
            assert_eq!(xfrunner.can_run(), true);

            let mut i = 1;

            loop {
                xfrunner.step();
                if xfrunner.is_completed() {
                    break;
                }
                i += 1;
            }
            assert_eq!(i, xfs.nodes.len());
        }
        Err(_) => assert!(false),
    }

}

#[test]
fn test_run_simple_branch() {
    let _ = env_logger::init();

    let json_string = read_json_file("data/flows/branch_boolean.json");
    let xfs = XFlowStruct::from_json(&json_string);
    assert_eq!(xfs.nodes.len(), 4);
    assert_eq!(xfs.edges.len(), 3);
    assert_eq!(xfs.branches.len(), 2);

    let dispatcher = build_dispatcher();
    let mut xfrunner = XFlowRunner::new(&xfs, &dispatcher);

    assert_eq!(xfrunner.can_run(), true);

    xfrunner.run();

    assert_eq!(xfrunner.is_completed_ok(), true);
}

#[test]
fn test_run_arithmetic() {
    let _ = env_logger::init();

    let json_string = read_json_file("data/flows/arithmetic_addition.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let dispatcher = build_dispatcher();
    let mut xfrunner = XFlowRunner::new(&xfs, &dispatcher);

    xfrunner.run();

    assert_eq!(xfrunner.is_completed_ok(), true);

    match xfrunner.get_output().unwrap().get("ReturnValue").unwrap().value {
        XFlowValue::Integer(i) => assert_eq!(i, 3),
        _ => assert!(false),
    }
}

#[test]
fn test_run_arithmetic_multiple_return_values() {
    let _ = env_logger::init();

    let json_string = read_json_file("data/flows/arithmetic_addition_multiple_return_values.json");
    let xfs = XFlowStruct::from_json(&json_string);

    let dispatcher = build_dispatcher();
    let mut xfrunner = XFlowRunner::new(&xfs, &dispatcher);

    xfrunner.run();

    assert_eq!(xfrunner.is_completed_ok(), true);

    match xfrunner.get_output() {
        Ok(xfstate) => {
            match xfstate.get("ReturnValueA").unwrap().value {
                XFlowValue::Integer(i) => assert_eq!(i, 3),
                _ => assert!(false),
            }
            match xfstate.get("ReturnValueB").unwrap().value {
                XFlowValue::Integer(i) => assert_eq!(i, 16),
                _ => assert!(false),
            }
        }
        Err(err) => {
            println!("{:?}", err);
            assert!(false);
        }
    }

}
