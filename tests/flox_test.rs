extern crate xflow;
use xflow::*;


fn expect_integer(input: &str, expected: i64) -> () {
    match flox::parse_arithmetic(input) {
        Err(err) => {
            println!("RES for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            println!("RES for ('{:?}') is {:?}", input, res);
            match res {
                flox::FloxResult::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_boolean(input: &str, expected: bool) -> () {
    match flox::parse_boolean(input) {
        Err(err) => {
            println!("RES for ('{:?}') is {:?}", input, err);
            assert!(false);
        }
        Ok(res) => {
            println!("RES for ('{:?}') is {:?}", input, res);
            match res {
                flox::FloxResult::Boolean(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_flox_arithmetic() {
    expect_integer("1+2", 3);
    expect_integer("1 + 2", 3);
    // expect_integer("1+2+3", 6);
    expect_integer("1*2", 2);
    expect_integer("1 * 2", 2);
    expect_integer("11+255", 266);
    expect_integer("11 + 255", 266);
}

#[test]
fn test_flox_boolean() {
    expect_boolean("1==1", true);
    expect_boolean("1 == 1", true);
    expect_boolean("1==2", false);
    expect_boolean("false == false", true);
    expect_boolean("true == true", true);
    expect_boolean("true == false", false);
    expect_boolean("false == true", false);

    expect_boolean("1!=1", false);
    expect_boolean("1!=2", true);
    expect_boolean("1 != 2", true);

    expect_boolean("1 < 2", true);
    expect_boolean("1 > 2", false);

    expect_boolean("1 > 2", false);
    expect_boolean("1 < 2", true);

    expect_boolean("1 >= 2", false);
    expect_boolean("2 >= 1", true);
    expect_boolean("2 >= 2", true);

    expect_boolean("1 <= 2", true);
    expect_boolean("2 <= 1", false);
    expect_boolean("2 <= 2", true);

    // expect_boolean("false", false);
    // expect_boolean("true", true);

    expect_boolean("!false", true);
    expect_boolean("!true", false);

    expect_boolean("true && true", true);
    expect_boolean("true && false", false);
    expect_boolean("false && false", false);
    expect_boolean("false && true", false);

    expect_boolean("true || true", true);
    expect_boolean("true || false", true);
    expect_boolean("false || false", false);
    expect_boolean("false || true", true);
}


#[test]
fn test_combined_expressions() {
    match flox::parse("1+2") {
        Err(_) => assert!(false),
        Ok(res) => {
            match res {
                flox::FloxResult::Integer(res) => assert_eq!(res, 3),
                _ => assert!(false),
            }
        }
    }
}
