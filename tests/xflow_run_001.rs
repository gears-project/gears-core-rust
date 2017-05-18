extern crate env_logger;

extern crate xflow;
use xflow::*;

mod helper;

fn read_json_file(filename: &str) -> String {
    helper::read_json_file(filename)
}

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
    let mut xfrunner = XFlowRunner::new(&xfs, &dispatcher);

    assert_eq!(xfrunner.can_run(), true);

    let mut i = 1;

    loop {
        if !xfrunner.step() {
            break;
        }
        i += 1;
    }

    assert_eq!(i, xfs.nodes.len());
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
}
