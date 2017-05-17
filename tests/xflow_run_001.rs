extern crate xflow;
use xflow::*;

#[cfg(test)]

mod helper;

fn read_json_file(filename: &str) -> String {
    helper::read_json_file(filename)
}

fn build_dispatcher<'a>() -> Dispatcher<'a> {
    let mut dispatcher = Dispatcher::default();
    let flow_dispatcher = actiondispatch::flow::Flow::default();
    let flox_dispatcher = actiondispatch::flox::Flox::default();
    dispatcher.register_dispatcher("flow", flow_dispatcher);
    dispatcher.register_dispatcher("flox", flox_dispatcher);
    dispatcher
}

#[test]
fn test_run_10_steps() {
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
