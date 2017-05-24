extern crate env_logger;
extern crate xflow;
use xflow::*;


fn expect_integer(input: &str, expected: i64) -> () {
    let _ = env_logger::init();
    match flox::parse_arithmetic(input) {
        Err(err) => {
            println!("Parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            println!("Parsing result for ('{:?}') is {:?}", input, res);
            match res {
                xfstruct::XFlowValue::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_boolean(input: &str, expected: bool) -> () {
    let _ = env_logger::init();
    match flox::parse_boolean(input) {
        Err(err) => {
            println!("Parsing result for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            println!("Parsing result for ('{:?}') is {:?}", input, res);
            match res {
                xfstruct::XFlowValue::Boolean(res) => assert_eq!(res, expected),
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

    expect_boolean("true&&true", true);
    expect_boolean("true && false", false);
    expect_boolean("false&& false", false);
    expect_boolean("false &&true", false);

    expect_boolean("true||true", true);
    expect_boolean("true || false", true);
    expect_boolean("false|| false", false);
    expect_boolean("false ||true", true);
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
            println!("Parsing result for ('{:?}') is {:?}", "1", err);
            assert!(false);
        }
        Ok(res) => {
            match res {
                xfstruct::XFlowValue::Integer(res) => assert_eq!(res, 1),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_combined_expressions() {
    let _ = env_logger::init();
    // expect_integer("(2)", 2);
}
