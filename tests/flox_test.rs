extern crate xflow;
use xflow::*;

fn expect_ok_integer(input: &str, expected: i64) -> () {
    match flox::parse(input) {
        Err(_) => assert!(false),
        Ok(res) => {
            match res {
                flox::FloxResult::Integer(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

fn expect_ok_boolean(input: &str, expected: bool) -> () {
    match flox::parse_boolean(input) {
        Err(err) => {
            println!("RES is {:?}", err);
            assert!(false);
        }
        Ok(res) => {
            println!("RES is {:?}", res);
            match res {
                flox::FloxResult::Boolean(res) => assert_eq!(res, expected),
                _ => assert!(false),
            }
        }
    }
}

#[test]
fn test_flox_arithmetic() {
    expect_ok_integer("1+2", 3);
    expect_ok_integer("1 + 2", 3);
    // expect_ok_integer("1+2+3", 6);
    expect_ok_integer("1*2", 2);
    expect_ok_integer("1 * 2", 2);
    expect_ok_integer("11+255", 266);
    expect_ok_integer("11 + 255", 266);
}

#[test]
fn test_flox_boolean() {
    expect_ok_boolean("1==1", true);
    expect_ok_boolean("1 == 1", true);
    expect_ok_boolean("1==2", false);

    expect_ok_boolean("1!=1", false);
    expect_ok_boolean("1!=2", true);
    expect_ok_boolean("1 != 2", true);

    expect_ok_boolean("1 < 2", true);
    expect_ok_boolean("1 > 2", false);

    expect_ok_boolean("1 > 2", false);
    expect_ok_boolean("1 < 2", true);

    expect_ok_boolean("1 >= 2", false);
    expect_ok_boolean("2 >= 1", true);
    expect_ok_boolean("2 >= 2", true);

    expect_ok_boolean("1 <= 2", true);
    expect_ok_boolean("2 <= 1", false);
    expect_ok_boolean("2 <= 2", true);

}
