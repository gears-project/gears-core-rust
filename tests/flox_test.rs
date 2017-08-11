extern crate env_logger;
extern crate gears;

use gears::structure::xflow::*;
use gears::*;
use xfstate::*;

fn expect_context_integer(input: &str, context: &XFState, expected: i64) -> () {
    let _ = env_logger::init();
    match flox::parse_context(input, context) {
        Err(err) => {
            println!("Error parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                XFlowValue::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_integer(input: &str, expected: i64) -> () {
    let _ = env_logger::init();
    match flox::parse(input) {
        Err(err) => {
            println!("Error parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                XFlowValue::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_context_boolean(input: &str, context: &XFState, expected: bool) -> () {
    let _ = env_logger::init();
    match flox::parse_context(input, context) {
        Err(err) => {
            println!("Error parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                XFlowValue::Boolean(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_boolean(input: &str, expected: bool) -> () {
    let _ = env_logger::init();
    match flox::parse_boolean(input) {
        Err(err) => {
            println!("Error parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                XFlowValue::Boolean(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_flox_arithmetic() {
    let _ = env_logger::init();
    expect_integer("1+2", 3);
    expect_integer("1+2", 3);
    expect_integer("1*2", 2);
    expect_integer("1*2", 2);
    expect_integer("4/2", 2);
    expect_integer("10-2", 8);
    expect_integer("10^2", 100);
    expect_integer("3^3", 27);
    expect_integer("(3^3)", 27);
    expect_integer("((3^(3)))", 27);
    expect_integer("(3)", 3);
    expect_integer("(3) + 1", 4);
    expect_integer("(3)^1", 3);
    expect_integer("(3)^(2+1)", 27);
    expect_integer("(3+(3+3+(3+3)))", 15);
    expect_integer("(3+(5-2)+(3+3+(3+3)))", 18);
}

#[test]
fn test_flox_boolean() {
    let _ = env_logger::init();
    expect_boolean("1==1", true);
    expect_boolean("1 == 1", true);
    expect_boolean("1==2", false);
    expect_boolean("false == false", true);
    expect_boolean("true==true", true);
    expect_boolean("true ==false", false);
    expect_boolean("false== true", false);

    expect_boolean("1!=1", false);
    expect_boolean("1 != 2", true);
    expect_boolean("1 !=2", true);

    expect_boolean("1< 2", true);
    expect_boolean("1 >2", false);

    expect_boolean("1 >2", false);
    expect_boolean("1<2", true);

    expect_boolean("1 >= 2", false);
    expect_boolean("2 >=1", true);
    expect_boolean("2>= 2", true);

    expect_boolean("1 <= 2", true);
    expect_boolean("2 <= 1", false);
    expect_boolean("2<=2", true);

    expect_boolean("false", false);
    expect_boolean("true", true);

    expect_boolean("!false", true);
    expect_boolean("!true", false);
    expect_boolean("!(true)", false);

    expect_boolean("true&&true", true);
    expect_boolean("true && false", false);
    expect_boolean("false&& false", false);
    expect_boolean("false &&true", false);

    expect_boolean("true||true", true);
    expect_boolean("true || false", true);
    expect_boolean("false|| false", false);
    expect_boolean("false ||true", true);
    expect_boolean("(((false || true) && true))", true);
}

#[test]
fn test_flox_arithmetic_precedence() {
    let _ = env_logger::init();

    expect_integer("1+1+1", 3);
    expect_integer("1+2/3", 1);
}

#[test]
fn test_flox_boolean_precedence() {
    let _ = env_logger::init();

    expect_boolean("false&&false&&false", false);
    expect_boolean("false&&false||true", false);
}

#[test]
fn test_flox_atom() {
    let _ = env_logger::init();
    match flox::parse("1") {
        Err(err) => {
            println!("Error parsing result for ('{:?}') is {:?}", "1", err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                XFlowValue::Integer(res) => assert_eq!(res, 1),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_combined_expressions() {
    let _ = env_logger::init();
    expect_integer("(2)", 2);
    expect_integer("(2+2)", 4);
    expect_boolean("(2 == 2)", true);
    expect_boolean("(true && true)", true);
    expect_boolean("(true && true && true)", true);
    expect_boolean("(true && false && true)", false);
    expect_boolean("(true && false || true)", true);
    expect_boolean("true && false || true", true);
    expect_boolean("(2 == 2) || false", true);
    expect_boolean("(2 < 2) || false", false);
    expect_boolean("(1 < 2) || (5 > 2)", true);
    expect_boolean("(1 < 2) && (5 > 2)", true);
    expect_boolean("(((1 < 2) && (5 > 2)) && false)", false);
    expect_boolean("(((1 < 2) && (5 > 2)) || false)", true);
    expect_boolean("(true && ((1 < 2) && (5 > 2)))", true);
}

#[test]
// partof #TST-flox
// partof #TST-flox-variable
fn test_variables() {
    let _ = env_logger::init();
    let mut state = XFState::default();
    state.add(&XFlowVariable {
        name: "CounterValue".to_owned(),
        vtype: XFlowValueType::Integer,
        value: XFlowValue::Integer(0),
    });

    state.add(&XFlowVariable {
        name: "ComparisonValue".to_owned(),
        vtype: XFlowValueType::Boolean,
        value: XFlowValue::Boolean(true),
    });

    state.add(&XFlowVariable {
        name: "TrueValue".to_owned(),
        vtype: XFlowValueType::Boolean,
        value: XFlowValue::Boolean(true),
    });

    state.add(&XFlowVariable {
        name: "FalseValue".to_owned(),
        vtype: XFlowValueType::Boolean,
        value: XFlowValue::Boolean(false),
    });

    state.add(&XFlowVariable {
        name: "One".to_owned(),
        vtype: XFlowValueType::Integer,
        value: XFlowValue::Integer(1),
    });

    state.add(&XFlowVariable {
        name: "Two".to_owned(),
        vtype: XFlowValueType::Integer,
        value: XFlowValue::Integer(2),
    });

    expect_context_integer("$CounterValue+1", &state, 1);
    expect_context_integer("$CounterValue+99", &state, 99);
    expect_context_integer("$CounterValue-99", &state, -99);
    expect_context_integer("($One + $Two)", &state, 3);
    expect_context_integer("($One + $Two + $Two)", &state, 5);
    expect_context_integer("(($One + $Two) + $Two)", &state, 5);

    expect_context_boolean("$CounterValue > 0", &state, false);
    expect_context_boolean("$CounterValue == 0", &state, true);
    expect_context_boolean("($CounterValue == 0)", &state, true);
    expect_context_boolean("($CounterValue == 0) && true", &state, true);
    expect_context_boolean("($CounterValue == 0) || false", &state, true);
    expect_context_boolean("($CounterValue == 0) && false", &state, false);

    expect_context_boolean("$ComparisonValue==true", &state, true);
    expect_context_boolean("$ComparisonValue == true", &state, true);
    expect_context_boolean("$ComparisonValue == false", &state, false);
    expect_context_boolean("$ComparisonValue!=true", &state, false);
    expect_context_boolean("$ComparisonValue != true", &state, false);
    expect_context_boolean("$ComparisonValue != false", &state, true);
    expect_context_boolean("$ComparisonValue && true", &state, true);
    expect_context_boolean("$ComparisonValue&&true", &state, true);
    expect_context_boolean("$ComparisonValue && false", &state, false);

    expect_context_boolean("$TrueValue && $FalseValue", &state, false);
    expect_context_boolean("$TrueValue && $TrueValue && $TrueValue", &state, true);
    expect_context_boolean("(($TrueValue && $TrueValue) && $TrueValue)", &state, true);
    expect_context_boolean(
        "((($TrueValue && $TrueValue) && $TrueValue) && $TrueValue)",
        &state,
        true,
    );

    expect_context_boolean(
        "((($TrueValue && $TrueValue) && $TrueValue) && $FalseValue)",
        &state,
        false,
    );

    expect_context_boolean(
        "((($TrueValue && $TrueValue) && $TrueValue) && ($FalseValue || \
                            $TrueValue))",
        &state,
        true,
    );

    expect_context_boolean(
        "((($TrueValue && $FalseValue) && $TrueValue) && ($FalseValue || \
                            $TrueValue))",
        &state,
        false,
    );

    expect_context_boolean(
        "((($TrueValue && $FalseValue) && $TrueValue) && ($FalseValue \
                            ||\n \r \n\n $TrueValue))",
        &state,
        false,
    );

    expect_context_boolean("$TrueValue == $FalseValue", &state, false);
    expect_context_boolean("$TrueValue != $FalseValue", &state, true);
    expect_context_boolean("$TrueValue == $TrueValue", &state, true);
    expect_context_boolean("($TrueValue == $TrueValue)", &state, true);

    expect_context_boolean("!($TrueValue)", &state, false);
    expect_context_boolean("!$TrueValue", &state, false);

    expect_context_boolean("!($TrueValue == $TrueValue)", &state, false);
    expect_context_boolean("!($TrueValue != $TrueValue)", &state, true);
    expect_context_boolean("!(($TrueValue != $TrueValue) || false)", &state, true);

}

#[test]
// #TST-flox-variable-extraction
fn test_variable_extraction() {
    let _ = env_logger::init();

    match flox::extract_variable_names("$CounterValue+6") {
        Ok(res) => assert_eq!(res, vec!["CounterValue"]),
        Err(err) => {
            println!("Error {:?}", err);
            assert!(false)
        }
    }

    match flox::extract_variable_names("6+$CounterValue+6") {
        Ok(res) => assert_eq!(res, vec!["CounterValue"]),
        Err(err) => {
            println!("Error {:?}", err);
            assert!(false)
        }
    }

    match flox::extract_variable_names("6+$CounterValue+6+$CounterValue") {
        Ok(res) => assert_eq!(res, vec!["CounterValue", "CounterValue"]),
        Err(err) => {
            println!("Error {:?}", err);
            assert!(false)
        }
    }

    match flox::extract_variable_names("6+$CounterValue+6+$CounterValue / $ComparisonValue") {
        Ok(res) => assert_eq!(res, vec!["CounterValue", "CounterValue", "ComparisonValue"]),
        Err(err) => {
            println!("Error {:?}", err);
            assert!(false)
        }
    }

    match flox::extract_variable_names("1+2") {
        Ok(res) => assert_eq!(res.len(), 0),
        Err(err) => {
            println!("Error {:?}", err);
            assert!(false)
        }
    }

}

#[test]
// #TST-flox-variable-error-reporting
fn test_variable_error_reporting() {
    let _ = env_logger::init();

    match flox::parse_arithmetic("snork") {
        Ok(_) => assert!(false),
        Err(err) => {
            match err {
                flox::Error::ParseError(_) => assert!(true),
            }
        }
    }

    match flox::parse("snork") {
        Ok(_) => assert!(false),
        Err(err) => {
            match err {
                flox::Error::ParseError(_) => assert!(true),
            }
        }
    }

    match flox::parse_boolean("snork") {
        Ok(_) => assert!(false),
        Err(err) => {
            match err {
                flox::Error::ParseError(_) => assert!(true),
            }
        }
    }

    let state = XFState::default();

    match flox::parse_context("snork", &state) {
        Ok(_) => assert!(false),
        Err(err) => {
            match err {
                flox::Error::ParseError(_) => assert!(true),
            }
        }
    }

    match flox::extract_variable_names("snork") {
        Ok(_) => assert!(true),
        Err(err) => {
            println!("flox::extract_variable_names Error {:?}", err);
            match err {
                flox::Error::ParseError(_) => assert!(true),
            }
        }
    }

}
