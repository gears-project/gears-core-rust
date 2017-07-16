extern crate env_logger;

extern crate gears;

use gears::structure::xflow::*;
use gears::xfstate::*;
use gears::runtime::xfrunner::*;
use gears::runtime::dispatcher::*;
use gears::runtime::actiondispatch;

mod common;
use common::load_doc;

fn build_dispatcher<'a>() -> Dispatcher<'a> {
    let mut dispatcher = Dispatcher::default();
    let flow_receiver = actiondispatch::flow::Flow::default();
    let flox_receiver = actiondispatch::flox::Flox::default();
    dispatcher.register_receiver(XFlowNodeType::Flow, flow_receiver);
    dispatcher.register_receiver(XFlowNodeType::Flox, flox_receiver);
    dispatcher
}

fn fail_and_report_error(err: String) -> () {
    println!("fail_and_report_error : {:?}", err);
    assert!(false);
}

fn run_xflow(flow_file: &str) -> Result<XFState, String> {

    let xfs = load_doc::<XFlowDocument>(flow_file);
    let dispatcher = build_dispatcher();
    let state = XFState::default();

    match XFlowRunner::new(&xfs, &dispatcher, &state) {
        Ok(mut xfrunner) => {
            assert_eq!(xfrunner.can_run(), true);
            xfrunner.run();
            Ok(xfrunner.get_output().unwrap())
        }
        Err(err) => {
            let res = err.clone();
            fail_and_report_error(err);
            Err(res)
        }
    }

}

#[test]
fn test_run_10_steps() {
    let _ = env_logger::init();

    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/10_steps.json");
    assert_eq!(xfs.doc.nodes.len(), 10);

    let dispatcher = build_dispatcher();
    let mut state = XFState::default();

    state.add(&XFlowVariable {
                  name: "CounterValue".to_owned(),
                  vtype: XFlowValueType::Integer,
                  value: XFlowValue::Integer(0),
              });

    match XFlowRunner::new(&xfs, &dispatcher, &state) {
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
            assert_eq!(i, xfs.doc.nodes.len());
            match xfrunner
                      .get_output()
                      .unwrap()
                      .get("CounterValue")
                      .unwrap()
                      .value {
                XFlowValue::Integer(i) => assert_eq!(i, 8),
                _ => assert!(false),
            }
        }
        Err(err) => fail_and_report_error(err),
    }

}

#[test]
fn test_run_simple_branch() {
    let _ = env_logger::init();

    let xfs = load_doc::<XFlowDocument>("resource/docs/xflow/flows/branch_boolean.json");
    assert_eq!(xfs.doc.nodes.len(), 4);
    assert_eq!(xfs.doc.edges.len(), 3);
    assert_eq!(xfs.doc.branches.len(), 2);

    let dispatcher = build_dispatcher();
    let mut state = XFState::default();

    state.add(&XFlowVariable {
                  name: "MatchValue".to_owned(),
                  vtype: XFlowValueType::Boolean,
                  value: XFlowValue::Boolean(false),
              });


    match XFlowRunner::new(&xfs, &dispatcher, &state) {
        Ok(mut xfrunner) => {
            assert_eq!(xfrunner.can_run(), true);
            xfrunner.run();
            assert_eq!(xfrunner.is_completed_ok(), true);
        }
        Err(err) => fail_and_report_error(err),
    }
}

#[test]
fn test_run_arithmetic() {
    let _ = env_logger::init();

    match run_xflow("resource/docs/xflow/flows/arithmetic_addition.json") {
        Ok(output) => {
            match output.get("ReturnValue").unwrap().value {
                XFlowValue::Integer(i) => assert_eq!(i, 3),
                _ => assert!(false),
            }
        }
        Err(err) => fail_and_report_error(err),
    }
}

#[test]
fn test_run_arithmetic_multiple_return_values() {
    let _ = env_logger::init();

    match run_xflow("resource/docs/xflow/flows/arithmetic_addition_multiple_return_values.json") {
        Ok(output) => {
            match output.get("ReturnValueA").unwrap().value {
                XFlowValue::Integer(i) => assert_eq!(i, 3),
                _ => assert!(false),
            }
            match output.get("ReturnValueB").unwrap().value {
                XFlowValue::Integer(i) => assert_eq!(i, 16),
                _ => assert!(false),
            }
        }
        Err(err) => fail_and_report_error(err),
    }
}
